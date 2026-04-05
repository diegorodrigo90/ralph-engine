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
use crate::i18n;

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
            i18n::tui_dashboard_title(locale).to_owned()
        } else {
            i18n::tui_no_project_title(locale).to_owned()
        },
        agent_id: detect_agent_id(locale),
        locale: locale.to_owned(),
    };

    let mut shell = re_tui::TuiShell::new(config);
    shell.set_labels(build_labels(locale));
    // Dashboard starts idle (no agent running)
    shell.set_state(re_tui::TuiState::Complete);
    let cwd = std::env::current_dir().unwrap_or_default();

    // Auto-discover: enable input bar if any plugin requests it
    if catalog::any_plugin_wants_input_bar() {
        shell.enable_input();
    } else {
        // Dashboard always has input for slash commands
        shell.enable_input();
    }

    // Auto-discover: command prefix from configured agent plugin (Model B)
    let prefix = if let Ok(config) = super::runtime_state::load_project_config() {
        config
            .run
            .agent_plugin
            .map(catalog::agent_command_prefix)
            .unwrap_or_else(|| "/".to_owned())
    } else {
        "/".to_owned()
    };

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

    // Add plugin-discovered agent commands (auto-discovery)
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

    // Add plugin-contributed CLI commands (auto-discovery)
    let cli_commands: Vec<re_tui::CommandEntry> = catalog::collect_cli_contributions_from_plugins()
        .into_iter()
        .map(|(plugin_id, contrib)| re_tui::CommandEntry {
            name: contrib.name.clone(),
            description: contrib.description,
            source: re_tui::CommandSource::Plugin,
            source_name: plugin_id,
        })
        .collect();
    commands.extend(cli_commands);

    if !commands.is_empty() {
        shell.set_agent_commands(commands, prefix);
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

    // Auto-discover keybindings from plugins
    let keybindings = catalog::collect_tui_keybindings_from_plugins();
    shell.set_plugin_keybindings(keybindings);

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
                        let action = shell.handle_key_with_modifiers(key.code, key.modifiers);

                        // Dispatch unhandled keys to plugin runtimes
                        if action == re_tui::PluginKeyAction::NotHandled
                            && let ratatui::crossterm::event::KeyCode::Char(c) = key.code
                        {
                            let state_label = format!("{:?}", shell.state());
                            if let Some(binding) = shell.find_active_binding(c, &state_label) {
                                let plugin_id = binding.plugin_id.clone();
                                let result = catalog::dispatch_plugin_tui_key(
                                    &plugin_id,
                                    &c.to_string(),
                                    &state_label,
                                );
                                shell.apply_plugin_action(&result);
                            }
                        }

                        // Dispatch slash commands or text input
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
        // Not a slash command — try plugin text input handlers first
        let cwd = std::env::current_dir().unwrap_or_default();
        let result = catalog::dispatch_plugin_text_input(trimmed, &cwd);
        if result != re_tui::PluginKeyAction::NotHandled {
            shell.apply_plugin_action(&result);
            return;
        }
        // No plugin handled it — show "no agent" message
        let you = shell.labels().you_label.clone();
        let msg = shell.labels().no_agent_message.clone();
        shell.push_activity(format!("  ╭─ {you}: {trimmed}"));
        shell.push_activity(format!("  ╰─ {msg}"));
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
            shell.push_activity(format!("── {} ──", i18n::tui_available_commands(locale)));
            for (name, desc) in DASHBOARD_COMMANDS {
                shell.push_activity(format!("  /{name:<12} {desc}"));
            }
        }
        "init" => {
            // Use --auto for non-interactive TUI init
            let mut args = vec!["--auto".to_owned()];
            args.extend(parts[1..].iter().map(|s| (*s).to_owned()));
            match super::dispatch_command("init", &args, locale) {
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
        "run" | "doctor" | "plugins" | "agents" | "config" | "runtime" | "checks" | "templates"
        | "prompts" | "hooks" | "mcp" | "capabilities" | "providers" | "locales" => {
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
                "  {}: /{other}. {}",
                i18n::tui_unknown_command(locale),
                i18n::tui_type_help_hint(locale)
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
                shell.push_activity(format!("  {}", i18n::tui_project_run_hint(locale)));
            }
        }
    } else {
        shell.push_activity(format!("  {}", i18n::tui_no_config_found(locale)));
        shell.push_activity(format!("  {}", i18n::tui_type_init_tui(locale)));
    }
}

/// Detects the configured agent ID from project config, if available.
fn detect_agent_id(locale: &str) -> String {
    if let Ok(config) = super::runtime_state::load_project_config() {
        config
            .run
            .agent_id
            .unwrap_or(i18n::tui_no_agent_label(locale))
            .to_owned()
    } else {
        i18n::tui_no_project_label(locale).to_owned()
    }
}

