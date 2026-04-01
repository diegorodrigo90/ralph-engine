//! Plugin command handlers.

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

    Ok(render_plugin_detail(&plugin))
}
