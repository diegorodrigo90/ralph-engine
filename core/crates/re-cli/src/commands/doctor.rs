//! Runtime doctor command handlers.

use std::path::Path;

use re_core::render_runtime_doctor_report_for_locale;

use super::runtime_state::{
    apply_official_runtime_patched_config, render_official_runtime_patched_config,
    with_official_runtime_snapshot,
};
use crate::{CliError, i18n};

/// Executes the doctor command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("runtime") => Ok(show_runtime_doctor(locale)),
        Some("config") => Ok(show_runtime_patched_config()),
        Some("apply-config") => apply_runtime_patched_config(
            args.get(1).map(String::as_str),
            locale,
            "doctor apply-config",
        ),
        Some("write-config") => apply_runtime_patched_config(
            args.get(1).map(String::as_str),
            locale,
            "doctor write-config",
        ),
        Some(other) => Err(CliError::usage(i18n::unknown_subcommand(
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

fn apply_runtime_patched_config(
    output_path: Option<&str>,
    locale: &str,
    command_path: &str,
) -> Result<String, CliError> {
    let output_path = output_path
        .ok_or_else(|| CliError::new(i18n::missing_output_path(locale, command_path)))?;

    apply_official_runtime_patched_config(Path::new(output_path), locale, command_path)
}
