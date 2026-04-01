//! Runtime command handlers.

use re_core::{
    build_runtime_action_plan, collect_runtime_issues, evaluate_runtime_status,
    render_runtime_action_plan, render_runtime_issues, render_runtime_status,
    render_runtime_topology,
};

use crate::{CliError, catalog};

/// Executes the runtime command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("show") => Ok(show_runtime()),
        Some("issues") => Ok(show_runtime_issues()),
        Some("plan") => Ok(show_runtime_action_plan()),
        Some("status") => Ok(show_runtime_status()),
        Some(other) => Err(CliError::new(format!("unknown runtime command: {other}"))),
    }
}

fn show_runtime() -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();

    render_runtime_topology(&topology)
}

fn show_runtime_status() -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let status = evaluate_runtime_status(&topology);

    render_runtime_status(&status)
}

fn show_runtime_issues() -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let issues = collect_runtime_issues(&topology);

    render_runtime_issues(&issues)
}

fn show_runtime_action_plan() -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let actions = build_runtime_action_plan(&topology);

    render_runtime_action_plan(&actions)
}
