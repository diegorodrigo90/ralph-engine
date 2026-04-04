//! Run command — executes a work item through a workflow plugin and agent.

use crate::{CliError, catalog, i18n};

use super::runtime_state::load_project_config;

/// Executes the run command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    let verbose = args.iter().any(|a| a == "--verbose" || a == "-v");
    let filtered: Vec<&str> = args
        .iter()
        .map(String::as_str)
        .filter(|a| *a != "--verbose" && *a != "-v")
        .collect();

    match filtered.first().copied() {
        Some("--list") => list_work_items(locale, verbose),
        Some("plan") => run_plan(filtered.get(1).copied(), locale, verbose),
        Some(id) if !id.starts_with('-') => run_work_item(id, locale, verbose),
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

/// Prints a debug line when verbose mode is active.
fn dbg_log(verbose: bool, msg: &str) {
    if verbose {
        eprintln!("[debug] {msg}");
    }
}

/// Lists available work items from the workflow plugin.
fn list_work_items(locale: &str, verbose: bool) -> Result<String, CliError> {
    dbg_log(verbose, "loading config...");
    let (workflow_runtime, _) = resolve_run_plugins(locale, verbose)?;
    let cwd = current_dir_or_error(locale)?;
    dbg_log(verbose, &format!("cwd: {}", cwd.display()));

    dbg_log(
        verbose,
        &format!(
            "calling list_work_items on plugin '{}'",
            workflow_runtime.plugin_id()
        ),
    );
    let items = workflow_runtime
        .list_work_items(&cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    dbg_log(verbose, &format!("found {} work items", items.len()));

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
fn run_plan(work_item_id: Option<&str>, locale: &str, verbose: bool) -> Result<String, CliError> {
    let work_item_id = work_item_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "run plan",
            locale_str!(locale, "work item ID", "ID do work item"),
        ))
    })?;

    dbg_log(verbose, "loading config...");
    let (workflow_runtime, agent_runtime) = resolve_run_plugins(locale, verbose)?;
    let cwd = current_dir_or_error(locale)?;
    let config = load_project_config()?;

    // Resolve work item
    dbg_log(verbose, &format!("resolving work item '{work_item_id}'..."));
    let resolution = workflow_runtime
        .resolve_work_item(work_item_id, &cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    dbg_log(
        verbose,
        &format!(
            "resolved: '{}' → title='{}', source={:?}, metadata={:?}",
            resolution.canonical_id, resolution.title, resolution.source_path, resolution.metadata
        ),
    );

    // Build prompt
    dbg_log(verbose, "building prompt context...");
    let context = workflow_runtime
        .build_prompt_context(&resolution, &cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    dbg_log(
        verbose,
        &format!(
            "prompt assembled: {} bytes, {} context files",
            context.prompt_text.len(),
            context.context_files.len()
        ),
    );
    for file in &context.context_files {
        dbg_log(
            verbose,
            &format!(
                "  context file: '{}' ({} bytes)",
                file.label,
                file.content.len()
            ),
        );
    }

    // Probe agent readiness
    let agent_id = config.run.agent_id.unwrap_or("unknown");
    dbg_log(verbose, &format!("probing agent '{agent_id}'..."));
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

    for (key, val) in &resolution.metadata {
        lines.push(format!("  {key}: {val}"));
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

    if verbose {
        lines.push(String::new());
        lines.push("--- prompt preview (first 500 chars) ---".to_owned());
        let preview: String = context.prompt_text.chars().take(500).collect();
        lines.push(preview);
        lines.push("--- end preview ---".to_owned());
    }

    Ok(lines.join("\n"))
}

/// Executes one work item: resolve → build prompt → launch agent.
fn run_work_item(work_item_id: &str, locale: &str, verbose: bool) -> Result<String, CliError> {
    dbg_log(verbose, "=== ralph-engine run: starting ===");
    dbg_log(verbose, "loading config...");

    let (workflow_runtime, agent_runtime) = resolve_run_plugins(locale, verbose)?;
    let cwd = current_dir_or_error(locale)?;
    let config = load_project_config()?;

    dbg_log(verbose, &format!("cwd: {}", cwd.display()));
    dbg_log(
        verbose,
        &format!(
            "config: workflow={:?}, agent={:?}, agent_id={:?}",
            config.run.workflow_plugin, config.run.agent_plugin, config.run.agent_id
        ),
    );

    // Check autonomous mode acceptance
    ensure_autonomous_acceptance(&cwd, locale, verbose)?;

    let agent_id = config.run.agent_id.ok_or_else(|| {
        CliError::new(locale_str!(
            locale,
            "Missing 'run.agent_id' in .ralph-engine/config.yaml.",
            "Campo 'run.agent_id' ausente em .ralph-engine/config.yaml."
        ))
    })?;

    // 1. Probe agent
    dbg_log(
        verbose,
        &format!("[step 1/5] probing agent '{agent_id}'..."),
    );
    let bootstrap = agent_runtime
        .bootstrap_agent(agent_id)
        .map_err(|err| CliError::new(err.to_string()))?;

    dbg_log(
        verbose,
        &format!(
            "[step 1/5] agent ready={}, message='{}'",
            bootstrap.ready, bootstrap.message
        ),
    );

    if !bootstrap.ready {
        return Err(CliError::new(format!(
            "{}: {}",
            locale_str!(locale, "Agent not ready", "Agente não está pronto"),
            bootstrap.message
        )));
    }

    // 2. Resolve work item
    dbg_log(
        verbose,
        &format!("[step 2/5] resolving work item '{work_item_id}'..."),
    );
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

    dbg_log(
        verbose,
        &format!(
            "[step 2/5] resolved: id='{}', title='{}', source={:?}, metadata={:?}",
            resolution.canonical_id, resolution.title, resolution.source_path, resolution.metadata
        ),
    );

    // 3. Build prompt
    dbg_log(verbose, "[step 3/5] building prompt context...");
    let context = workflow_runtime
        .build_prompt_context(&resolution, &cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    dbg_log(
        verbose,
        &format!(
            "[step 3/5] prompt: {} bytes, {} context files",
            context.prompt_text.len(),
            context.context_files.len()
        ),
    );
    for file in &context.context_files {
        dbg_log(
            verbose,
            &format!(
                "[step 3/5]   file: '{}' ({} bytes)",
                file.label,
                file.content.len()
            ),
        );
    }

    // 4. Print launch info
    dbg_log(verbose, "[step 4/5] printing launch info...");
    let launch_msg = format!(
        "--- {} ---\n{}: {} — {}\n{}: {}\n",
        locale_str!(locale, "Launching agent", "Lançando agente"),
        locale_str!(locale, "Work item", "Work item"),
        resolution.canonical_id,
        resolution.title,
        locale_str!(locale, "Agent", "Agente"),
        agent_id,
    );
    println!("{launch_msg}");

    use std::io::Write as _;
    let _ = std::io::stdout().flush();

    // 5. Launch agent
    dbg_log(verbose, "[step 5/5] spawning agent process...");
    let result = agent_runtime
        .launch_agent(agent_id, &context, &cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    dbg_log(
        verbose,
        &format!(
            "[step 5/5] agent result: success={}, exit_code={:?}, message='{}'",
            result.success, result.exit_code, result.message
        ),
    );

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
fn resolve_run_plugins(locale: &str, verbose: bool) -> Result<PluginRuntimePair, CliError> {
    let config = load_project_config().map_err(|_| {
        CliError::new(locale_str!(
            locale,
            "Project config not found. Run `ralph-engine templates materialize official.bmad.starter .` to create one.",
            "Configuração do projeto não encontrada. Execute `ralph-engine templates materialize official.bmad.starter .` para criar."
        ))
    })?;

    dbg_log(
        verbose,
        &format!(
            "config loaded: schema_version={}, locale={}, plugins={}, run.workflow={:?}, run.agent={:?}, run.agent_id={:?}",
            config.schema_version,
            config.default_locale,
            config.plugins.len(),
            config.run.workflow_plugin,
            config.run.agent_plugin,
            config.run.agent_id,
        ),
    );

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

    dbg_log(
        verbose,
        &format!("resolving workflow runtime: '{workflow_plugin_id}'..."),
    );
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
    dbg_log(
        verbose,
        &format!("workflow runtime: OK ({})", workflow_runtime.plugin_id()),
    );

    dbg_log(
        verbose,
        &format!("resolving agent runtime: '{agent_plugin_id}'..."),
    );
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
    dbg_log(
        verbose,
        &format!("agent runtime: OK ({})", agent_runtime.plugin_id()),
    );

    Ok((workflow_runtime, agent_runtime))
}

/// Path to the autonomous mode acceptance file.
const AUTONOMOUS_ACCEPTANCE_FILE: &str = ".ralph-engine/.accepted-autonomous";

/// Ensures the user has accepted autonomous execution mode.
///
/// On first run, displays a warning and asks for confirmation via stdin.
/// Saves acceptance to a file so subsequent runs skip the prompt.
fn ensure_autonomous_acceptance(
    project_root: &std::path::Path,
    locale: &str,
    verbose: bool,
) -> Result<(), CliError> {
    let acceptance_path = project_root.join(AUTONOMOUS_ACCEPTANCE_FILE);

    if acceptance_path.exists() {
        dbg_log(verbose, "autonomous mode: previously accepted");
        return Ok(());
    }

    dbg_log(verbose, "autonomous mode: first run, asking for acceptance");

    let warning = locale_str!(
        locale,
        "⚠️  AUTONOMOUS MODE WARNING\n\n\
         ralph-engine run launches an AI agent that can:\n\
         - Read and write files in this project\n\
         - Execute shell commands\n\
         - Make git commits\n\n\
         The agent runs with auto-accept permissions to work autonomously.\n\
         This is equivalent to --dangerously-skip-permissions in Claude Code.\n\n\
         Only run this in projects you trust.\n\n\
         Accept and continue? [y/N] ",
        "⚠️  AVISO DE MODO AUTÔNOMO\n\n\
         ralph-engine run lança um agente de IA que pode:\n\
         - Ler e escrever arquivos neste projeto\n\
         - Executar comandos no shell\n\
         - Fazer commits no git\n\n\
         O agente roda com permissões auto-aceitas para trabalhar de forma autônoma.\n\
         Equivalente a --dangerously-skip-permissions no Claude Code.\n\n\
         Execute apenas em projetos que você confia.\n\n\
         Aceitar e continuar? [y/N] "
    );

    eprint!("{warning}");
    use std::io::Write as _;
    let _ = std::io::stderr().flush();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|err| CliError::new(format!("Failed to read input: {err}")))?;

    let accepted = matches!(
        input.trim().to_lowercase().as_str(),
        "y" | "yes" | "s" | "sim"
    );

    if !accepted {
        return Err(CliError::new(locale_str!(
            locale,
            "Autonomous mode not accepted. Aborting.",
            "Modo autônomo não aceito. Abortando."
        )));
    }

    // Save acceptance
    std::fs::write(&acceptance_path, format!(
        "# Autonomous mode accepted on {}\n# User confirmed risk awareness for ralph-engine run.\n",
        chrono_free_now()
    ))
    .map_err(|err| CliError::new(format!("Failed to save acceptance: {err}")))?;

    dbg_log(
        verbose,
        &format!(
            "autonomous mode: accepted, saved to {}",
            acceptance_path.display()
        ),
    );
    Ok(())
}

/// Returns a basic ISO-ish timestamp without chrono dependency.
fn chrono_free_now() -> String {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("unix:{}", duration.as_secs())
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

    #[test]
    fn verbose_flag_is_stripped_from_args() {
        // --verbose alone should still require work item ID
        let result = execute(&["--verbose".to_owned()], "en");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.0.contains("Work item ID required"));
    }
}
