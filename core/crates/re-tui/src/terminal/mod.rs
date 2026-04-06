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

mod autocomplete;
mod render;
mod render_input;
mod render_modals;
mod render_panels;
mod shell;
mod style;
mod types;

// Re-export the public API
pub use shell::TuiShell;
pub use types::{
    CommandEntry, CommandSource, PanelHint, PanelItem, PanelSeverity, PluginKeyAction,
    RegisteredKeybinding, SidebarPanel, Toast, ToastLevel, TuiConfig, TuiLabels, TuiState,
};

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests;
