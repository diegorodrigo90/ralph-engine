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
