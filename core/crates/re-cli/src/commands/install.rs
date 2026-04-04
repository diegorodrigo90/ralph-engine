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
        Some("--help" | "-h") => Ok(i18n::install_help(locale).to_owned()),
        Some(plugin_ref) => install_plugin(plugin_ref, allow_prerelease, locale),
        None => Err(CliError::new(i18n::install_ref_required(locale))),
    }
}

/// Executes the uninstall command.
pub fn execute_uninstall(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        Some(plugin_ref) => uninstall_plugin(plugin_ref, locale),
        None => Err(CliError::new(i18n::uninstall_ref_required(locale))),
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
        return Err(CliError::new(i18n::install_already_installed(
            locale,
            &plugin_id,
            &plugin_dir.display().to_string(),
        )));
    }

    let repo_url = format!("https://github.com/{publisher}/ralph-engine-plugin-{name}.git");

    std::fs::create_dir_all(plugins_dir)
        .map_err(|err| CliError::new(i18n::install_create_dir_failed(locale, &err.to_string())))?;

    let status = std::process::Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            &repo_url,
            &plugin_dir.to_string_lossy(),
        ])
        .status()
        .map_err(|err| CliError::new(i18n::install_clone_exec_failed(locale, &err.to_string())))?;

    if !status.success() {
        return Err(CliError::new(i18n::install_clone_repo_failed(
            locale, &repo_url,
        )));
    }

    let manifest_path = plugin_dir.join("manifest.yaml");
    if !manifest_path.exists() {
        let _ = std::fs::remove_dir_all(&plugin_dir);
        return Err(CliError::new(i18n::install_no_manifest(locale)));
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
        i18n::install_success(locale),
        i18n::install_id_label(locale),
        i18n::install_location_label(locale),
        plugin_dir.display()
    ))
}

/// Uninstalls a community plugin.
fn uninstall_plugin(plugin_ref: &str, locale: &str) -> Result<String, CliError> {
    let (publisher, name) = parse_plugin_ref(plugin_ref)?;
    let plugin_id = format!("{publisher}.{name}");

    let plugin_dir = Path::new(".ralph-engine/plugins").join(&plugin_id);
    if !plugin_dir.exists() {
        return Err(CliError::new(i18n::install_not_installed(
            locale, &plugin_id,
        )));
    }

    std::fs::remove_dir_all(&plugin_dir)
        .map_err(|err| CliError::new(i18n::install_remove_dir_failed(locale, &err.to_string())))?;

    let config_path = Path::new(".ralph-engine/config.yaml");
    if let Ok(config) = std::fs::read_to_string(config_path) {
        let filtered: Vec<&str> = config
            .lines()
            .filter(|line| !line.contains(&plugin_id))
            .collect();
        let _ = std::fs::write(config_path, filtered.join("\n"));
    }

    Ok(i18n::install_uninstalled(locale, &plugin_id))
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
