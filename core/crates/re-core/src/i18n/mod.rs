mod en;
mod pt_br;

#[derive(Clone, Copy)]
enum RuntimeLocale {
    En,
    PtBr,
}

impl RuntimeLocale {
    fn resolve(locale: &str) -> Self {
        if re_config::resolve_locale_or_default(locale) == "pt-br" {
            Self::PtBr
        } else {
            Self::En
        }
    }
}

macro_rules! locale_label {
    ($fn_name:ident, $en:ident, $pt:ident) => {
        pub(crate) fn $fn_name(locale: &str) -> &'static str {
            match RuntimeLocale::resolve(locale) {
                RuntimeLocale::En => en::$en,
                RuntimeLocale::PtBr => pt_br::$pt,
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

pub(crate) fn translate_runtime_reason(locale: &str, reason: &str) -> String {
    match RuntimeLocale::resolve(locale) {
        RuntimeLocale::En => en::translate_runtime_reason(reason),
        RuntimeLocale::PtBr => pt_br::translate_runtime_reason(reason),
    }
}
