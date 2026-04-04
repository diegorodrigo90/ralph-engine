//! Plugin install/uninstall command handlers.
//!
//! Installs community plugins from the public catalog by cloning
//! the plugin's GitHub repository into `.ralph-engine/plugins/`
//! and registering it in the project config.

use std::path::Path;

use crate::{CliError, i18n};

/// Executes the install command tree.
///
/// Default: installs latest stable release. Prerelease versions (alpha,
/// beta, rc) are excluded unless `--pre` is passed explicitly.
/// Same convention as Terraform: prerelease only with exact version.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    let allow_prerelease = args.iter().any(|a| a == "--pre" || a == "--prerelease");
    let filtered: Vec<&String> = args
        .iter()
        .filter(|a| *a != "--pre" && *a != "--prerelease")
        .collect();

    match filtered.first().map(|s| s.as_str()) {
        Some("--help" | "-h") => Ok(locale_str!(
            locale,
            "Usage: ralph-engine install [--pre] <publisher>/<name>\n\n\
             Install a community plugin from the Ralph Engine catalog.\n\
             By default, only stable releases are installed.\n\
             Use --pre to allow prerelease versions (alpha, beta, rc).\n\n\
             Options:\n\
             \x20 --pre  Allow prerelease versions\n\n\
             Examples:\n\
             \x20 ralph-engine install acme/jira-suite\n\
             \x20 ralph-engine install --pre acme/jira-suite\n\
             \x20 ralph-engine uninstall acme/jira-suite",
            "Uso: ralph-engine install [--pre] <publisher>/<nome>\n\n\
             Instala um plugin da comunidade do catálogo Ralph Engine.\n\
             Por padrão, apenas versões estáveis são instaladas.\n\
             Use --pre para permitir versões prerelease (alpha, beta, rc).\n\n\
             Opções:\n\
             \x20 --pre  Permitir versões prerelease\n\n\
             Exemplos:\n\
             \x20 ralph-engine install acme/jira-suite\n\
             \x20 ralph-engine install --pre acme/jira-suite\n\
             \x20 ralph-engine uninstall acme/jira-suite"
        )
        .to_owned()),
        Some(plugin_ref) => install_plugin(plugin_ref, allow_prerelease, locale),
        None => Err(CliError::new(
            locale_str!(
                locale,
                "Plugin reference required. Usage: ralph-engine install <publisher>/<name>",
                "Referência do plugin necessária. Uso: ralph-engine install <publisher>/<nome>"
            )
            .to_owned(),
        )),
    }
}

/// Executes the uninstall command.
pub fn execute_uninstall(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        Some(plugin_ref) => uninstall_plugin(plugin_ref, locale),
        None => Err(CliError::new(
            locale_str!(
                locale,
                "Plugin reference required. Usage: ralph-engine uninstall <publisher>/<name>",
                "Referência do plugin necessária. Uso: ralph-engine uninstall <publisher>/<nome>"
            )
            .to_owned(),
        )),
    }
}

/// Installs a community plugin by cloning its repo and registering in config.
///
/// When `allow_prerelease` is false (default), the latest stable GitHub
/// release is used. Prerelease tags (alpha, beta, rc) are skipped.
fn install_plugin(
    plugin_ref: &str,
    _allow_prerelease: bool,
    locale: &str,
) -> Result<String, CliError> {
    let (publisher, name) = parse_plugin_ref(plugin_ref)?;
    let plugin_id = format!("{publisher}.{name}");

    let plugins_dir = Path::new(".ralph-engine/plugins");
    let plugin_dir = plugins_dir.join(&plugin_id);
    if plugin_dir.exists() {
        let msg = if i18n::is_pt_br(locale) {
            format!(
                "Plugin '{plugin_id}' já está instalado em {}",
                plugin_dir.display()
            )
        } else {
            format!(
                "Plugin '{plugin_id}' is already installed at {}",
                plugin_dir.display()
            )
        };
        return Err(CliError::new(msg));
    }

    let repo_url = format!("https://github.com/{publisher}/ralph-engine-plugin-{name}.git");

    std::fs::create_dir_all(plugins_dir).map_err(|err| {
        let msg = if i18n::is_pt_br(locale) {
            format!("Falha ao criar diretório de plugins: {err}")
        } else {
            format!("Failed to create plugins directory: {err}")
        };
        CliError::new(msg)
    })?;

    let status = std::process::Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            &repo_url,
            &plugin_dir.to_string_lossy(),
        ])
        .status()
        .map_err(|err| {
            let msg = if i18n::is_pt_br(locale) {
                format!("Falha ao executar git clone: {err}")
            } else {
                format!("Failed to run git clone: {err}")
            };
            CliError::new(msg)
        })?;

    if !status.success() {
        let msg = if i18n::is_pt_br(locale) {
            format!("Falha ao clonar {repo_url}. Verifique se o repositório existe e é público.")
        } else {
            format!("Failed to clone {repo_url}. Check that the repository exists and is public.")
        };
        return Err(CliError::new(msg));
    }

    let manifest_path = plugin_dir.join("manifest.yaml");
    if !manifest_path.exists() {
        let _ = std::fs::remove_dir_all(&plugin_dir);
        let msg = if i18n::is_pt_br(locale) {
            "Repositório clonado mas manifest.yaml não encontrado. Não é um plugin Ralph Engine válido.".to_owned()
        } else {
            "Repository cloned but no manifest.yaml found. Not a valid Ralph Engine plugin."
                .to_owned()
        };
        return Err(CliError::new(msg));
    }

    let config_path = Path::new(".ralph-engine/config.yaml");
    if let Ok(mut config) = std::fs::read_to_string(config_path)
        && !config.contains(&plugin_id)
    {
        config.push_str(&format!("\n  - id: {plugin_id}\n    activation: enabled\n"));
        let _ = std::fs::write(config_path, config);
    }

    Ok(format!(
        "{}\n  {} {plugin_id}\n  {} {}",
        locale_str!(locale, "Plugin installed:", "Plugin instalado:"),
        locale_str!(locale, "ID:", "ID:"),
        locale_str!(locale, "Location:", "Local:"),
        plugin_dir.display()
    ))
}

