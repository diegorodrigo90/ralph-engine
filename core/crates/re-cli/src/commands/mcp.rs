//! MCP command handlers.

use re_mcp::render_mcp_server_listing;

use crate::{CliError, catalog};

/// Executes the MCP command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_mcp_server_listing(&catalog::official_mcp_servers())),
        Some(other) => Err(CliError::new(format!("unknown mcp command: {other}"))),
    }
}
