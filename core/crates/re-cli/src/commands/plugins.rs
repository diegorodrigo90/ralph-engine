//! Plugin command handlers.

use re_plugin::render_plugin_listing;

use crate::{CliError, catalog};

/// Executes the plugins command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_plugin_listing(&catalog::official_plugins())),
        Some(other) => Err(CliError::new(format!("unknown plugins command: {other}"))),
    }
}
