//! Runtime command handlers.

use re_core::{
    build_runtime_snapshot, render_runtime_action_plan_for_locale,
    render_runtime_config_patch_yaml, render_runtime_issues_for_locale,
    render_runtime_status_for_locale, render_runtime_topology_for_locale,
};

use crate::{CliError, catalog, i18n};

/// Executes the runtime command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("show") => Ok(show_runtime(locale)),
        Some("issues") => Ok(show_runtime_issues(locale)),
        Some("patch") => Ok(show_runtime_config_patch()),
        Some("plan") => Ok(show_runtime_action_plan(locale)),
        Some("status") => Ok(show_runtime_status(locale)),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "runtime", other,
        ))),
    }
}

fn show_runtime(locale: &str) -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let runtime = build_runtime_snapshot(&topology);

    render_runtime_topology_for_locale(&runtime.topology, locale)
}

fn show_runtime_status(locale: &str) -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let runtime = build_runtime_snapshot(&topology);

    render_runtime_status_for_locale(&runtime.status, locale)
}

fn show_runtime_issues(locale: &str) -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let runtime = build_runtime_snapshot(&topology);

    render_runtime_issues_for_locale(&runtime.issues, locale)
}

fn show_runtime_action_plan(locale: &str) -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let runtime = build_runtime_snapshot(&topology);

    render_runtime_action_plan_for_locale(&runtime.actions, locale)
}

fn show_runtime_config_patch() -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let runtime = build_runtime_snapshot(&topology);

    render_runtime_config_patch_yaml(&runtime.config_patch)
}
