pub(super) struct RuntimeLocaleCatalog {
    pub runtime_phase: &'static str,
    pub runtime_health: &'static str,
    pub locale: &'static str,
    pub plugins: &'static str,
    pub capabilities: &'static str,
    pub templates: &'static str,
    pub prompts: &'static str,
    pub agent_runtimes: &'static str,
    pub checks: &'static str,
    pub providers: &'static str,
    pub policies: &'static str,
    pub runtime_hooks: &'static str,
    pub mcp_servers: &'static str,
    pub runtime_issues: &'static str,
    pub runtime_action_plan: &'static str,
    pub runtime_doctor: &'static str,
    pub translate_runtime_reason: fn(&str) -> String,
}

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

fn locale_catalog(locale: &str) -> &'static RuntimeLocaleCatalog {
    match RuntimeLocale::resolve(locale) {
        RuntimeLocale::En => &en::LOCALE,
        RuntimeLocale::PtBr => &pt_br::LOCALE,
    }
}

macro_rules! locale_label {
    ($fn_name:ident, $field:ident) => {
        pub(crate) fn $fn_name(locale: &str) -> &'static str {
            locale_catalog(locale).$field
        }
    };
}

locale_label!(runtime_phase_label, runtime_phase);
locale_label!(runtime_health_label, runtime_health);
locale_label!(locale_label, locale);
locale_label!(plugins_label, plugins);
locale_label!(capabilities_label, capabilities);
locale_label!(templates_label, templates);
locale_label!(prompts_label, prompts);
locale_label!(agent_runtimes_label, agent_runtimes);
locale_label!(checks_label, checks);
locale_label!(providers_label, providers);
locale_label!(policies_label, policies);
locale_label!(runtime_hooks_label, runtime_hooks);
locale_label!(mcp_servers_label, mcp_servers);
locale_label!(runtime_issues_label, runtime_issues);
locale_label!(runtime_action_plan_label, runtime_action_plan);
locale_label!(runtime_doctor_label, runtime_doctor);

pub(crate) fn translate_runtime_reason(locale: &str, reason: &str) -> String {
    (locale_catalog(locale).translate_runtime_reason)(reason)
}
