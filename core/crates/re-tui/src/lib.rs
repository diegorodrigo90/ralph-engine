//! Terminal dashboard framework for Ralph Engine.
//!
//! Provides a ratatui-based TUI shell with zone-based layout,
//! structured logging, and terminal lifecycle management.
//!
//! Two rendering modes:
//! - **Autonomous**: read-only dashboard (progress, tools, metrics).
//! - **Guided**: interactive panels (pause/resume, feedback, plugins).
//!
//! Plugins contribute panels and keybindings via auto-discovery
//! (`tui_contributions()` trait method on `PluginRuntime`).

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod events;
pub mod layout;
mod logging;
mod terminal;

pub use events::{AgentEvent, parse_stream_line, parse_stream_lines};
pub use layout::{LayoutTier, LayoutZones, compute_zones, is_terminal_too_small};
pub use logging::{LogConfig, init_logging};
pub use terminal::{SidebarPanel, TuiConfig, TuiMode, TuiShell, TuiState};

/// Minimum terminal width supported (columns).
pub const MIN_TERMINAL_WIDTH: u16 = 80;

/// Minimum terminal height supported (rows).
pub const MIN_TERMINAL_HEIGHT: u16 = 24;

/// Width threshold for two-column layout.
pub const TWO_COLUMN_WIDTH: u16 = 120;

/// Width threshold for three-column (full guided) layout.
pub const THREE_COLUMN_WIDTH: u16 = 160;
