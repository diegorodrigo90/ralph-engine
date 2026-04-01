//! Runtime command handlers.

use re_core::{evaluate_runtime_status, render_runtime_status, render_runtime_topology};

use crate::{CliError, catalog};

/// Executes the runtime command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("show") => Ok(show_runtime()),
        Some("status") => Ok(show_runtime_status()),
        Some(other) => Err(CliError::new(format!("unknown runtime command: {other}"))),
    }
}

fn show_runtime() -> String {
    let plugins = catalog::official_runtime_plugins();
    let capabilities = catalog::official_runtime_capabilities();
    let mcp_servers = catalog::official_runtime_mcp_registrations();
    let topology = catalog::official_runtime_topology(&plugins, &capabilities, &mcp_servers);

    render_runtime_topology(&topology)
}

fn show_runtime_status() -> String {
    let plugins = catalog::official_runtime_plugins();
    let capabilities = catalog::official_runtime_capabilities();
    let mcp_servers = catalog::official_runtime_mcp_registrations();
    let topology = catalog::official_runtime_topology(&plugins, &capabilities, &mcp_servers);
    let status = evaluate_runtime_status(&topology);

    render_runtime_status(&status)
}
