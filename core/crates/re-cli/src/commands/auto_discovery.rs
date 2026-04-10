//! Shared auto-discovery helpers for TUI shell setup.
//!
//! Both the dashboard (`tui.rs`) and the run command (`run.rs`) need to
//! discover plugins and configure the TUI shell identically. This module
//! extracts that shared setup to avoid duplication (DRY at monorepo level).

use crate::catalog;

/// Returns the current directory name as the project name.
///
/// Used as display label in the TUI header. Falls back to empty string
/// if the working directory cannot be resolved.
pub(crate) fn current_project_name() -> String {
    std::env::current_dir()
        .ok()
        .and_then(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_default()
}

/// Discovers all plugin-provided panels, hints, commands, and input bar
/// preferences, then applies them to the given TUI shell.
///
/// This is the single source of truth for plugin auto-discovery setup.
/// Called by both `ralph-engine tui` (dashboard) and `ralph-engine run` (agent mode).
pub(crate) fn configure_shell_from_plugins(
    shell: &mut re_tui::TuiShell,
    cwd: &std::path::Path,
    command_prefix: String,
) {
    enable_input_bar_if_requested(shell);
    register_agent_commands(shell, cwd, command_prefix);
    register_sidebar_panels(shell);
    register_idle_hints(shell);
    register_plugin_keybindings(shell);
}

/// Discovers TUI keybindings from all plugins and registers them in the shell.
fn register_plugin_keybindings(shell: &mut re_tui::TuiShell) {
    let keybindings = catalog::collect_tui_keybindings_from_plugins();
    shell.set_plugin_keybindings(keybindings);
}

/// Enables the TUI input bar when any plugin declares it needs user input.
fn enable_input_bar_if_requested(shell: &mut re_tui::TuiShell) {
    if catalog::any_plugin_wants_input_bar() {
        shell.enable_input();
    }
}

/// Discovers agent commands from all plugins and registers them for
/// autocomplete in the TUI input bar.
fn register_agent_commands(shell: &mut re_tui::TuiShell, cwd: &std::path::Path, prefix: String) {
    let commands: Vec<re_tui::CommandEntry> = catalog::collect_agent_commands_from_plugins(cwd)
        .into_iter()
        .map(|cmd| re_tui::CommandEntry {
            name: cmd.name,
            description: cmd.description,
            source: re_tui::CommandSource::Agent,
            source_name: cmd.plugin_id,
        })
        .collect();

    if !commands.is_empty() {
        shell.set_agent_commands(commands, prefix);
    }
}

/// Discovers sidebar panels from all plugins and registers them in the shell.
///
/// Each panel is tagged with whether it comes from an agent plugin (used
/// for visual grouping in the TUI sidebar).
fn register_sidebar_panels(shell: &mut re_tui::TuiShell) {
    let all_panels = catalog::collect_tui_panels_from_plugins();
    let sidebar_panels: Vec<re_tui::SidebarPanel> = all_panels
        .into_iter()
        .map(|(plugin_id, kind, panel)| re_tui::SidebarPanel {
            title: panel.title,
            items: panel.blocks.into_iter().map(convert_tui_block).collect(),
            is_agent: kind == re_plugin::PluginKind::AgentRuntime,
            plugin_id,
        })
        .collect();

    shell.set_sidebar_panels(sidebar_panels);
}

/// Discovers idle hints from all plugins (shown when agent completes work).
fn register_idle_hints(shell: &mut re_tui::TuiShell) {
    let plugin_hints = catalog::collect_idle_hints_from_plugins();
    let idle_hints: Vec<re_tui::IdleHint> = plugin_hints
        .into_iter()
        .map(|h| re_tui::IdleHint {
            command: h.command,
            description: h.description,
        })
        .collect();

    shell.set_idle_hints(idle_hints);
}

/// Converts a plugin `TuiBlock` into the TUI's `PanelItem` (struct-to-struct bridge).
///
/// Plugins produce `re_plugin::TuiBlock` values; the TUI shell expects
/// `re_tui::PanelItem`. This function maps the enum variants one-to-one.
pub(crate) fn convert_tui_block(block: re_plugin::TuiBlock) -> re_tui::PanelItem {
    re_tui::PanelItem {
        label: block.label,
        value: block.value,
        hint: match block.hint {
            re_plugin::RenderHint::Inline => re_tui::PanelHint::Inline,
            re_plugin::RenderHint::Bar => re_tui::PanelHint::Bar,
            re_plugin::RenderHint::Indicator => re_tui::PanelHint::Indicator,
            re_plugin::RenderHint::Pairs => re_tui::PanelHint::Pairs,
            re_plugin::RenderHint::List => re_tui::PanelHint::List,
            re_plugin::RenderHint::Text => re_tui::PanelHint::Text,
            re_plugin::RenderHint::Separator => re_tui::PanelHint::Separator,
        },
        severity: match block.severity {
            re_plugin::Severity::Success => re_tui::PanelSeverity::Success,
            re_plugin::Severity::Warning => re_tui::PanelSeverity::Warning,
            re_plugin::Severity::Error => re_tui::PanelSeverity::Error,
            re_plugin::Severity::Neutral => re_tui::PanelSeverity::Neutral,
        },
        numeric: block.numeric,
        total: block.total,
        pairs: block.pairs,
        items: block.items,
    }
}
