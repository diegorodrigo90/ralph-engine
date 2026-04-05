//! TUI dashboard command.
//!
//! Launches the ratatui-based orchestration dashboard. When invoked
//! directly (`ralph-engine tui`) or as the default (`ralph-engine`
//! with no args), opens the interactive TUI.
//!
//! The dashboard is **functional** — slash commands typed in the input
//! bar are dispatched to the real CLI command handlers. `/run` starts
//! orchestration, `/doctor` checks health, `/plugins` lists plugins, etc.

use ratatui::crossterm::event::{self, Event, KeyEventKind, MouseEvent};

use crate::CliError;
use crate::catalog;

/// Built-in slash commands available in the dashboard.
const DASHBOARD_COMMANDS: &[(&str, &str)] = &[
    ("run", "Start orchestration with TUI"),
    ("doctor", "Check project health"),
    ("plugins", "List installed plugins"),
    ("agents", "List available agents"),
    ("init", "Initialize project"),
    ("config", "Show configuration"),
    ("runtime", "Inspect runtime state"),
    ("help", "Show available commands"),
];

/// Executes the TUI dashboard.
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn execute(_args: &[String], locale: &str) -> Result<String, CliError> {
    let has_config = std::path::Path::new(".ralph-engine/config.yaml").exists();

    let config = re_tui::TuiConfig {
        title: if has_config {
            "Dashboard".to_owned()
        } else {
            "No project — type /init".to_owned()
        },
        agent_id: detect_agent_id(),
        locale: locale.to_owned(),
    };

    let mut shell = re_tui::TuiShell::new(config);
    let cwd = std::env::current_dir().unwrap_or_default();

    // Enable input bar — the dashboard is interactive
    shell.enable_input();

    // Register built-in dashboard commands for autocomplete
    let mut commands: Vec<re_tui::CommandEntry> = DASHBOARD_COMMANDS
        .iter()
        .map(|(name, desc)| re_tui::CommandEntry {
            name: (*name).to_owned(),
            description: (*desc).to_owned(),
            source: re_tui::CommandSource::Plugin,
            source_name: "dashboard".to_owned(),
        })
        .collect();

    // Add plugin-discovered agent commands
    let agent_commands: Vec<re_tui::CommandEntry> =
        catalog::collect_agent_commands_from_plugins(&cwd)
            .into_iter()
            .map(|cmd| re_tui::CommandEntry {
                name: cmd.name.clone(),
                description: cmd.description,
                source: re_tui::CommandSource::Agent,
                source_name: cmd.plugin_id,
            })
            .collect();
    commands.extend(agent_commands);

    if !commands.is_empty() {
        shell.set_agent_commands(commands, "/".to_owned());
    }

    // Auto-discover sidebar panels from plugins
    let panels: Vec<re_tui::SidebarPanel> = catalog::collect_tui_panels_from_plugins()
        .into_iter()
        .map(|(plugin_id, panel)| re_tui::SidebarPanel {
            title: panel.title,
            lines: panel.lines,
            plugin_id,
        })
        .collect();
    shell.set_sidebar_panels(panels);

    // Show project status on startup
    push_project_status(&mut shell, has_config, locale);

    let mut terminal = ratatui::init();

    let result: Result<(), String> = (|| {
        loop {
            terminal
                .draw(|frame| shell.render_frame(frame))
                .map_err(|e| format!("render: {e}"))?;

            if event::poll(std::time::Duration::from_millis(50))
                .map_err(|e| format!("poll: {e}"))?
            {
                match event::read().map_err(|e| format!("read: {e}"))? {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        shell.handle_key_with_modifiers(key.code, key.modifiers);

                        // Dispatch slash commands
                        if let Some(text) = shell.take_text_input() {
                            handle_dashboard_command(&mut shell, &text, locale);
                        }
                    }
                    Event::Mouse(MouseEvent { kind, .. }) => {
                        shell.handle_mouse(kind);
                    }
                    _ => {}
                }
            }

            if shell.should_quit() {
                break;
            }
        }
        Ok(())
    })();

    ratatui::restore();
    result.map_err(CliError::new)?;
    Ok(String::new())
}

/// Dispatches a slash command typed in the TUI input bar.
///
/// Commands starting with `/` are parsed and dispatched to the real
/// CLI command handlers. The output is pushed to the activity feed.
fn handle_dashboard_command(shell: &mut re_tui::TuiShell, input: &str, locale: &str) {
    let trimmed = input.trim();

    // Parse slash command
    let Some(command_text) = trimmed.strip_prefix('/') else {
        // Not a slash command — show as message
        shell.push_activity(format!(">> {trimmed}"));
        return;
    };

    let parts: Vec<&str> = command_text.split_whitespace().collect();
    let Some(cmd_name) = parts.first() else {
        return;
    };

    // Show what we're running
    shell.push_activity(format!(">> /{command_text}"));

    match *cmd_name {
        "help" => {
            shell.push_activity("── Available commands ──".to_owned());
            for (name, desc) in DASHBOARD_COMMANDS {
                shell.push_activity(format!("  /{name:<12} {desc}"));
            }
        }
        "run" | "doctor" | "plugins" | "agents" | "config" | "runtime" | "init" | "checks"
        | "templates" | "prompts" | "hooks" | "mcp" | "capabilities" | "providers" | "locales" => {
            // Build args for the command handler
            let args: Vec<String> = parts[1..].iter().map(|s| (*s).to_owned()).collect();

            // Dispatch to the real command handler
            match super::dispatch_command(cmd_name, &args, locale) {
                Ok(output) => {
                    for line in output.lines() {
                        shell.push_activity(format!("  {line}"));
                    }
                }
                Err(e) => {
                    shell.push_activity(format!("  Error: {e}"));
                }
            }
        }
        other => {
            shell.push_activity(format!(
                "  Unknown command: /{other}. Type /help for available commands."
            ));
        }
    }
}

/// Pushes project status lines to the activity feed on startup.
fn push_project_status(shell: &mut re_tui::TuiShell, has_config: bool, locale: &str) {
    if has_config {
        // Run doctor silently to get status
        match super::dispatch_command("doctor", &[], locale) {
            Ok(output) => {
                for line in output.lines() {
                    shell.push_activity(format!("  {line}"));
                }
            }
            Err(_) => {
                shell.push_activity("  Project configured. Type /run to start.".to_owned());
            }
        }
    } else {
        shell.push_activity("  No .ralph-engine/ config found.".to_owned());
        shell.push_activity("  Type /init to set up this project.".to_owned());
    }
}

/// Detects the configured agent ID from project config, if available.
fn detect_agent_id() -> String {
    if let Ok(config) = super::runtime_state::load_project_config() {
        config.run.agent_id.unwrap_or("no agent").to_owned()
    } else {
        "no project".to_owned()
    }
}
