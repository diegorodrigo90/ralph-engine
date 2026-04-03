//! Run command — executes a work item through a workflow plugin and agent.

use crate::{CliError, catalog, i18n};

use super::runtime_state::load_project_config;

/// Executes the run command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        Some("--list") => list_work_items(locale),
        Some("plan") => run_plan(args.get(1).map(String::as_str), locale),
        Some(id) if !id.starts_with('-') => run_work_item(id, locale),
        None => Err(CliError::new(locale_str!(
            locale,
            "Work item ID required. Use `ralph-engine run <id>` or `ralph-engine run --list`.",
            "ID do work item obrigatório. Use `ralph-engine run <id>` ou `ralph-engine run --list`."
        ))),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "run", other,
        ))),
    }
}

/// Lists available work items from the workflow plugin.
fn list_work_items(locale: &str) -> Result<String, CliError> {
    let (workflow_runtime, _) = resolve_run_plugins(locale)?;
    let cwd = current_dir_or_error(locale)?;

    let items = workflow_runtime
        .list_work_items(&cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    if items.is_empty() {
        return Ok(locale_str!(
            locale,
            "No actionable work items found.",
            "Nenhum work item encontrado."
        )
        .to_owned());
    }

    let heading = locale_str!(locale, "Available work items", "Work items disponíveis");
    let mut lines = vec![format!("{heading} ({}):", items.len())];
    for item in &items {
        lines.push(format!("  {} | {} | {}", item.id, item.title, item.status));
    }
    Ok(lines.join("\n"))
}

/// Shows the execution plan without launching the agent (dry run).
fn run_plan(work_item_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let work_item_id = work_item_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "run plan",
            locale_str!(locale, "work item ID", "ID do work item"),
        ))
    })?;

    let (workflow_runtime, agent_runtime) = resolve_run_plugins(locale)?;
    let cwd = current_dir_or_error(locale)?;
    let config = load_project_config()?;

    // Resolve work item
    let resolution = workflow_runtime
        .resolve_work_item(work_item_id, &cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    // Build prompt
    let context = workflow_runtime
        .build_prompt_context(&resolution, &cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    // Probe agent readiness
    let agent_id = config.run.agent_id.unwrap_or("unknown");
    let agent_status = agent_runtime.bootstrap_agent(agent_id);

    let workflow_label = locale_str!(locale, "Workflow", "Workflow");
    let agent_label = locale_str!(locale, "Agent", "Agente");
    let story_label = locale_str!(locale, "Work item", "Work item");
    let prompt_label = locale_str!(locale, "Prompt size", "Tamanho do prompt");
    let ready_label = locale_str!(locale, "Agent ready", "Agente pronto");

    let ready = agent_status.as_ref().is_ok_and(|r| r.ready);
    let ready_display = if ready { "[OK]" } else { "[NOT READY]" };

    let mut lines = vec![
        format!(
            "--- {}: {} ---",
            locale_str!(locale, "Execution plan", "Plano de execução"),
            resolution.canonical_id
        ),
        format!(
            "{workflow_label}: {}",
            config.run.workflow_plugin.unwrap_or("(not configured)")
        ),
        format!(
            "{agent_label}: {}",
            config.run.agent_plugin.unwrap_or("(not configured)")
        ),
        format!(
            "{story_label}: {} — {}",
            resolution.canonical_id, resolution.title
        ),
    ];

    if let Some(ref path) = resolution.source_path {
        lines.push(format!(
            "{}: {path}",
            locale_str!(locale, "Source", "Fonte")
        ));
    }

    lines.push(format!(
        "{prompt_label}: {} bytes ({} {})",
        context.prompt_text.len(),
        context.context_files.len(),
        locale_str!(locale, "context files", "arquivos de contexto")
    ));
    lines.push(format!("{ready_label}: {ready_display}"));

    if !ready && let Err(err) = &agent_status {
        lines.push(format!(
            "{}: {}",
            locale_str!(locale, "Hint", "Dica"),
            err.message
        ));
    }

    Ok(lines.join("\n"))
}

/// Executes one work item: resolve → build prompt → launch agent.
fn run_work_item(work_item_id: &str, locale: &str) -> Result<String, CliError> {
    let (workflow_runtime, agent_runtime) = resolve_run_plugins(locale)?;
    let cwd = current_dir_or_error(locale)?;
    let config = load_project_config()?;

    let agent_id = config.run.agent_id.ok_or_else(|| {
        CliError::new(locale_str!(
            locale,
            "Missing 'run.agent_id' in .ralph-engine/config.yaml.",
            "Campo 'run.agent_id' ausente em .ralph-engine/config.yaml."
        ))
    })?;

    // 1. Probe agent
    let bootstrap = agent_runtime
        .bootstrap_agent(agent_id)
        .map_err(|err| CliError::new(err.to_string()))?;

    if !bootstrap.ready {
        return Err(CliError::new(format!(
            "{}: {}",
            locale_str!(locale, "Agent not ready", "Agente não está pronto"),
            bootstrap.message
        )));
    }

    // 2. Resolve work item
    let resolution = workflow_runtime
        .resolve_work_item(work_item_id, &cwd)
        .map_err(|err| {
            CliError::new(format!(
                "{}: {}\n{}",
                locale_str!(locale, "Work item not found", "Work item não encontrado"),
                err.message,
                locale_str!(
                    locale,
                    "Use `ralph-engine run --list` to see available items.",
                    "Use `ralph-engine run --list` para ver itens disponíveis."
                )
            ))
        })?;

    // 3. Build prompt
    let context = workflow_runtime
        .build_prompt_context(&resolution, &cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    // 4. Print launch info
    let launch_msg = format!(
        "--- {} ---\n{}: {} — {}\n{}: {}\n",
        locale_str!(locale, "Launching agent", "Lançando agente"),
        locale_str!(locale, "Work item", "Work item"),
        resolution.canonical_id,
        resolution.title,
        locale_str!(locale, "Agent", "Agente"),
        agent_id,
    );
    // Print before blocking spawn — agent takes over stdout
    println!("{launch_msg}");

    // Flush before blocking agent process
    use std::io::Write as _;
    let _ = std::io::stdout().flush();

    // 5. Launch agent
    let result = agent_runtime
        .launch_agent(agent_id, &context, &cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    if result.success {
        Ok(format!(
            "\n--- {} ---\n{}",
            locale_str!(locale, "Agent completed", "Agente finalizado"),
            result.message
        ))
    } else {
        let code_info = result
            .exit_code
            .map(|c| format!(" (exit code: {c})"))
            .unwrap_or_default();
        Err(CliError::new(format!(
            "{}{code_info}: {}",
            locale_str!(locale, "Agent failed", "Agente falhou"),
            result.message
        )))
    }
}

// ── Private helpers ──────────────────────────────────────────────

/// Pair of workflow and agent plugin runtimes.
type PluginRuntimePair = (
    Box<dyn re_plugin::PluginRuntime>,
    Box<dyn re_plugin::PluginRuntime>,
);

/// Resolves the workflow and agent plugin runtimes from project config.
fn resolve_run_plugins(locale: &str) -> Result<PluginRuntimePair, CliError> {
    let config = load_project_config().map_err(|_| {
        CliError::new(locale_str!(
            locale,
            "Project config not found. Run `ralph-engine templates materialize official.bmad.starter .` to create one.",
            "Configuração do projeto não encontrada. Execute `ralph-engine templates materialize official.bmad.starter .` para criar."
        ))
    })?;

    let workflow_plugin_id = config.run.workflow_plugin.ok_or_else(|| {
        CliError::new(locale_str!(
            locale,
            "Missing 'run.workflow_plugin' in .ralph-engine/config.yaml.\nExample:\n  run:\n    workflow_plugin: official.bmad\n    agent_plugin: official.claude\n    agent_id: official.claude.session",
            "Campo 'run.workflow_plugin' ausente em .ralph-engine/config.yaml.\nExemplo:\n  run:\n    workflow_plugin: official.bmad\n    agent_plugin: official.claude\n    agent_id: official.claude.session"
        ))
    })?;

    let agent_plugin_id = config.run.agent_plugin.ok_or_else(|| {
        CliError::new(locale_str!(
            locale,
            "Missing 'run.agent_plugin' in .ralph-engine/config.yaml.\nExample:\n  run:\n    agent_plugin: official.claude",
            "Campo 'run.agent_plugin' ausente em .ralph-engine/config.yaml.\nExemplo:\n  run:\n    agent_plugin: official.claude"
        ))
    })?;

    let workflow_runtime =
        catalog::official_plugin_runtime(workflow_plugin_id).ok_or_else(|| {
            CliError::new(format!(
                "{}: {workflow_plugin_id}",
                locale_str!(
                    locale,
                    "Workflow plugin does not provide a runtime",
                    "Plugin de workflow não fornece runtime"
                ),
            ))
        })?;

    let agent_runtime = catalog::official_plugin_runtime(agent_plugin_id).ok_or_else(|| {
        CliError::new(format!(
            "{}: {agent_plugin_id}",
            locale_str!(
                locale,
                "Agent plugin does not provide a runtime",
                "Plugin de agente não fornece runtime"
            ),
        ))
    })?;

    Ok((workflow_runtime, agent_runtime))
}

/// Returns the current working directory or a typed error.
fn current_dir_or_error(locale: &str) -> Result<std::path::PathBuf, CliError> {
    std::env::current_dir().map_err(|err| {
        CliError::new(format!(
            "{}: {err}",
            locale_str!(
                locale,
                "Failed to resolve working directory",
                "Falha ao resolver diretório de trabalho"
            )
        ))
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn execute_without_args_returns_usage_error() {
        let result = execute(&[], "en");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.0.contains("Work item ID required"));
    }

    #[test]
    fn execute_without_args_returns_usage_error_pt_br() {
        let result = execute(&[], "pt-br");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.0.contains("ID do work item"));
    }

    #[test]
    fn execute_with_unknown_flag_returns_error() {
        let result = execute(&["--unknown".to_owned()], "en");
        assert!(result.is_err());
    }
}
