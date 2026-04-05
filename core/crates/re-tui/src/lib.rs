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

pub mod events;
pub mod keybindings;
pub mod layout;
mod logging;
pub mod logo;
pub mod process;
mod terminal;

pub use events::{AgentEvent, parse_stream_line, parse_stream_lines};
pub use keybindings::{KeybindingRegistry, format_key, is_core_key};
pub use layout::{LayoutTier, LayoutZones, compute_zones, is_terminal_too_small};
pub use logging::{LogConfig, init_logging};
pub use process::{SessionState, StateEvent, StateListener};
pub use terminal::{
    CommandEntry, CommandSource, PluginKeyAction, RegisteredKeybinding, SidebarPanel, TuiConfig,
    TuiShell, TuiState,
};

/// Minimum terminal width supported (columns).
pub const MIN_TERMINAL_WIDTH: u16 = 80;

/// Minimum terminal height supported (rows).
pub const MIN_TERMINAL_HEIGHT: u16 = 24;

/// Width threshold for two-column layout.
pub const TWO_COLUMN_WIDTH: u16 = 120;

/// Width threshold for three-column (full guided) layout.
pub const THREE_COLUMN_WIDTH: u16 = 160;
