//! Shared runtime snapshot helpers for CLI commands.

use std::path::Path;

use re_config::{
    ConfigScope, OwnedProjectConfig, PluginConfig, ProjectConfig, ProjectConfigLayer,
    canonical_config_layers, parse_owned_project_config_yaml, render_owned_project_config_yaml,
};
use re_core::{RuntimeSnapshot, build_runtime_snapshot};

use super::embedded_assets::write_text_output;
use crate::catalog;
use crate::{CliError, i18n};

/// Standard path for the project configuration file.
const PROJECT_CONFIG_PATH: &str = ".ralph-engine/config.yaml";

/// Loads the effective configuration layers: built-in defaults plus an
/// optional project-level override from `.ralph-engine/config.yaml`.
///
/// When the file does not exist the function returns the canonical defaults.
/// When the file exists but cannot be parsed the error is returned so the
/// caller can surface a clear diagnostic.
pub fn load_effective_config_layers() -> Result<Vec<ProjectConfigLayer>, CliError> {
    let mut layers = canonical_config_layers().to_vec();

    let config_path = Path::new(PROJECT_CONFIG_PATH);
    if config_path.exists() {
        let content = std::fs::read_to_string(config_path)
            .map_err(|err| CliError::new(format!("failed to read {PROJECT_CONFIG_PATH}: {err}")))?;

        let owned = parse_owned_project_config_yaml(&content).map_err(|err| {
            CliError::new(format!("failed to parse {PROJECT_CONFIG_PATH}: {err}"))
        })?;

        let project_config = owned_to_static_project_config(&owned);
        layers.push(ProjectConfigLayer::new(
            ConfigScope::Project,
            project_config,
        ));
    }

    Ok(layers)
}

/// Runs one operation against the runtime snapshot resolved from the effective
/// configuration (built-in defaults + project file when present).
pub fn with_official_runtime_snapshot<T>(callback: impl FnOnce(&RuntimeSnapshot<'_>) -> T) -> T {
    let layers =
        load_effective_config_layers().unwrap_or_else(|_| canonical_config_layers().to_vec());
    with_official_runtime_snapshot_using_layers(&layers, callback)
}

/// Runs one operation against a runtime snapshot resolved with the given
/// configuration layers.
pub fn with_official_runtime_snapshot_using_layers<T>(
    layers: &[ProjectConfigLayer],
    callback: impl FnOnce(&RuntimeSnapshot<'_>) -> T,
) -> T {
    let snapshot = catalog::official_runtime_snapshot_with_layers(layers);
    let topology = snapshot.topology();
    let runtime = build_runtime_snapshot(&topology);

    callback(&runtime)
}

/// Converts an owned project config into a static one by leaking the
/// dynamic allocations.
///
/// This is acceptable because config is loaded once at process startup and
/// lives for the entire process lifetime.
fn owned_to_static_project_config(owned: &OwnedProjectConfig) -> ProjectConfig {
    let plugins: &'static [PluginConfig] = Box::leak(owned.plugins.clone().into_boxed_slice());
    let mcp_servers: &'static [re_config::McpServerConfig] =
        Box::leak(owned.mcp.servers.clone().into_boxed_slice());

    ProjectConfig {
        schema_version: owned.schema_version,
        default_locale: owned.default_locale,
        plugins,
        mcp: re_config::McpConfig {
            enabled: owned.mcp.enabled,
            discovery: owned.mcp.discovery,
            servers: mcp_servers,
        },
        budgets: owned.budgets,
    }
}

/// Loads the project configuration from `.ralph-engine/config.yaml`.
///
/// Returns the parsed config or an error if the file is missing or malformed.
/// The `run` command uses this to read `RunConfig` fields.
pub fn load_project_config() -> Result<OwnedProjectConfig, CliError> {
    let config_path = Path::new(PROJECT_CONFIG_PATH);
    if !config_path.exists() {
        return Err(CliError::new(format!(
            "project config not found: {PROJECT_CONFIG_PATH}"
        )));
    }

    let content = std::fs::read_to_string(config_path)
        .map_err(|err| CliError::new(format!("failed to read {PROJECT_CONFIG_PATH}: {err}")))?;

    parse_owned_project_config_yaml(&content)
        .map_err(|err| CliError::new(format!("failed to parse {PROJECT_CONFIG_PATH}: {err}")))
}

/// Renders the fully materialized project configuration after applying the
/// canonical runtime remediation patch.
#[must_use]
pub fn render_official_runtime_patched_config() -> String {
    render_owned_project_config_yaml(&official_runtime_patched_config())
}

/// Builds the fully materialized project configuration after applying the
/// canonical runtime remediation patch.
#[must_use]
pub fn official_runtime_patched_config() -> OwnedProjectConfig {
    #[allow(clippy::redundant_closure_for_method_calls)]
    with_official_runtime_snapshot(|runtime| runtime.patched_config())
}

/// Writes the fully materialized project configuration after applying the
/// canonical runtime remediation patch to one output path.
pub fn apply_official_runtime_patched_config(
    output_path: &Path,
    locale: &str,
    command_path: &str,
) -> Result<String, CliError> {
    if output_path.as_os_str().is_empty() {
        return Err(CliError::new(i18n::missing_output_path(
            locale,
            command_path,
        )));
    }

    write_text_output(
        output_path,
        &render_official_runtime_patched_config(),
        locale,
    )
}
