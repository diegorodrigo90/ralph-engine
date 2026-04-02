//! Shared CLI locale resolution and message helpers.

use std::env;

use crate::CliError;
use re_config::SupportedLocale;

pub(super) struct CliLocaleCatalog {
    pub root_bootstrapped: &'static str,
    pub providers_label: &'static str,
    pub resolved_activation_label: &'static str,
    pub resolved_from_label: &'static str,
    pub activation_label: &'static str,
    pub load_boundary_label: &'static str,
    pub policy_label: &'static str,
    pub policies_label: &'static str,
    pub policy_enforcement_hook_label: &'static str,
    pub capability_label: &'static str,
    pub capabilities_label: &'static str,
    pub check_label: &'static str,
    pub checks_label: &'static str,
    pub hook_label: &'static str,
    pub hooks_label: &'static str,
    pub provider_label: &'static str,
    pub locale_id_entity_label: &'static str,
    pub mcp_server_id_entity_label: &'static str,
    pub plugin_config_entity_label: &'static str,
    pub plugin_id_entity_label: &'static str,
    pub agent_id_entity_label: &'static str,
    pub template_id_entity_label: &'static str,
    pub prompt_id_entity_label: &'static str,
    pub policy_id_entity_label: &'static str,
    pub capability_id_entity_label: &'static str,
    pub check_id_entity_label: &'static str,
    pub hook_id_entity_label: &'static str,
    pub provider_id_entity_label: &'static str,
    pub plugin_entity_label: &'static str,
    pub agent_runtime_entity_label: &'static str,
    pub template_entity_label: &'static str,
    pub prompt_entity_label: &'static str,
    pub policy_entity_label: &'static str,
    pub capability_entity_label: &'static str,
    pub check_entity_label: &'static str,
    pub hook_entity_label: &'static str,
    pub provider_entity_label: &'static str,
    pub locale_entity_label: &'static str,
    pub mcp_server_entity_label: &'static str,
    pub unknown_command: fn(&str) -> String,
    pub unknown_subcommand: fn(&str, &str) -> String,
    pub missing_id: fn(&str, &str) -> String,
    pub unknown_entity: fn(&str, &str) -> String,
    pub missing_asset_path: fn(&str) -> String,
    pub unknown_template_asset: fn(&str) -> String,
    pub unknown_prompt_asset: fn(&str) -> String,
}

mod en;
mod pt_br;

const LOCALE_ENV_KEY: &str = "RALPH_ENGINE_LOCALE";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CliLocale {
    En,
    PtBr,
}

impl CliLocale {
    const fn from_supported(locale: SupportedLocale) -> Self {
        match locale {
            SupportedLocale::En => Self::En,
            SupportedLocale::PtBr => Self::PtBr,
        }
    }
}

#[must_use]
pub fn is_pt_br(locale: &str) -> bool {
    matches!(
        re_config::resolve_supported_locale_or_default(locale),
        SupportedLocale::PtBr
    )
}

fn parse_locale(locale: &str) -> CliLocale {
    CliLocale::from_supported(re_config::resolve_supported_locale_or_default(locale))
}

fn locale_catalog(locale: &str) -> &'static CliLocaleCatalog {
    match parse_locale(locale) {
        CliLocale::En => &en::LOCALE,
        CliLocale::PtBr => &pt_br::LOCALE,
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
    let normalized = value.trim();
    match re_config::parse_supported_locale(normalized) {
        Some(locale) => Ok(locale.as_str()),
        None => {
            let other = normalized.to_ascii_lowercase();
            Err(CliError::new(format!(
                "unsupported locale: {other}. supported locales: {}",
                supported_locale_ids(),
            )))
        }
    }
}

fn supported_locale_ids() -> String {
    re_config::supported_locales()
        .iter()
        .map(|locale| locale.id)
        .collect::<Vec<_>>()
        .join(", ")
}

macro_rules! catalog_str {
    ($name:ident, $field:ident) => {
        #[must_use]
        pub fn $name(locale: &str) -> &'static str {
            locale_catalog(locale).$field
        }
    };
}

#[must_use]
pub fn root_bootstrapped(locale: &str) -> &'static str {
    locale_catalog(locale).root_bootstrapped
}

#[must_use]
pub fn unknown_command(locale: &str, command_name: &str) -> String {
    (locale_catalog(locale).unknown_command)(command_name)
}

#[must_use]
pub fn unknown_subcommand(locale: &str, command_group: &str, command_name: &str) -> String {
    (locale_catalog(locale).unknown_subcommand)(command_group, command_name)
}

#[must_use]
pub fn missing_id(locale: &str, command_group: &str, entity_label: &str) -> String {
    (locale_catalog(locale).missing_id)(command_group, entity_label)
}

#[must_use]
pub fn unknown_entity(locale: &str, entity_label: &str, value: &str) -> String {
    (locale_catalog(locale).unknown_entity)(entity_label, value)
}

#[must_use]
pub fn missing_asset_path(locale: &str, command_group: &str) -> String {
    (locale_catalog(locale).missing_asset_path)(command_group)
}

