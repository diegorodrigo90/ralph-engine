//! Shared CLI locale resolution and message helpers.

mod en;
mod pt_br;

use std::env;

use crate::CliError;

const SUPPORTED_LOCALES: &[&str] = &["en", "pt-br"];
const LOCALE_ENV_KEY: &str = "RALPH_ENGINE_LOCALE";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CliLocale {
    En,
    PtBr,
}

impl CliLocale {
    const fn as_str(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::PtBr => "pt-br",
        }
    }
}

#[must_use]
pub fn is_pt_br(locale: &str) -> bool {
    locale.eq_ignore_ascii_case("pt-br")
}

fn parse_locale(locale: &str) -> CliLocale {
    if is_pt_br(locale) {
        CliLocale::PtBr
    } else {
        CliLocale::En
    }
}

pub fn resolve_cli_locale() -> Result<&'static str, CliError> {
    resolve_cli_locale_from_env_result(env::var(LOCALE_ENV_KEY))
}

fn resolve_cli_locale_from_env_result(
    env_result: Result<String, env::VarError>,
) -> Result<&'static str, CliError> {
    match env_result {
        Ok(value) => normalize_cli_locale(&value),
        Err(env::VarError::NotPresent) => {
            normalize_cli_locale(re_config::default_project_config().default_locale)
        }
        Err(error) => Err(CliError::new(format!(
            "failed to read {LOCALE_ENV_KEY}: {error}"
        ))),
    }
}

fn normalize_cli_locale(value: &str) -> Result<&'static str, CliError> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "en" => Ok(CliLocale::En.as_str()),
        "pt-br" => Ok(CliLocale::PtBr.as_str()),
        other => Err(CliError::new(format!(
            "unsupported locale: {other}. supported locales: {}",
            SUPPORTED_LOCALES.join(", ")
        ))),
    }
}

#[must_use]
pub fn root_bootstrapped(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::ROOT_BOOTSTRAPPED,
        CliLocale::PtBr => pt_br::ROOT_BOOTSTRAPPED,
    }
}

#[must_use]
pub fn unknown_command(locale: &str, command_name: &str) -> String {
    match parse_locale(locale) {
        CliLocale::En => en::unknown_command(command_name),
        CliLocale::PtBr => pt_br::unknown_command(command_name),
    }
}

#[must_use]
pub fn unknown_subcommand(locale: &str, command_group: &str, command_name: &str) -> String {
    match parse_locale(locale) {
        CliLocale::En => en::unknown_subcommand(command_group, command_name),
        CliLocale::PtBr => pt_br::unknown_subcommand(command_group, command_name),
    }
}

#[must_use]
pub fn missing_id(locale: &str, command_group: &str, entity_label: &str) -> String {
    match parse_locale(locale) {
        CliLocale::En => en::missing_id(command_group, entity_label),
        CliLocale::PtBr => pt_br::missing_id(command_group, entity_label),
    }
}

#[must_use]
pub fn unknown_entity(locale: &str, entity_label: &str, value: &str) -> String {
    match parse_locale(locale) {
        CliLocale::En => en::unknown_entity(entity_label, value),
        CliLocale::PtBr => pt_br::unknown_entity(entity_label, value),
    }
}

#[must_use]
pub fn list_heading(locale: &str, singular_en: &str, singular_pt: &str, count: usize) -> String {
    match parse_locale(locale) {
        CliLocale::En => format!("{singular_en} ({count})"),
        CliLocale::PtBr => format!("{singular_pt} ({count})"),
    }
}

#[must_use]
pub fn providers_heading(locale: &str, count: usize) -> String {
    match parse_locale(locale) {
        CliLocale::En => format!("{} ({count})", en::PROVIDERS_LABEL),
        CliLocale::PtBr => format!("{} ({count})", pt_br::PROVIDERS_LABEL),
    }
}

#[must_use]
pub fn detail_heading(locale: &str, label_en: &str, label_pt: &str, value: &str) -> String {
    match parse_locale(locale) {
        CliLocale::En => format!("{label_en}: {value}"),
        CliLocale::PtBr => format!("{label_pt}: {value}"),
    }
}

#[must_use]
pub fn resolved_activation_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::RESOLVED_ACTIVATION_LABEL,
        CliLocale::PtBr => pt_br::RESOLVED_ACTIVATION_LABEL,
    }
}

#[must_use]
pub fn resolved_from_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::RESOLVED_FROM_LABEL,
        CliLocale::PtBr => pt_br::RESOLVED_FROM_LABEL,
    }
}

