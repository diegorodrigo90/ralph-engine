//! Terminal lifecycle and TUI shell.
//!
//! Manages the ratatui terminal: enters raw mode on start,
//! restores on exit/crash/signal. Provides the main render
//! skeleton with zone-based layout.
//!
//! The TUI shell is a **generic framework**. It knows about rendering,
//! core keys (q, ?), and dispatching unknown keys to plugin-contributed
//! keybindings. Interactive features (pause, feedback, resume) live in
//! plugins — not here. When no interactive plugin is enabled, the TUI
//! is a read-only dashboard.

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use crate::layout;

/// Current state of the TUI session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiState {
    /// Agent is running, streaming output.
    Running,
    /// Agent is paused (set by plugin via `SetState`).
    Paused,
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
            Self::Complete => Color::Cyan,
            Self::Error => Color::Red,
        }
    }

    /// Parses a state from its label string (e.g. `"Running"` → `Running`).
    #[must_use]
    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "Running" => Some(Self::Running),
            "Paused" => Some(Self::Paused),
            "Complete" => Some(Self::Complete),
            "Error" => Some(Self::Error),
            _ => None,
        }
    }
}

/// Configuration for the TUI shell.
#[derive(Debug, Clone)]
pub struct TuiConfig {
    /// Title shown in the header bar.
    pub title: String,
    /// Agent identifier shown in header.
    pub agent_id: String,
    /// Resolved locale for i18n (e.g. `"en"`, `"pt-br"`).
    pub locale: String,
}

/// A sidebar panel provided by a plugin, ready to render.
#[derive(Debug, Clone)]
pub struct SidebarPanel {
    /// Panel title (localized).
    pub title: String,
    /// Content lines to render.
    pub lines: Vec<String>,
    /// Source plugin ID (for attribution).
    pub plugin_id: String,
}

/// A keybinding registered by a plugin for the TUI.
///
/// Collected from plugins via auto-discovery and stored in the shell
/// for dispatch and help-bar rendering.
#[derive(Debug, Clone)]
pub struct RegisteredKeybinding {
    /// Key character (e.g. `'p'`).
    pub key: char,
    /// Short description for the help bar.
    pub description: String,
    /// Plugin that owns this keybinding.
    pub plugin_id: String,
    /// TUI states where this keybinding is active (empty = all states).
    pub active_states: Vec<String>,
}

/// Result of dispatching a key to a plugin.
///
/// The caller (CLI `run` command) interprets these results to
/// perform actual operations (pause agent, re-spawn, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginKeyAction {
    /// Key was handled by the plugin. No further action needed from core.
    Handled,
    /// Plugin requests a TUI state change.
    SetState(TuiState),
    /// Plugin requests entering text-input mode with the given prompt.
    EnterTextInput {
        /// Prompt text shown to the user.
        prompt: String,
    },
    /// Plugin wants to show a message in the activity stream.
    ShowMessage(String),
    /// No plugin handled this key.
    NotHandled,
}

/// Agent process ID (set when a real agent is launched).
pub type AgentPid = Option<u32>;

/// An entry in the autocomplete command list.
#[derive(Debug, Clone)]
pub struct CommandEntry {
    /// Command name (e.g. `"compact"`, `"skill"`).
    pub name: String,
    /// Short description for the popup.
    pub description: String,
}

/// Autocomplete popup state for agent slash commands.
#[derive(Debug)]
pub struct AutocompleteState {
    /// All available commands (discovered from agent plugin).
    commands: Vec<CommandEntry>,
    /// Indices into `commands` matching the current filter.
    filtered: Vec<usize>,
    /// Selected index within `filtered`.
    selected: usize,
    /// Whether the popup is visible.
    visible: bool,
    /// The command prefix character (e.g. `/`).
    prefix: String,
}

impl AutocompleteState {
    /// Creates a new autocomplete state with the given commands and prefix.
    fn new(commands: Vec<CommandEntry>, prefix: String) -> Self {
        Self {
            commands,
            filtered: Vec::new(),
            selected: 0,
            visible: false,
            prefix,
        }
    }

    /// Updates the filter based on current input text.
    fn update_filter(&mut self, input: &str) {
        if input.starts_with(self.prefix.as_str()) && !self.commands.is_empty() {
            let query = &input[self.prefix.len()..];
            self.filtered = self
                .commands
                .iter()
                .enumerate()
                .filter(|(_, cmd)| {
                    query.is_empty()
                        || cmd.name.contains(query)
                        || cmd
                            .description
                            .to_lowercase()
                            .contains(&query.to_lowercase())
                })
                .map(|(i, _)| i)
                .collect();
            self.selected = self.selected.min(self.filtered.len().saturating_sub(1));
            self.visible = !self.filtered.is_empty();
        } else {
            self.visible = false;
        }
    }

