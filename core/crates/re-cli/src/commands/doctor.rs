//! Runtime doctor command handlers.

use re_config::{
    apply_project_config_patch, default_project_config, render_owned_project_config_yaml,
};
use re_core::{build_runtime_snapshot, render_runtime_doctor_report_for_locale};

use crate::{CliError, catalog, i18n};

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
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let runtime = build_runtime_snapshot(&topology);
    let report = runtime.doctor_report();

    render_runtime_doctor_report_for_locale(&report, locale)
}

fn show_runtime_patched_config() -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let runtime = build_runtime_snapshot(&topology);
    let config = apply_project_config_patch(
        &default_project_config(),
        &runtime.config_patch.plugins,
        &runtime.config_patch.mcp_servers,
    );

    render_owned_project_config_yaml(&config)
}
