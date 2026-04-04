//! TUI dashboard demo command.
//!
//! Launches the ratatui-based terminal dashboard with simulated
//! agent activity. In production, `ralph-engine run` uses the TUI
//! by default — this command exists for testing the dashboard.

use crate::CliError;

/// Executes the TUI demo with simulated agent activity.
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn execute(_args: &[String], locale: &str) -> Result<String, CliError> {
    let config = re_tui::TuiConfig {
        mode: re_tui::TuiMode::Autonomous,
        title: "Demo task".to_owned(),
        agent_id: "demo.claude".to_owned(),
        locale: locale.to_owned(),
    };

    let mut shell = re_tui::TuiShell::new(config);

    shell.set_sidebar_panels(vec![
        re_tui::SidebarPanel {
            title: "Sprint".to_owned(),
            lines: vec![
                "Done: 26".to_owned(),
                "Doing: 1".to_owned(),
                "Todo: 0".to_owned(),
            ],
            plugin_id: "official.bmad".to_owned(),
        },
        re_tui::SidebarPanel {
            title: "Findings".to_owned(),
            lines: vec!["3 sections".to_owned(), "42 lines".to_owned()],
            plugin_id: "official.findings".to_owned(),
        },
    ]);

    // Seed initial activity
    let demo_lines = [
        r#"{"type":"system","text":"Session initialized"}"#,
        r#"{"type":"content_block_start","content_block":{"type":"tool_use","name":"Read","id":"t1"}}"#,
        r#"{"type":"content_block_delta","delta":{"type":"text_delta","text":"Reading AGENTS.md..."}}"#,
        r#"{"type":"tool_result","name":"Read","content":"58 Golden Rules"}"#,
        r#"{"type":"content_block_start","content_block":{"type":"tool_use","name":"Grep","id":"t2"}}"#,
        r#"{"type":"content_block_delta","delta":{"type":"text_delta","text":"Scanning for violations..."}}"#,
        r#"{"type":"tool_result","name":"Grep","content":"0 violations"}"#,
        r#"{"type":"content_block_delta","delta":{"type":"text_delta","text":"All checks passed."}}"#,
    ];

    for (i, line) in demo_lines.iter().enumerate() {
        let event = re_tui::parse_stream_line(line);
        shell.process_event(&event);
        let pct = ((i + 1) as f32 / demo_lines.len() as f32 * 100.0) as u16;
        shell.set_progress(pct);
    }

    shell.push_activity(String::new());
    shell.push_activity(">> Press 'p' to pause, '?' for help, 'q' to quit".to_owned());

    shell.run_demo().map_err(|e| CliError::new(e.message))?;

    Ok("TUI demo exited.".to_owned())
}
