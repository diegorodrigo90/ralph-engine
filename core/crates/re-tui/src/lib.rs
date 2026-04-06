//! Terminal dashboard framework for Ralph Engine.
//!
//! Provides a ratatui-based TUI shell with zone-based layout,
//! structured logging, and terminal lifecycle management.
//!
//! The TUI is a generic framework. Without interactive plugins, it is
//! a read-only dashboard. When plugins contribute keybindings (e.g.
//! `official.guided`), the TUI becomes interactive with pause,
//! feedback, and resume controls.
//!
//! Plugins contribute panels and keybindings via auto-discovery
//! (`tui_contributions()` and `tui_keybindings()` on `PluginRuntime`).

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod clipboard;
pub mod events;
pub mod feed;
pub mod indicators;
pub mod keybindings;
pub mod layout;
mod logging;
pub mod logo;
pub mod process;
mod terminal;
pub mod theme;

pub use events::{AgentEvent, parse_stream_line, parse_stream_lines};
pub use feed::{BlockKind, Feed, FeedBlock, ToolKindMapping, process_agent_event};
pub use indicators::{IndicatorPanel, IndicatorState, StatusIndicator};
pub use keybindings::{KeybindingRegistry, format_key, is_core_key};
pub use layout::{LayoutTier, LayoutZones, compute_zones, is_terminal_too_small};
pub use logging::{LogConfig, init_logging};
pub use process::{SessionState, StateEvent, StateListener};
pub use terminal::{
    CommandEntry, CommandSource, FocusTarget, PanelHint, PanelItem, PanelSeverity, PluginKeyAction,
    RegisteredKeybinding, SidebarPanel, Toast, ToastLevel, TuiConfig, TuiLabels, TuiShell,
    TuiState, TuiTab,
};
pub use theme::{
    BLOCK_PADDING, INDENT_WIDTH, MAX_COLLAPSED_LINES, Theme, available_theme_ids, builtin_themes,
    no_color_active, resolve_theme,
};

/// Minimum terminal width supported (columns).
pub const MIN_TERMINAL_WIDTH: u16 = 80;

/// Minimum terminal height supported (rows).
pub const MIN_TERMINAL_HEIGHT: u16 = 24;

/// Width threshold for two-column layout.
pub const TWO_COLUMN_WIDTH: u16 = 120;

/// Width threshold for three-column (full guided) layout.
pub const THREE_COLUMN_WIDTH: u16 = 160;
