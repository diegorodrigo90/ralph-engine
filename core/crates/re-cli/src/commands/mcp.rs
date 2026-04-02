//! MCP command handlers.

use re_mcp::{render_mcp_server_detail_for_locale, render_mcp_server_listing_for_locale};

use crate::{CliError, catalog, i18n};

/// Executes the MCP command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_mcp_server_listing_for_locale(
            &catalog::official_mcp_servers(),
            locale,
        )),
        Some("show") => show_server(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "mcp", other,
        ))),
    }
}

fn show_server(server_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let server_id = server_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "mcp",
            i18n::mcp_server_id_entity_label(locale),
        ))
    })?;
    let server = catalog::find_official_mcp_server(server_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::mcp_server_entity_label(locale),
            server_id,
        ))
    })?;

    Ok(render_mcp_server_detail_for_locale(&server, locale))
}
