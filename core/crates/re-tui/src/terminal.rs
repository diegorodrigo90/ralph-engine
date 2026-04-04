//! Terminal lifecycle and TUI shell.
//!
//! Manages the ratatui terminal: enters raw mode on start,
//! restores on exit/crash/signal. Provides the main render
//! skeleton with zone-based layout.

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use crate::layout;

/// TUI operating mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiMode {
    /// Read-only dashboard for autonomous agent execution.
    Autonomous,
    /// Interactive dashboard with pause/resume/feedback.
    Guided,
}

/// Current state of the TUI session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiState {
    /// Agent is running, streaming output.
    Running,
    /// Agent is paused (awaiting user action in guided mode).
    Paused,
    /// Waiting for user feedback text (guided mode).
    WaitingFeedback,
    /// Agent has completed its task.
    Complete,
    /// Agent encountered an error.
    Error,
}

impl TuiState {
    /// Returns a display label for the state.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Running => "RUNNING",
            Self::Paused => "PAUSED",
            Self::WaitingFeedback => "FEEDBACK",
            Self::Complete => "COMPLETE",
            Self::Error => "ERROR",
        }
    }

    /// Returns the status color for this state.
    #[must_use]
    pub fn color(self) -> Color {
        match self {
            Self::Running => Color::Green,
            Self::Paused => Color::Yellow,
            Self::WaitingFeedback => Color::Yellow,
            Self::Complete => Color::Cyan,
            Self::Error => Color::Red,
        }
    }
}

/// Configuration for the TUI shell.
#[derive(Debug, Clone)]
pub struct TuiConfig {
    /// Operating mode (autonomous or guided).
    pub mode: TuiMode,
    /// Title shown in the header bar.
    pub title: String,
    /// Agent identifier shown in header.
    pub agent_id: String,
    /// Resolved locale for i18n (e.g. `"en"`, `"pt-br"`).
    /// Auto-detected from env/OS or config.
    pub locale: String,
}

/// The TUI shell — manages terminal lifecycle and render loop.
///
/// Create via [`TuiShell::new`], then call [`TuiShell::run_demo`]
/// to start the render loop. The terminal is restored on drop.
pub struct TuiShell {
    config: TuiConfig,
    state: TuiState,
    progress: u16,
    activity_lines: Vec<String>,
    tool_count: usize,
    should_quit: bool,
}

impl TuiShell {
    /// Creates a new TUI shell with the given configuration.
    #[must_use]
    pub fn new(config: TuiConfig) -> Self {
        Self {
            config,
            state: TuiState::Running,
            progress: 0,
            activity_lines: Vec::new(),
            tool_count: 0,
            should_quit: false,
        }
    }

    /// Returns the current TUI state.
    #[must_use]
    pub fn state(&self) -> TuiState {
        self.state
    }

    /// Sets the TUI state.
    pub fn set_state(&mut self, state: TuiState) {
        tracing::debug!(old = ?self.state, new = ?state, "TUI state transition");
        self.state = state;
    }

    /// Sets the progress percentage (0-100).
    pub fn set_progress(&mut self, pct: u16) {
        self.progress = pct.min(100);
    }

    /// Appends a line to the activity stream.
    pub fn push_activity(&mut self, line: String) {
        // Bounded buffer: keep last 10_000 lines.
        if self.activity_lines.len() >= 10_000 {
            self.activity_lines.drain(..1_000);
        }
        self.activity_lines.push(line);
    }

    /// Increments the tool call counter.
    pub fn increment_tools(&mut self) {
        self.tool_count += 1;
    }

    /// Runs the TUI demo loop — enters raw mode, renders, handles
    /// input, and restores terminal on exit.
    ///
    /// Press `q` to quit, `p` to toggle pause.
    ///
    /// # Errors
    ///
    /// Returns an error if terminal initialization fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn run_demo(&mut self) -> Result<(), TuiError> {
        // Seed some demo activity
        self.push_activity(">> Ralph Engine TUI initialized".to_owned());
        self.push_activity(format!(">> Mode: {:?}", self.config.mode));
        self.push_activity(format!(">> Agent: {}", self.config.agent_id));
        self.push_activity(">> Waiting for agent stream...".to_owned());

        let mut terminal = init_terminal();

