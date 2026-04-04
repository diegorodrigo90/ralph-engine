//! TUI dashboard demo command.
//!
//! Launches the ratatui-based terminal dashboard with simulated
//! agent activity and logo rendering. In production, `ralph-engine run`
//! uses the TUI by default — this command exists for testing.

use crate::CliError;

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
    use ratatui::crossterm::event::{self, Event, KeyEventKind};
    use ratatui::layout::{Constraint, Layout};

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

    shell.push_startup_banner();

    // Try to create logo image (best-effort — works without it)
    let mut logo_protocol = re_tui::logo::create_logo();

    let mut terminal = ratatui::init();
    let mut event_index = 0usize;
    let mut tick_counter = 0u32;

    let result: Result<(), String> = (|| {
        loop {
            terminal
                .draw(|frame| {
                    let area = frame.area();

                    // If logo available, split top area for logo + dashboard
                    if let Some(ref mut protocol) = logo_protocol {
                        let rows = Layout::vertical([
                            Constraint::Length(6), // Logo area
                            Constraint::Fill(1),   // Dashboard
                        ])
                        .split(area);

                        re_tui::logo::render_logo(frame, rows[0], protocol);

                        // Render dashboard in remaining space using a sub-frame trick:
                        // we render the shell into the lower area
                        shell.render_frame_in_area(frame, rows[1]);
                    } else {
                        shell.render_frame(frame);
                    }
                })
                .map_err(|e| format!("render: {e}"))?;

            // Feed demo events
            #[allow(clippy::manual_is_multiple_of)]
            if tick_counter % 8 == 0 && event_index < DEMO_EVENTS.len() {
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
                shell.handle_key(key.code);
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
