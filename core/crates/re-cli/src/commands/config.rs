//! Configuration command handlers.

use re_config::{
    ConfigScope, PluginActivation, ResolvedPluginConfig, canonical_config_layers,
    default_project_config, render_config_layers_yaml, render_project_config_yaml,
    render_resolved_plugin_config_yaml, render_runtime_budgets_yaml, resolve_plugin_config,
};

use crate::{CliError, catalog};

/// Executes the config command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("show-defaults") => Ok(render_project_config_yaml(&default_project_config())),
        Some("budgets") | Some("show-budgets") => Ok(render_runtime_budgets_yaml(
            &default_project_config().budgets,
        )),
        Some("layers") | Some("show-layers") => {
            Ok(render_config_layers_yaml(canonical_config_layers()))
        }
        Some("show-plugin") => show_plugin(args.get(1).map(String::as_str)),
        Some(other) => Err(CliError::new(format!("unknown config command: {other}"))),
    }
}

fn show_plugin(plugin_id: Option<&str>) -> Result<String, CliError> {
    let plugin_id =
        plugin_id.ok_or_else(|| CliError::new("config show-plugin requires a plugin id"))?;
    let plugin = catalog::find_official_plugin(plugin_id)
        .ok_or_else(|| CliError::new(format!("unknown plugin config: {plugin_id}")))?;
    let resolved = resolve_plugin_config(canonical_config_layers(), plugin.id).unwrap_or(
        ResolvedPluginConfig::new(
            plugin.id,
            PluginActivation::Disabled,
            ConfigScope::BuiltInDefaults,
        ),
    );

    Ok(render_resolved_plugin_config_yaml(&resolved))
}
