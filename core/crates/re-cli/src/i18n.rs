//! Shared CLI locale resolution and message helpers.

use std::env;

use crate::CliError;

const SUPPORTED_LOCALES: &[&str] = &["en", "pt-br"];
const LOCALE_ENV_KEY: &str = "RALPH_ENGINE_LOCALE";

#[must_use]
pub fn is_pt_br(locale: &str) -> bool {
    locale.eq_ignore_ascii_case("pt-br")
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
        "en" => Ok("en"),
        "pt-br" => Ok("pt-br"),
        other => Err(CliError::new(format!(
            "unsupported locale: {other}. supported locales: {}",
            SUPPORTED_LOCALES.join(", ")
        ))),
    }
}

#[must_use]
pub fn root_bootstrapped(locale: &str) -> &'static str {
    if is_pt_br(locale) {
        "Fundação Rust inicializada."
    } else {
        "Rust foundation bootstrapped."
    }
}

#[must_use]
pub fn unknown_command(locale: &str, command_name: &str) -> String {
    if is_pt_br(locale) {
        format!("comando desconhecido: {command_name}")
    } else {
        format!("unknown command: {command_name}")
    }
}

#[must_use]
pub fn unknown_subcommand(locale: &str, command_group: &str, command_name: &str) -> String {
    if is_pt_br(locale) {
        format!("subcomando desconhecido em {command_group}: {command_name}")
    } else {
        format!("unknown {command_group} command: {command_name}")
    }
}

#[must_use]
pub fn missing_id(locale: &str, command_group: &str, entity_label: &str) -> String {
    if is_pt_br(locale) {
        format!("{command_group} show exige {entity_label}")
    } else {
        format!("{command_group} show requires {entity_label}")
    }
}

#[must_use]
pub fn unknown_entity(locale: &str, entity_label: &str, value: &str) -> String {
    if is_pt_br(locale) {
        format!("{entity_label} desconhecido: {value}")
    } else {
        format!("unknown {entity_label}: {value}")
    }
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
    if is_pt_br(locale) {
        format!("Provedores ({count})")
    } else {
        format!("Providers ({count})")
    }
}

#[must_use]
pub fn detail_heading(locale: &str, label_en: &str, label_pt: &str, value: &str) -> String {
    if is_pt_br(locale) {
        format!("{label_pt}: {value}")
    } else {
        format!("{label_en}: {value}")
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
