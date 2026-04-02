//! Runtime command handlers.

use re_core::{
    render_runtime_action_plan_for_locale, render_runtime_agent_bootstrap_plans_for_locale,
    render_runtime_check_execution_plans_for_locale, render_runtime_config_patch_yaml,
    render_runtime_issues_for_locale, render_runtime_mcp_launch_plans_for_locale,
    render_runtime_policy_enforcement_plans_for_locale,
    render_runtime_provider_registration_plans_for_locale, render_runtime_status_for_locale,
    render_runtime_topology_for_locale,
};

use super::runtime_state::{
    render_official_runtime_patched_config, with_official_runtime_snapshot,
};
use crate::{CliError, i18n};

/// Executes the runtime command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("show") => Ok(show_runtime(locale)),
        Some("issues") => Ok(show_runtime_issues(locale)),
        Some("patch") => Ok(show_runtime_config_patch()),
        Some("patched-config") => Ok(show_runtime_patched_config()),
        Some("plan") => Ok(show_runtime_action_plan(locale)),
        Some("agent-plans") => Ok(show_runtime_agent_bootstrap_plans(locale)),
        Some("provider-plans") => Ok(show_runtime_provider_registration_plans(locale)),
        Some("check-plans") => Ok(show_runtime_check_execution_plans(locale)),
        Some("policy-plans") => Ok(show_runtime_policy_enforcement_plans(locale)),
        Some("mcp-plans") => Ok(show_runtime_mcp_launch_plans(locale)),
        Some("status") => Ok(show_runtime_status(locale)),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "runtime", other,
        ))),
    }
}

fn show_runtime(locale: &str) -> String {
    with_official_runtime_snapshot(|runtime| {
        render_runtime_topology_for_locale(&runtime.topology, locale)
    })
}

fn show_runtime_status(locale: &str) -> String {
    with_official_runtime_snapshot(|runtime| {
        render_runtime_status_for_locale(&runtime.status, locale)
    })
}

fn show_runtime_issues(locale: &str) -> String {
    with_official_runtime_snapshot(|runtime| {
        render_runtime_issues_for_locale(&runtime.issues, locale)
    })
}

fn show_runtime_action_plan(locale: &str) -> String {
    with_official_runtime_snapshot(|runtime| {
        render_runtime_action_plan_for_locale(&runtime.actions, locale)
    })
}

fn show_runtime_agent_bootstrap_plans(locale: &str) -> String {
    with_official_runtime_snapshot(|runtime| {
        render_runtime_agent_bootstrap_plans_for_locale(&runtime.agent_bootstrap_plans, locale)
    })
}

fn show_runtime_provider_registration_plans(locale: &str) -> String {
    with_official_runtime_snapshot(|runtime| {
        render_runtime_provider_registration_plans_for_locale(
            &runtime.provider_registration_plans,
            locale,
        )
    })
}

fn show_runtime_check_execution_plans(locale: &str) -> String {
    with_official_runtime_snapshot(|runtime| {
        render_runtime_check_execution_plans_for_locale(&runtime.check_execution_plans, locale)
    })
}

fn show_runtime_policy_enforcement_plans(locale: &str) -> String {
    with_official_runtime_snapshot(|runtime| {
        render_runtime_policy_enforcement_plans_for_locale(
            &runtime.policy_enforcement_plans,
            locale,
        )
    })
}

fn show_runtime_mcp_launch_plans(locale: &str) -> String {
    with_official_runtime_snapshot(|runtime| {
        render_runtime_mcp_launch_plans_for_locale(&runtime.mcp_launch_plans, locale)
    })
}

fn show_runtime_config_patch() -> String {
    with_official_runtime_snapshot(|runtime| {
        render_runtime_config_patch_yaml(&runtime.config_patch)
    })
}

fn show_runtime_patched_config() -> String {
    render_official_runtime_patched_config()
}