    /// Returns the currently selected command name (with prefix), if any.
    fn selected_command(&self) -> Option<String> {
        if !self.visible || self.filtered.is_empty() {
            return None;
        }
        let idx = self.filtered[self.selected];
        Some(format!("{}{}", self.prefix, self.commands[idx].name))
    }
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
    quit_pending: bool,
    sidebar_panels: Vec<SidebarPanel>,
    agent_pid: AgentPid,
    /// Plugin-contributed keybindings (from auto-discovery).
    plugin_keybindings: Vec<RegisteredKeybinding>,
    /// Whether the input bar is enabled (set by plugin via `tui_input_placeholder`).
    input_enabled: bool,
    /// Text buffer for the chat input.
    text_input_buffer: String,
    /// Completed text input ready to be consumed by the caller.
    pending_text_input: Option<String>,
    /// Autocomplete state for slash commands.
    autocomplete: AutocompleteState,
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
            plugin_keybindings: Vec::new(),
            input_enabled: false,
            text_input_buffer: String::new(),
            pending_text_input: None,
            autocomplete: AutocompleteState::new(Vec::new(), "/".to_owned()),
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

    /// Sets the plugin keybindings from auto-discovered contributions.
    pub fn set_plugin_keybindings(&mut self, bindings: Vec<RegisteredKeybinding>) {
        self.plugin_keybindings = bindings;
    }

    /// Sets the agent process ID for pause/resume signal delivery.
    pub fn set_agent_pid(&mut self, pid: u32) {
        self.agent_pid = Some(pid);
    }

    /// Pushes the startup banner with config details into the activity stream.
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

    /// Whether the input bar is enabled (a plugin requested it).
    #[must_use]
    pub fn is_input_enabled(&self) -> bool {
        self.input_enabled
    }

    /// Enables the input bar. Called by the CLI layer when a plugin
    /// returns `Some` from `tui_input_placeholder()`.
    pub fn enable_input(&mut self) {
        self.input_enabled = true;
    }

    /// Sets the agent commands for autocomplete.
    ///
    /// Called by the CLI layer after discovering commands from the
    /// active agent plugin's `discover_agent_commands()`.
    pub fn set_agent_commands(&mut self, commands: Vec<CommandEntry>, prefix: String) {
        self.autocomplete = AutocompleteState::new(commands, prefix);
    }

    /// Returns the current text input buffer (for rendering).
    #[must_use]
    pub fn text_input_buffer(&self) -> &str {
        &self.text_input_buffer
    }

    /// Takes the pending text input (if any) — consumed by the caller.
    pub fn take_text_input(&mut self) -> Option<String> {
        self.pending_text_input.take()
    }

    /// Returns the agent PID if set.
    #[must_use]
    pub fn agent_pid(&self) -> AgentPid {
        self.agent_pid
    }

