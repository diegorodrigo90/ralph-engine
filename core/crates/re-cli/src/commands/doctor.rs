//! Runtime doctor command handlers.

use re_core::{build_runtime_doctor_report, render_runtime_doctor_report_for_locale};

use crate::{CliError, catalog, i18n};

/// Executes the doctor command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("runtime") => Ok(show_runtime_doctor(locale)),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "doctor", other,
        ))),
    }
}

fn show_runtime_doctor(locale: &str) -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let report = build_runtime_doctor_report(&topology);

    render_runtime_doctor_report_for_locale(&report, locale)
}
