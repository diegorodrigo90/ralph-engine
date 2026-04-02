mod en;
mod pt_br;

fn is_pt_br(locale: &str) -> bool {
    locale.eq_ignore_ascii_case("pt-br")
}

macro_rules! locale_label {
    ($fn_name:ident, $en:ident, $pt:ident) => {
        pub(crate) fn $fn_name(locale: &str) -> &'static str {
            if is_pt_br(locale) {
                pt_br::$pt
            } else {
                en::$en
            }
        }
    };
}

locale_label!(runtime_phase_label, RUNTIME_PHASE, RUNTIME_PHASE);
locale_label!(runtime_health_label, RUNTIME_HEALTH, RUNTIME_HEALTH);
locale_label!(locale_label, LOCALE, LOCALE);
locale_label!(plugins_label, PLUGINS, PLUGINS);
locale_label!(capabilities_label, CAPABILITIES, CAPABILITIES);
locale_label!(templates_label, TEMPLATES, TEMPLATES);
locale_label!(prompts_label, PROMPTS, PROMPTS);
locale_label!(agent_runtimes_label, AGENT_RUNTIMES, AGENT_RUNTIMES);
locale_label!(checks_label, CHECKS, CHECKS);
locale_label!(providers_label, PROVIDERS, PROVIDERS);
locale_label!(policies_label, POLICIES, POLICIES);
locale_label!(runtime_hooks_label, RUNTIME_HOOKS, RUNTIME_HOOKS);
locale_label!(mcp_servers_label, MCP_SERVERS, MCP_SERVERS);
locale_label!(runtime_issues_label, RUNTIME_ISSUES, RUNTIME_ISSUES);
locale_label!(
    runtime_action_plan_label,
    RUNTIME_ACTION_PLAN,
    RUNTIME_ACTION_PLAN
);
locale_label!(runtime_doctor_label, RUNTIME_DOCTOR, RUNTIME_DOCTOR);
