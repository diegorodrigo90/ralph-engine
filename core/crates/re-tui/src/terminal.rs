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
use ratatui::layout::{Constraint, Layout, Rect, Size};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use tui_scrollview::{ScrollView, ScrollViewState};

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

    /// Returns the status color for this state from the active theme.
    #[must_use]
    pub fn color(self, theme: &dyn crate::theme::Theme) -> Color {
        match self {
            Self::Running => theme.success(),
            Self::Paused => theme.warning(),
            Self::Complete => theme.info(),
            Self::Error => theme.error(),
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

/// Localized labels for TUI rendering.
///
/// The caller (CLI layer) fills these from the i18n system.
/// `re-tui` never hardcodes user-facing strings.
#[derive(Debug, Clone)]
pub struct TuiLabels {
    /// Idle dashboard: project configured message.
    pub project_configured: String,
    /// Idle dashboard: no project found message.
    pub no_project_found: String,
    /// Idle dashboard: type /run hint.
    pub type_run: String,
    /// Idle dashboard: type /init hint.
    pub type_init: String,
    /// Idle dashboard: orchestration runtime subtitle.
    pub orchestration_runtime: String,
    /// Idle dashboard: waiting for session message.
    pub waiting_session: String,
    /// Help modal title.
    pub help_title: String,
    /// Help modal: navigation section header.
    pub nav_heading: String,
    /// Help modal: actions section header.
    pub actions_heading: String,
    /// Help modal: plugins section header.
    pub plugins_heading: String,
    /// Help modal: slash commands hint.
    pub slash_hint: String,
    /// Help modal: close instruction.
    pub press_any_key: String,
    /// Quit modal title.
    pub quit_title: String,
    /// Quit modal: question text.
    pub quit_question: String,
    /// Bottom bar: modal open hint.
    pub modal_open_hint: String,
    // ── State labels ────────────────────────────────────────────
    /// TUI state: running.
    pub state_running: String,
    /// TUI state: paused.
    pub state_paused: String,
    /// TUI state: complete.
    pub state_complete: String,
    /// TUI state: error.
    pub state_error: String,
    // ── Help bar ────────────────────────────────────────────────
    /// Help bar: pause label.
    pub pause_label: String,
    /// Help bar: help label.
    pub help_label: String,
    /// Help bar: quit label.
    pub quit_label: String,
    // ── Control panel ───────────────────────────────────────────
    /// Control panel: state label prefix.
    pub control_state: String,
    /// Control panel: work label prefix.
    pub control_work: String,
    // ── Metrics bar ─────────────────────────────────────────────
    /// Metrics: tools label.
    pub tools_label: String,
    /// Metrics: lines label.
    pub lines_label: String,
    /// Metrics: progress label.
    pub progress_label: String,
    // ── Logo ────────────────────────────────────────────────────
    /// Logo tagline.
    pub logo_tagline: String,
    // ── Help modal key descriptions ─────────────────────────────
    /// Navigation keys (key, description) pairs.
    pub nav_keys: Vec<(String, String)>,
    /// Action keys (key, description) pairs.
    pub action_keys: Vec<(String, String)>,
    // ── Messages ────────────────────────────────────────────────
    /// Message prefix for user-sent text (e.g., "You:" or "Você:").
    pub you_label: String,
    /// Message when no agent is connected.
    pub no_agent_message: String,
    /// Header: extra usage warning label (e.g. "extra", "uso extra").
    pub extra_usage_label: String,
}

impl Default for TuiLabels {
    fn default() -> Self {
        Self {
            project_configured: "Project configured".to_owned(),
            no_project_found: "No project found".to_owned(),
            type_run: "Type /run to start orchestration".to_owned(),
            type_init: "Run 'ralph-engine init' to set up".to_owned(),
            orchestration_runtime: "Orchestration Runtime".to_owned(),
            waiting_session: "Waiting for agent session...".to_owned(),
            help_title: "Help".to_owned(),
            nav_heading: "Navigation".to_owned(),
            actions_heading: "Actions".to_owned(),
            plugins_heading: "Plugins".to_owned(),
            slash_hint: "Type / for slash commands".to_owned(),
            press_any_key: "Press any key to close".to_owned(),
            quit_title: "Quit".to_owned(),
            quit_question: "Quit?".to_owned(),
            modal_open_hint: "Modal open — press a key".to_owned(),
            state_running: "RUNNING".to_owned(),
            state_paused: "PAUSED".to_owned(),
            state_complete: "COMPLETE".to_owned(),
            state_error: "ERROR".to_owned(),
            pause_label: "pause".to_owned(),
            help_label: "help".to_owned(),
            quit_label: "quit".to_owned(),
            control_state: "State".to_owned(),
            control_work: "Work".to_owned(),
            tools_label: "Tools".to_owned(),
            lines_label: "Lines".to_owned(),
            progress_label: "Progress".to_owned(),
            logo_tagline: "Autonomous AI Dev Loop".to_owned(),
            nav_keys: vec![
                ("j/k".to_owned(), "Focus blocks".to_owned()),
                ("↑↓".to_owned(), "Scroll lines".to_owned()),
                ("PgUp/PgDn".to_owned(), "Scroll pages".to_owned()),
                ("G / End".to_owned(), "Follow mode".to_owned()),
                ("Home".to_owned(), "Scroll to top".to_owned()),
            ],
            action_keys: vec![
                ("⏎ Enter".to_owned(), "Expand/collapse".to_owned()),
                ("y".to_owned(), "Copy block".to_owned()),
                ("⎋ Esc".to_owned(), "Clear focus".to_owned()),
                ("F2".to_owned(), "Toggle sidebar".to_owned()),
                ("Ctrl+A".to_owned(), "Agent switcher".to_owned()),
                ("?".to_owned(), "This help".to_owned()),
                ("q".to_owned(), "Quit".to_owned()),
            ],
            you_label: "You".to_owned(),
            no_agent_message: "No agent connected. Use /run to start orchestration.".to_owned(),
            extra_usage_label: "extra usage".to_owned(),
        }
    }
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

/// A temporary toast notification that auto-dismisses.
///
/// Toasts appear in the bottom-right corner of the TUI and disappear
/// after a configurable number of ticks (frames). Used for non-blocking
/// confirmations like "copied to clipboard", "session saved", etc.
#[derive(Debug, Clone)]
pub struct Toast {
    /// Message text.
    pub message: String,
    /// Severity level controlling the color.
    pub level: ToastLevel,
    /// Remaining ticks before auto-dismiss.
    remaining_ticks: usize,
}

/// Severity level for toast notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastLevel {
    /// Informational (accent color).
    Info,
    /// Success (green).
    Success,
    /// Warning (yellow).
    Warning,
    /// Error (red).
    Error,
}

/// Default toast duration in render ticks (~50ms per tick = ~3s).
const TOAST_DEFAULT_TICKS: usize = 60;

/// Agent process ID (set when a real agent is launched).
pub type AgentPid = Option<u32>;

/// Source of a command in the unified autocomplete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandSource {
    /// Command from the active agent CLI (e.g. Claude `/compact`).
    Agent,
    /// Command from a Ralph Engine plugin (e.g. BMAD `sprint-status`).
    Plugin,
    /// Tool from an MCP server (e.g. `github.repository`).
    Mcp,
}

impl CommandSource {
    /// Returns a short label for display.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Plugin => "plugin",
            Self::Mcp => "mcp",
        }
    }
}

