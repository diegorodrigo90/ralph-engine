//! Runtime command handlers.

use re_core::render_runtime_topology;

use crate::{CliError, catalog};

/// Executes the runtime command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("show") => Ok(show_runtime()),
        Some(other) => Err(CliError::new(format!("unknown runtime command: {other}"))),
    }
}

fn show_runtime() -> String {
    let plugins = catalog::official_runtime_plugins();
    let mcp_servers = catalog::official_runtime_mcp_registrations();
    let topology = catalog::official_runtime_topology(&plugins, &mcp_servers);

    render_runtime_topology(&topology)
}
