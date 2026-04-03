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
            lines.push(format!("policy: plugin_runtime ({})", server.plugin_id));

            match catalog::official_plugin_runtime(server.plugin_id) {
                Some(runtime) => match runtime.register_mcp_server(server.id) {
                    Ok(result) => {
                        let status = if result.ready { "[OK]" } else { "[NOT READY]" };
                        lines.push(format!("{status} {}", result.message));
                    }
                    Err(err) => {
                        lines.push(format!("[UNSUPPORTED] {err}"));
                    }
                },
                None => {
                    let msg = if locale == "pt-br" {
                        "Plugin não fornece implementação de runtime."
                    } else {
                        "Plugin does not provide a runtime implementation."
                    };
                    lines.push(msg.to_owned());
                }
            }
        }
        McpLaunchPolicy::SpawnProcess(ref command) => {
            lines.push(format!("command: {}", command.render_invocation()));
            lines.push(format!("working_directory: {}", command.working_directory));
            lines.push(format!("environment: {}", command.environment));

            match probe_binary(command.program) {
                Some(path) => {
                    let label = if locale == "pt-br" {
                        "Binário encontrado"
                    } else {
                        "Binary found"
                    };
                    lines.push(format!("[OK] {label}: {path}"));

                    // Spawn the process in foreground for validation
                    let spawn_label = if locale == "pt-br" {
                        "Iniciando"
                    } else {
                        "Spawning"
                    };
                    lines.push(format!("{spawn_label}: {}", command.render_invocation()));

                    // Print probe output before spawning (spawn blocks)
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
                            let exit_label = if locale == "pt-br" {
                                "Processo encerrado"
                            } else {
                                "Process exited"
                            };
                            lines.push(format!("{exit_label}: {exit}"));
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

/// Wraps the shared binary probe for local use with labeled output.
fn probe_binary(program: &str) -> Option<String> {
    re_plugin::probe_binary_on_path(program)
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
        assert!(super::probe_binary("sh").is_some());
    }

    #[test]
    fn probe_binary_reports_missing_for_nonexistent() {
        assert!(super::probe_binary("ralph-engine-nonexistent-binary-xyz").is_none());
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
