//! Plugin command handlers.

use re_config::{
    ConfigScope, PluginActivation, ResolvedPluginConfig, default_project_config_layer,
    resolve_plugin_config,
};
use re_plugin::{render_plugin_detail, render_plugin_listing};

use crate::{CliError, catalog};

/// Executes the plugins command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_plugin_listing(&catalog::official_plugins())),
        Some("show") => show_plugin(args.get(1).map(String::as_str)),
        Some(other) => Err(CliError::new(format!("unknown plugins command: {other}"))),
    }
}

fn show_plugin(plugin_id: Option<&str>) -> Result<String, CliError> {
    let plugin_id = plugin_id.ok_or_else(|| CliError::new("plugins show requires a plugin id"))?;
    let plugin = catalog::find_official_plugin(plugin_id)
        .ok_or_else(|| CliError::new(format!("unknown plugin: {plugin_id}")))?;
    let layers = [default_project_config_layer()];
    let resolved = resolve_plugin_config(&layers, plugin.id).unwrap_or(ResolvedPluginConfig::new(
        plugin.id,
        PluginActivation::Disabled,
        ConfigScope::BuiltInDefaults,
    ));
    let mut detail = render_plugin_detail(&plugin);
    detail.push_str(&format!(
        "\nResolved activation: {}",
        resolved.activation.as_str()
    ));
    detail.push_str(&format!(
        "\nResolved from: {}",
        resolved.resolved_from.as_str()
    ));

    Ok(detail)
}