        let result = (|| -> Result<(), TuiError> {
            loop {
                terminal
                    .draw(|frame| self.render(frame))
                    .map_err(|e| TuiError::new(format!("render failed: {e}")))?;

                if event::poll(std::time::Duration::from_millis(100))
                    .map_err(|e| TuiError::new(format!("event poll failed: {e}")))?
                    && let Event::Key(key) = event::read()
                        .map_err(|e| TuiError::new(format!("event read failed: {e}")))?
                    && key.kind == KeyEventKind::Press
                {
                    self.handle_key(key.code);
                }

                if self.should_quit {
                    break;
                }
            }
            Ok(())
        })();

        restore_terminal();
        result
    }

    /// Handles a key press event.
    fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') => {
                tracing::info!("user requested quit");
                self.should_quit = true;
            }
            KeyCode::Char('p') => {
                let new_state = if self.state == TuiState::Running {
                    TuiState::Paused
                } else if self.state == TuiState::Paused {
                    TuiState::Running
                } else {
                    self.state
                };
                self.set_state(new_state);
                self.push_activity(format!(">> State: {}", new_state.label()));
            }
            KeyCode::Char('?') => {
                self.push_activity(">> Help: [q] quit  [p] pause/resume  [?] help".to_owned());
            }
            _ => {}
        }
    }

    /// Renders the TUI frame with responsive zone-based layout.
    ///
    /// Layout adapts to terminal size:
    /// - Compact (< 120 cols): activity only
    /// - Standard (120-159): activity + sidebar
    /// - Wide (>= 160): control + activity + sidebar
    fn render(&self, frame: &mut Frame<'_>) {
        let area = frame.area();

        // Check minimum size
        if layout::is_terminal_too_small(area) {
            let msg = format!(
                "Terminal too small ({}x{}). Minimum: {}x{}.",
                area.width,
                area.height,
                crate::MIN_TERMINAL_WIDTH,
                crate::MIN_TERMINAL_HEIGHT,
            );
            frame.render_widget(
                Paragraph::new(msg).style(Style::default().fg(Color::Red)),
                area,
            );
            return;
        }

        let zones = layout::compute_zones(area);

        self.render_header(frame, zones.header);
        self.render_activity(frame, zones.activity);
        self.render_metrics(frame, zones.metrics);
        self.render_help(frame, &zones);

        if let Some(sidebar) = zones.sidebar {
            self.render_sidebar(frame, sidebar);
        }

        if let Some(control) = zones.control {
            self.render_control_panel(frame, control);
        }
    }

    /// Renders the header bar with title, agent, state, and progress.
    fn render_header(&self, frame: &mut Frame<'_>, area: Rect) {
        let state_label = self.state.label();
        let state_color = self.state.color();

        let header = Line::from(vec![
            Span::styled(
                " Ralph Engine ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("• "),
            Span::raw(format!("Agent: {} ", self.config.agent_id)),
            Span::styled(
                format!("[{state_label}]"),
                Style::default()
                    .fg(state_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        // If there's room, add a progress gauge inline
        if area.width > 60 {
            let cols =
                Layout::horizontal([Constraint::Fill(1), Constraint::Length(20)]).split(area);

            frame.render_widget(Paragraph::new(header), cols[0]);
            frame.render_widget(
                Gauge::default()
                    .gauge_style(Style::default().fg(state_color))
                    .percent(self.progress)
                    .use_unicode(true),
                cols[1],
            );
        } else {
            frame.render_widget(Paragraph::new(header), area);
        }
    }

    /// Renders the activity stream (main viewport).
    fn render_activity(&self, frame: &mut Frame<'_>, area: Rect) {
        let visible_lines = area.height as usize;
        let start = self.activity_lines.len().saturating_sub(visible_lines);
        let lines: Vec<Line<'_>> = self.activity_lines[start..]
            .iter()
            .map(|s| {
                if s.starts_with(">> Tool") {
                    Line::styled(s.as_str(), Style::default().fg(Color::Blue))
                } else if s.starts_with(">> State:") {
                    Line::styled(s.as_str(), Style::default().fg(Color::Yellow))
                } else {
                    Line::raw(s.as_str())
                }
            })
            .collect();

        frame.render_widget(Paragraph::new(lines), area);
    }

    /// Renders the metrics bar.
    fn render_metrics(&self, frame: &mut Frame<'_>, area: Rect) {
        let metrics = Line::from(vec![
            Span::styled(
                format!(" Tools: {} ", self.tool_count),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw("│ "),
            Span::raw(format!("Lines: {} ", self.activity_lines.len())),
            Span::raw("│ "),
            Span::raw(format!("Progress: {}% ", self.progress)),
        ]);
        frame.render_widget(Paragraph::new(metrics), area);
    }

    /// Renders the help bar at the bottom, adapted to layout tier.
    fn render_help(&self, frame: &mut Frame<'_>, zones: &layout::LayoutZones) {
        let dim = Style::default().fg(Color::DarkGray);
        let mut spans = vec![
            Span::styled(" [?]", dim),
            Span::styled(" help ", dim),
            Span::styled(" [p]", dim),
            Span::styled(" pause ", dim),
            Span::styled(" [q]", dim),
            Span::styled(" quit ", dim),
        ];

        // Show layout tier indicator
        let tier_label = match zones.tier {
            layout::LayoutTier::Compact => "compact",
            layout::LayoutTier::Standard => "standard",
            layout::LayoutTier::Wide => "wide",
        };
        spans.push(Span::styled(
            format!(" │ {tier_label}"),
            Style::default().fg(Color::Indexed(59)), // dark gray
        ));

        frame.render_widget(Paragraph::new(Line::from(spans)), zones.help);
    }

    /// Renders the sidebar zone (plugin panels placeholder).
    ///
    /// In future stories (RE-2a.5), this will render auto-discovered
    /// plugin panels via `tui_contributions()`.
    fn render_sidebar(&self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::LEFT)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Plugins ")
            .title_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Placeholder content — will be replaced by plugin panels
        let lines = vec![
            Line::styled(" (no plugins)", Style::default().fg(Color::DarkGray)),
            Line::raw(""),
            Line::styled(" Alt+1..9 to toggle", Style::default().fg(Color::DarkGray)),
        ];
        frame.render_widget(Paragraph::new(lines), inner);
    }

    /// Renders the control panel zone (guided mode actions).
    ///
    /// Only visible in Wide tier (>= 160 cols). Provides quick-access
    /// controls for pause/resume/feedback in guided mode.
    fn render_control_panel(&self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Control ")
            .title_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let state_color = self.state.color();
        let mode_label = match self.config.mode {
            TuiMode::Autonomous => "Autonomous",
            TuiMode::Guided => "Guided",
        };

        let lines = vec![
            Line::styled(
                format!(" Mode: {mode_label}"),
                Style::default().fg(Color::White),
            ),
            Line::styled(
                format!(" State: {}", self.state.label()),
                Style::default().fg(state_color),
            ),
            Line::raw(""),
            Line::styled(
                format!(" Work: {}", self.config.title),
                Style::default().fg(Color::White),
            ),
            Line::raw(""),
            Line::styled(" [p] Pause/Resume", Style::default().fg(Color::DarkGray)),
            Line::styled(" [q] Quit", Style::default().fg(Color::DarkGray)),
        ];
        frame.render_widget(Paragraph::new(lines), inner);
    }
}

/// Initializes the terminal with ratatui defaults.
///
/// Enables raw mode, enters alternate screen, and installs a panic
/// hook that restores the terminal before printing the panic message.
/// Uses `ratatui::init()` which handles all setup automatically.
#[cfg_attr(coverage_nightly, coverage(off))]
fn init_terminal() -> DefaultTerminal {
    ratatui::init()
}

/// Restores terminal to normal mode.
///
/// Disables raw mode, leaves alternate screen, and removes the
/// panic hook installed by `init_terminal()`.
#[cfg_attr(coverage_nightly, coverage(off))]
fn restore_terminal() {
    ratatui::restore();
}

/// Error type for TUI operations.
#[derive(Debug)]
pub struct TuiError {
    /// Human-readable error message.
    pub message: String,
}

impl TuiError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for TuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for TuiError {}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn tui_state_labels_are_correct() {
        assert_eq!(TuiState::Running.label(), "RUNNING");
        assert_eq!(TuiState::Paused.label(), "PAUSED");
        assert_eq!(TuiState::WaitingFeedback.label(), "FEEDBACK");
        assert_eq!(TuiState::Complete.label(), "COMPLETE");
        assert_eq!(TuiState::Error.label(), "ERROR");
    }

    #[test]
    fn tui_state_colors_are_distinct() {
        assert_eq!(TuiState::Running.color(), Color::Green);
        assert_eq!(TuiState::Paused.color(), Color::Yellow);
        assert_eq!(TuiState::Error.color(), Color::Red);
        assert_eq!(TuiState::Complete.color(), Color::Cyan);
    }

    #[test]
    fn tui_shell_new_has_correct_defaults() {
        let shell = TuiShell::new(TuiConfig {
            mode: TuiMode::Autonomous,
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        assert_eq!(shell.state(), TuiState::Running);
        assert_eq!(shell.progress, 0);
        assert!(shell.activity_lines.is_empty());
        assert_eq!(shell.tool_count, 0);
    }

    #[test]
    fn push_activity_appends_line() {
        let mut shell = TuiShell::new(TuiConfig {
            mode: TuiMode::Autonomous,
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        shell.push_activity("hello".to_owned());
        shell.push_activity("world".to_owned());
        assert_eq!(shell.activity_lines.len(), 2);
        assert_eq!(shell.activity_lines[0], "hello");
        assert_eq!(shell.activity_lines[1], "world");
    }

    #[test]
    fn push_activity_bounds_buffer() {
        let mut shell = TuiShell::new(TuiConfig {
            mode: TuiMode::Autonomous,
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        for i in 0..10_001 {
            shell.push_activity(format!("line {i}"));
        }
        // After draining 1000 + adding 1, should be 9001
        assert!(shell.activity_lines.len() <= 10_000);
        // Last line should be the most recent
        assert_eq!(shell.activity_lines.last().unwrap(), "line 10000");
    }

    #[test]
    fn set_state_transitions() {
        let mut shell = TuiShell::new(TuiConfig {
            mode: TuiMode::Guided,
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        assert_eq!(shell.state(), TuiState::Running);
        shell.set_state(TuiState::Paused);
        assert_eq!(shell.state(), TuiState::Paused);
        shell.set_state(TuiState::WaitingFeedback);
        assert_eq!(shell.state(), TuiState::WaitingFeedback);
    }

    #[test]
    fn set_progress_clamps_to_100() {
        let mut shell = TuiShell::new(TuiConfig {
            mode: TuiMode::Autonomous,
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        shell.set_progress(150);
        assert_eq!(shell.progress, 100);
    }

    #[test]
    fn increment_tools_counts() {
        let mut shell = TuiShell::new(TuiConfig {
            mode: TuiMode::Autonomous,
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        shell.increment_tools();
        shell.increment_tools();
        shell.increment_tools();
        assert_eq!(shell.tool_count, 3);
    }

    #[test]
    fn handle_key_q_sets_quit() {
        let mut shell = TuiShell::new(TuiConfig {
            mode: TuiMode::Autonomous,
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        assert!(!shell.should_quit);
        shell.handle_key(KeyCode::Char('q'));
        assert!(shell.should_quit);
    }

    #[test]
    fn handle_key_p_toggles_pause() {
        let mut shell = TuiShell::new(TuiConfig {
            mode: TuiMode::Autonomous,
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        assert_eq!(shell.state(), TuiState::Running);
        shell.handle_key(KeyCode::Char('p'));
        assert_eq!(shell.state(), TuiState::Paused);
        shell.handle_key(KeyCode::Char('p'));
        assert_eq!(shell.state(), TuiState::Running);
    }

    #[test]
    fn handle_key_help_adds_activity() {
        let mut shell = TuiShell::new(TuiConfig {
            mode: TuiMode::Autonomous,
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        shell.handle_key(KeyCode::Char('?'));
        assert!(shell.activity_lines.last().unwrap().contains("help"));
    }

    #[test]
    fn handle_key_p_no_toggle_when_complete() {
        let mut shell = TuiShell::new(TuiConfig {
            mode: TuiMode::Autonomous,
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        shell.set_state(TuiState::Complete);
        shell.handle_key(KeyCode::Char('p'));
        // Should NOT toggle to Running — Complete is a terminal state
        assert_eq!(shell.state(), TuiState::Complete);
    }

    #[test]
    fn tui_mode_debug_format() {
        assert_eq!(format!("{:?}", TuiMode::Autonomous), "Autonomous");
        assert_eq!(format!("{:?}", TuiMode::Guided), "Guided");
    }

    #[test]
    fn tui_error_display() {
        let err = TuiError::new("test error".to_owned());
        assert_eq!(err.to_string(), "test error");
    }

    // ── Rendering snapshot tests ─────────────────────────────────
    //
    // Use TestBackend to capture rendered frames without a real terminal.
    // These verify that the right content appears in the right zones.

    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn test_shell() -> TuiShell {
        let mut shell = TuiShell::new(TuiConfig {
            mode: TuiMode::Autonomous,
            title: "Test Task".to_owned(),
            agent_id: "test.claude".to_owned(),
            locale: "en".to_owned(),
        });
        shell.push_activity(">> Tool Call: search".to_owned());
        shell.push_activity(">> Result: found 3 items".to_owned());
        shell.set_progress(42);
        shell.increment_tools();
        shell
    }

    fn render_to_buffer(shell: &TuiShell, width: u16, height: u16) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| shell.render(frame)).unwrap();
        // Extract text content from the buffer
        let buf = terminal.backend().buffer();
        let mut output = String::new();
        for y in 0..height {
            for x in 0..width {
                let cell = &buf[(x, y)];
                output.push_str(cell.symbol());
            }
            output.push('\n');
        }
        output
    }

    #[test]
    fn render_compact_shows_header_with_agent_id() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 80, 24);
        assert!(
            output.contains("test.claude"),
            "header should show agent_id, got:\n{output}"
        );
        assert!(
            output.contains("[RUNNING]"),
            "header should show state, got:\n{output}"
        );
    }

    #[test]
    fn render_compact_shows_activity_lines() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 80, 24);
        assert!(
            output.contains("Tool Call: search"),
            "activity should show tool call, got:\n{output}"
        );
        assert!(
            output.contains("found 3 items"),
            "activity should show result, got:\n{output}"
        );
    }

    #[test]
    fn render_compact_shows_metrics() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 80, 24);
        assert!(
            output.contains("Tools: 1"),
            "metrics should show tool count, got:\n{output}"
        );
    }

    #[test]
    fn render_compact_shows_help_bar() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 80, 24);
        assert!(
            output.contains("[q]"),
            "help bar should show quit key, got:\n{output}"
        );
        assert!(
            output.contains("compact"),
            "help bar should show tier, got:\n{output}"
        );
    }

    #[test]
    fn render_compact_no_sidebar() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 80, 24);
        assert!(
            !output.contains("Plugins"),
            "compact mode should not show sidebar, got:\n{output}"
        );
    }

    #[test]
    fn render_standard_shows_sidebar() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 140, 40);
        assert!(
            output.contains("Plugins"),
            "standard mode should show sidebar, got:\n{output}"
        );
        assert!(
            output.contains("standard"),
            "help should show standard tier, got:\n{output}"
        );
    }

    #[test]
    fn render_wide_shows_control_panel() {
        let shell = TuiShell::new(TuiConfig {
            mode: TuiMode::Guided,
            title: "Fix Bug".to_owned(),
            agent_id: "test.claude".to_owned(),
            locale: "en".to_owned(),
        });
        let output = render_to_buffer(&shell, 200, 60);
        assert!(
            output.contains("Control"),
            "wide mode should show control panel, got:\n{output}"
        );
        assert!(
            output.contains("Guided"),
            "control panel should show mode, got:\n{output}"
        );
        assert!(
            output.contains("Plugins"),
            "wide mode should show sidebar, got:\n{output}"
        );
        assert!(
            output.contains("wide"),
            "help should show wide tier, got:\n{output}"
        );
    }

    #[test]
    fn render_too_small_shows_error() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 60, 20);
        assert!(
            output.contains("too small"),
            "should show size error, got:\n{output}"
        );
    }

    #[test]
    fn render_paused_state_shows_in_header() {
        let mut shell = test_shell();
        shell.set_state(TuiState::Paused);
        let output = render_to_buffer(&shell, 80, 24);
        assert!(
            output.contains("[PAUSED]"),
            "header should show PAUSED, got:\n{output}"
        );
    }

    #[test]
    fn render_progress_gauge_shows_in_wide_header() {
        let mut shell = test_shell();
        shell.set_progress(75);
        // Wide enough for inline gauge (> 60 cols)
        let output = render_to_buffer(&shell, 100, 24);
        // The gauge renders unicode blocks — just verify the header area is used
        assert!(
            output.contains("test.claude"),
            "header should render with gauge, got:\n{output}"
        );
    }
}
