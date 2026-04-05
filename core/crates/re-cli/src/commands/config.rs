//! Configuration command handlers.

use re_config::{
    ConfigScope, PluginActivation, ResolvedMcpServerConfig, ResolvedPluginConfig,
    canonical_config_layers, default_project_config, render_config_layers_yaml,
    render_default_locale_yaml, render_project_config_yaml, render_resolved_mcp_server_config_yaml,
    render_resolved_plugin_config_yaml, render_runtime_budgets_yaml, resolve_mcp_server_config,
    resolve_plugin_config,
};

use crate::{CliError, catalog, i18n};

/// Executes the config command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("show-defaults") => Ok(render_project_config_yaml(&default_project_config())),
        Some("locale" | "show-locale") => Ok(render_default_locale_yaml(&default_project_config())),
        Some("budgets" | "show-budgets") => Ok(render_runtime_budgets_yaml(
            &default_project_config().budgets,
        )),
        Some("layers" | "show-layers") => Ok(render_config_layers_yaml(canonical_config_layers())),
        Some("show-mcp-server") => show_mcp_server(args.get(1).map(String::as_str), locale),
        Some("show-plugin") => show_plugin(args.get(1).map(String::as_str), locale),
        Some("preset") => apply_preset(args.get(1).map(String::as_str), locale),
        Some("migrate") => migrate_config(locale),
        Some(other) => Err(CliError::usage(i18n::unknown_subcommand(
            locale, "config", other,
        ))),
    }
}

fn show_plugin(plugin_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let plugin_id = plugin_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "config",
            i18n::plugin_id_entity_label(locale),
        ))
    })?;
    let plugin = catalog::find_official_plugin(plugin_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::plugin_config_entity_label(locale),
            plugin_id,
        ))
    })?;
    let resolved = resolve_plugin_config(canonical_config_layers(), plugin.id).unwrap_or(
        ResolvedPluginConfig::new(
            plugin.id,
            PluginActivation::Disabled,
            ConfigScope::BuiltInDefaults,
        ),
    );

    Ok(render_resolved_plugin_config_yaml(&resolved))
}

fn show_mcp_server(server_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let server_id = server_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "config",
            i18n::mcp_server_id_entity_label(locale),
        ))
    })?;
    let server = catalog::find_official_mcp_server(server_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::mcp_server_entity_label(locale),
            server_id,
        ))
    })?;
    let default_enabled = matches!(server.availability, re_mcp::McpAvailability::OnDemand);
    let resolved = resolve_mcp_server_config(canonical_config_layers(), server.id).unwrap_or(
        ResolvedMcpServerConfig::new(server.id, default_enabled, ConfigScope::BuiltInDefaults),
    );

    Ok(render_resolved_mcp_server_config_yaml(&resolved))
}

/// Applies a preset to the current project.
fn apply_preset(preset_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let preset_id = preset_id
        .ok_or_else(|| CliError::usage(i18n::missing_id(locale, "config preset", "preset-id")))?;

    let cwd =
        std::env::current_dir().map_err(|e| CliError::new(format!("Cannot determine cwd: {e}")))?;

    let files = catalog::apply_preset(preset_id, &cwd).map_err(CliError::new)?;

    let mut lines = vec![format!("Preset '{preset_id}' applied:")];
    for (path, _) in &files {
        lines.push(format!("  ✓ {path}"));
    }
    Ok(lines.join("\n"))
}

/// Runs config migration across all plugins.
fn migrate_config(_locale: &str) -> Result<String, CliError> {
    let cwd =
        std::env::current_dir().map_err(|e| CliError::new(format!("Cannot determine cwd: {e}")))?;
    let config_path = cwd.join(".ralph-engine/config.yaml");

    let content = std::fs::read_to_string(&config_path)
        .map_err(|_| CliError::new("No config file found at .ralph-engine/config.yaml"))?;

    let current_version = env!("CARGO_PKG_VERSION");
    match catalog::migrate_config(&content, "0", current_version) {
        Some(migrated) => {
            std::fs::write(&config_path, &migrated)
                .map_err(|e| CliError::new(format!("Failed to write config: {e}")))?;
            Ok(format!("Config migrated to v{current_version}"))
        }
        None => Ok("Config is up to date, no migration needed.".to_owned()),
    }
}