    /// Processes a normalized agent event, updating TUI state and activity.
    pub fn process_event(&mut self, event: &crate::events::AgentEvent) {
        use crate::events::AgentEvent;

        match event {
            AgentEvent::TextDelta(_) => {
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

    /// Runs the TUI demo loop.
    ///
    /// # Errors
    ///
    /// Returns an error if terminal initialization fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn run_demo(&mut self) -> Result<(), TuiError> {
        self.push_activity(">> Ralph Engine TUI initialized".to_owned());
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
    ///
    /// Priority: quit confirmation → typing mode → core keys → plugin keybinding → chat input.
    ///
    /// When the input buffer has text, ALL character keys go to the buffer.
    /// When the buffer is empty, keybindings and core keys take priority.
    /// Non-keybinding characters start typing (go to buffer).
    pub fn handle_key(&mut self, code: KeyCode) -> PluginKeyAction {
        self.handle_key_with_modifiers(code, KeyModifiers::NONE)
    }

    /// Handles a key press with modifiers (e.g., Ctrl+J for newline).
    pub fn handle_key_with_modifiers(
        &mut self,
        code: KeyCode,
        modifiers: KeyModifiers,
    ) -> PluginKeyAction {
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
            return PluginKeyAction::Handled;
        }

        let typing = self.input_enabled && !self.text_input_buffer.is_empty();

        // Typing mode: buffer has content → all chars go to buffer
        if typing {
            // Autocomplete intercepts when visible
            if self.autocomplete.visible {
                match code {
                    KeyCode::Up => {
                        self.autocomplete.selected = self.autocomplete.selected.saturating_sub(1);
                        return PluginKeyAction::Handled;
                    }
                    KeyCode::Down => {
                        self.autocomplete.selected = (self.autocomplete.selected + 1)
                            .min(self.autocomplete.filtered.len().saturating_sub(1));
                        return PluginKeyAction::Handled;
                    }
                    KeyCode::Tab => {
                        // Tab → complete into buffer (don't send)
                        if let Some(cmd) = self.autocomplete.selected_command() {
                            self.text_input_buffer = cmd;
                            self.autocomplete.visible = false;
                        }
                        return PluginKeyAction::Handled;
                    }
                    KeyCode::Enter => {
                        // Enter with autocomplete → select and send
                        if let Some(cmd) = self.autocomplete.selected_command() {
                            self.text_input_buffer = cmd;
                        }
                        // Fall through to normal Enter handling below
                    }
                    KeyCode::Esc => {
                        // Close popup, keep buffer
                        self.autocomplete.visible = false;
                        return PluginKeyAction::Handled;
                    }
                    _ => {} // Other keys fall through to normal typing
                }
            }

            match code {
                KeyCode::Enter if modifiers.contains(KeyModifiers::ALT) => {
                    self.text_input_buffer.push('\n');
                }
                KeyCode::Enter => {
                    if !self.text_input_buffer.trim().is_empty() {
                        let text = self.text_input_buffer.trim().to_owned();
                        self.push_activity(format!(">> You: {text}"));
                        self.pending_text_input = Some(text);
                    }
                    self.text_input_buffer.clear();
                    self.autocomplete.visible = false;
                }
                KeyCode::Esc => {
                    self.text_input_buffer.clear();
                    self.autocomplete.visible = false;
                }
                KeyCode::Backspace => {
                    self.text_input_buffer.pop();
                    self.autocomplete.update_filter(&self.text_input_buffer);
                }
                KeyCode::Char(c) => {
                    self.text_input_buffer.push(c);
                    self.autocomplete.update_filter(&self.text_input_buffer);
                }
                _ => {}
            }
            return PluginKeyAction::Handled;
        }

        // Core keys (always available when not typing)
        match code {
            KeyCode::Char('q') => {
                self.quit_pending = true;
                self.push_activity(
                    ">> Quit? Press 'y' to confirm, any other key to cancel.".to_owned(),
                );
                return PluginKeyAction::Handled;
            }
            KeyCode::Char('p') => {
                if self.state == TuiState::Running {
                    self.set_state(TuiState::Paused);
                    self.push_activity(">> PAUSED — press [p] to resume".to_owned());
                } else if self.state == TuiState::Paused {
                    self.set_state(TuiState::Running);
                    self.push_activity(">> RUNNING".to_owned());
                }
                return PluginKeyAction::Handled;
            }
            KeyCode::Char('?') => {
                self.push_help_to_activity();
                return PluginKeyAction::Handled;
            }
            _ => {}
        }

        // Plugin keybinding dispatch (buffer is empty)
        if let KeyCode::Char(c) = code {
            let state_label = format!("{:?}", self.state);
            if self.find_active_binding(c, &state_label).is_some() {
                tracing::debug!(key = %c, "dispatching key to plugin");
                return PluginKeyAction::NotHandled; // Caller dispatches to plugin runtime
            }

            // Not a keybinding → start typing (if plugin enabled input)
            if self.input_enabled {
                self.text_input_buffer.push(c);
                // Trigger autocomplete on prefix (e.g. `/`)
                self.autocomplete.update_filter(&self.text_input_buffer);
                return PluginKeyAction::Handled;
            }
        }

        PluginKeyAction::NotHandled
    }

    /// Applies a plugin key action to the TUI state.
    ///
    /// Called by the CLI layer after converting a `re_plugin::TuiKeyResult`
    /// into a `PluginKeyAction`. The TUI shell doesn't depend on re-plugin
    /// directly — the CLI layer does the translation.
    pub fn apply_plugin_action(&mut self, action: &PluginKeyAction) {
        match action {
            PluginKeyAction::Handled | PluginKeyAction::NotHandled => {}
            PluginKeyAction::EnterTextInput { prompt } => {
                self.input_enabled = true;
                self.text_input_buffer.clear();
                self.push_activity(format!(">> {prompt}"));
            }
            PluginKeyAction::SetState(new_state) => {
                self.set_state(*new_state);
                // Show state change + available keybindings in the new state
                let new_label = format!("{new_state:?}");
                let available: Vec<String> = self
                    .plugin_keybindings
                    .iter()
                    .filter(|b| {
                        b.active_states.is_empty()
                            || b.active_states.iter().any(|s| s == &new_label)
                    })
                    .map(|b| format!("[{}] {}", b.key, b.description))
                    .collect();
                if available.is_empty() {
                    self.push_activity(format!(">> {}", new_state.label()));
                } else {
                    self.push_activity(format!(
                        ">> {} — {}",
                        new_state.label(),
                        available.join("  ")
                    ));
                }
            }
            PluginKeyAction::ShowMessage(msg) => {
                self.push_activity(format!(">> {msg}"));
            }
        }
    }

    /// Finds an active plugin keybinding for the given key and state.
    #[must_use]
    pub fn find_active_binding(
        &self,
        key: char,
        state_label: &str,
    ) -> Option<&RegisteredKeybinding> {
        self.plugin_keybindings.iter().find(|b| {
            b.key == key
                && (b.active_states.is_empty() || b.active_states.iter().any(|s| s == state_label))
        })
    }

    /// Pushes help text to the activity stream based on state and plugin bindings.
    fn push_help_to_activity(&mut self) {
        let state_label = format!("{:?}", self.state);
        let mut parts = vec![">> Keys:".to_owned()];

        // Plugin keybindings active in current state
        for binding in &self.plugin_keybindings {
            if binding.active_states.is_empty()
                || binding.active_states.iter().any(|s| s == &state_label)
            {
                parts.push(format!("[{}] {}", binding.key, binding.description));
            }
        }

        // Core keys (always active)
        parts.push("[q] quit".to_owned());
        parts.push("[?] help".to_owned());

        self.push_activity(parts.join("  "));
    }

    /// Renders the TUI frame with responsive zone-based layout.
    pub fn render_frame(&self, frame: &mut Frame<'_>) {
        let area = frame.area();
        self.render_in(frame, area);
    }

    /// Renders the TUI into a specific sub-area of the frame.
    pub fn render_frame_in_area(&self, frame: &mut Frame<'_>, area: Rect) {
        self.render_in(frame, area);
    }

    /// Internal render implementation for a given area.
    fn render_in(&self, frame: &mut Frame<'_>, area: Rect) {
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

        let zones = layout::compute_zones(area, self.input_enabled);

        self.render_header(frame, zones.header);
        self.render_activity(frame, zones.activity);
        self.render_metrics(frame, zones.metrics);
        self.render_help(frame, &zones);

        if let Some(input_area) = zones.input {
            self.render_input_bar(frame, input_area);
        }

        if let Some(sidebar) = zones.sidebar {
            self.render_sidebar(frame, sidebar);
        }

        if let Some(control) = zones.control {
            self.render_control_panel(frame, control);
        }

        // Autocomplete popup — rendered LAST (on top of everything)
        if let Some(input_area) = zones.input {
            self.render_autocomplete(frame, input_area);
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
                    .fg(Color::Indexed(105))
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

        let logo_lines = crate::logo::build_logo_lines(area.width);
        let logo_count = logo_lines.len();

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

        let total = logo_count + activity.len();
        let start = total.saturating_sub(visible_lines);

        let mut all_lines: Vec<Line<'_>> = Vec::with_capacity(visible_lines);

        for (i, line) in logo_lines.into_iter().enumerate() {
            if i >= start {
                all_lines.push(line);
            }
        }

        let activity_start = start.saturating_sub(logo_count);
        for line in activity.into_iter().skip(activity_start) {
            if all_lines.len() >= visible_lines {
                break;
            }
            all_lines.push(line);
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

    /// Renders the chat input bar.
    ///
    /// Only shown when an interactive plugin (e.g. guided) registers
    /// keybindings. Shows a persistent input prompt with cursor.
    /// When empty, shows placeholder. When typing, shows buffer + cursor.
    /// Renders the chat input bar — clean `>` prompt, separator above.
    ///
    /// Renders the chat input — separator, `>` prompt, multi-line, native cursor.
    ///
    /// Ctrl+J inserts newline. Enter sends. Esc cancels.
    /// Native terminal cursor blinks at insertion point.
    fn render_input_bar(&self, frame: &mut Frame<'_>, area: Rect) {
        let prompt_color = Color::Indexed(105); // brand purple
        let sep = "─".repeat(area.width as usize);
        let prompt = " > ";
        let prompt_width = prompt.len() as u16;

        // Split: separator (1) + text area (rest)
        let rows = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).split(area);

        frame.render_widget(
            Paragraph::new(Line::styled(sep, Style::default().fg(Color::Indexed(59)))),
            rows[0],
        );

        let text_area = rows[1];

        if self.text_input_buffer.is_empty() {
            // Empty: just the prompt
            let line = Line::from(vec![Span::styled(
                prompt,
                Style::default()
                    .fg(prompt_color)
                    .add_modifier(Modifier::BOLD),
            )]);
            frame.render_widget(Paragraph::new(line), text_area);
            // Native blinking cursor
            frame.set_cursor_position((text_area.x + prompt_width, text_area.y));
        } else {
            // Build wrapped/multi-line content
            let content_width = text_area.width.saturating_sub(prompt_width).max(1) as usize;
            let mut display_lines: Vec<Line<'_>> = Vec::new();
            let mut first = true;

            for text_line in self.text_input_buffer.split('\n') {
                if text_line.is_empty() {
                    let pfx = if first { prompt } else { "   " };
                    first = false;
                    display_lines.push(Line::from(Span::styled(
                        pfx.to_owned(),
                        Style::default()
                            .fg(prompt_color)
                            .add_modifier(Modifier::BOLD),
                    )));
                    continue;
                }
                let mut pos = 0;
                while pos < text_line.len() {
                    let end = (pos + content_width).min(text_line.len());
                    let chunk = &text_line[pos..end];
                    let pfx = if first { prompt } else { "   " };
                    first = false;
                    display_lines.push(Line::from(vec![
                        Span::styled(
                            pfx.to_owned(),
                            Style::default()
                                .fg(prompt_color)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(chunk.to_owned(), Style::default().fg(Color::White)),
                    ]));
                    pos = end;
                }
            }

            // Scroll to show last visible lines
            let visible = text_area.height as usize;
            let start = display_lines.len().saturating_sub(visible);
            let shown: Vec<Line<'_>> = display_lines.into_iter().skip(start).collect();
            let line_count = shown.len();

            frame.render_widget(Paragraph::new(shown), text_area);

            // Cursor at end of last line
            let last_text_line = self.text_input_buffer.rsplit('\n').next().unwrap_or("");
            let cursor_col = (last_text_line.len() % content_width) as u16;
            let cursor_row = (line_count.saturating_sub(1)) as u16;
            frame.set_cursor_position((
                text_area.x + prompt_width + cursor_col,
                text_area.y + cursor_row.min(text_area.height.saturating_sub(1)),
            ));
        }
    }

    /// Renders the autocomplete popup above the input bar.
    ///
    /// Uses painter's algorithm: `Clear` + render on top.
    fn render_autocomplete(&self, frame: &mut Frame<'_>, input_area: Rect) {
        if !self.autocomplete.visible || self.autocomplete.filtered.is_empty() {
            return;
        }

        let max_visible = 8u16;
        let item_count = (self.autocomplete.filtered.len() as u16).min(max_visible);
        let popup_height = item_count + 2; // +2 for borders
        let popup_width = input_area.width.min(60);

        let popup_area = Rect {
            x: input_area.x,
            y: input_area.y.saturating_sub(popup_height),
            width: popup_width,
            height: popup_height,
        };

        let items: Vec<ListItem<'_>> = self
            .autocomplete
            .filtered
            .iter()
            .map(|&idx| {
                let cmd = &self.autocomplete.commands[idx];
                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("{}{}", self.autocomplete.prefix, cmd.name),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("  "),
                    Span::styled(&cmd.description, Style::default().fg(Color::DarkGray)),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Indexed(59)))
                    .title(" Commands ")
                    .title_style(Style::default().fg(Color::Indexed(105))),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Indexed(236))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_widget(Clear, popup_area);
        frame.render_stateful_widget(
            list,
            popup_area,
            &mut ListState::default().with_selected(Some(self.autocomplete.selected)),
        );
    }

    /// Renders the help bar at the bottom.
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

        let dim = Style::default().fg(Color::DarkGray);

        // When typing, show input-specific help
        if self.input_enabled && !self.text_input_buffer.is_empty() {
            let spans = vec![
                Span::styled(" [Enter]", dim),
                Span::styled(" send ", dim),
                Span::styled(" [Alt+Enter]", dim),
                Span::styled(" newline ", dim),
                Span::styled(" [Esc]", dim),
                Span::styled(" cancel ", dim),
            ];
            frame.render_widget(Paragraph::new(Line::from(spans)), zones.help);
            return;
        }

        let state_label = format!("{:?}", self.state);
        let mut spans: Vec<Span<'_>> = Vec::new();

        // Plugin keybindings active in current state
        for binding in &self.plugin_keybindings {
            if binding.active_states.is_empty()
                || binding.active_states.iter().any(|s| s == &state_label)
            {
                spans.push(Span::styled(format!(" [{}]", binding.key), dim));
                spans.push(Span::styled(format!(" {} ", binding.description), dim));
            }
        }

        // Core keys
        spans.push(Span::styled(" [p]", dim));
        spans.push(Span::styled(" pause ", dim));
        spans.push(Span::styled(" [?]", dim));
        spans.push(Span::styled(" help ", dim));
        spans.push(Span::styled(" [q]", dim));
        spans.push(Span::styled(" quit ", dim));

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

    /// Renders the control panel zone (wide tier only).
    ///
    /// Shows current state and work item. Plugin-specific controls
    /// appear via sidebar panels, not here.
    fn render_control_panel(&self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Control ")
            .title_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let state_color = self.state.color();

        let lines = vec![
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
            Line::styled(" [q] Quit", Style::default().fg(Color::DarkGray)),
        ];
        frame.render_widget(Paragraph::new(lines), inner);
    }
}

/// Initializes the terminal with ratatui defaults.
#[cfg_attr(coverage_nightly, coverage(off))]
fn init_terminal() -> DefaultTerminal {
    ratatui::init()
}

/// Restores terminal to normal mode.
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
    fn tui_state_from_label_roundtrips() {
        assert_eq!(TuiState::from_label("Running"), Some(TuiState::Running));
        assert_eq!(TuiState::from_label("Paused"), Some(TuiState::Paused));
        assert_eq!(TuiState::from_label("Complete"), Some(TuiState::Complete));
        assert_eq!(TuiState::from_label("Error"), Some(TuiState::Error));
        assert_eq!(TuiState::from_label("Unknown"), None);
    }

    #[test]
    fn tui_shell_new_has_correct_defaults() {
        let shell = TuiShell::new(TuiConfig {
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        assert_eq!(shell.state(), TuiState::Running);
        assert_eq!(shell.progress, 0);
        assert!(shell.activity_lines.is_empty());
        assert_eq!(shell.tool_count, 0);
        assert!(!shell.is_input_enabled());
    }

    #[test]
    fn push_activity_appends_line() {
        let mut shell = empty_shell();
        shell.push_activity("hello".to_owned());
        shell.push_activity("world".to_owned());
        assert_eq!(shell.activity_lines.len(), 2);
    }

    #[test]
    fn push_activity_bounds_buffer() {
        let mut shell = empty_shell();
        for i in 0..10_001 {
            shell.push_activity(format!("line {i}"));
        }
        assert!(shell.activity_lines.len() <= 10_000);
        assert_eq!(shell.activity_lines.last().unwrap(), "line 10000");
    }

    #[test]
    fn set_state_transitions() {
        let mut shell = empty_shell();
        assert_eq!(shell.state(), TuiState::Running);
        shell.set_state(TuiState::Paused);
        assert_eq!(shell.state(), TuiState::Paused);
    }

    #[test]
    fn set_progress_clamps_to_100() {
        let mut shell = empty_shell();
        shell.set_progress(150);
        assert_eq!(shell.progress, 100);
    }

    #[test]
    fn increment_tools_counts() {
        let mut shell = empty_shell();
        shell.increment_tools();
        shell.increment_tools();
        shell.increment_tools();
        assert_eq!(shell.tool_count, 3);
    }

    #[test]
    fn handle_key_q_requires_confirmation() {
        let mut shell = empty_shell();
        assert!(!shell.should_quit());

        shell.handle_key(KeyCode::Char('q'));
        assert!(!shell.should_quit());
        assert!(shell.is_quit_pending());

        shell.handle_key(KeyCode::Char('n'));
        assert!(!shell.should_quit());
        assert!(!shell.is_quit_pending());

        shell.handle_key(KeyCode::Char('q'));
        shell.handle_key(KeyCode::Char('y'));
        assert!(shell.should_quit());
    }

    #[test]
    fn handle_key_help_adds_activity() {
        let mut shell = empty_shell();
        shell.handle_key(KeyCode::Char('?'));
        assert!(shell.activity_lines.last().unwrap().contains("help"));
    }

    #[test]
    fn handle_key_unknown_returns_not_handled() {
        let mut shell = empty_shell();
        let result = shell.handle_key(KeyCode::Char('x'));
        assert_eq!(result, PluginKeyAction::NotHandled);
    }

    #[test]
    fn plugin_keybinding_dispatch() {
        let mut shell = empty_shell();
        shell.set_plugin_keybindings(vec![RegisteredKeybinding {
            key: 'p',
            description: "Pause".to_owned(),
            plugin_id: "test.guided".to_owned(),
            active_states: vec!["Running".to_owned()],
        }]);

        // Key 'p' should find the binding while Running
        let binding = shell.find_active_binding('p', "Running");
        assert!(binding.is_some());
        assert_eq!(binding.unwrap().plugin_id, "test.guided");

        // Key 'p' should NOT find the binding while Complete
        let binding = shell.find_active_binding('p', "Complete");
        assert!(binding.is_none());
    }

    #[test]
    fn apply_plugin_action_set_state() {
        let mut shell = empty_shell();
        shell.apply_plugin_action(&PluginKeyAction::SetState(TuiState::Paused));
        assert_eq!(shell.state(), TuiState::Paused);
    }

    #[test]
    fn apply_plugin_action_enter_text_input() {
        let mut shell = empty_shell();
        shell.apply_plugin_action(&PluginKeyAction::EnterTextInput {
            prompt: "Type feedback:".to_owned(),
        });
        assert!(shell.is_input_enabled());
        assert!(shell.activity_lines.last().unwrap().contains("feedback"));
    }

    #[test]
    fn apply_plugin_action_show_message() {
        let mut shell = empty_shell();
        shell.apply_plugin_action(&PluginKeyAction::ShowMessage("Agent paused.".to_owned()));
        assert!(shell.activity_lines.last().unwrap().contains("paused"));
    }

    #[test]
    fn chat_input_type_and_send() {
        let mut shell = interactive_shell();

        // Type text (non-keybinding chars go to buffer)
        shell.handle_key(KeyCode::Char('f')); // not a keybinding in Running
        shell.handle_key(KeyCode::Char('i'));
        shell.handle_key(KeyCode::Char('x'));
        assert_eq!(shell.text_input_buffer(), "fix");

        // Submit
        shell.handle_key(KeyCode::Enter);
        assert_eq!(shell.take_text_input(), Some("fix".to_owned()));
        assert!(shell.text_input_buffer().is_empty());
    }

    #[test]
    fn chat_input_esc_clears() {
        let mut shell = interactive_shell();
        shell.handle_key(KeyCode::Char('a'));
        shell.handle_key(KeyCode::Char('b'));
        assert_eq!(shell.text_input_buffer(), "ab");

        shell.handle_key(KeyCode::Esc);
        assert!(shell.text_input_buffer().is_empty());
        assert!(shell.take_text_input().is_none());
    }

    #[test]
    fn chat_input_backspace_deletes() {
        let mut shell = interactive_shell();
        shell.handle_key(KeyCode::Char('a'));
        shell.handle_key(KeyCode::Char('b'));
        shell.handle_key(KeyCode::Backspace);
        assert_eq!(shell.text_input_buffer(), "a");
    }

    #[test]
    fn chat_input_empty_enter_does_nothing() {
        let mut shell = interactive_shell();
        shell.handle_key(KeyCode::Enter);
        assert!(shell.take_text_input().is_none());
    }

    #[test]
    fn chat_input_keybinding_while_typing_goes_to_buffer() {
        let mut shell = interactive_shell();
        // Start typing
        shell.handle_key(KeyCode::Char('h'));
        // Now 'p' goes to buffer (not keybinding) because we're typing
        shell.handle_key(KeyCode::Char('p'));
        assert_eq!(shell.text_input_buffer(), "hp");
    }

    #[test]
    fn no_chat_input_when_input_disabled() {
        let mut shell = empty_shell(); // input not enabled = read-only
        shell.handle_key(KeyCode::Char('a'));
        // Without input enabled, chars are not captured
        assert!(shell.text_input_buffer().is_empty());
    }

    #[test]
    fn explicit_enter_text_input_activates() {
        let mut shell = interactive_shell();
        shell.apply_plugin_action(&PluginKeyAction::EnterTextInput {
            prompt: "Feedback:".to_owned(),
        });
        assert!(shell.is_input_enabled());
        // Now typing works
        shell.handle_key(KeyCode::Char('x'));
        assert_eq!(shell.text_input_buffer(), "x");
    }

    // ── Rendering snapshot tests ─────────────────────────────────

    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn test_shell() -> TuiShell {
        let mut shell = TuiShell::new(TuiConfig {
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

    fn empty_shell() -> TuiShell {
        TuiShell::new(TuiConfig {
            title: "Test".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        })
    }

    /// Shell with plugin keybindings (interactive mode — has input bar).
    /// Shell with input enabled (simulates guided plugin active).
    fn interactive_shell() -> TuiShell {
        let mut shell = empty_shell();
        shell.enable_input();
        shell
    }

    #[test]
    fn render_compact_shows_header_with_agent_id() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 80, 24);
        assert!(output.contains("test.claude"));
        assert!(output.contains("[RUNNING]"));
    }

    #[test]
    fn render_compact_shows_activity_lines() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 80, 24);
        assert!(output.contains("Tool Call: search"));
        assert!(output.contains("found 3 items"));
    }

    #[test]
    fn render_compact_shows_metrics() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 80, 24);
        assert!(output.contains("Tools: 1"));
    }

    #[test]
    fn render_compact_shows_help_bar() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 80, 24);
        assert!(output.contains("[q]"));
        assert!(output.contains("compact"));
    }

