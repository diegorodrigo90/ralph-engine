//! MCP command handlers.

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
        Some(other) => Err(CliError::usage(i18n::unknown_subcommand(
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
        CliError::usage(i18n::unknown_entity(
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
        CliError::usage(i18n::unknown_entity(
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
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::mcp_server_entity_label(locale),
            server_id,
        ))
    })?;

    with_official_runtime_snapshot(|runtime| {
        let result = build_mcp_server_status(server_id, &runtime.topology).ok_or_else(|| {
            CliError::usage(i18n::unknown_entity(
                locale,
                i18n::mcp_server_entity_label(locale),
                server_id,
            ))
        })?;

        Ok(render_mcp_server_status_for_locale(&result, locale))
    })
}

/// Validates and optionally launches an MCP server.
fn probe_launch(server_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let server_id = server_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "mcp launch",
            i18n::mcp_server_id_entity_label(locale),
        ))
    })?;
    let server = catalog::find_official_mcp_server(server_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::mcp_server_entity_label(locale),
            server_id,
        ))
    })?;

    let plan = build_mcp_launch_plan(&server);
    let heading = i18n::mcp_launch_probe(locale);

    let mut lines = vec![
        format!("--- {heading}: {} ---", server.id),
        format!("transport: {}", server.transport),
    ];

    match server.launch_policy {
        McpLaunchPolicy::PluginRuntime => {
            launch_plugin_runtime_server(&server, &mut lines, locale);
        }
        McpLaunchPolicy::SpawnProcess(ref command) => {
            launch_spawn_process_server(command, &mut lines, locale)?;
        }
    }

    lines.push(String::new());
    lines.push(render_mcp_launch_plan_for_locale(&plan, locale));
    Ok(lines.join("\n"))
}

/// Handles MCP launch for plugin-managed servers.
fn launch_plugin_runtime_server(
    server: &re_mcp::McpServerDescriptor,
    lines: &mut Vec<String>,
    locale: &str,
) {
    lines.push(format!("policy: plugin_runtime ({})", server.plugin_id));

    match catalog::official_plugin_runtime(server.plugin_id) {
        Some(runtime) => match runtime.register_mcp_server(server.id) {
            Ok(result) => {
                let status = if result.ready {
                    super::STATUS_OK
                } else {
                    super::STATUS_NOT_READY
                };
                lines.push(format!("{status} {}", result.message));
            }
            Err(err) => {
                lines.push(format!("{} {err}", super::STATUS_UNSUPPORTED));
            }
        },
        None => {
            lines.push(i18n::mcp_no_runtime(locale).to_owned());
        }
    }
}

/// Handles MCP launch for externally spawned servers.
fn launch_spawn_process_server(
    command: &re_mcp::McpCommandDescriptor,
    lines: &mut Vec<String>,
    locale: &str,
) -> Result<(), CliError> {
    lines.push(format!("command: {}", command.render_invocation()));
    lines.push(format!("working_directory: {}", command.working_directory));
    lines.push(format!("environment: {}", command.environment));

    match re_plugin::probe_binary_on_path(command.program) {
        Some(path) => {
            lines.push(format!(
                "{} {}: {path}",
                super::STATUS_OK,
                i18n::mcp_binary_found(locale)
            ));

            lines.push(format!(
                "{}: {}",
                i18n::mcp_spawning_label(locale),
                command.render_invocation()
            ));

            // Flush output before blocking spawn
            println!("{}", lines.join("\n"));
            lines.clear();

            let cwd = std::env::current_dir().unwrap_or_default();
            let status = std::process::Command::new(command.program)
                .args(command.args)
                .current_dir(cwd)
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .status();

            match status {
                Ok(exit) => {
                    lines.push(format!("{}: {exit}", i18n::mcp_process_exited(locale)));
                }
                Err(e) => {
                    return Err(CliError::new(format!(
                        "Failed to spawn '{}': {e}",
                        command.program
                    )));
                }
            }
        }
        None => {
            lines.push(format!(
                "{} {}: {}",
                super::STATUS_MISSING,
                i18n::mcp_binary_not_found(locale),
                command.program
            ));

            lines.push(i18n::mcp_install_hint(locale, command.program));
        }
    }

    Ok(())
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
        assert!(re_plugin::probe_binary_on_path("sh").is_some());
    }

    #[test]
    fn probe_binary_reports_missing_for_nonexistent() {
        assert!(re_plugin::probe_binary_on_path("ralph-engine-nonexistent-binary-xyz").is_none());
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
        // Claude has runtime — should report readiness
        assert!(output.contains("[OK]") || output.contains("[NOT READY]"));
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
