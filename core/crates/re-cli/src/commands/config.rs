//! Configuration command handlers.

use re_config::{default_project_config, render_project_config_yaml};

use crate::CliError;

/// Executes the config command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("show-defaults") => Ok(render_project_config_yaml(&default_project_config())),
        Some(other) => Err(CliError::new(format!("unknown config command: {other}"))),
    }
}
