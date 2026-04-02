//! MCP command handlers.

use re_core::{
    build_mcp_server_status, build_mcp_server_statuses, render_mcp_server_status_for_locale,
    render_mcp_server_statuses_for_locale,
};
use re_mcp::{
    build_mcp_launch_plan, render_mcp_launch_plan_for_locale, render_mcp_server_detail_for_locale,
    render_mcp_server_listing_for_locale,
};

use super::runtime_state::with_official_runtime_snapshot;
use crate::{CliError, catalog, i18n};

/// Executes the MCP command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_mcp_server_listing_for_locale(
            &catalog::official_mcp_servers(),
            locale,
        )),
        Some("show") => show_server(args.get(1).map(String::as_str), locale),
        Some("plan") => render_launch_plan(args.get(1).map(String::as_str), locale),
        Some("status") => show_status(args.get(1).map(String::as_str), locale),
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

fn render_launch_plan(server_id: Option<&str>, locale: &str) -> Result<String, CliError> {
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

    Ok(render_mcp_launch_plan_for_locale(
        &build_mcp_launch_plan(&server),
        locale,
    ))
}

fn show_status(server_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    match server_id {
        Some(id) => show_single_status(id, locale),
        None => Ok(show_all_statuses(locale)),
    }
}

fn show_single_status(server_id: &str, locale: &str) -> Result<String, CliError> {
    catalog::find_official_mcp_server(server_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::mcp_server_entity_label(locale),
            server_id,
        ))
    })?;

    with_official_runtime_snapshot(|runtime| {
        let result = build_mcp_server_status(server_id, &runtime.topology).ok_or_else(|| {
            CliError::new(i18n::unknown_entity(
                locale,
                i18n::mcp_server_entity_label(locale),
                server_id,
            ))
        })?;

        Ok(render_mcp_server_status_for_locale(&result, locale))
    })
}

fn show_all_statuses(locale: &str) -> String {
    with_official_runtime_snapshot(|runtime| {
        let results = build_mcp_server_statuses(&runtime.topology);

        render_mcp_server_statuses_for_locale(&results, locale)
    })
}

#[cfg(test)]
mod tests {
    use re_core::{McpServerReadiness, McpServerStatusResult, RuntimeHealth};
    use re_mcp::McpTransport;

    use super::*;

    #[test]
    fn execute_status_returns_all_server_statuses() {
        let args = vec!["status".to_owned()];
        let result = execute(&args, "en");

        assert!(result.is_ok());
        let output = result.ok().unwrap_or_default();
        assert!(output.contains("MCP server statuses ("));
    }

    #[test]
    fn execute_status_rejects_unknown_server_id() {
        let args = vec!["status".to_owned(), "unknown.server".to_owned()];
        let result = execute(&args, "en");

        assert!(result.is_err());
    }

    #[test]
    fn show_all_statuses_produces_human_readable_output() {
        let output = show_all_statuses("en");

        assert!(output.contains("MCP server statuses"));
    }

    #[test]
    fn show_all_statuses_supports_pt_br() {
        let output = show_all_statuses("pt-br");

        assert!(output.contains("Status dos servidores MCP"));
    }

    #[test]
    fn render_mcp_server_status_round_trips_through_locale() {
        let result = McpServerStatusResult::new(
            "test.fixture",
            "test.plugin",
            McpServerReadiness::Ready,
            RuntimeHealth::Healthy,
            true,
            McpTransport::Stdio,
            vec![],
            vec![],
        );

        let en = render_mcp_server_status_for_locale(&result, "en");
        let pt = render_mcp_server_status_for_locale(&result, "pt-br");

        assert!(en.contains("MCP server status: test.fixture"));
        assert!(en.contains("Readiness: ready"));
        assert!(pt.contains("Status do servidor MCP: test.fixture"));
        assert!(pt.contains("Prontidão: pronto"));
    }
}
