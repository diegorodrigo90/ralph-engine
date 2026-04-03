//! Shared CLI locale resolution and message helpers.

use std::env;

use crate::CliError;
use re_config::SupportedLocale;

pub(super) struct CliLocaleCatalog {
    pub root_bootstrapped: &'static str,
    pub providers_label: &'static str,
    pub name_label: &'static str,
    pub summary_label: &'static str,
    pub kind_label: &'static str,
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
    pub registration_hook_label: &'static str,
    pub assets_label: &'static str,
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
    pub missing_argument: fn(&str, &str) -> String,
    pub unknown_entity: fn(&str, &str) -> String,
    pub missing_asset_path: fn(&str) -> String,
    #[allow(dead_code)]
    pub missing_output_directory: fn(&str) -> String,
    #[allow(dead_code)]
    pub missing_output_path: fn(&str) -> String,
    #[allow(dead_code)]
    pub invalid_embedded_asset_path: fn(&str) -> String,
    #[allow(dead_code)]
    pub failed_to_write_output: fn(&str, &str) -> String,
    #[allow(dead_code)]
    pub wrote_output: fn(&str) -> String,
    pub unknown_template_asset: fn(&str) -> String,
    pub unknown_prompt_asset: fn(&str) -> String,
    pub unknown_check_asset: fn(&str) -> String,
    pub unknown_policy_asset: fn(&str) -> String,
    #[allow(dead_code)]
    pub materialized_assets_heading: &'static str,
}

// Hand-coded fn implementations for each locale
mod fn_en;
mod fn_pt_br;

// Locale modules and dispatch function generated from locales/*.toml
include!(concat!(env!("OUT_DIR"), "/i18n_generated.rs"));

const LOCALE_ENV_KEY: &str = "RALPH_ENGINE_LOCALE";
const LOCALE_FLAG: &str = "--locale";
const LOCALE_SHORT_FLAG: &str = "-L";

#[derive(Debug)]
pub struct ResolvedCliInvocation {
    pub locale: &'static str,
    pub command_index: usize,
}

#[must_use]
pub fn is_pt_br(locale: &str) -> bool {
    matches!(
        re_config::resolve_supported_locale_or_default(locale),
        SupportedLocale::PtBr
    )
}

pub fn resolve_cli_invocation(args: &[String]) -> Result<ResolvedCliInvocation, CliError> {
    resolve_cli_invocation_from_env_result(args, env::var(LOCALE_ENV_KEY))
}

fn resolve_cli_invocation_from_env_result(
    args: &[String],
    env_result: Result<String, env::VarError>,
) -> Result<ResolvedCliInvocation, CliError> {
    match args.get(1).map(String::as_str) {
        Some(LOCALE_FLAG | LOCALE_SHORT_FLAG) => {
            let locale_value = args
                .get(2)
                .ok_or_else(|| CliError::new(format!("{LOCALE_FLAG} requires a locale id")))?;

            Ok(ResolvedCliInvocation {
                locale: normalize_cli_locale(locale_value)?,
                command_index: 3,
            })
        }
        _ => Ok(ResolvedCliInvocation {
            locale: resolve_cli_locale_from_env_result(env_result)?,
            command_index: 1,
        }),
    }
}

fn resolve_cli_locale_from_env_result(
    env_result: Result<String, env::VarError>,
) -> Result<&'static str, CliError> {
    let os_values: Vec<Option<String>> = ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .map(|key| env::var(key).ok())
        .collect();

    resolve_cli_locale_from_env_and_os(env_result, &os_values)
}

fn resolve_cli_locale_from_env_and_os(
    env_result: Result<String, env::VarError>,
    os_values: &[Option<String>],
) -> Result<&'static str, CliError> {
    match env_result {
        Ok(value) => normalize_cli_locale(&value),
        Err(env::VarError::NotPresent) => resolve_locale_from_os_values(os_values),
        Err(error) => Err(CliError::new(format!(
            "failed to read {LOCALE_ENV_KEY}: {error}"
        ))),
    }
}

