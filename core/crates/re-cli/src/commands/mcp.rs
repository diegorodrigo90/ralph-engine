//! MCP command handlers.

use re_mcp::{render_mcp_server_detail, render_mcp_server_listing};

use crate::{CliError, catalog};

/// Executes the MCP command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_mcp_server_listing(&catalog::official_mcp_servers())),
        Some("show") => show_server(args.get(1).map(String::as_str)),
        Some(other) => Err(CliError::new(format!("unknown mcp command: {other}"))),
    }
}

fn show_server(server_id: Option<&str>) -> Result<String, CliError> {
    let server_id = server_id.ok_or_else(|| CliError::new("mcp show requires a server id"))?;
    let server = catalog::find_official_mcp_server(server_id)
        .ok_or_else(|| CliError::new(format!("unknown mcp server: {server_id}")))?;

    Ok(render_mcp_server_detail(&server))
}
