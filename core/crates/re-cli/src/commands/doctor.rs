//! Runtime doctor command handlers.

use re_core::{build_runtime_doctor_report, render_runtime_doctor_report};

use crate::{CliError, catalog};

/// Executes the doctor command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("runtime") => Ok(show_runtime_doctor()),
        Some(other) => Err(CliError::new(format!("unknown doctor command: {other}"))),
    }
}

fn show_runtime_doctor() -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let report = build_runtime_doctor_report(&topology);

    render_runtime_doctor_report(&report)
}
