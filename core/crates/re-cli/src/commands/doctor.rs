//! Runtime doctor command handlers.

use re_core::render_runtime_doctor_report_for_locale;

use super::runtime_state::{
    render_official_runtime_patched_config, with_official_runtime_snapshot,
};
use crate::{CliError, i18n};

/// Executes the doctor command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("runtime") => Ok(show_runtime_doctor(locale)),
        Some("config") => Ok(show_runtime_patched_config()),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "doctor", other,
        ))),
    }
}

fn show_runtime_doctor(locale: &str) -> String {
    with_official_runtime_snapshot(|runtime| {
        let report = runtime.doctor_report();

        render_runtime_doctor_report_for_locale(&report, locale)
    })
}

fn show_runtime_patched_config() -> String {
    render_official_runtime_patched_config()
}