#[must_use]
pub fn unknown_template_asset(locale: &str, value: &str) -> String {
    (locale_catalog(locale).unknown_template_asset)(value)
}

#[must_use]
pub fn unknown_prompt_asset(locale: &str, value: &str) -> String {
    (locale_catalog(locale).unknown_prompt_asset)(value)
}

#[must_use]
pub fn list_heading(locale: &str, singular_en: &str, singular_pt: &str, count: usize) -> String {
    if is_pt_br(locale) {
        format!("{singular_pt} ({count})")
    } else {
        format!("{singular_en} ({count})")
    }
}

#[must_use]
pub fn providers_heading(locale: &str, count: usize) -> String {
    format!("{} ({count})", locale_catalog(locale).providers_label)
}

#[must_use]
pub fn detail_heading(locale: &str, label_en: &str, label_pt: &str, value: &str) -> String {
    if is_pt_br(locale) {
        format!("{label_pt}: {value}")
    } else {
        format!("{label_en}: {value}")
    }
}

catalog_str!(resolved_activation_label, resolved_activation_label);
catalog_str!(resolved_from_label, resolved_from_label);
catalog_str!(activation_label, activation_label);
catalog_str!(load_boundary_label, load_boundary_label);
catalog_str!(policy_label, policy_label);
catalog_str!(policies_label, policies_label);
catalog_str!(policy_enforcement_hook_label, policy_enforcement_hook_label);
catalog_str!(capability_label, capability_label);
catalog_str!(capabilities_label, capabilities_label);
catalog_str!(check_label, check_label);
catalog_str!(checks_label, checks_label);
catalog_str!(hook_label, hook_label);
catalog_str!(hooks_label, hooks_label);
catalog_str!(providers_label, providers_label);
catalog_str!(provider_label, provider_label);
catalog_str!(plugin_id_entity_label, plugin_id_entity_label);
catalog_str!(policy_id_entity_label, policy_id_entity_label);
catalog_str!(capability_id_entity_label, capability_id_entity_label);
catalog_str!(check_id_entity_label, check_id_entity_label);
catalog_str!(hook_id_entity_label, hook_id_entity_label);
catalog_str!(provider_id_entity_label, provider_id_entity_label);
catalog_str!(locale_id_entity_label, locale_id_entity_label);
catalog_str!(mcp_server_id_entity_label, mcp_server_id_entity_label);
catalog_str!(plugin_config_entity_label, plugin_config_entity_label);
catalog_str!(agent_id_entity_label, agent_id_entity_label);
catalog_str!(template_id_entity_label, template_id_entity_label);
catalog_str!(prompt_id_entity_label, prompt_id_entity_label);
catalog_str!(plugin_entity_label, plugin_entity_label);
catalog_str!(mcp_server_entity_label, mcp_server_entity_label);
catalog_str!(capability_entity_label, capability_entity_label);
catalog_str!(check_entity_label, check_entity_label);
catalog_str!(hook_entity_label, hook_entity_label);
catalog_str!(provider_entity_label, provider_entity_label);
catalog_str!(locale_entity_label, locale_entity_label);
catalog_str!(agent_runtime_entity_label, agent_runtime_entity_label);
catalog_str!(template_entity_label, template_entity_label);
catalog_str!(prompt_entity_label, prompt_entity_label);
catalog_str!(policy_entity_label, policy_entity_label);

#[cfg(test)]
mod tests {
    use std::{env, ffi::OsString};

    use super::{
        activation_label, detail_heading, list_heading, load_boundary_label, missing_id,
        normalize_cli_locale, providers_heading, resolve_cli_locale_from_env_result,
        root_bootstrapped, unknown_command, unknown_entity, unknown_subcommand,
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
            unknown_entity("pt-br", "plugin", "fixture.missing"),
            "plugin desconhecido: fixture.missing"
        );
        assert_eq!(
            unknown_entity("pt-br", "capacidade", "template"),
            "capacidade desconhecida: template"
        );
        assert_eq!(
            list_heading("pt-br", "Plugins", "Plugins", 3),
            "Plugins (3)"
        );
        assert_eq!(providers_heading("pt-br", 2), "Provedores (2)");
        assert_eq!(activation_label("pt-br"), "Ativação");
        assert_eq!(load_boundary_label("pt-br"), "Fronteira de carregamento");
        assert_eq!(
            detail_heading("pt-br", "Plugin", "Plugin", "fixture.basic"),
            "Plugin: fixture.basic"
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
            unknown_entity("en", "plugin", "fixture.missing"),
            "unknown plugin: fixture.missing"
        );
        assert_eq!(list_heading("en", "Plugins", "Plugins", 3), "Plugins (3)");
        assert_eq!(providers_heading("en", 2), "Providers (2)");
        assert_eq!(activation_label("en"), "Activation");
        assert_eq!(load_boundary_label("en"), "Load boundary");
        assert_eq!(
            detail_heading("en", "Plugin", "Plugin", "fixture.basic"),
            "Plugin: fixture.basic"
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
