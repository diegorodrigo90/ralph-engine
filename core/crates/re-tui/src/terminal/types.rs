//! Shared types for the TUI shell.
//!
//! All public types used across the terminal module live here.
//! This module has no dependencies on other terminal submodules.

use ratatui::style::Color;

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

/// Active tab in the TUI dashboard.
///
/// Tabs route the main content area to different views.
/// Plugins register which tabs are available — core renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiTab {
    /// Activity feed — block-based agent output stream (default).
    Feed,
    /// Files touched this session — tree view of reads/edits/creates.
    Files,
    /// Raw agent log — filterable, scrollable text output.
    Log,
    /// Active configuration — plugins, hooks, agent flags (read-only).
    Config,
}

impl TuiTab {
    /// All tabs in display order.
    pub const ALL: &[Self] = &[Self::Feed, Self::Files, Self::Log, Self::Config];

    /// Display label for this tab.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Feed => "Feed",
            Self::Files => "Files",
            Self::Log => "Log",
            Self::Config => "Config",
        }
    }

    /// Next tab (wraps around).
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Feed => Self::Files,
            Self::Files => Self::Log,
            Self::Log => Self::Config,
            Self::Config => Self::Feed,
        }
    }

    /// Previous tab (wraps around).
    #[must_use]
    pub fn prev(self) -> Self {
        match self {
            Self::Feed => Self::Config,
            Self::Files => Self::Feed,
            Self::Log => Self::Files,
            Self::Config => Self::Log,
        }
    }
}

/// Which panel currently owns keyboard focus.
///
/// Tab key cycles through these in order. The focused panel
/// gets an accent border and the help bar shows its keybindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusTarget {
    /// Main content area (Feed, Files, Log, Config tab content).
    Activity,
    /// Sidebar panels (Standard + Wide tiers).
    Sidebar,
    /// Text input bar (when input is enabled).
    Input,
}

impl FocusTarget {
    /// Next focus target (wraps, skips unavailable targets).
    #[must_use]
    pub fn next(self, has_sidebar: bool, has_input: bool) -> Self {
        match self {
            Self::Activity if has_sidebar => Self::Sidebar,
            Self::Activity if has_input => Self::Input,
            Self::Sidebar if has_input => Self::Input,
            Self::Input | Self::Sidebar => Self::Activity,
            _ => Self::Activity,
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
    /// Project directory name shown in header (e.g. `"my-project"`).
    pub project_name: String,
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
    // ── State labels ───────────��────────────────────────────────
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
    // ── Control panel ───────���───────────────────────────────────
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
    // ── Logo ──────��──────────────────────────────��──────────────
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
    // ── Paste ───────────────────────────────────────────────────
    /// Paste indicator prefix (e.g., "Pasted text" / "Texto colado").
    pub pasted_text_label: String,
    /// Paste lines suffix (e.g., "lines" / "linhas").
    pub paste_lines_suffix: String,
    /// Paste chars suffix (e.g., "chars" / "caracteres").
    pub paste_chars_suffix: String,
    /// File attachment indicator (e.g., "File" / "Arquivo").
    pub file_label: String,
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
            pasted_text_label: "Pasted text".to_owned(),
            paste_lines_suffix: "lines".to_owned(),
            paste_chars_suffix: "chars".to_owned(),
            file_label: "File".to_owned(),
        }
    }
}

/// A sidebar panel provided by a plugin, ready to render.
#[derive(Debug, Clone)]
pub struct SidebarPanel {
    /// Panel title (localized).
    pub title: String,
    /// Typed content items for theme-driven rendering.
    pub items: Vec<PanelItem>,
    /// Source plugin ID (for attribution).
    pub plugin_id: String,
    /// Whether this panel belongs to an agent plugin (`AgentRuntime` kind).
    ///
    /// Set by the CLI layer based on `PluginKind`. Used by the idle
    /// dashboard to identify agent status panels without hardcoding
    /// specific plugin IDs (Model B compliance).
    pub is_agent: bool,
}

/// A hint displayed on the idle dashboard (e.g. `/run — start loop`).
///
/// Plugins contribute idle hints via `idle_hints()` on the `PluginRuntime`
/// trait. Core collects and renders them — never hardcoding plugin-specific
/// command examples.
#[derive(Debug, Clone)]
pub struct IdleHint {
    /// The command or action (e.g. `/run 5.3`).
    pub command: String,
    /// Short description (e.g. `execute story 5.3`).
    pub description: String,
}