    #[test]
    fn render_compact_no_sidebar() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 80, 24);
        assert!(!output.contains("Plugins"));
    }

    #[test]
    fn render_standard_shows_sidebar() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 140, 40);
        assert!(output.contains("Plugins"));
        assert!(output.contains("standard"));
    }

    #[test]
    fn render_wide_shows_control_panel() {
        let shell = TuiShell::new(TuiConfig {
            title: "Fix Bug".to_owned(),
            agent_id: "test.claude".to_owned(),
            locale: "en".to_owned(),
        });
        let output = render_to_buffer(&shell, 200, 60);
        assert!(output.contains("Control"));
        assert!(output.contains("Plugins"));
        assert!(output.contains("wide"));
    }

    #[test]
    fn render_too_small_shows_error() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 60, 20);
        assert!(output.contains("too small"));
    }

    #[test]
    fn render_paused_state_shows_in_header() {
        let mut shell = test_shell();
        shell.set_state(TuiState::Paused);
        let output = render_to_buffer(&shell, 80, 24);
        assert!(output.contains("[PAUSED]"));
    }

    #[test]
    fn render_progress_gauge_shows_in_wide_header() {
        let mut shell = test_shell();
        shell.set_progress(75);
        let output = render_to_buffer(&shell, 100, 24);
        assert!(output.contains("test.claude"));
    }

    // ── process_event tests ──────────────────────────────────────

    use crate::events::AgentEvent;

    #[test]
    fn process_event_text_delta_appends() {
        let mut shell = empty_shell();
        shell.process_event(&AgentEvent::TextDelta("Hello".to_owned()));
        assert_eq!(shell.activity_lines.len(), 1);
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
        assert!(output.contains("Findings"));
        assert!(output.contains("Sprint"));
        assert!(output.contains("3 issues"));
    }

    #[test]
    fn render_standard_empty_panels_shows_placeholder() {
        let shell = test_shell();
        let output = render_to_buffer(&shell, 140, 40);
        assert!(output.contains("no panels"));
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

    // ── Help bar with plugin keybindings ─────────────────────────

    #[test]
    fn help_bar_shows_plugin_keybindings() {
        let mut shell = test_shell();
        shell.set_plugin_keybindings(vec![
            RegisteredKeybinding {
                key: 'p',
                description: "Pause".to_owned(),
                plugin_id: "test".to_owned(),
                active_states: vec![], // always active
            },
            RegisteredKeybinding {
                key: 'f',
                description: "Feedback".to_owned(),
                plugin_id: "test".to_owned(),
                active_states: vec!["Paused".to_owned()], // only when paused
            },
        ]);
        let output = render_to_buffer(&shell, 80, 24);
        // Running state → 'p' should appear, 'f' should not
        assert!(
            output.contains("[p]"),
            "help should show [p], got:\n{output}"
        );
        assert!(
            !output.contains("[f]"),
            "help should NOT show [f] in Running, got:\n{output}"
        );
    }

    #[test]
    fn help_bar_shows_state_specific_bindings() {
        let mut shell = test_shell();
        shell.set_plugin_keybindings(vec![RegisteredKeybinding {
            key: 'f',
            description: "Feedback".to_owned(),
            plugin_id: "test".to_owned(),
            active_states: vec!["Paused".to_owned()],
        }]);
        shell.set_state(TuiState::Paused);
        let output = render_to_buffer(&shell, 80, 24);
        assert!(
            output.contains("[f]"),
            "help should show [f] when Paused, got:\n{output}"
        );
    }
}
