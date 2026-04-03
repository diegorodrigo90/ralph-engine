//! MCP command handlers.

use std::process::Command;

use re_core::{
    build_mcp_server_status, build_mcp_server_statuses, render_mcp_server_status_for_locale,
    render_mcp_server_statuses_for_locale,
};
use re_mcp::{
    McpLaunchPolicy, build_mcp_launch_plan, render_mcp_launch_plan_for_locale,
    render_mcp_server_detail_for_locale, render_mcp_server_listing_for_locale,
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
        Some("launch") => probe_launch(args.get(1).map(String::as_str), locale),
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

fn probe_launch(server_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let server_id = server_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "mcp launch",
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

    let plan = build_mcp_launch_plan(&server);
    let mut lines = Vec::new();

    let heading = if locale == "pt-br" {
        "Verificação de lançamento MCP"
    } else {
        "MCP launch probe"
    };
    lines.push(format!("--- {heading}: {} ---", server.id));
    lines.push(format!("transport: {}", server.transport));

    match server.launch_policy {
        McpLaunchPolicy::PluginRuntime => {
            let msg = if locale == "pt-br" {
                format!(
                    "Política: plugin_runtime — o plugin '{}' gerencia o bootstrap internamente",
                    server.plugin_id
                )
            } else {
                format!(
                    "Policy: plugin_runtime — plugin '{}' manages bootstrap internally",
                    server.plugin_id
                )
            };
            lines.push(msg);

            let note = if locale == "pt-br" {
                "Nota: lançamento real requer o trait PluginRuntime (ainda não implementado)"
            } else {
                "Note: real launch requires the PluginRuntime trait (not yet implemented)"
            };
            lines.push(note.to_owned());
        }
        McpLaunchPolicy::SpawnProcess(ref command) => {
            lines.push(format!("command: {}", command.render_invocation()));
            lines.push(format!("working_directory: {}", command.working_directory));
            lines.push(format!("environment: {}", command.environment));

            let probe_result = probe_binary(command.program);
            match probe_result {
                BinaryProbeResult::Found(path) => {
                    let label = if locale == "pt-br" {
                        "Binário encontrado"
                    } else {
                        "Binary found"
                    };
                    lines.push(format!("[OK] {label}: {path}"));
                }
                BinaryProbeResult::NotFound => {
                    let label = if locale == "pt-br" {
                        "Binário NÃO encontrado no PATH"
                    } else {
                        "Binary NOT found in PATH"
                    };
                    lines.push(format!("[MISSING] {label}: {}", command.program));

                    let hint = if locale == "pt-br" {
                        format!(
                            "Dica: instale '{}' ou adicione-o ao PATH para habilitar este servidor MCP",
                            command.program
                        )
                    } else {
                        format!(
                            "Hint: install '{}' or add it to PATH to enable this MCP server",
                            command.program
                        )
                    };
                    lines.push(hint);
                }
            }
        }
    }

    lines.push(String::new());
    lines.push(render_mcp_launch_plan_for_locale(&plan, locale));

    Ok(lines.join("\n"))
}

enum BinaryProbeResult {
    Found(String),
    NotFound,
}

/// Probes whether a binary is available on the system PATH.
fn probe_binary(program: &str) -> BinaryProbeResult {
    let which_cmd = if cfg!(windows) { "where" } else { "which" };
    match Command::new(which_cmd).arg(program).output() {
        Ok(output) if output.status.success() => {
            let path = String::from_utf8_lossy(&output.stdout)
                .trim()
                .lines()
                .next()
                .unwrap_or(program)
                .to_owned();
            BinaryProbeResult::Found(path)
        }
        _ => BinaryProbeResult::NotFound,
    }
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

    #[test]
    fn probe_binary_finds_common_system_binary() {
        let result = super::probe_binary("sh");
        assert!(matches!(result, super::BinaryProbeResult::Found(_)));
    }

    #[test]
    fn probe_binary_reports_missing_for_nonexistent() {
        let result = super::probe_binary("ralph-engine-nonexistent-binary-xyz");
        assert!(matches!(result, super::BinaryProbeResult::NotFound));
    }

    #[test]
    fn probe_launch_rejects_unknown_server_id() {
        let args = vec!["launch".to_owned(), "unknown.server".to_owned()];
        let result = execute(&args, "en");
        assert!(result.is_err());
    }

    #[test]
    fn probe_launch_requires_server_id() {
        let args = vec!["launch".to_owned()];
        let result = execute(&args, "en");
        assert!(result.is_err());
    }

    #[test]
    fn probe_launch_reports_plugin_runtime_policy() {
        // official.claude.session uses PluginRuntime launch policy
        let args = vec!["launch".to_owned(), "official.claude.session".to_owned()];
        let result = execute(&args, "en");
        assert!(result.is_ok());
        let output = result.ok().unwrap_or_default();
        assert!(output.contains("plugin_runtime"));
        assert!(output.contains("PluginRuntime trait"));
    }

    #[test]
    fn probe_launch_reports_spawn_process_policy() {
        // official.github.repository uses SpawnProcess launch policy
        let args = vec!["launch".to_owned(), "official.github.repository".to_owned()];
        let result = execute(&args, "en");
        assert!(result.is_ok());
        let output = result.ok().unwrap_or_default();
        assert!(output.contains("ralph-engine-github-mcp"));
    }

    #[test]
    fn probe_launch_supports_pt_br() {
        let args = vec!["launch".to_owned(), "official.claude.session".to_owned()];
        let result = execute(&args, "pt-br");
        assert!(result.is_ok());
        let output = result.ok().unwrap_or_default();
        assert!(output.contains("Verificação de lançamento MCP"));
    }
}