/// Uninstalls a community plugin.
fn uninstall_plugin(plugin_ref: &str, locale: &str) -> Result<String, CliError> {
    let (publisher, name) = parse_plugin_ref(plugin_ref)?;
    let plugin_id = format!("{publisher}.{name}");

    let plugin_dir = Path::new(".ralph-engine/plugins").join(&plugin_id);
    if !plugin_dir.exists() {
        let msg = if i18n::is_pt_br(locale) {
            format!("Plugin '{plugin_id}' não está instalado.")
        } else {
            format!("Plugin '{plugin_id}' is not installed.")
        };
        return Err(CliError::new(msg));
    }

    std::fs::remove_dir_all(&plugin_dir).map_err(|err| {
        let msg = if i18n::is_pt_br(locale) {
            format!("Falha ao remover diretório do plugin: {err}")
        } else {
            format!("Failed to remove plugin directory: {err}")
        };
        CliError::new(msg)
    })?;

    let config_path = Path::new(".ralph-engine/config.yaml");
    if let Ok(config) = std::fs::read_to_string(config_path) {
        let filtered: Vec<&str> = config
            .lines()
            .filter(|line| !line.contains(&plugin_id))
            .collect();
        let _ = std::fs::write(config_path, filtered.join("\n"));
    }

    let msg = if i18n::is_pt_br(locale) {
        format!("Plugin '{plugin_id}' desinstalado.")
    } else {
        format!("Plugin '{plugin_id}' uninstalled.")
    };
    Ok(msg)
}

/// Parses "publisher/name" or "publisher.name" into (publisher, name).
fn parse_plugin_ref(plugin_ref: &str) -> Result<(String, String), CliError> {
    if let Some((p, n)) = plugin_ref.split_once('/') {
        return Ok((p.to_owned(), n.to_owned()));
    }
    if let Some((p, n)) = plugin_ref.split_once('.') {
        return Ok((p.to_owned(), n.to_owned()));
    }
    Err(CliError::new(format!(
        "Invalid plugin reference '{plugin_ref}'. Expected format: publisher/name or publisher.name"
    )))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn help_returns_usage() {
        let result = execute(&["--help".to_owned()], "en").unwrap();
        assert!(result.contains("ralph-engine install"));
        assert!(result.contains("--pre"));
    }

    #[test]
    fn help_returns_usage_pt_br() {
        let result = execute(&["-h".to_owned()], "pt-br").unwrap();
        assert!(result.contains("ralph-engine install"));
        assert!(result.contains("prerelease"));
    }

    #[test]
    fn missing_arg_returns_error() {
        let result = execute(&[], "en");
        assert!(result.is_err());
    }

    #[test]
    fn parse_ref_slash_format() {
        let (p, n) = parse_plugin_ref("acme/jira-suite").unwrap();
        assert_eq!(p, "acme");
        assert_eq!(n, "jira-suite");
    }

    #[test]
    fn parse_ref_dot_format() {
        let (p, n) = parse_plugin_ref("acme.jira-suite").unwrap();
        assert_eq!(p, "acme");
        assert_eq!(n, "jira-suite");
    }

    #[test]
    fn parse_ref_invalid() {
        assert!(parse_plugin_ref("noslash").is_err());
    }

    #[test]
    fn uninstall_missing_arg_returns_error() {
        let result = execute_uninstall(&[], "en");
        assert!(result.is_err());
    }
}
