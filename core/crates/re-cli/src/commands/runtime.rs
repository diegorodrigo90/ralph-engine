//! Runtime command handlers.

use re_core::{
    build_runtime_action_plan, collect_runtime_issues, evaluate_runtime_status,
    render_runtime_action_plan_for_locale, render_runtime_issues_for_locale,
    render_runtime_status_for_locale, render_runtime_topology_for_locale,
};

use crate::{CliError, catalog, i18n};

/// Executes the runtime command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("show") => Ok(show_runtime(locale)),
        Some("issues") => Ok(show_runtime_issues(locale)),
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

    render_runtime_topology_for_locale(&topology, locale)
}

fn show_runtime_status(locale: &str) -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let status = evaluate_runtime_status(&topology);

    render_runtime_status_for_locale(&status, locale)
}

fn show_runtime_issues(locale: &str) -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let issues = collect_runtime_issues(&topology);

    render_runtime_issues_for_locale(&issues, locale)
}

fn show_runtime_action_plan(locale: &str) -> String {
    let snapshot = catalog::official_runtime_snapshot();
    let topology = snapshot.topology();
    let actions = build_runtime_action_plan(&topology);

    render_runtime_action_plan_for_locale(&actions, locale)
}