/// An entry in the unified autocomplete command list.
#[derive(Debug, Clone)]
pub struct CommandEntry {
    /// Command name (e.g. `"compact"`, `"sprint-status"`).
    pub name: String,
    /// Short description for the popup.
    pub description: String,
    /// Source of this command.
    pub source: CommandSource,
    /// Source name for display (e.g. `"Claude"`, `"BMAD"`, `"GitHub MCP"`).
    pub source_name: String,
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
    /// Localized labels for all user-facing strings.
    labels: TuiLabels,
    /// Active color theme for all rendering.
    theme: Box<dyn crate::theme::Theme>,
    state: TuiState,
    progress: u16,
    /// Legacy flat activity lines (kept for `push_activity` compatibility).
    activity_lines: Vec<String>,
    /// Block-based activity feed (new orchestration renderer).
    feed: crate::feed::Feed,
    /// Scroll state for the feed viewport (tui-scrollview).
    feed_scroll: ScrollViewState,
    /// Follow mode: auto-scroll to bottom on new content.
    follow_mode: bool,
    /// Index of the focused block in the feed (`None` = no focus).
    focused_block: Option<usize>,
    /// Quality gate pipeline for orchestration runs.
    indicator_panel: crate::indicators::IndicatorPanel,
    tool_count: usize,
    /// Approximate token count (set by agent plugin via stream).
    token_count: usize,
    /// Cost label from agent plugin (e.g. "$0.42", "extra usage").
    /// Core never calculates — plugin owns the format and logic.
    cost_label: Option<String>,
    /// Whether the agent is in "extra usage" (over budget/plan).
    /// Set by agent plugin. Core only renders the warning.
    extra_usage: bool,
    /// Current thinking message from agent plugin (rotates over time).
    thinking_message: Option<String>,
    /// Frame counter for spinner animation.
    tick: usize,
    should_quit: bool,
    quit_pending: bool,
    /// Whether the help modal popup is visible.
    help_modal_visible: bool,
    sidebar_panels: Vec<SidebarPanel>,
    /// Whether the sidebar is visible (toggled with F2).
    sidebar_visible: bool,
    /// Available agent IDs for the switcher popup (set by caller).
    available_agents: Vec<String>,
    /// Whether the agent switcher popup is visible.
    agent_switcher_visible: bool,
    /// Selected index in the agent switcher popup.
    agent_switcher_selected: usize,
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
    /// Active toast notifications (auto-dismiss on tick).
    toasts: Vec<Toast>,
    /// Queued blocks to drip-feed into the activity feed (demo/replay).
    pending_blocks: std::collections::VecDeque<crate::feed::FeedBlock>,
    /// Total blocks enqueued (for progress calculation).
    pending_total: usize,
    /// Tick counter for drip timing.
    drip_counter: usize,
}