/// Pure function that resolves a locale from a list of OS locale values.
/// Checks each value in priority order and returns the first match.
fn resolve_locale_from_os_values(os_values: &[Option<String>]) -> Result<&'static str, CliError> {
    for value in os_values.iter().flatten() {
        if let Some(locale) = re_config::parse_os_locale(value) {
            return Ok(locale.as_str());
        }
    }

    // No OS locale matched — use the project config default (English)
    normalize_cli_locale(re_config::default_project_config().default_locale)
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
pub fn missing_argument(locale: &str, command_path: &str, entity_label: &str) -> String {
    (locale_catalog(locale).missing_argument)(command_path, entity_label)
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
#[allow(dead_code)]
pub fn missing_output_directory(locale: &str, command_group: &str) -> String {
    (locale_catalog(locale).missing_output_directory)(command_group)
}

#[must_use]
#[allow(dead_code)]
pub fn missing_output_path(locale: &str, command_group: &str) -> String {
    (locale_catalog(locale).missing_output_path)(command_group)
}

#[must_use]
#[allow(dead_code)]
pub fn invalid_embedded_asset_path(locale: &str, value: &str) -> String {
    (locale_catalog(locale).invalid_embedded_asset_path)(value)
}

#[must_use]
#[allow(dead_code)]
pub fn failed_to_write_output(locale: &str, path: &str, error: &str) -> String {
    (locale_catalog(locale).failed_to_write_output)(path, error)
}

#[must_use]
#[allow(dead_code)]
pub fn wrote_output(locale: &str, path: &str) -> String {
    (locale_catalog(locale).wrote_output)(path)
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
pub fn unknown_check_asset(locale: &str, value: &str) -> String {
    (locale_catalog(locale).unknown_check_asset)(value)
}

#[must_use]
pub fn unknown_policy_asset(locale: &str, value: &str) -> String {
    (locale_catalog(locale).unknown_policy_asset)(value)
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

#[must_use]
#[allow(dead_code)]
pub fn materialized_assets_heading(locale: &str, count: usize) -> String {
    format!(
        "{} ({count})",
        locale_catalog(locale).materialized_assets_heading
    )
}

catalog_str!(resolved_activation_label, resolved_activation_label);
catalog_str!(resolved_from_label, resolved_from_label);
catalog_str!(name_label, name_label);
catalog_str!(summary_label, summary_label);
catalog_str!(kind_label, kind_label);
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
catalog_str!(registration_hook_label, registration_hook_label);
catalog_str!(assets_label, assets_label);
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
    use std::env;

    use crate::CliError;

    use super::{
        LOCALE_FLAG, activation_label, detail_heading, list_heading, load_boundary_label,
        missing_id, normalize_cli_locale, providers_heading, resolve_cli_invocation,
        resolve_cli_invocation_from_env_result, resolve_cli_locale_from_env_and_os,
        resolve_cli_locale_from_env_result, resolve_locale_from_os_values, root_bootstrapped,
        unknown_command, unknown_entity, unknown_subcommand,
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
    fn resolve_cli_invocation_reads_locale_flag() {
        let args = vec![
            "ralph-engine".to_owned(),
            "--locale".to_owned(),
            "pt-br".to_owned(),
            "plugins".to_owned(),
        ];

        let invocation = resolve_cli_invocation(&args);

        assert!(invocation.is_ok());

        if let Ok(resolved) = invocation {
            assert_eq!(resolved.locale, "pt-br");
            assert_eq!(resolved.command_index, 3);
        }
    }

    #[test]
    fn resolve_cli_invocation_falls_back_to_default_when_env_not_set() {
        let invocation =
            resolve_cli_locale_from_env_and_os(Err(env::VarError::NotPresent), &[None, None, None]);

        // With no env and no OS locale, falls back to English
        assert_eq!(invocation, Ok("en"));
    }

    #[test]
    fn resolve_cli_locale_from_env_result_accepts_supported_value() {
        let result = resolve_cli_locale_from_env_result(Ok("pt-br".to_owned()));

        assert_eq!(result, Ok("pt-br"));
    }

    #[test]
    fn resolve_cli_locale_from_env_result_falls_back_to_en_when_not_set() {
        let result: Result<&'static str, CliError> =
            resolve_cli_locale_from_env_result(Err(env::VarError::NotPresent));

        assert!(result.is_ok());
    }

    #[test]
    fn resolve_cli_locale_from_env_and_os_picks_explicit_over_os() {
        let result = resolve_cli_locale_from_env_and_os(
            Ok("pt-br".to_owned()),
            &[Some("en_US.UTF-8".to_owned())],
        );
        assert_eq!(result, Ok("pt-br"));
    }

    #[test]
    fn resolve_cli_locale_from_env_and_os_reads_os_when_env_not_set() {
        let result = resolve_cli_locale_from_env_and_os(
            Err(env::VarError::NotPresent),
            &[None, None, Some("pt_BR.UTF-8".to_owned())],
        );
        assert_eq!(result, Ok("pt-br"));
    }

    #[test]
    fn resolve_locale_from_os_values_returns_first_match() {
        let result = resolve_locale_from_os_values(&[
            None,
            Some("pt_BR.UTF-8".to_owned()),
            Some("en_US.UTF-8".to_owned()),
        ]);
        assert_eq!(result, Ok("pt-br"));
    }

    #[test]
    fn resolve_locale_from_os_values_falls_back_to_english() {
        let result = resolve_locale_from_os_values(&[None, None, None]);
        assert_eq!(result, Ok("en"));
    }

    #[test]
    fn resolve_locale_from_os_values_skips_unsupported() {
        let result = resolve_locale_from_os_values(&[
            Some("ja_JP.UTF-8".to_owned()),
            Some("en_US.UTF-8".to_owned()),
        ]);
        assert_eq!(result, Ok("en"));
    }

    #[test]
    fn resolve_cli_invocation_reads_short_locale_flag() {
        let args = vec![
            "ralph-engine".to_owned(),
            "-L".to_owned(),
            "pt-br".to_owned(),
            "plugins".to_owned(),
        ];

        let result = resolve_cli_invocation_from_env_result(&args, Err(env::VarError::NotPresent));

        assert!(result.is_ok());
        if let Ok(resolved) = result {
            assert_eq!(resolved.locale, "pt-br");
            assert_eq!(resolved.command_index, 3);
        }
    }

    #[test]
    fn resolve_cli_invocation_locale_flag_without_value_errors() {
        let args = vec!["ralph-engine".to_owned(), LOCALE_FLAG.to_owned()];

        let result = resolve_cli_invocation_from_env_result(&args, Err(env::VarError::NotPresent));

        assert!(result.is_err());
    }
}
