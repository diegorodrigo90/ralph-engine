pub(super) struct RuntimeLocaleCatalog {
    pub runtime_check: &'static str,
    pub runtime_check_outcome: &'static str,
    pub runtime_check_passed: &'static str,
    pub runtime_check_failed: &'static str,
    pub runtime_policy: &'static str,
    pub runtime_policy_outcome: &'static str,
    pub runtime_policy_passed: &'static str,
    pub runtime_policy_failed: &'static str,
    pub runtime_phase: &'static str,
    pub runtime_health: &'static str,
    pub provider: &'static str,
    pub load_boundary: &'static str,
    pub policy_enforcement_hook: &'static str,
    pub locale: &'static str,
    pub plugins: &'static str,
    pub capabilities: &'static str,
    pub templates: &'static str,
    pub prompts: &'static str,
    pub agent_runtimes: &'static str,
    pub runtime_agent_bootstrap_plans: &'static str,
    pub runtime_provider_registration_plans: &'static str,
    pub runtime_check_execution_plans: &'static str,
    pub runtime_policy_enforcement_plans: &'static str,
    pub checks: &'static str,
    pub providers: &'static str,
    pub policies: &'static str,
    pub runtime_hooks: &'static str,
    pub mcp_servers: &'static str,
    pub runtime_mcp_launch_plans: &'static str,
    pub runtime_issues: &'static str,
    pub runtime_action_plan: &'static str,
    pub runtime_doctor: &'static str,
    pub mcp_server_status: &'static str,
    pub mcp_server_statuses: &'static str,
    pub mcp_readiness: &'static str,
    pub mcp_readiness_ready: &'static str,
    pub mcp_readiness_not_ready: &'static str,
    pub mcp_transport: &'static str,
    pub mcp_enabled: &'static str,
    pub translate_runtime_reason: fn(&str) -> String,
}

// Hand-coded fn implementations for each locale
mod fn_en;
mod fn_pt_br;

// Locale modules and dispatch function generated from locales/*.toml
include!(concat!(env!("OUT_DIR"), "/i18n_generated.rs"));

macro_rules! locale_label {
    ($fn_name:ident, $field:ident) => {
        pub(crate) fn $fn_name(locale: &str) -> &'static str {
            locale_catalog(locale).$field
        }
    };
}

locale_label!(runtime_phase_label, runtime_phase);
locale_label!(runtime_health_label, runtime_health);
locale_label!(runtime_check_label, runtime_check);
locale_label!(runtime_check_outcome_label, runtime_check_outcome);
locale_label!(runtime_policy_label, runtime_policy);
locale_label!(runtime_policy_outcome_label, runtime_policy_outcome);
locale_label!(locale_label, locale);
locale_label!(provider_label, provider);
locale_label!(load_boundary_label, load_boundary);
locale_label!(policy_enforcement_hook_label, policy_enforcement_hook);
locale_label!(plugins_label, plugins);
locale_label!(capabilities_label, capabilities);
locale_label!(templates_label, templates);
locale_label!(prompts_label, prompts);
locale_label!(agent_runtimes_label, agent_runtimes);
locale_label!(
    runtime_agent_bootstrap_plans_label,
    runtime_agent_bootstrap_plans
);
locale_label!(
    runtime_provider_registration_plans_label,
    runtime_provider_registration_plans
);
locale_label!(
    runtime_check_execution_plans_label,
    runtime_check_execution_plans
);
locale_label!(
    runtime_policy_enforcement_plans_label,
    runtime_policy_enforcement_plans
);
locale_label!(checks_label, checks);
locale_label!(providers_label, providers);
locale_label!(policies_label, policies);
locale_label!(runtime_hooks_label, runtime_hooks);
locale_label!(mcp_servers_label, mcp_servers);
locale_label!(runtime_mcp_launch_plans_label, runtime_mcp_launch_plans);
locale_label!(runtime_issues_label, runtime_issues);
locale_label!(runtime_action_plan_label, runtime_action_plan);
locale_label!(runtime_doctor_label, runtime_doctor);
locale_label!(runtime_check_passed_label, runtime_check_passed);
locale_label!(runtime_check_failed_label, runtime_check_failed);
locale_label!(runtime_policy_passed_label, runtime_policy_passed);
locale_label!(runtime_policy_failed_label, runtime_policy_failed);

locale_label!(mcp_server_status_label, mcp_server_status);
locale_label!(mcp_server_statuses_label, mcp_server_statuses);
locale_label!(mcp_readiness_label, mcp_readiness);
locale_label!(mcp_readiness_ready_label, mcp_readiness_ready);
locale_label!(mcp_readiness_not_ready_label, mcp_readiness_not_ready);
locale_label!(mcp_transport_label, mcp_transport);
locale_label!(mcp_enabled_label, mcp_enabled);

pub(crate) fn translate_runtime_reason(locale: &str, reason: &str) -> String {
    (locale_catalog(locale).translate_runtime_reason)(reason)
}