/// A typed content item for sidebar panels.
///
/// A renderable UI block — the core's view of a `TuiBlock`.
///
/// Mirrors the structural contract from `re_plugin::TuiBlock` but is
/// owned by re-tui. The CLI layer converts plugin blocks to this type.
/// The renderer uses `hint` + `severity` to decide visual treatment.
#[derive(Debug, Clone, Default)]
pub struct PanelItem {
    /// Primary text.
    pub label: Option<String>,
    /// Secondary text / value.
    pub value: Option<String>,
    /// Layout hint.
    pub hint: PanelHint,
    /// Semantic severity (maps to theme color slot).
    pub severity: PanelSeverity,
    /// Numeric value (for bars, metrics).
    pub numeric: Option<u32>,
    /// Denominator (for ratio display).
    pub total: Option<u32>,
    /// Key-value pairs (for `Pairs` hint).
    pub pairs: Vec<(String, String)>,
    /// Child items (for `List` hint).
    pub items: Vec<String>,
}

/// Layout hint for panel item rendering.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PanelHint {
    /// `label: value` inline.
    #[default]
    Inline,
    /// Progress bar using `numeric`.
    Bar,
    /// Icon + label + value (icon from severity).
    Indicator,
    /// Key-value table.
    Pairs,
    /// Bulleted list.
    List,
    /// Plain text.
    Text,
    /// Separator line.
    Separator,
}

/// Severity level for panel items.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PanelSeverity {
    /// Theme success color.
    Success,
    /// Theme warning color.
    Warning,
    /// Theme error color.
    Error,
    /// Theme default text color.
    #[default]
    Neutral,
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
    pub(super) remaining_ticks: usize,
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
pub(super) const TOAST_DEFAULT_TICKS: usize = 60;

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

/// Error type for TUI operations.
#[derive(Debug)]
pub struct TuiError {
    /// Human-readable error message.
    pub message: String,
}

impl TuiError {
    pub(super) fn new(message: String) -> Self {
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
    fn command_source_labels() {
        assert_eq!(CommandSource::Agent.label(), "agent");
        assert_eq!(CommandSource::Plugin.label(), "plugin");
        assert_eq!(CommandSource::Mcp.label(), "mcp");
    }

    #[test]
    fn toast_level_variants() {
        let t = Toast {
            message: "test".to_owned(),
            level: ToastLevel::Info,
            remaining_ticks: TOAST_DEFAULT_TICKS,
        };
        assert_eq!(t.remaining_ticks, 60);
    }

    // ── TuiTab ─────────────────────────────────────────────────

    #[test]
    fn tab_all_has_four_entries() {
        assert_eq!(TuiTab::ALL.len(), 4);
    }

    #[test]
    fn tab_labels_are_distinct() {
        let labels: Vec<&str> = TuiTab::ALL.iter().map(|t| t.label()).collect();
        for (i, a) in labels.iter().enumerate() {
            for b in &labels[i + 1..] {
                assert_ne!(a, b, "duplicate tab label");
            }
        }
    }

    #[test]
    fn tab_next_cycles() {
        let mut tab = TuiTab::Feed;
        for _ in 0..TuiTab::ALL.len() {
            tab = tab.next();
        }
        assert_eq!(tab, TuiTab::Feed, "next should cycle back to Feed");
    }

    #[test]
    fn tab_prev_cycles() {
        let mut tab = TuiTab::Feed;
        for _ in 0..TuiTab::ALL.len() {
            tab = tab.prev();
        }
        assert_eq!(tab, TuiTab::Feed, "prev should cycle back to Feed");
    }

    #[test]
    fn tab_next_prev_inverse() {
        for tab in TuiTab::ALL {
            assert_eq!(tab.next().prev(), *tab);
            assert_eq!(tab.prev().next(), *tab);
        }
    }

    #[test]
    fn tab_default_is_feed() {
        assert_eq!(TuiTab::Feed.label(), "Feed");
    }

    // ── FocusTarget ────────────────────────────────────────────

    #[test]
    fn focus_next_with_sidebar_and_input() {
        let focus = FocusTarget::Activity;
        assert_eq!(focus.next(true, true), FocusTarget::Sidebar);
        assert_eq!(FocusTarget::Sidebar.next(true, true), FocusTarget::Input);
        assert_eq!(FocusTarget::Input.next(true, true), FocusTarget::Activity);
    }

    #[test]
    fn focus_next_without_sidebar() {
        let focus = FocusTarget::Activity;
        assert_eq!(focus.next(false, true), FocusTarget::Input);
        assert_eq!(FocusTarget::Input.next(false, true), FocusTarget::Activity);
    }

    #[test]
    fn focus_next_without_input() {
        let focus = FocusTarget::Activity;
        assert_eq!(focus.next(true, false), FocusTarget::Sidebar);
        assert_eq!(
            FocusTarget::Sidebar.next(true, false),
            FocusTarget::Activity
        );
    }

    #[test]
    fn focus_next_activity_only() {
        let focus = FocusTarget::Activity;
        assert_eq!(focus.next(false, false), FocusTarget::Activity);
    }
}
