//! TUI dashboard demo command.
//!
//! Launches the ratatui-based terminal dashboard with simulated
//! agent activity and logo. In production, `ralph-engine run` uses
//! the TUI by default — this command exists for testing.

use ratatui::crossterm::event::{self, Event, KeyEventKind};

use crate::CliError;
use crate::catalog;

/// Simulated agent events for the demo.
const DEMO_EVENTS: &[&str] = &[
    r#"{"type":"system","text":"Session initialized"}"#,
    r#"{"type":"content_block_start","content_block":{"type":"tool_use","name":"Read","id":"t1"}}"#,
    r#"{"type":"content_block_delta","delta":{"type":"text_delta","text":"Reading AGENTS.md..."}}"#,
    r#"{"type":"tool_result","name":"Read","content":"58 Golden Rules"}"#,
    r#"{"type":"content_block_start","content_block":{"type":"tool_use","name":"Grep","id":"t2"}}"#,
    r#"{"type":"content_block_delta","delta":{"type":"text_delta","text":"Scanning for violations..."}}"#,
    r#"{"type":"tool_result","name":"Grep","content":"0 violations"}"#,
    r#"{"type":"content_block_delta","delta":{"type":"text_delta","text":"All checks passed. Quality gates green."}}"#,
];

/// Executes the TUI demo.
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn execute(_args: &[String], locale: &str) -> Result<String, CliError> {
    let config = re_tui::TuiConfig {
        title: "Demo task".to_owned(),
        agent_id: "demo.agent".to_owned(),
        locale: locale.to_owned(),
    };

    let mut shell = re_tui::TuiShell::new(config);
    let cwd = std::env::current_dir().unwrap_or_default();

    // Auto-discover: does any plugin want an input bar?
    if catalog::any_plugin_wants_input_bar() {
        shell.enable_input();
    }

    // Auto-discover: agent commands for autocomplete
    let commands: Vec<re_tui::CommandEntry> = catalog::collect_agent_commands_from_plugins(&cwd)
        .into_iter()
        .map(|cmd| re_tui::CommandEntry {
            name: cmd.name,
            description: cmd.description,
        })
        .collect();
    if !commands.is_empty() {
        let prefix = catalog::agent_command_prefix("official.claude");
        shell.set_agent_commands(commands, prefix);
    }

    // Auto-discover: sidebar panels from all plugins
    let panels: Vec<re_tui::SidebarPanel> = catalog::collect_tui_panels_from_plugins()
        .into_iter()
        .map(|(plugin_id, panel)| re_tui::SidebarPanel {
            title: panel.title,
            lines: panel.lines,
            plugin_id,
        })
        .collect();
    shell.set_sidebar_panels(panels);

    shell.push_startup_banner();

    let mut terminal = ratatui::init();
    let mut event_index = 0usize;
    let mut tick_counter = 0u32;

    let result: Result<(), String> = (|| {
        loop {
            terminal
                .draw(|frame| shell.render_frame(frame))
                .map_err(|e| format!("render: {e}"))?;

            // Feed demo events every ~800ms
            if tick_counter.rem_euclid(8) == 0 && event_index < DEMO_EVENTS.len() {
                let ev = re_tui::parse_stream_line(DEMO_EVENTS[event_index]);
                shell.process_event(&ev);
                event_index += 1;
                let pct = ((event_index as f32 / DEMO_EVENTS.len() as f32) * 100.0) as u16;
                shell.set_progress(pct);
            }
            tick_counter += 1;

            if event::poll(std::time::Duration::from_millis(100))
                .map_err(|e| format!("poll: {e}"))?
                && let Event::Key(key) = event::read().map_err(|e| format!("read: {e}"))?
                && key.kind == KeyEventKind::Press
            {
                shell.handle_key_with_modifiers(key.code, key.modifiers);

                // If user submitted text, show it (demo — no real agent to dispatch to)
                if let Some(text) = shell.take_text_input() {
                    shell.push_activity(format!(">> [demo] Would send to agent: {text}"));
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
    Ok("TUI demo exited.".to_owned())
}