/// Builds localized TUI labels from the CLI i18n system.
fn build_labels(locale: &str) -> re_tui::TuiLabels {
    re_tui::TuiLabels {
        project_configured: i18n::tui_project_configured(locale).to_owned(),
        no_project_found: i18n::tui_no_project_found(locale).to_owned(),
        type_run: i18n::tui_type_run(locale).to_owned(),
        type_init: i18n::tui_type_init(locale).to_owned(),
        orchestration_runtime: i18n::tui_orchestration_runtime(locale).to_owned(),
        waiting_session: i18n::tui_waiting_session(locale).to_owned(),
        help_title: i18n::tui_help_keys_heading(locale).to_owned(),
        nav_heading: i18n::tui_help_keys_heading(locale).to_owned(),
        actions_heading: i18n::tui_help_commands_heading(locale).to_owned(),
        plugins_heading: i18n::tui_help_plugin_keys(locale).to_owned(),
        slash_hint: i18n::tui_help_type_slash(locale).to_owned(),
        press_any_key: if locale == "pt-br" {
            "Pressione qualquer tecla para fechar".to_owned()
        } else {
            "Press any key to close".to_owned()
        },
        quit_title: if locale == "pt-br" {
            "Sair".to_owned()
        } else {
            "Quit".to_owned()
        },
        quit_question: if locale == "pt-br" {
            "Sair?".to_owned()
        } else {
            "Quit?".to_owned()
        },
        modal_open_hint: if locale == "pt-br" {
            "Modal aberto — pressione uma tecla".to_owned()
        } else {
            "Modal open — press a key".to_owned()
        },
        state_running: if locale == "pt-br" {
            "EXECUTANDO"
        } else {
            "RUNNING"
        }
        .to_owned(),
        state_paused: if locale == "pt-br" {
            "PAUSADO"
        } else {
            "PAUSED"
        }
        .to_owned(),
        state_complete: if locale == "pt-br" {
            "COMPLETO"
        } else {
            "COMPLETE"
        }
        .to_owned(),
        state_error: if locale == "pt-br" { "ERRO" } else { "ERROR" }.to_owned(),
        pause_label: if locale == "pt-br" { "pausar" } else { "pause" }.to_owned(),
        help_label: if locale == "pt-br" { "ajuda" } else { "help" }.to_owned(),
        quit_label: if locale == "pt-br" { "sair" } else { "quit" }.to_owned(),
        control_state: if locale == "pt-br" { "Estado" } else { "State" }.to_owned(),
        control_work: if locale == "pt-br" { "Tarefa" } else { "Work" }.to_owned(),
        tools_label: if locale == "pt-br" {
            "Ferramentas"
        } else {
            "Tools"
        }
        .to_owned(),
        lines_label: if locale == "pt-br" { "Linhas" } else { "Lines" }.to_owned(),
        progress_label: if locale == "pt-br" {
            "Progresso"
        } else {
            "Progress"
        }
        .to_owned(),
        logo_tagline: if locale == "pt-br" {
            "Loop Autônomo de Desenvolvimento IA".to_owned()
        } else {
            "Autonomous AI Dev Loop".to_owned()
        },
        nav_keys: if locale == "pt-br" {
            vec![
                ("j/k".into(), "Focar blocos".into()),
                ("↑↓".into(), "Rolar linhas".into()),
                ("PgUp/PgDn".into(), "Rolar páginas".into()),
                ("G / End".into(), "Seguir".into()),
                ("Home".into(), "Início".into()),
            ]
        } else {
            vec![
                ("j/k".into(), "Focus blocks".into()),
                ("↑↓".into(), "Scroll lines".into()),
                ("PgUp/PgDn".into(), "Scroll pages".into()),
                ("G / End".into(), "Follow mode".into()),
                ("Home".into(), "Scroll to top".into()),
            ]
        },
        action_keys: if locale == "pt-br" {
            vec![
                ("⏎ Enter".into(), "Expandir/recolher".into()),
                ("y".into(), "Copiar bloco".into()),
                ("⎋ Esc".into(), "Limpar foco".into()),
                ("F2".into(), "Alternar sidebar".into()),
                ("Ctrl+A".into(), "Trocar agente".into()),
                ("?".into(), "Esta ajuda".into()),
                ("q".into(), "Sair".into()),
            ]
        } else {
            vec![
                ("⏎ Enter".into(), "Expand/collapse".into()),
                ("y".into(), "Copy block".into()),
                ("⎋ Esc".into(), "Clear focus".into()),
                ("F2".into(), "Toggle sidebar".into()),
                ("Ctrl+A".into(), "Agent switcher".into()),
                ("?".into(), "This help".into()),
                ("q".into(), "Quit".into()),
            ]
        },
        you_label: if locale == "pt-br" {
            "Você".to_owned()
        } else {
            "You".to_owned()
        },
        no_agent_message: if locale == "pt-br" {
            "Nenhum agente conectado. Use /run para iniciar orquestração.".to_owned()
        } else {
            "No agent connected. Use /run to start orchestration.".to_owned()
        },
    }
}