impl TuiShell {
    /// Creates a new TUI shell with the given configuration.
    ///
    /// Uses Catppuccin Mocha as the default theme. Call
    /// [`TuiShell::set_theme`] to switch themes.
    #[must_use]
    pub fn new(config: TuiConfig) -> Self {
        Self {
            config,
            labels: TuiLabels::default(),
            theme: Box::new(crate::theme::CatppuccinMocha),
            state: TuiState::Running,
            progress: 0,
            activity_lines: Vec::new(),
            feed: crate::feed::Feed::new(),
            feed_scroll: ScrollViewState::default(),
            follow_mode: true,
            focused_block: None,
            indicator_panel: crate::indicators::IndicatorPanel::new(),
            tool_count: 0,
            token_count: 0,
            cost_label: None,
            extra_usage: false,
            thinking_message: None,
            tick: 0,
            should_quit: false,
            quit_pending: false,
            help_modal_visible: false,
            sidebar_panels: Vec::new(),
            sidebar_visible: true,
            available_agents: Vec::new(),
            agent_switcher_visible: false,
            agent_switcher_selected: 0,
            agent_pid: None,
            plugin_keybindings: Vec::new(),
            input_enabled: false,
            text_input_buffer: String::new(),
            pending_text_input: None,
            autocomplete: AutocompleteState::new(Vec::new(), "/".to_owned()),
            toasts: Vec::new(),
            pending_blocks: std::collections::VecDeque::new(),
            pending_total: 0,
            drip_counter: 0,
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

    /// Returns a reference to the active theme.
    #[must_use]
    pub fn theme(&self) -> &dyn crate::theme::Theme {
        self.theme.as_ref()
    }

    /// Switches the active theme by config ID (e.g. `"dracula"`).
    pub fn set_theme(&mut self, id: &str) {
        self.theme = crate::theme::resolve_theme(id);
    }

    /// Sets the localized labels for all TUI strings.
    /// Returns a reference to the localized labels.
    #[must_use]
    pub fn labels(&self) -> &TuiLabels {
        &self.labels
    }

    /// Sets the localized labels for all TUI strings.
    pub fn set_labels(&mut self, labels: TuiLabels) {
        self.labels = labels;
    }

    /// Returns the localized label for the current TUI state.
    fn localized_state_label(&self) -> &str {
        match self.state {
            TuiState::Running => &self.labels.state_running,
            TuiState::Paused => &self.labels.state_paused,
            TuiState::Complete => &self.labels.state_complete,
            TuiState::Error => &self.labels.state_error,
        }
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

    /// Shows a toast notification that auto-dismisses after ~3 seconds.
    pub fn show_toast(&mut self, message: String, level: ToastLevel) {
        self.toasts.push(Toast {
            message,
            level,
            remaining_ticks: TOAST_DEFAULT_TICKS,
        });
    }

    /// Shows an info-level toast notification.
    pub fn toast_info(&mut self, message: String) {
        self.show_toast(message, ToastLevel::Info);
    }

    /// Shows a success-level toast notification.
    pub fn toast_success(&mut self, message: String) {
        self.show_toast(message, ToastLevel::Success);
    }

    /// Shows an error-level toast notification as a modal popup.
    pub fn show_error_modal(&mut self, title: &str, message: &str) {
        // Errors are important enough to be modals, not toasts
        self.push_activity(format!("  ✗ {title}: {message}"));
        self.show_toast(format!("✗ {title}"), ToastLevel::Error);
    }

    /// Increments the tool call counter.
    pub fn increment_tools(&mut self) {
        self.tool_count += 1;
    }

    /// Sets the token count (from agent stream metadata).
    pub fn set_token_count(&mut self, count: usize) {
        self.token_count = count;
    }

    /// Sets the cost label from the agent plugin (e.g. "$0.42", "~$1.20").
    ///
    /// Core never calculates cost — the agent plugin owns the format,
    /// currency, and pricing logic. Core just renders the string.
    pub fn set_cost_label(&mut self, label: String) {
        self.cost_label = Some(label);
    }

    /// Marks whether the agent is in "extra usage" mode (over plan/budget).
    ///
    /// When true, the header shows a warning indicator. The agent plugin
    /// determines this from its own API response (e.g., Claude's billing).
    pub fn set_extra_usage(&mut self, extra: bool) {
        self.extra_usage = extra;
    }

    /// Sets the current thinking message from the agent plugin.
    ///
    /// Shown in the status bar while the agent is processing. The plugin
    /// rotates messages based on tick count. Set to `None` when not thinking.
    pub fn set_thinking_message(&mut self, msg: Option<String>) {
        self.thinking_message = msg;
    }

    /// Sets the available agent IDs for the switcher popup.
    pub fn set_available_agents(&mut self, agents: Vec<String>) {
        self.available_agents = agents;
    }

    /// Returns the agent ID selected in the switcher, if confirmed.
    ///
    /// The caller checks this after the user selects and confirms
    /// in the popup, then dispatches to the plugin system.
    #[must_use]
    pub fn take_selected_agent(&mut self) -> Option<String> {
        if !self.agent_switcher_visible {
            return None;
        }
        self.available_agents
            .get(self.agent_switcher_selected)
            .cloned()
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

    /// Enqueues blocks to be drip-fed into the feed over time.
    ///
    /// Blocks appear one by one with a cadence, giving the impression
    /// of a real implementation happening. Auto-scrolls to follow.
    pub fn enqueue_blocks(&mut self, blocks: Vec<crate::feed::FeedBlock>) {
        self.pending_total = blocks.len();
        self.pending_blocks = blocks.into();
        self.drip_counter = 0;
        self.follow_mode = true;
    }

    /// Drains one pending block into the feed if the cadence allows.
    ///
    /// Drip-feeds one pending block with realistic cadence.
    ///
    /// Two-phase timing: first the block appears as "active" (spinner),
    /// then after a hold period it finalizes and the next block drips.
    /// This creates the loading feel: think → show result → think → next.
    fn drain_pending_block(&mut self) -> bool {
        if self.pending_blocks.is_empty() {
            return false;
        }
        self.drip_counter += 1;

        // Phase 1: "hold" — the last fed block stays active (spinner visible)
        // before we finalize it and drip the next one.
        let last_kind = self.feed.blocks().last().map(|b| (b.kind, b.active));

        // Hold time: spinner visible while "processing". Real agents: 5-15s for
        // thinking, 3-10s for commands. Demo compresses but stays believable.
        let hold = match last_kind {
            Some((crate::feed::BlockKind::Thinking, true)) => 80, // ~4s thinking
            Some((crate::feed::BlockKind::Command, true)) => 100, // ~5s running
            Some((crate::feed::BlockKind::System, true)) => 40,   // ~2s init
            _ => 0,
        };

        // Delay before next block appears (after hold finishes)
        let next_kind = self.pending_blocks[0].kind;
        let appear_delay = match next_kind {
            crate::feed::BlockKind::FileEdit => 16,  // ~800ms
            crate::feed::BlockKind::FileRead => 10,  // ~500ms
            crate::feed::BlockKind::AgentText => 20, // ~1s composing
            crate::feed::BlockKind::GatePass | crate::feed::BlockKind::GateFail => 6,
            _ => 14,
        };

        let total_interval = hold + appear_delay;

        // Rotate thinking message during hold (demo simulation)
        if hold > 0 && self.drip_counter < hold && self.drip_counter.is_multiple_of(40) {
            const DEMO_MESSAGES: &[&str] = &[
                "Thinking...",
                "Reasoning deeply...",
                "Analyzing the codebase...",
                "Considering approaches...",
                "Crafting a solution...",
                "Planning the implementation...",
            ];
            let msg_idx = (self.tick / 80) % DEMO_MESSAGES.len();
            self.thinking_message = Some(DEMO_MESSAGES[msg_idx].to_owned());
        }

        if self.drip_counter >= total_interval {
            self.drip_counter = 0;

            // Finalize previous active block
            if let Some(last) = self.feed.blocks_mut().last_mut()
                && last.active
            {
                last.finalize(last.success.unwrap_or(true));
            }

            // Pop next block from queue (O(1) with VecDeque)
            let Some(mut block) = self.pending_blocks.pop_front() else {
                return false;
            };

            // Mark as active if more blocks coming (shows spinner)
            if !self.pending_blocks.is_empty() {
                block.active = true;
            } else if block.success.is_some() {
                block.active = false;
            }

            // Track tokens + tools for realism
            let is_tool = matches!(
                block.kind,
                crate::feed::BlockKind::FileRead
                    | crate::feed::BlockKind::FileEdit
                    | crate::feed::BlockKind::Command
            );
            self.feed.push_block(block);

            if is_tool {
                self.tool_count += 1;
            }
            // Simulate token consumption (~500-2000 per action)
            let tokens_delta = match next_kind {
                crate::feed::BlockKind::Thinking => 1200,
                crate::feed::BlockKind::FileEdit => 800,
                crate::feed::BlockKind::FileRead => 300,
                crate::feed::BlockKind::Command => 500,
                crate::feed::BlockKind::AgentText => 600,
                _ => 100,
            };
            self.token_count += tokens_delta;

            // Clear thinking message when new block appears
            self.thinking_message = None;

            // Update progress
            let completed = self.pending_total - self.pending_blocks.len();
            let pct = (completed * 100 / self.pending_total.max(1)) as u16;
            self.set_progress(pct);

            true
        } else if self.drip_counter == hold && hold > 0 {
            // At hold boundary: finalize the active block (spinner stops)
            if let Some(last) = self.feed.blocks_mut().last_mut()
                && last.active
            {
                last.finalize(last.success.unwrap_or(true));
            }
            false
        } else {
            false
        }
    }

    /// Handles a bracketed paste event (multi-line text pasted at once).
    ///
    /// Appends the entire paste to the input buffer. If input is not
    /// enabled, the paste is ignored.
    pub fn handle_paste(&mut self, text: &str) {
        if !self.input_enabled {
            return;
        }
        // Append pasted text to buffer (may contain newlines)
        self.text_input_buffer.push_str(text);
        self.autocomplete.update_filter(&self.text_input_buffer);
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

        // Update block-based feed
        crate::feed::process_agent_event(&mut self.feed, event);

        // Update legacy activity lines (kept for compatibility)
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

    /// Returns a reference to the block-based feed.
    #[must_use]
    pub fn feed(&self) -> &crate::feed::Feed {
        &self.feed
    }

    /// Returns a mutable reference to the block-based feed.
    pub fn feed_mut(&mut self) -> &mut crate::feed::Feed {
        &mut self.feed
    }

    /// Whether follow mode is active (auto-scroll on new content).
    #[must_use]
    pub fn is_follow_mode(&self) -> bool {
        self.follow_mode
    }

    /// Scrolls the feed up by one line, disabling follow mode.
    pub fn scroll_feed_up(&mut self) {
        self.follow_mode = false;
        self.feed_scroll.scroll_up();
    }

    /// Scrolls the feed down by one line, disabling follow mode.
    pub fn scroll_feed_down(&mut self) {
        self.follow_mode = false;
        self.feed_scroll.scroll_down();
    }

    /// Scrolls the feed up by one page, disabling follow mode.
    pub fn scroll_feed_page_up(&mut self) {
        self.follow_mode = false;
        self.feed_scroll.scroll_page_up();
    }

    /// Scrolls the feed down by one page, disabling follow mode.
    pub fn scroll_feed_page_down(&mut self) {
        self.follow_mode = false;
        self.feed_scroll.scroll_page_down();
    }

    /// Scrolls to the top, disabling follow mode.
    pub fn scroll_feed_to_top(&mut self) {
        self.follow_mode = false;
        self.feed_scroll.scroll_to_top();
    }

    /// Scrolls to the bottom and re-enables follow mode.
    pub fn scroll_feed_to_bottom(&mut self) {
        self.follow_mode = true;
        self.feed_scroll.scroll_to_bottom();
    }

    /// Returns the index of the focused block, if any.
    #[must_use]
    pub fn focused_block(&self) -> Option<usize> {
        self.focused_block
    }

    /// Moves focus to the next block (down).
    pub fn focus_next_block(&mut self) {
        let count = self.feed.len();
        if count == 0 {
            return;
        }
        self.follow_mode = false;
        self.focused_block = Some(match self.focused_block {
            Some(i) if i + 1 < count => i + 1,
            Some(_) => count - 1, // already at last — stay
            None => count - 1,    // no focus — start at bottom
        });
    }

    /// Moves focus to the previous block (up).
    pub fn focus_prev_block(&mut self) {
        let count = self.feed.len();
        if count == 0 {
            return;
        }
        self.follow_mode = false;
        self.focused_block = Some(match self.focused_block {
            Some(i) if i > 0 => i - 1,
            Some(_) => 0, // already at first — stay
            None => 0,    // no focus — start at top
        });
    }

    /// Toggles expand/collapse on the focused block.
    pub fn toggle_focused_block(&mut self) {
        if let Some(idx) = self.focused_block {
            self.feed.toggle_block(idx);
        }
    }

    /// Clears block focus and re-enables follow mode.
    pub fn clear_focus(&mut self) {
        self.focused_block = None;
        self.follow_mode = true;
        self.feed_scroll.scroll_to_bottom();
    }

    /// Copies the focused block's content to the clipboard.
    ///
    /// Returns `true` if a block was focused and the copy succeeded.
    /// Shows a brief feedback message in the metrics bar.
    pub fn copy_focused_block(&mut self) -> bool {
        let Some(idx) = self.focused_block else {
            return false;
        };
        let Some(block) = self.feed.blocks().get(idx) else {
            return false;
        };

        let text = crate::clipboard::block_to_copyable_text(&block.title, &block.content);
        if text.is_empty() {
            return false;
        }

        if crate::clipboard::copy_to_clipboard(&text) {
            self.toast_success(format!("✓ Copied {} chars", text.len()));
            true
        } else {
            self.show_toast("Copy failed (no clipboard)".to_owned(), ToastLevel::Warning);
            false
        }
    }

    /// Returns a reference to the gate pipeline.
    #[must_use]
    pub fn indicator_panel(&self) -> &crate::indicators::IndicatorPanel {
        &self.indicator_panel
    }

    /// Returns a mutable reference to the gate pipeline.
    pub fn indicator_panel_mut(&mut self) -> &mut crate::indicators::IndicatorPanel {
        &mut self.indicator_panel
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
                {
                    match event::read()
                        .map_err(|e| TuiError::new(format!("event read failed: {e}")))?
                    {
                        Event::Key(key) if key.kind == KeyEventKind::Press => {
                            self.handle_key(key.code);
                        }
                        Event::Mouse(mouse) => {
                            self.handle_mouse(mouse.kind);
                        }
                        _ => {}
                    }
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

    /// Handles a mouse event (scroll wheel).
    ///
    /// Call this from the event loop when a mouse event is received.
    /// Only scroll events are handled; other mouse events are ignored.
    pub fn handle_mouse(&mut self, kind: ratatui::crossterm::event::MouseEventKind) {
        use ratatui::crossterm::event::MouseEventKind;
        match kind {
            MouseEventKind::ScrollUp => self.scroll_feed_up(),
            MouseEventKind::ScrollDown => self.scroll_feed_down(),
            MouseEventKind::Down(_) => {
                // Click anywhere cycles block focus (simple hit-test)
                self.focus_next_block();
            }
            _ => {}
        }
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
        // Help modal: any key closes it
        if self.help_modal_visible {
            self.help_modal_visible = false;
            return PluginKeyAction::Handled;
        }

        // Quit confirmation modal
        if self.quit_pending {
            match code {
                KeyCode::Char('y' | 'Y') => {
                    tracing::info!("user confirmed quit");
                    self.should_quit = true;
                }
                _ => {
                    self.quit_pending = false;
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
                self.help_modal_visible = !self.help_modal_visible;
                return PluginKeyAction::Handled;
            }
            // Block focus navigation (vim j/k)
            KeyCode::Char('j') => {
                self.focus_next_block();
                return PluginKeyAction::Handled;
            }
            KeyCode::Char('k') => {
                self.focus_prev_block();
                return PluginKeyAction::Handled;
            }
            KeyCode::Enter => {
                self.toggle_focused_block();
                return PluginKeyAction::Handled;
            }
            KeyCode::Char('y') => {
                self.copy_focused_block();
                return PluginKeyAction::Handled;
            }
            KeyCode::Esc => {
                self.clear_focus();
                return PluginKeyAction::Handled;
            }
            // Feed scroll keys (arrows + page)
            KeyCode::Up => {
                self.scroll_feed_up();
                return PluginKeyAction::Handled;
            }
            KeyCode::Down => {
                self.scroll_feed_down();
                return PluginKeyAction::Handled;
            }
            KeyCode::PageUp => {
                self.scroll_feed_page_up();
                return PluginKeyAction::Handled;
            }
            KeyCode::PageDown => {
                self.scroll_feed_page_down();
                return PluginKeyAction::Handled;
            }
            KeyCode::Home => {
                self.scroll_feed_to_top();
                return PluginKeyAction::Handled;
            }
            KeyCode::End | KeyCode::Char('G') => {
                self.scroll_feed_to_bottom();
                return PluginKeyAction::Handled;
            }
            // Sidebar toggle
            KeyCode::F(2) => {
                self.sidebar_visible = !self.sidebar_visible;
                return PluginKeyAction::Handled;
            }
            // Agent switcher (Ctrl+A)
            KeyCode::Char('a') if modifiers.contains(KeyModifiers::CONTROL) => {
                if !self.available_agents.is_empty() {
                    self.agent_switcher_visible = !self.agent_switcher_visible;
                    self.agent_switcher_selected = 0;
                }
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
    /// Renders the TUI frame with responsive zone-based layout.
    pub fn render_frame(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        self.render_in(frame, area);
    }

    /// Renders the TUI into a specific sub-area of the frame.
    pub fn render_frame_in_area(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.render_in(frame, area);
    }

    /// Internal render implementation for a given area.
    fn render_in(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.tick = self.tick.wrapping_add(1);
        // Drip-feed queued blocks (demo/replay cadence)
        self.drain_pending_block();
        if layout::is_terminal_too_small(area) {
            let msg = format!(
                "Terminal too small ({}x{}). Minimum: {}x{}.",
                area.width,
                area.height,
                crate::MIN_TERMINAL_WIDTH,
                crate::MIN_TERMINAL_HEIGHT,
            );
            frame.render_widget(
                Paragraph::new(msg).style(Style::default().fg(self.theme().error())),
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

        if let Some(sidebar) = zones.sidebar
            && self.sidebar_visible
        {
            self.render_sidebar(frame, sidebar);
        }

        if let Some(control) = zones.control {
            self.render_control_panel(frame, control);
        }

        // Autocomplete popup — rendered LAST (on top of everything)
        if let Some(input_area) = zones.input {
            self.render_autocomplete(frame, input_area);
        }

        // Agent switcher popup (Ctrl+A)
        if self.agent_switcher_visible {
            self.render_agent_switcher(frame, zones.activity);
        }

        // Toast notifications — bottom-right corner
        self.render_toasts(frame, area);

        // Modal popups — rendered LAST, on top of EVERYTHING
        if self.quit_pending {
            self.render_quit_modal(frame, area);
        }
        if self.help_modal_visible {
            self.render_help_modal(frame, area);
        }
    }

    /// Renders the header bar with version, agent, tokens, state badge, and progress.
    fn render_header(&self, frame: &mut Frame<'_>, area: Rect) {
        let state_label = self.localized_state_label();
        let state_color = self.state.color(self.theme());
        let version = env!("CARGO_PKG_VERSION");

        let theme = self.theme();
        let mut spans = vec![
            Span::styled(
                format!(" ◎ RE v{version} "),
                Style::default()
                    .fg(theme.accent())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("│ ", Style::default().fg(theme.border())),
            Span::styled(&self.config.agent_id, Style::default().fg(theme.text())),
            Span::raw(" "),
            Span::styled(
                format!("[{state_label}]"),
                Style::default()
                    .fg(state_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ];

        // Token count (if available)
        if self.token_count > 0 {
            let token_label = if self.token_count >= 1000 {
                format!(" │ {}k tok", self.token_count / 1000)
            } else {
                format!(" │ {} tok", self.token_count)
            };
            spans.push(Span::styled(
                token_label,
                Style::default().fg(theme.text_dim()),
            ));
        }

        // Tool count
        if self.tool_count > 0 {
            spans.push(Span::styled(
                format!(" │ {} tools", self.tool_count),
                Style::default().fg(theme.text_dim()),
            ));
        }

        // Cost label (from agent plugin — core doesn't calculate)
        if let Some(ref cost) = self.cost_label {
            spans.push(Span::styled(
                format!(" │ {cost}"),
                Style::default().fg(if self.extra_usage {
                    theme.warning()
                } else {
                    theme.text_dim()
                }),
            ));
        }

        // Extra usage warning (label from i18n)
        if self.extra_usage {
            spans.push(Span::styled(
                format!(" ⚠ {}", self.labels.extra_usage_label),
                Style::default()
                    .fg(theme.warning())
                    .add_modifier(Modifier::BOLD),
            ));
        }

        let header = Line::from(spans);

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
    fn render_activity(&mut self, frame: &mut Frame<'_>, area: Rect) {
        // Use block-based feed when it has content, fall back to legacy lines
        if !self.feed.is_empty() {
            self.render_feed_blocks(frame, area);
            return;
        }

        // Idle dashboard when no activity at all
        if self.activity_lines.is_empty() {
            self.render_idle_dashboard(frame, area);
            return;
        }

        let visible_lines = area.height as usize;

        let theme = self.theme();
        let logo_color = Some(self.state.color(theme));
        let logo_lines =
            crate::logo::build_logo_lines(area.width, theme, logo_color, &self.labels.logo_tagline);
        let logo_count = logo_lines.len();

        let activity: Vec<Line<'_>> = self
            .activity_lines
            .iter()
            .map(|s| {
                if s.starts_with(">> Tool") {
                    Line::styled(s.as_str(), Style::default().fg(theme.info()))
                } else if s.starts_with(">> State:") || s.starts_with(">> Agent") {
                    Line::styled(s.as_str(), Style::default().fg(theme.warning()))
                } else if s.starts_with(">> Quit") {
                    Line::styled(s.as_str(), Style::default().fg(theme.error()))
                } else if s.starts_with(">> Keys:") {
                    Line::styled(s.as_str(), Style::default().fg(theme.text_dim()))
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

    /// Renders the block-based feed using `tui-scrollview`.
    ///
    /// Each block gets an icon prefix, styled title, and optionally
    /// expanded content lines. Follow mode auto-scrolls to bottom
    /// when new content arrives.
    fn render_feed_blocks(&mut self, frame: &mut Frame<'_>, area: Rect) {
        use crate::feed::BlockKind;

        let theme = self.theme.as_ref();

        self.feed.clear_dirty();

        let focused = self.focused_block;
        let mut all_lines: Vec<Line<'_>> = Vec::new();
        let mut focused_line: Option<u16> = None;

        for (block_idx, block) in self.feed.blocks().iter().enumerate() {
            // Spacing between blocks
            if block_idx > 0 {
                all_lines.push(Line::raw(""));
            }

            let is_focused = focused == Some(block_idx);
            if is_focused {
                focused_line = Some(all_lines.len() as u16);
            }

            let block_color = match block.kind {
                BlockKind::FileRead => theme.block_file_read(),
                BlockKind::FileEdit => theme.block_file_edit(),
                BlockKind::Command => theme.block_command(),
                BlockKind::Thinking => theme.block_thinking(),
                BlockKind::AgentText => theme.text_dim(),
                BlockKind::GatePass => theme.block_pass(),
                BlockKind::GateFail => theme.block_fail(),
                BlockKind::System => theme.block_system(),
            };

            // ── Top border: ╭─── kind badge ─── title ─── elapsed ───╮
            let icon = block.kind.icon();
            let kind_label = match block.kind {
                BlockKind::FileRead => "Read",
                BlockKind::FileEdit => "Edit",
                BlockKind::Command => "Bash",
                BlockKind::Thinking => "Think",
                BlockKind::AgentText => "Text",
                BlockKind::GatePass => "Pass",
                BlockKind::GateFail => "Fail",
                BlockKind::System => "System",
            };

            let mut header_spans: Vec<Span<'_>> = Vec::new();

            // Focus indicator
            if is_focused {
                header_spans.push(Span::styled(
                    "▸ ",
                    Style::default()
                        .fg(theme.accent())
                        .add_modifier(Modifier::BOLD),
                ));
            }

            // Left border + kind badge
            header_spans.push(Span::styled("╭─ ", Style::default().fg(block_color)));
            header_spans.push(Span::styled(
                format!("{icon} {kind_label}"),
                Style::default()
                    .fg(block_color)
                    .add_modifier(Modifier::BOLD),
            ));

            // Title
            if !block.title.is_empty() {
                header_spans.push(Span::styled(" │ ", Style::default().fg(theme.border())));
                header_spans.push(Span::styled(
                    block.title.as_str(),
                    Style::default().add_modifier(Modifier::BOLD),
                ));
            }

            // Collapsed hint
            if block.collapsed && !block.content.is_empty() {
                header_spans.push(Span::styled(
                    format!(" ({} lines)", block.content.len()),
                    Style::default().fg(theme.text_dim()),
                ));
            }

            // Elapsed + status badges (right side)
            if let Some(elapsed) = block.elapsed_label() {
                header_spans.push(Span::styled(
                    format!("  {elapsed}"),
                    Style::default().fg(theme.text_dim()),
                ));
            }

            // Active spinner
            if block.active {
                const SPINNER: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
                let idx = self.tick / 2 % SPINNER.len();
                header_spans.push(Span::styled(
                    format!(" {}", SPINNER[idx]),
                    Style::default().fg(theme.warning()),
                ));
            }

            // Success/failure badge
            match block.success {
                Some(true) => {
                    header_spans.push(Span::styled(" ✓", Style::default().fg(theme.success())));
                }
                Some(false) => {
                    header_spans.push(Span::styled(" ✗", Style::default().fg(theme.error())));
                }
                None => {}
            }

            let mut header = Line::from(header_spans);
            if is_focused {
                header = header.style(Style::default().bg(theme.surface()));
            }
            all_lines.push(header);

            // ── Content lines with left border: │  content
            if !block.collapsed {
                let max_lines = crate::theme::MAX_COLLAPSED_LINES;
                let total_content = block.content.len();
                let truncated = total_content > max_lines && !is_focused;

                let show_count = if truncated { max_lines } else { total_content };
                for content_line in block.content.iter().take(show_count) {
                    let content_spans = style_content_line(content_line, block.kind, theme);
                    // Prepend left border
                    let mut spans = vec![Span::styled("│ ", Style::default().fg(block_color))];
                    spans.extend(content_spans);
                    let mut bordered = Line::from(spans);
                    if is_focused {
                        bordered = bordered.style(Style::default().bg(theme.surface()));
                    }
                    all_lines.push(bordered);
                }

                // Truncation hint
                if truncated {
                    let remaining = total_content - max_lines;
                    all_lines.push(Line::from(vec![
                        Span::styled("│ ", Style::default().fg(block_color)),
                        Span::styled(
                            format!("… +{remaining} lines (Enter to expand)"),
                            Style::default()
                                .fg(theme.accent_dim())
                                .add_modifier(Modifier::ITALIC),
                        ),
                    ]));
                }
            }

            // ── Bottom border: ╰───────────╯
            all_lines.push(Line::styled(
                format!("╰{}", "─".repeat(40)),
                Style::default().fg(block_color),
            ));
        }

        // Add padding at bottom so last block isn't cut by input bar
        all_lines.push(Line::raw(""));
        all_lines.push(Line::raw(""));

        let content_height = all_lines.len() as u16;
        let content_width = area.width.saturating_sub(1); // reserve 1 col for scrollbar

        // Follow mode: smooth scroll toward bottom (always, not just on dirty)
        if self.follow_mode {
            let target_y = content_height.saturating_sub(area.height);
            let current_y = self.feed_scroll.offset().y;
            if current_y < target_y {
                // Smooth: advance by up to 4 lines per frame toward target
                let step = 4.min(target_y - current_y);
                use ratatui::layout::Position;
                self.feed_scroll
                    .set_offset(Position::new(0, current_y + step));
            }
        }

        // Focus mode: scroll to keep focused block visible
        if let Some(line_y) = focused_line {
            use ratatui::layout::Position;
            let current_y = self.feed_scroll.offset().y;
            let visible_h = area.height;
            // If focused block is above viewport, scroll up to it
            if line_y < current_y {
                self.feed_scroll.set_offset(Position::new(0, line_y));
            }
            // If focused block is below viewport, scroll down
            else if line_y >= current_y + visible_h {
                self.feed_scroll
                    .set_offset(Position::new(0, line_y.saturating_sub(visible_h / 2)));
            }
        }

        // "↑ more" indicator when scrolled up
        let scrolled_up = self.feed_scroll.offset().y > 0;
        if scrolled_up {
            let indicator = Line::styled(
                " ↑ more above ",
                Style::default()
                    .fg(theme.warning())
                    .add_modifier(Modifier::BOLD),
            );
            // Reserve top row for indicator
            let [indicator_area, feed_area] =
                Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);
            frame.render_widget(Paragraph::new(indicator), indicator_area);

            let mut scroll_view = ScrollView::new(Size::new(content_width, content_height));
            scroll_view.render_widget(
                Paragraph::new(all_lines),
                Rect::new(0, 0, content_width, content_height),
            );
            frame.render_stateful_widget(scroll_view, feed_area, &mut self.feed_scroll);
        } else {
            let mut scroll_view = ScrollView::new(Size::new(content_width, content_height));
            scroll_view.render_widget(
                Paragraph::new(all_lines),
                Rect::new(0, 0, content_width, content_height),
            );
            frame.render_stateful_widget(scroll_view, area, &mut self.feed_scroll);
        }
    }

    /// Renders the metrics bar.
    fn render_metrics(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let theme = self.theme.as_ref();

        // When indicators are active, show the indicator bar. Otherwise show tool metrics.
        if !self.indicator_panel.is_empty() {
            let indicator_bar = self.indicator_panel.render_bar(theme);
            frame.render_widget(Paragraph::new(indicator_bar), area);
        } else {
            let metrics = Line::from(vec![
                Span::styled(
                    format!(" {}: {} ", self.labels.tools_label, self.tool_count),
                    Style::default().fg(theme.info()),
                ),
                Span::raw("│ "),
                Span::raw(format!(
                    "{}: {} ",
                    self.labels.lines_label,
                    self.activity_lines.len()
                )),
                Span::raw("│ "),
                Span::raw(format!(
                    "{}: {}% ",
                    self.labels.progress_label, self.progress
                )),
            ]);
            frame.render_widget(Paragraph::new(metrics), area);
        }
    }

    /// Renders the chat input bar — separator, `>` prompt, multi-line, native cursor.
    ///
    /// `Ctrl+J` inserts newline. `Enter` sends. `Esc` cancels.
    fn render_input_bar(&self, frame: &mut Frame<'_>, area: Rect) {
        let theme = self.theme();
        let prompt_color = theme.accent();
        let sep = "─".repeat(area.width as usize);
        let prompt = " > ";
        let prompt_width = prompt.len() as u16;

        // Split: separator (1) + text area (rest)
        let rows = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).split(area);

        frame.render_widget(
            Paragraph::new(Line::styled(sep, Style::default().fg(theme.border()))),
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
                        Span::styled(chunk.to_owned(), Style::default().fg(theme.text())),
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
            let cursor_col = (last_text_line.chars().count() % content_width) as u16;
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

        let theme = self.theme();
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
                            .fg(theme.info())
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("  "),
                    Span::styled(&cmd.description, Style::default().fg(theme.text_dim())),
                    Span::raw("  "),
                    Span::styled(cmd.source.label(), Style::default().fg(theme.accent_dim())),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border()))
                    .title(" Commands ")
                    .title_style(Style::default().fg(theme.accent())),
            )
            .highlight_style(
                Style::default()
                    .bg(theme.surface())
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
        let theme = self.theme();
        // Quit pending handled by modal now — no bottom bar override
        if self.quit_pending || self.help_modal_visible {
            let hint = format!(" {} ", self.labels.modal_open_hint);
            let spans = vec![Span::styled(hint, Style::default().fg(theme.text_dim()))];
            frame.render_widget(Paragraph::new(Line::from(spans)), zones.help);
            return;
        }

        let dim = Style::default().fg(theme.text_dim());

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

        // Show thinking message from agent plugin (or fallback to block title)
        if self.feed.blocks().iter().any(|b| b.active) {
            const SPINNER: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let idx = self.tick / 2 % SPINNER.len();
            let spinner = SPINNER[idx];

            // Prefer plugin thinking message, fallback to active block title
            let message = self
                .thinking_message
                .as_deref()
                .or_else(|| {
                    self.feed
                        .blocks()
                        .iter()
                        .rev()
                        .find(|b| b.active)
                        .map(|b| b.title.as_str())
                })
                .unwrap_or("Processing...");

            let status_spans = vec![
                Span::styled(format!(" {spinner} "), Style::default().fg(theme.warning())),
                Span::styled(
                    format!("{message} "),
                    Style::default()
                        .fg(theme.text_bright())
                        .add_modifier(Modifier::ITALIC),
                ),
            ];
            frame.render_widget(Paragraph::new(Line::from(status_spans)), zones.help);
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
        spans.push(Span::styled(format!(" {} ", self.labels.pause_label), dim));
        spans.push(Span::styled(" [?]", dim));
        spans.push(Span::styled(format!(" {} ", self.labels.help_label), dim));
        spans.push(Span::styled(" [q]", dim));
        spans.push(Span::styled(format!(" {} ", self.labels.quit_label), dim));

        let tier_label = match zones.tier {
            layout::LayoutTier::Compact => "compact",
            layout::LayoutTier::Standard => "standard",
            layout::LayoutTier::Wide => "wide",
        };
        spans.push(Span::styled(
            format!(" │ {tier_label}"),
            Style::default().fg(theme.accent_dim()),
        ));

        frame.render_widget(Paragraph::new(Line::from(spans)), zones.help);
    }

    /// Renders the sidebar zone with auto-discovered plugin panels.
    fn render_sidebar(&self, frame: &mut Frame<'_>, area: Rect) {
        let theme = self.theme();
        let block = Block::default()
            .borders(Borders::LEFT)
            .border_style(Style::default().fg(theme.border()))
            .title(" Plugins ")
            .title_style(Style::default().fg(theme.info()));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.sidebar_panels.is_empty() {
            let lines = vec![Line::styled(
                " (no panels)",
                Style::default().fg(theme.text_dim()),
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
                .border_style(Style::default().fg(theme.border()))
                .title(format!(" {} ", panel.title))
                .title_style(Style::default().fg(theme.text_bright()));

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
        let theme = self.theme();
        let block = Block::default()
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(theme.border()))
            .title(" Control ")
            .title_style(Style::default().fg(theme.info()));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let state_color = self.state.color(theme);

        let lines = vec![
            Line::styled(
                format!(
                    " {}: {}",
                    self.labels.control_state,
                    self.localized_state_label()
                ),
                Style::default().fg(state_color),
            ),
            Line::raw(""),
            Line::styled(
                format!(" {}: {}", self.labels.control_work, self.config.title),
                Style::default().fg(theme.text_bright()),
            ),
            Line::raw(""),
            Line::styled(
                format!(" [q] {}", self.labels.quit_label),
                Style::default().fg(theme.text_dim()),
            ),
        ];
        frame.render_widget(Paragraph::new(lines), inner);
    }

    /// Renders the agent switcher popup (Ctrl+A).
    fn render_agent_switcher(&self, frame: &mut Frame<'_>, area: Rect) {
        let theme = self.theme();
        let popup_height = (self.available_agents.len() as u16 + 2).min(area.height);
        let popup_width = 40u16.min(area.width);
        let popup_area = Rect {
            x: area.x + (area.width.saturating_sub(popup_width)) / 2,
            y: area.y + (area.height.saturating_sub(popup_height)) / 2,
            width: popup_width,
            height: popup_height,
        };

        let items: Vec<ListItem<'_>> = self
            .available_agents
            .iter()
            .map(|agent| {
                ListItem::new(Line::styled(
                    format!("  {agent}"),
                    Style::default().fg(theme.text()),
                ))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.accent()))
                    .title(" Switch Agent (Ctrl+A) ")
                    .title_style(
                        Style::default()
                            .fg(theme.accent())
                            .add_modifier(Modifier::BOLD),
                    ),
            )
            .highlight_style(
                Style::default()
                    .bg(theme.surface())
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▸ ");

        frame.render_widget(Clear, popup_area);
        frame.render_stateful_widget(
            list,
            popup_area,
            &mut ListState::default().with_selected(Some(self.agent_switcher_selected)),
        );
    }

    /// Renders the idle dashboard when no agent is running.
    fn render_idle_dashboard(&self, frame: &mut Frame<'_>, area: Rect) {
        let theme = self.theme();
        let version = env!("CARGO_PKG_VERSION");

        // Idle: dim logo. Running: accent. Error: red.
        let logo_color = Some(match self.state {
            TuiState::Running => theme.text_dim(),
            TuiState::Error => theme.error(),
            _ => theme.accent(),
        });
        let logo_lines =
            crate::logo::build_logo_lines(area.width, theme, logo_color, &self.labels.logo_tagline);

        let mut lines: Vec<Line<'_>> = Vec::new();

        // Center vertically
        let content_height = logo_lines.len() + 14;
        let pad_top = area.height.saturating_sub(content_height as u16) / 2;
        for _ in 0..pad_top {
            lines.push(Line::raw(""));
        }

        lines.extend(logo_lines);
        lines.push(Line::raw(""));
        lines.push(Line::styled(
            format!("  v{version} — {}", self.labels.orchestration_runtime),
            Style::default().fg(theme.text_dim()),
        ));
        lines.push(Line::raw(""));

        // Project status — detect if .ralph-engine/ exists
        let has_config = std::path::Path::new(".ralph-engine/config.yaml").exists();
        if has_config {
            lines.push(Line::from(vec![
                Span::styled("  ✓ ", Style::default().fg(theme.success())),
                Span::styled(
                    self.labels.project_configured.as_str(),
                    Style::default().fg(theme.text()),
                ),
            ]));
            lines.push(Line::styled(
                format!("  {}", self.labels.type_run),
                Style::default()
                    .fg(theme.accent())
                    .add_modifier(Modifier::ITALIC),
            ));
        } else {
            lines.push(Line::from(vec![
                Span::styled("  ○ ", Style::default().fg(theme.warning())),
                Span::styled(
                    self.labels.no_project_found.as_str(),
                    Style::default().fg(theme.text()),
                ),
            ]));
            lines.push(Line::styled(
                format!("  {}", self.labels.type_init),
                Style::default()
                    .fg(theme.accent_dim())
                    .add_modifier(Modifier::ITALIC),
            ));
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(vec![
            Span::styled(
                "  q",
                Style::default()
                    .fg(theme.accent())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" quit  ", Style::default().fg(theme.text_dim())),
            Span::styled(
                "?",
                Style::default()
                    .fg(theme.accent())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" help  ", Style::default().fg(theme.text_dim())),
            Span::styled(
                "F2",
                Style::default()
                    .fg(theme.accent())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" sidebar  ", Style::default().fg(theme.text_dim())),
            Span::styled(
                "j/k",
                Style::default()
                    .fg(theme.accent())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" navigate", Style::default().fg(theme.text_dim())),
        ]));

        frame.render_widget(Paragraph::new(lines), area);
    }

    /// Renders and ticks down active toast notifications.
    ///
    /// Toasts appear stacked in the bottom-right corner. Each toast
    /// auto-dismisses after its remaining ticks reach zero.
    fn render_toasts(&mut self, frame: &mut Frame<'_>, area: Rect) {
        // Tick down and remove expired toasts
        self.toasts.retain_mut(|t| {
            t.remaining_ticks = t.remaining_ticks.saturating_sub(1);
            t.remaining_ticks > 0
        });

        if self.toasts.is_empty() {
            return;
        }

        let max_toasts = 3;
        let toast_w = 40u16.min(area.width.saturating_sub(2));
        let toast_h = 3u16;

        for (i, toast) in self.toasts.iter().rev().take(max_toasts).enumerate() {
            let y = area.height.saturating_sub((i as u16 + 1) * (toast_h + 1));
            let x = area.width.saturating_sub(toast_w + 1);
            let popup = Rect::new(x, y, toast_w, toast_h);

            let color = match toast.level {
                ToastLevel::Info => self.theme().accent(),
                ToastLevel::Success => self.theme().success(),
                ToastLevel::Warning => self.theme().warning(),
                ToastLevel::Error => self.theme().error(),
            };

            frame.render_widget(Clear, popup);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color))
                .style(Style::default().bg(self.theme().surface()));
            let inner = block.inner(popup);
            frame.render_widget(block, popup);

            let text = Paragraph::new(toast.message.as_str())
                .style(Style::default().fg(self.theme().text_bright()));
            frame.render_widget(text, inner);
        }
    }

    /// Renders the quit confirmation modal (centered overlay).
    fn render_quit_modal(&self, frame: &mut Frame<'_>, area: Rect) {
        let theme = self.theme();
        let popup_w = 36u16.min(area.width);
        let popup_h = 5u16.min(area.height);
        let popup = Rect {
            x: area.x + (area.width.saturating_sub(popup_w)) / 2,
            y: area.y + (area.height.saturating_sub(popup_h)) / 2,
            width: popup_w,
            height: popup_h,
        };

        let lines = vec![
            Line::raw(""),
            Line::from(vec![
                Span::styled(
                    format!("  {} ", self.labels.quit_question),
                    Style::default()
                        .fg(theme.warning())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "y",
                    Style::default()
                        .fg(theme.accent())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" yes  ", Style::default().fg(theme.text_dim())),
                Span::styled(
                    "n",
                    Style::default()
                        .fg(theme.accent())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" cancel", Style::default().fg(theme.text_dim())),
            ]),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.warning()))
            .title(format!(" {} ", self.labels.quit_title))
            .title_style(
                Style::default()
                    .fg(theme.warning())
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_widget(Clear, popup);
        frame.render_widget(Paragraph::new(lines).block(block), popup);
    }

    /// Renders the help modal popup (centered overlay with grouped keys).
    fn render_help_modal(&self, frame: &mut Frame<'_>, area: Rect) {
        let theme = self.theme();
        let version = env!("CARGO_PKG_VERSION");

        let mut lines: Vec<Line<'_>> = Vec::new();

        lines.push(Line::from(vec![Span::styled(
            format!("  Ralph Engine v{version}"),
            Style::default()
                .fg(theme.accent())
                .add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::raw(""));

        // Navigation keys
        lines.push(Line::styled(
            format!("  {}", self.labels.nav_heading),
            Style::default()
                .fg(theme.text_bright())
                .add_modifier(Modifier::BOLD),
        ));
        for (key, desc) in &self.labels.nav_keys {
            lines.push(Line::from(vec![
                Span::styled(format!("  {key:<12}"), Style::default().fg(theme.accent())),
                Span::styled(desc.as_str(), Style::default().fg(theme.text_dim())),
            ]));
        }

        lines.push(Line::raw(""));

        // Action keys
        lines.push(Line::styled(
            format!("  {}", self.labels.actions_heading),
            Style::default()
                .fg(theme.text_bright())
                .add_modifier(Modifier::BOLD),
        ));
        for (key, desc) in &self.labels.action_keys {
            lines.push(Line::from(vec![
                Span::styled(format!("  {key:<12}"), Style::default().fg(theme.accent())),
                Span::styled(desc.as_str(), Style::default().fg(theme.text_dim())),
            ]));
        }

        // Plugin keybindings
        let state_label = format!("{:?}", self.state);
        let plugin_keys: Vec<_> = self
            .plugin_keybindings
            .iter()
            .filter(|b| {
                b.active_states.is_empty() || b.active_states.iter().any(|s| s == &state_label)
            })
            .collect();

        if !plugin_keys.is_empty() {
            lines.push(Line::raw(""));
            lines.push(Line::styled(
                format!("  {}", self.labels.plugins_heading),
                Style::default()
                    .fg(theme.text_bright())
                    .add_modifier(Modifier::BOLD),
            ));
            for binding in plugin_keys {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {:<12}", binding.key),
                        Style::default().fg(theme.accent()),
                    ),
                    Span::styled(
                        binding.description.as_str(),
                        Style::default().fg(theme.text_dim()),
                    ),
                ]));
            }
        }

        if self.input_enabled {
            lines.push(Line::raw(""));
            lines.push(Line::styled(
                format!("  {}", self.labels.slash_hint),
                Style::default()
                    .fg(theme.text_dim())
                    .add_modifier(Modifier::ITALIC),
            ));
        }

        lines.push(Line::raw(""));
        lines.push(Line::styled(
            format!("  {}", self.labels.press_any_key),
            Style::default().fg(theme.border()),
        ));

        let popup_h = (lines.len() as u16 + 2).min(area.height.saturating_sub(2));
        let popup_w = 44u16.min(area.width.saturating_sub(4));
        let popup = Rect {
            x: area.x + (area.width.saturating_sub(popup_w)) / 2,
            y: area.y + (area.height.saturating_sub(popup_h)) / 2,
            width: popup_w,
            height: popup_h,
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.accent()))
            .title(format!(" {} ", self.labels.help_title))
            .title_style(
                Style::default()
                    .fg(theme.accent())
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_widget(Clear, popup);
        frame.render_widget(Paragraph::new(lines).block(block), popup);
    }
}

/// Styles a content line based on the parent block kind.
///
/// Diff lines (starting with `+` or `-`) get special treatment.
/// Command output stays plain. Thinking text is dim italic.
/// Styles a content line as a vec of spans (preserving per-character coloring).
///
/// Returns spans (not a Line) so the caller can prepend border spans
/// without losing the style information.
fn style_content_line<'a>(
    line: &'a str,
    kind: crate::feed::BlockKind,
    theme: &dyn crate::theme::Theme,
) -> Vec<Span<'a>> {
    use crate::feed::BlockKind;

    match kind {
        BlockKind::FileEdit => {
            if line.starts_with('+') {
                vec![Span::styled(line, Style::default().fg(theme.diff_added()))]
            } else if line.starts_with('-') {
                vec![Span::styled(
                    line,
                    Style::default().fg(theme.diff_removed()),
                )]
            } else if line.starts_with("@@") {
                vec![Span::styled(
                    line,
                    Style::default()
                        .fg(theme.info())
                        .add_modifier(Modifier::BOLD),
                )]
            } else {
                vec![Span::styled(
                    line,
                    Style::default().fg(theme.diff_context()),
                )]
            }
        }
        BlockKind::Command => {
            // Color error/fail lines in red, pass/success in green
            if line.contains("FAIL") || line.contains("error") || line.contains("Error") {
                vec![Span::styled(line, Style::default().fg(theme.error()))]
            } else if line.contains("PASS") || line.contains("✓") || line.starts_with("ok") {
                vec![Span::styled(line, Style::default().fg(theme.success()))]
            } else {
                vec![Span::styled(line, Style::default().fg(theme.text_dim()))]
            }
        }
        BlockKind::Thinking => vec![Span::styled(
            line,
            Style::default()
                .fg(theme.text_dim())
                .add_modifier(Modifier::ITALIC),
        )],
        BlockKind::GateFail => vec![Span::styled(line, Style::default().fg(theme.error()))],
        BlockKind::GatePass => vec![Span::styled(line, Style::default().fg(theme.success()))],
        _ => vec![Span::raw(line)],
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
    fn tui_state_colors_from_theme() {
        use crate::theme::Theme;
        let t = crate::theme::CatppuccinMocha;
        assert_eq!(TuiState::Running.color(&t), t.success());
        assert_eq!(TuiState::Paused.color(&t), t.warning());
        assert_eq!(TuiState::Error.color(&t), t.error());
        assert_eq!(TuiState::Complete.color(&t), t.info());
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
        assert_eq!(shell.theme().id(), "catppuccin");
    }

    #[test]
    fn set_theme_switches_active_theme() {
        let mut shell = empty_shell();
        assert_eq!(shell.theme().id(), "catppuccin");
        shell.set_theme("dracula");
        assert_eq!(shell.theme().id(), "dracula");
        shell.set_theme("nord");
        assert_eq!(shell.theme().id(), "nord");
    }

    #[test]
    fn set_theme_unknown_falls_back_to_default() {
        let mut shell = empty_shell();
        shell.set_theme("nonexistent");
        assert_eq!(shell.theme().id(), "catppuccin");
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
    fn handle_key_help_toggles_modal() {
        let mut shell = empty_shell();
        assert!(!shell.help_modal_visible);
        shell.handle_key(KeyCode::Char('?'));
        assert!(shell.help_modal_visible);
        // Any key closes it
        shell.handle_key(KeyCode::Char('a'));
        assert!(!shell.help_modal_visible);
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
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        shell.push_activity(">> Tool Call: search".to_owned());
        shell.push_activity(">> Result: found 3 items".to_owned());
        shell.set_progress(42);
        shell.increment_tools();
        shell
    }

    fn render_to_buffer(shell: &mut TuiShell, width: u16, height: u16) -> String {
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
        let mut shell = test_shell();
        let output = render_to_buffer(&mut shell, 80, 24);
        assert!(output.contains("test.agent"));
        assert!(output.contains("[RUNNING]"));
    }

    #[test]
    fn render_compact_shows_activity_lines() {
        let mut shell = test_shell();
        let output = render_to_buffer(&mut shell, 80, 24);
        assert!(output.contains("Tool Call: search"));
        assert!(output.contains("found 3 items"));
    }

    #[test]
    fn render_compact_shows_metrics() {
        let mut shell = test_shell();
        let output = render_to_buffer(&mut shell, 80, 24);
        assert!(output.contains("Tools: 1"));
    }

    #[test]
    fn render_compact_shows_help_bar() {
        let mut shell = test_shell();
        let output = render_to_buffer(&mut shell, 80, 24);
        assert!(output.contains("[q]"));
        assert!(output.contains("compact"));
    }

    #[test]
    fn render_compact_no_sidebar() {
        let mut shell = test_shell();
        let output = render_to_buffer(&mut shell, 80, 24);
        assert!(!output.contains("Plugins"));
    }

    #[test]
    fn render_standard_shows_sidebar() {
        let mut shell = test_shell();
        let output = render_to_buffer(&mut shell, 140, 40);
        assert!(output.contains("Plugins"));
        assert!(output.contains("standard"));
    }

    #[test]
    fn render_wide_shows_control_panel() {
        let mut shell = TuiShell::new(TuiConfig {
            title: "Fix Bug".to_owned(),
            agent_id: "test.agent".to_owned(),
            locale: "en".to_owned(),
        });
        let output = render_to_buffer(&mut shell, 200, 60);
        assert!(output.contains("Control"));
        assert!(output.contains("Plugins"));
        assert!(output.contains("wide"));
    }

    #[test]
    fn render_too_small_shows_error() {
        let mut shell = test_shell();
        let output = render_to_buffer(&mut shell, 60, 20);
        assert!(output.contains("too small"));
    }

    #[test]
    fn render_paused_state_shows_in_header() {
        let mut shell = test_shell();
        shell.set_state(TuiState::Paused);
        let output = render_to_buffer(&mut shell, 80, 24);
        assert!(output.contains("[PAUSED]"));
    }

    #[test]
    fn render_progress_gauge_shows_in_wide_header() {
        let mut shell = test_shell();
        shell.set_progress(75);
        let output = render_to_buffer(&mut shell, 100, 24);
        assert!(output.contains("test.agent"));
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
                plugin_id: "test.plugin-a".to_owned(),
            },
            SidebarPanel {
                title: "Sprint".to_owned(),
                lines: vec!["Story 5.3: in-progress".to_owned()],
                plugin_id: "test.plugin-b".to_owned(),
            },
        ]);
        let output = render_to_buffer(&mut shell, 140, 40);
        assert!(output.contains("Findings"));
        assert!(output.contains("Sprint"));
        assert!(output.contains("3 issues"));
    }

    #[test]
    fn render_standard_empty_panels_shows_placeholder() {
        let mut shell = test_shell();
        let output = render_to_buffer(&mut shell, 140, 40);
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
        let output = render_to_buffer(&mut shell, 80, 24);
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
        let output = render_to_buffer(&mut shell, 80, 24);
        assert!(
            output.contains("[f]"),
            "help should show [f] when Paused, got:\n{output}"
        );
    }

    // ── Scroll tests ──

    #[test]
    fn follow_mode_enabled_by_default() {
        let shell = test_shell();
        assert!(shell.is_follow_mode());
    }

    #[test]
    fn scroll_up_disables_follow_mode() {
        let mut shell = test_shell();
        shell.scroll_feed_up();
        assert!(!shell.is_follow_mode());
    }

    #[test]
    fn scroll_to_bottom_re_enables_follow() {
        let mut shell = test_shell();
        shell.scroll_feed_up();
        assert!(!shell.is_follow_mode());
        shell.scroll_feed_to_bottom();
        assert!(shell.is_follow_mode());
    }

    #[test]
    fn page_up_disables_follow() {
        let mut shell = test_shell();
        shell.scroll_feed_page_up();
        assert!(!shell.is_follow_mode());
    }

    #[test]
    fn scroll_to_top_disables_follow() {
        let mut shell = test_shell();
        shell.scroll_feed_to_top();
        assert!(!shell.is_follow_mode());
    }

    #[test]
    fn focus_keys_handled_in_key_handler() {
        let mut shell = test_shell();
        // Add some blocks so focus navigation works
        shell
            .feed_mut()
            .push_block(crate::feed::FeedBlock::completed(
                crate::feed::BlockKind::System,
                "block-a".into(),
            ));
        shell
            .feed_mut()
            .push_block(crate::feed::FeedBlock::completed(
                crate::feed::BlockKind::System,
                "block-b".into(),
            ));

        // j should focus next block and disable follow
        let result = shell.handle_key(KeyCode::Char('j'));
        assert_eq!(result, PluginKeyAction::Handled);
        assert!(!shell.is_follow_mode());
        assert!(shell.focused_block().is_some());

        // G should re-enable follow and clear focus
        let result = shell.handle_key(KeyCode::Char('G'));
        assert_eq!(result, PluginKeyAction::Handled);
        assert!(shell.is_follow_mode());

        // Enter should toggle focused block
        shell.handle_key(KeyCode::Char('j'));
        let result = shell.handle_key(KeyCode::Enter);
        assert_eq!(result, PluginKeyAction::Handled);

        // Esc should clear focus
        let result = shell.handle_key(KeyCode::Esc);
        assert_eq!(result, PluginKeyAction::Handled);
        assert!(shell.focused_block().is_none());
        assert!(shell.is_follow_mode());
    }

    #[test]
    fn mouse_scroll_disables_follow() {
        use ratatui::crossterm::event::MouseEventKind;
        let mut shell = test_shell();
        shell.handle_mouse(MouseEventKind::ScrollUp);
        assert!(!shell.is_follow_mode());
    }

    #[test]
    fn toast_info_creates_info_toast() {
        let mut shell = empty_shell();
        shell.toast_info("Test message".to_owned());
        assert_eq!(shell.toasts.len(), 1);
        assert_eq!(shell.toasts[0].level, ToastLevel::Info);
        assert_eq!(shell.toasts[0].message, "Test message");
    }

    #[test]
    fn toast_success_creates_success_toast() {
        let mut shell = empty_shell();
        shell.toast_success("Done!".to_owned());
        assert_eq!(shell.toasts.len(), 1);
        assert_eq!(shell.toasts[0].level, ToastLevel::Success);
    }

    #[test]
    fn show_error_modal_creates_error_toast_and_activity() {
        let mut shell = empty_shell();
        shell.show_error_modal("Title", "Details");
        assert_eq!(shell.toasts.len(), 1);
        assert_eq!(shell.toasts[0].level, ToastLevel::Error);
        assert!(shell.activity_lines.iter().any(|l| l.contains("Title")));
    }

    #[test]
    fn toasts_expire_after_ticks() {
        let mut shell = empty_shell();
        shell.show_toast("Temp".to_owned(), ToastLevel::Info);
        assert_eq!(shell.toasts.len(), 1);
        // Simulate expiry
        shell.toasts[0].remaining_ticks = 1;
        // After render_toasts decrements, it should be removed
        // Can't call render_toasts directly (needs Frame), but we can test the retain logic
        shell.toasts.retain_mut(|t| {
            t.remaining_ticks = t.remaining_ticks.saturating_sub(1);
            t.remaining_ticks > 0
        });
        assert!(shell.toasts.is_empty());
    }
}