#[must_use]
pub fn agent_runtime_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::AGENT_RUNTIME_LABEL,
        CliLocale::PtBr => pt_br::AGENT_RUNTIME_LABEL,
    }
}

#[must_use]
pub fn agent_runtimes_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::AGENT_RUNTIMES_LABEL,
        CliLocale::PtBr => pt_br::AGENT_RUNTIMES_LABEL,
    }
}

#[must_use]
pub fn template_provider_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::TEMPLATE_PROVIDER_LABEL,
        CliLocale::PtBr => pt_br::TEMPLATE_PROVIDER_LABEL,
    }
}

#[must_use]
pub fn templates_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::TEMPLATES_LABEL,
        CliLocale::PtBr => pt_br::TEMPLATES_LABEL,
    }
}

#[must_use]
pub fn prompt_provider_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::PROMPT_PROVIDER_LABEL,
        CliLocale::PtBr => pt_br::PROMPT_PROVIDER_LABEL,
    }
}

#[must_use]
pub fn prompts_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::PROMPTS_LABEL,
        CliLocale::PtBr => pt_br::PROMPTS_LABEL,
    }
}

#[must_use]
pub fn policy_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::POLICY_LABEL,
        CliLocale::PtBr => pt_br::POLICY_LABEL,
    }
}

#[must_use]
pub fn policies_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::POLICIES_LABEL,
        CliLocale::PtBr => pt_br::POLICIES_LABEL,
    }
}

#[must_use]
pub fn policy_enforcement_hook_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::POLICY_ENFORCEMENT_HOOK_LABEL,
        CliLocale::PtBr => pt_br::POLICY_ENFORCEMENT_HOOK_LABEL,
    }
}

#[must_use]
pub fn capability_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::CAPABILITY_LABEL,
        CliLocale::PtBr => pt_br::CAPABILITY_LABEL,
    }
}

#[must_use]
pub fn capabilities_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::CAPABILITIES_LABEL,
        CliLocale::PtBr => pt_br::CAPABILITIES_LABEL,
    }
}

#[must_use]
pub fn check_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::CHECK_LABEL,
        CliLocale::PtBr => pt_br::CHECK_LABEL,
    }
}

#[must_use]
pub fn checks_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::CHECKS_LABEL,
        CliLocale::PtBr => pt_br::CHECKS_LABEL,
    }
}

#[must_use]
pub fn hook_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::HOOK_LABEL,
        CliLocale::PtBr => pt_br::HOOK_LABEL,
    }
}

#[must_use]
pub fn hooks_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::HOOKS_LABEL,
        CliLocale::PtBr => pt_br::HOOKS_LABEL,
    }
}

#[must_use]
pub fn provider_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::PROVIDER_LABEL,
        CliLocale::PtBr => pt_br::PROVIDER_LABEL,
    }
}

#[must_use]
pub fn plugin_id_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::PLUGIN_ID_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::PLUGIN_ID_ENTITY_LABEL,
    }
}

#[must_use]
pub fn policy_id_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::POLICY_ID_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::POLICY_ID_ENTITY_LABEL,
    }
}

#[must_use]
pub fn capability_id_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::CAPABILITY_ID_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::CAPABILITY_ID_ENTITY_LABEL,
    }
}

#[must_use]
pub fn check_id_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::CHECK_ID_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::CHECK_ID_ENTITY_LABEL,
    }
}

#[must_use]
pub fn hook_id_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::HOOK_ID_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::HOOK_ID_ENTITY_LABEL,
    }
}

#[must_use]
pub fn provider_id_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::PROVIDER_ID_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::PROVIDER_ID_ENTITY_LABEL,
    }
}

#[must_use]
pub fn mcp_server_id_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::MCP_SERVER_ID_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::MCP_SERVER_ID_ENTITY_LABEL,
    }
}

#[must_use]
pub fn plugin_config_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::PLUGIN_CONFIG_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::PLUGIN_CONFIG_ENTITY_LABEL,
    }
}

#[must_use]
pub fn plugin_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::PLUGIN_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::PLUGIN_ENTITY_LABEL,
    }
}

#[must_use]
pub fn mcp_server_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::MCP_SERVER_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::MCP_SERVER_ENTITY_LABEL,
    }
}

#[must_use]
pub fn capability_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::CAPABILITY_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::CAPABILITY_ENTITY_LABEL,
    }
}

#[must_use]
pub fn check_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::CHECK_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::CHECK_ENTITY_LABEL,
    }
}

#[must_use]
pub fn hook_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::HOOK_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::HOOK_ENTITY_LABEL,
    }
}

#[must_use]
pub fn provider_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::PROVIDER_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::PROVIDER_ENTITY_LABEL,
    }
}

