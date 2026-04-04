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
/// A sidebar panel provided by a plugin, ready to render.
///
/// This is a render-ready snapshot of a plugin's `TuiPanel` — the
/// TUI shell receives these from the CLI layer which collects them
/// via auto-discovery.
#[derive(Debug, Clone)]
pub struct SidebarPanel {
    /// Panel title (localized).
    pub title: String,
    /// Content lines to render.
    pub lines: Vec<String>,
    /// Source plugin ID (for attribution).
    pub plugin_id: String,
}

/// Agent process ID (set when a real agent is launched).
/// Used by pause/resume to send `SIGSTOP`/`SIGCONT`.
pub type AgentPid = Option<u32>;

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
    /// Whether we're waiting for quit confirmation.
    quit_pending: bool,
    /// Plugin-contributed sidebar panels (from auto-discovery).
    sidebar_panels: Vec<SidebarPanel>,
    /// Agent process ID for pause/resume signals.
    agent_pid: AgentPid,
    /// Feedback text buffer (filled during `WaitingFeedback` state).
    feedback_buffer: String,
    /// Completed feedback ready to be consumed by the caller.
    pending_feedback: Option<String>,
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
            quit_pending: false,
            sidebar_panels: Vec::new(),
            agent_pid: None,
            feedback_buffer: String::new(),
            pending_feedback: None,
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

    /// Sets the sidebar panels from auto-discovered plugin contributions.
    pub fn set_sidebar_panels(&mut self, panels: Vec<SidebarPanel>) {
        self.sidebar_panels = panels;
    }

    /// Sets the agent process ID for pause/resume signal delivery.
    pub fn set_agent_pid(&mut self, pid: u32) {
        self.agent_pid = Some(pid);
    }

    /// Pushes the startup banner with config details into the activity
    /// stream. The logo image is rendered separately by the caller
    /// The logo is rendered inline in the activity stream via the logo module.
    pub fn push_startup_banner(&mut self) {
        self.push_activity(String::new());
        self.push_activity(format!("  ◎ Ralph Engine v{}", env!("CARGO_PKG_VERSION")));
        self.push_activity(format!("  Agent:   {}", self.config.agent_id));
        self.push_activity(format!("  Work:    {}", self.config.title));
        self.push_activity(format!("  Plugins: {} panels", self.sidebar_panels.len()));
        self.push_activity(String::new());
        self.push_activity("  Initializing...".to_owned());
        self.push_activity(String::new());
    }

    /// Whether the TUI is waiting for quit confirmation.
    #[must_use]
    pub fn is_quit_pending(&self) -> bool {
        self.quit_pending
    }

    /// Whether the TUI should exit.
    #[must_use]
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Processes a normalized agent event, updating TUI state and activity.
    ///
    /// This is the main integration point between the stream-json parser
    /// and the TUI display. Call this for each [`crate::events::AgentEvent`] received
    /// from the agent's stdout.
    pub fn process_event(&mut self, event: &crate::events::AgentEvent) {
        use crate::events::AgentEvent;

        match event {
            AgentEvent::TextDelta(_) => {
                // Text deltas are appended as-is to the activity stream.
                self.push_activity(event.activity_line());
            }
            AgentEvent::ToolUse { .. } => {
                self.increment_tools();
                self.push_activity(event.activity_line());
            }
            AgentEvent::ToolResult { .. } => {
                self.push_activity(event.activity_line());
            }
            AgentEvent::Complete { is_error } => {
                self.push_activity(event.activity_line());
                if *is_error {
                    self.set_state(TuiState::Error);
                } else {
                    self.set_state(TuiState::Complete);
                }
            }
            AgentEvent::System(_) | AgentEvent::Unknown(_) => {
                let line = event.activity_line();
                if !line.is_empty() {
                    self.push_activity(line);
                }
            }
        }
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
                    .draw(|frame| self.render_frame(frame))
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
    pub fn handle_key(&mut self, code: KeyCode) {
        // Quit confirmation flow
        if self.quit_pending {
            match code {
                KeyCode::Char('y' | 'Y') => {
                    tracing::info!("user confirmed quit");
                    self.should_quit = true;
                }
                _ => {
                    self.quit_pending = false;
                    self.push_activity(">> Quit cancelled.".to_owned());
                }
            }
            return;
        }

        // Feedback input mode — captures text until Enter or Esc
        if self.state == TuiState::WaitingFeedback {
            match code {
                KeyCode::Enter => {
                    if !self.feedback_buffer.trim().is_empty() {
                        let feedback = self.feedback_buffer.trim().to_owned();
                        self.push_activity(format!(">> Feedback: {feedback}"));
                        self.pending_feedback = Some(feedback);
                        self.feedback_buffer.clear();
                        self.set_state(TuiState::Paused);
                        self.push_activity(
                            ">> Feedback saved. Press 'p' to resume agent with feedback."
                                .to_owned(),
                        );
                    }
                }
                KeyCode::Esc => {
                    self.feedback_buffer.clear();
                    self.set_state(TuiState::Paused);
                    self.push_activity(">> Feedback cancelled.".to_owned());
                }
                KeyCode::Backspace => {
                    self.feedback_buffer.pop();
                }
                KeyCode::Char(c) => {
                    self.feedback_buffer.push(c);
                }
                _ => {}
            }
            return;
        }

        match code {
            KeyCode::Char('q') => {
                self.quit_pending = true;
                self.push_activity(
                    ">> Quit? Press 'y' to confirm, any other key to cancel.".to_owned(),
                );
            }
            KeyCode::Char('p') => {
                if self.state == TuiState::Running {
                    // TUI sets state only — caller handles SIGSTOP via plugin
                    self.set_state(TuiState::Paused);
                    self.push_activity(
                        ">> Agent paused. Press 'f' for feedback, 'p' to resume.".to_owned(),
                    );
                } else if self.state == TuiState::Paused {
                    // TUI sets state only — caller handles SIGCONT via plugin
                    self.set_state(TuiState::Running);
                    self.push_activity(">> Agent resumed.".to_owned());
                }
            }
            KeyCode::Char('f') if self.state == TuiState::Paused => {
                self.set_state(TuiState::WaitingFeedback);
                self.push_activity(
                    ">> Type your feedback, then press Enter to save or Esc to cancel.".to_owned(),
                );
            }
            KeyCode::Char('?') => {
                let help = if self.state == TuiState::Paused {
                    ">> Keys: [f] feedback  [p] resume  [q] quit  [?] help"
                } else {
                    ">> Keys: [p] pause  [q] quit  [?] help"
                };
                self.push_activity(help.to_owned());
            }
            _ => {}
        }
    }

    /// Takes the pending feedback (if any) — consumed by the caller
    /// to re-spawn the agent with merged context.
    pub fn take_feedback(&mut self) -> Option<String> {
        self.pending_feedback.take()
    }

    /// Returns the current feedback buffer (for rendering the input field).
    #[must_use]
    pub fn feedback_buffer(&self) -> &str {
        &self.feedback_buffer
    }

    /// Renders the TUI frame with responsive zone-based layout.
    ///
    /// Layout adapts to terminal size:
    ///
    /// - Compact (< 120 cols): activity only
    /// - Standard (120-159): activity + sidebar
    /// - Wide (>= 160): control + activity + sidebar
    pub fn render_frame(&self, frame: &mut Frame<'_>) {
        let area = frame.area();
        self.render_in(frame, area);
    }

    /// Renders the TUI into a specific sub-area of the frame.
    ///
    /// Used when the logo occupies the top portion of the screen.
    pub fn render_frame_in_area(&self, frame: &mut Frame<'_>, area: Rect) {
        self.render_in(frame, area);
    }

    /// Internal render implementation for a given area.
    fn render_in(&self, frame: &mut Frame<'_>, area: Rect) {
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
                " ◎ Ralph Engine ",
                Style::default()
                    .fg(Color::Indexed(105)) // #5B6AD0 — brand purple
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("• ", Style::default().fg(Color::DarkGray)),
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
    ///
    /// When the activity stream is short enough, the logo is shown
    /// at the top of the viewport. As more events arrive, the logo
    /// scrolls up naturally with the content.
    fn render_activity(&self, frame: &mut Frame<'_>, area: Rect) {
        let visible_lines = area.height as usize;

        // Build logo lines (responsive to width)
        let logo_lines = crate::logo::build_logo_lines(area.width);
        let logo_count = logo_lines.len();

        // Build activity lines with syntax coloring
        let activity: Vec<Line<'_>> = self
            .activity_lines
            .iter()
            .map(|s| {
                if s.starts_with(">> Tool") {
                    Line::styled(s.as_str(), Style::default().fg(Color::Blue))
                } else if s.starts_with(">> State:") || s.starts_with(">> Agent") {
                    Line::styled(s.as_str(), Style::default().fg(Color::Yellow))
                } else if s.starts_with(">> Quit") {
                    Line::styled(s.as_str(), Style::default().fg(Color::Red))
                } else if s.starts_with(">> Keys:") {
                    Line::styled(s.as_str(), Style::default().fg(Color::DarkGray))
                } else {
                    Line::raw(s.as_str())
                }
            })
            .collect();

        // Combine: logo + activity
        let total = logo_count + activity.len();
        let start = total.saturating_sub(visible_lines);

        let mut all_lines: Vec<Line<'_>> = Vec::with_capacity(visible_lines);

        // Add logo lines (skip if scrolled past)
        for (i, line) in logo_lines.into_iter().enumerate() {
            if i >= start {
                all_lines.push(line);
            }
        }

        // Add activity lines (skip if scrolled past)
        let activity_start = start.saturating_sub(logo_count);
        for line in activity.into_iter().skip(activity_start) {
            if all_lines.len() >= visible_lines {
                break;
            }
            all_lines.push(line);
        }

        // Show feedback input field at bottom when in WaitingFeedback state
        if self.state == TuiState::WaitingFeedback {
            let cursor = if self.feedback_buffer.is_empty() {
                "type your feedback...".to_owned()
            } else {
                format!("{}▌", self.feedback_buffer)
            };
            // Replace last lines with input field
            if all_lines.len() >= 2 {
                all_lines.pop();
            }
            all_lines.push(Line::styled(
                format!("  > {cursor}"),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        frame.render_widget(Paragraph::new(all_lines), area);
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
        if self.quit_pending {
            let warn = Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD);
            let spans = vec![
                Span::styled(" Quit? ", warn),
                Span::styled(
                    "[y] yes  [any key] cancel",
                    Style::default().fg(Color::DarkGray),
                ),
            ];
            frame.render_widget(Paragraph::new(Line::from(spans)), zones.help);
            return;
        }

        if self.state == TuiState::WaitingFeedback {
            let input_style = Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD);
            let spans = vec![
                Span::styled(" Feedback ", input_style),
                Span::styled(
                    "[Enter] send  [Esc] cancel  [Backspace] delete",
                    Style::default().fg(Color::DarkGray),
                ),
            ];
            frame.render_widget(Paragraph::new(Line::from(spans)), zones.help);
            return;
        }

        let dim = Style::default().fg(Color::DarkGray);
        let mut spans = vec![
            Span::styled(" [?]", dim),
            Span::styled(" help ", dim),
            Span::styled(" [p]", dim),
            Span::styled(" pause ", dim),
            Span::styled(" [q]", dim),
            Span::styled(" quit ", dim),
        ];

        let tier_label = match zones.tier {
            layout::LayoutTier::Compact => "compact",
            layout::LayoutTier::Standard => "standard",
            layout::LayoutTier::Wide => "wide",
        };
        spans.push(Span::styled(
            format!(" │ {tier_label}"),
            Style::default().fg(Color::Indexed(59)),
        ));

        frame.render_widget(Paragraph::new(Line::from(spans)), zones.help);
    }

    /// Renders the sidebar zone with auto-discovered plugin panels.
    fn render_sidebar(&self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::LEFT)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Plugins ")
            .title_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.sidebar_panels.is_empty() {
            let lines = vec![Line::styled(
                " (no panels)",
                Style::default().fg(Color::DarkGray),
            )];
            frame.render_widget(Paragraph::new(lines), inner);
            return;
        }

        // Distribute vertical space equally among panels.
        let panel_count = self.sidebar_panels.len();
        let constraints: Vec<Constraint> = (0..panel_count)
            .map(|i| {
                if i < panel_count - 1 {
                    Constraint::Ratio(1, panel_count as u32)
                } else {
                    Constraint::Fill(1)
                }
            })
            .collect();

        let panel_areas = Layout::vertical(constraints).split(inner);

        for (i, panel) in self.sidebar_panels.iter().enumerate() {
            let panel_block = Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(format!(" {} ", panel.title))
                .title_style(Style::default().fg(Color::White));

            let panel_inner = panel_block.inner(panel_areas[i]);
            frame.render_widget(panel_block, panel_areas[i]);

            let lines: Vec<Line<'_>> = panel
                .lines
                .iter()
                .map(|s| Line::raw(format!(" {s}")))
                .collect();
            frame.render_widget(Paragraph::new(lines), panel_inner);
        }
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
    fn handle_key_q_requires_confirmation() {
        let mut shell = empty_shell();
        assert!(!shell.should_quit());

        // First q: sets pending, does NOT quit
        shell.handle_key(KeyCode::Char('q'));
        assert!(!shell.should_quit());
        assert!(shell.is_quit_pending());

        // Any non-y key: cancels quit
        shell.handle_key(KeyCode::Char('n'));
        assert!(!shell.should_quit());
        assert!(!shell.is_quit_pending());

        // q then y: confirms quit
        shell.handle_key(KeyCode::Char('q'));
        shell.handle_key(KeyCode::Char('y'));
        assert!(shell.should_quit());
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
        terminal.draw(|frame| shell.render_frame(frame)).unwrap();
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

    // ── process_event tests ──────────────────────────────────────

    use crate::events::AgentEvent;

    fn empty_shell() -> TuiShell {
        TuiShell::new(TuiConfig {
            mode: TuiMode::Autonomous,
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        })
    }

    #[test]
    fn process_event_text_delta_appends() {
        let mut shell = empty_shell();
        shell.process_event(&AgentEvent::TextDelta("Hello".to_owned()));
        assert_eq!(shell.activity_lines.len(), 1);
        assert_eq!(shell.activity_lines[0], "Hello");
    }

    #[test]
    fn process_event_tool_use_increments_count() {
        let mut shell = empty_shell();
        shell.process_event(&AgentEvent::ToolUse {
            name: "Read".to_owned(),
        });
        assert_eq!(shell.tool_count, 1);
        assert!(shell.activity_lines[0].contains("Tool: Read"));
    }

    #[test]
    fn process_event_complete_sets_state() {
        let mut shell = empty_shell();
        shell.process_event(&AgentEvent::Complete { is_error: false });
        assert_eq!(shell.state(), TuiState::Complete);

        let mut shell2 = empty_shell();
        shell2.process_event(&AgentEvent::Complete { is_error: true });
        assert_eq!(shell2.state(), TuiState::Error);
    }

    #[test]
    fn process_event_tool_result_appends() {
        let mut shell = empty_shell();
        shell.process_event(&AgentEvent::ToolResult {
            name: "Bash".to_owned(),
            success: true,
        });
        assert!(shell.activity_lines[0].contains("Bash [OK]"));
    }

    #[test]
    fn process_event_system_appends() {
        let mut shell = empty_shell();
        shell.process_event(&AgentEvent::System("Starting session".to_owned()));
        assert!(shell.activity_lines[0].contains("Starting session"));
    }

    #[test]
    fn process_event_unknown_skips_empty() {
        let mut shell = empty_shell();
        shell.process_event(&AgentEvent::Unknown(String::new()));
        assert!(shell.activity_lines.is_empty());
    }

    #[test]
    fn process_event_sequence() {
        let mut shell = empty_shell();
        shell.process_event(&AgentEvent::System("Init".to_owned()));
        shell.process_event(&AgentEvent::ToolUse {
            name: "Read".to_owned(),
        });
        shell.process_event(&AgentEvent::ToolResult {
            name: "Read".to_owned(),
            success: true,
        });
        shell.process_event(&AgentEvent::TextDelta("Processing...".to_owned()));
        shell.process_event(&AgentEvent::Complete { is_error: false });

        assert_eq!(shell.activity_lines.len(), 5);
        assert_eq!(shell.tool_count, 1);
        assert_eq!(shell.state(), TuiState::Complete);
    }

    // ── Sidebar panel rendering tests ────────────────────────────

    #[test]
    fn render_standard_with_plugin_panels() {
        let mut shell = test_shell();
        shell.set_sidebar_panels(vec![
            SidebarPanel {
                title: "Findings".to_owned(),
                lines: vec!["3 issues found".to_owned(), "2 warnings".to_owned()],
                plugin_id: "official.findings".to_owned(),
            },
            SidebarPanel {
                title: "Sprint".to_owned(),
                lines: vec!["Story 5.3: in-progress".to_owned()],
                plugin_id: "official.bmad".to_owned(),
            },
        ]);
        let output = render_to_buffer(&shell, 140, 40);
        assert!(
            output.contains("Findings"),
            "sidebar should show Findings panel, got:\n{output}"
        );
        assert!(
            output.contains("Sprint"),
            "sidebar should show Sprint panel, got:\n{output}"
        );
        assert!(
            output.contains("3 issues"),
            "Findings panel should show content, got:\n{output}"
        );
    }

    #[test]
    fn render_standard_empty_panels_shows_placeholder() {
        let shell = test_shell();
        // No panels set — default is empty
        let output = render_to_buffer(&shell, 140, 40);
        assert!(
            output.contains("no panels"),
            "empty sidebar should show placeholder, got:\n{output}"
        );
    }

    #[test]
    fn set_sidebar_panels_replaces() {
        let mut shell = empty_shell();
        shell.set_sidebar_panels(vec![SidebarPanel {
            title: "A".to_owned(),
            lines: vec![],
            plugin_id: "test".to_owned(),
        }]);
        assert_eq!(shell.sidebar_panels.len(), 1);

        shell.set_sidebar_panels(vec![]);
        assert!(shell.sidebar_panels.is_empty());
    }

    // ── Feedback input tests ─────────────────────────────────────

    #[test]
    fn feedback_flow_pause_f_type_enter() {
        let mut shell = empty_shell();

        // Pause
        shell.handle_key(KeyCode::Char('p'));
        assert_eq!(shell.state(), TuiState::Paused);

        // Enter feedback mode
        shell.handle_key(KeyCode::Char('f'));
        assert_eq!(shell.state(), TuiState::WaitingFeedback);

        // Type feedback
        shell.handle_key(KeyCode::Char('f'));
        shell.handle_key(KeyCode::Char('i'));
        shell.handle_key(KeyCode::Char('x'));
        assert_eq!(shell.feedback_buffer(), "fix");

        // Submit
        shell.handle_key(KeyCode::Enter);
        assert_eq!(shell.state(), TuiState::Paused);
        assert_eq!(shell.take_feedback(), Some("fix".to_owned()));
        assert!(shell.feedback_buffer().is_empty());
    }

    #[test]
    fn feedback_esc_cancels() {
        let mut shell = empty_shell();
        shell.handle_key(KeyCode::Char('p'));
        shell.handle_key(KeyCode::Char('f'));
        shell.handle_key(KeyCode::Char('a'));
        shell.handle_key(KeyCode::Char('b'));
        assert_eq!(shell.feedback_buffer(), "ab");

        shell.handle_key(KeyCode::Esc);
        assert_eq!(shell.state(), TuiState::Paused);
        assert!(shell.feedback_buffer().is_empty());
        assert!(shell.take_feedback().is_none());
    }

    #[test]
    fn feedback_backspace_deletes() {
        let mut shell = empty_shell();
        shell.handle_key(KeyCode::Char('p'));
        shell.handle_key(KeyCode::Char('f'));
        shell.handle_key(KeyCode::Char('a'));
        shell.handle_key(KeyCode::Char('b'));
        shell.handle_key(KeyCode::Backspace);
        assert_eq!(shell.feedback_buffer(), "a");
    }

    #[test]
    fn feedback_empty_enter_ignored() {
        let mut shell = empty_shell();
        shell.handle_key(KeyCode::Char('p'));
        shell.handle_key(KeyCode::Char('f'));
        shell.handle_key(KeyCode::Enter); // empty, ignored
        assert_eq!(shell.state(), TuiState::WaitingFeedback);
        assert!(shell.take_feedback().is_none());
    }

    #[test]
    fn f_key_only_works_when_paused() {
        let mut shell = empty_shell();
        // Running state — f should not enter feedback
        shell.handle_key(KeyCode::Char('f'));
        assert_eq!(shell.state(), TuiState::Running);
    }

    #[test]
    fn take_feedback_returns_none_after_consumed() {
        let mut shell = empty_shell();
        shell.handle_key(KeyCode::Char('p'));
        shell.handle_key(KeyCode::Char('f'));
        shell.handle_key(KeyCode::Char('x'));
        shell.handle_key(KeyCode::Enter);

        assert!(shell.take_feedback().is_some());
        assert!(shell.take_feedback().is_none()); // second call = None
    }
}