#[must_use]
pub fn agent_runtime_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::AGENT_RUNTIME_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::AGENT_RUNTIME_ENTITY_LABEL,
    }
}

#[must_use]
pub fn template_provider_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::TEMPLATE_PROVIDER_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::TEMPLATE_PROVIDER_ENTITY_LABEL,
    }
}

#[must_use]
pub fn prompt_provider_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::PROMPT_PROVIDER_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::PROMPT_PROVIDER_ENTITY_LABEL,
    }
}

#[must_use]
pub fn policy_entity_label(locale: &str) -> &'static str {
    match parse_locale(locale) {
        CliLocale::En => en::POLICY_ENTITY_LABEL,
        CliLocale::PtBr => pt_br::POLICY_ENTITY_LABEL,
    }
}

#[cfg(test)]
mod tests {
    use std::{env, ffi::OsString};

    use super::{
        detail_heading, list_heading, missing_id, normalize_cli_locale, providers_heading,
        resolve_cli_locale_from_env_result, root_bootstrapped, unknown_command, unknown_entity,
        unknown_subcommand,
    };

    #[test]
    fn normalize_cli_locale_accepts_supported_values() {
        assert!(matches!(normalize_cli_locale("en"), Ok("en")));
        assert!(matches!(normalize_cli_locale("pt-BR"), Ok("pt-br")));
    }

    #[test]
    fn normalize_cli_locale_rejects_unknown_values() {
        let result = normalize_cli_locale("es");

        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "unsupported locale: es. supported locales: en, pt-br"
            );
        }
    }

    #[test]
    fn helper_messages_render_pt_br() {
        assert_eq!(root_bootstrapped("pt-br"), "Fundação Rust inicializada.");
        assert_eq!(
            unknown_command("pt-br", "oops"),
            "comando desconhecido: oops"
        );
        assert_eq!(
            unknown_subcommand("pt-br", "plugins", "oops"),
            "subcomando desconhecido em plugins: oops"
        );
        assert_eq!(
            missing_id("pt-br", "plugins", "um id de plugin"),
            "plugins show exige um id de plugin"
        );
        assert_eq!(
            unknown_entity("pt-br", "plugin", "official.missing"),
            "plugin desconhecido: official.missing"
        );
        assert_eq!(
            list_heading("pt-br", "Plugins", "Plugins", 3),
            "Plugins (3)"
        );
        assert_eq!(providers_heading("pt-br", 2), "Provedores (2)");
        assert_eq!(
            detail_heading("pt-br", "Plugin", "Plugin", "official.basic"),
            "Plugin: official.basic"
        );
    }

    #[test]
    fn helper_messages_render_english() {
        assert_eq!(root_bootstrapped("en"), "Rust foundation bootstrapped.");
        assert_eq!(unknown_command("en", "oops"), "unknown command: oops");
        assert_eq!(
            unknown_subcommand("en", "plugins", "oops"),
            "unknown plugins command: oops"
        );
        assert_eq!(
            missing_id("en", "plugins", "a plugin id"),
            "plugins show requires a plugin id"
        );
        assert_eq!(
            unknown_entity("en", "plugin", "official.missing"),
            "unknown plugin: official.missing"
        );
        assert_eq!(list_heading("en", "Plugins", "Plugins", 3), "Plugins (3)");
        assert_eq!(providers_heading("en", 2), "Providers (2)");
        assert_eq!(
            detail_heading("en", "Plugin", "Plugin", "official.basic"),
            "Plugin: official.basic"
        );
    }

    #[test]
    fn resolve_cli_locale_prefers_supported_environment_override() {
        assert!(matches!(
            resolve_cli_locale_from_env_result(Ok(String::from("pt-BR"))),
            Ok("pt-br")
        ));
    }

    #[test]
    fn resolve_cli_locale_falls_back_to_default_locale_when_env_is_missing() {
        assert!(matches!(
            resolve_cli_locale_from_env_result(Err(env::VarError::NotPresent)),
            Ok("en")
        ));
    }

    #[cfg(unix)]
    #[test]
    fn resolve_cli_locale_reports_non_unicode_environment_values() {
        use std::os::unix::ffi::OsStringExt;

        let result = resolve_cli_locale_from_env_result(Err(env::VarError::NotUnicode(
            OsString::from_vec(vec![0x66, 0x6f, 0x80]),
        )));

        assert!(result.is_err());

        if let Err(error) = result {
            assert!(
                error
                    .to_string()
                    .contains("failed to read RALPH_ENGINE_LOCALE")
            );
        }
    }
}
