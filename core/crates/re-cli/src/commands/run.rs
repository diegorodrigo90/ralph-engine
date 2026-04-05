//! Run command — executes a work item through a workflow plugin and agent.

use crate::{CliError, catalog, i18n};

use super::runtime_state::load_project_config;

/// Executes the `exec` command — headless run (no TUI).
///
/// Alias for `run --headless <args>`. Used when TUI and headless
/// are separate subcommands (Codex pattern).
pub fn execute_headless(args: &[String], locale: &str) -> Result<String, CliError> {
    let mut full_args = vec!["--headless".to_owned()];
    full_args.extend(args.iter().cloned());
    execute(&full_args, locale)
}

/// Executes the run command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    let verbose = args.iter().any(|a| a == "--verbose" || a == "-v");
    let accept = args
        .iter()
        .any(|a| a == "--i-understand-ai-can-make-mistakes" || a == "--accept-risk");
    let filtered: Vec<&str> = args
        .iter()
        .map(String::as_str)
        .filter(|a| {
            !matches!(
                *a,
                "--verbose"
                    | "-v"
                    | "--i-understand-ai-can-make-mistakes"
                    | "--accept-risk"
                    | "--headless"
                    | "--no-tui"
            )
        })
        .collect();

    // --i-understand-ai-can-make-mistakes: pre-accept autonomous mode without interactive prompt
    if accept {
        let cwd = current_dir_or_error(locale)?;
        save_autonomous_acceptance(&cwd)?;
        if filtered.is_empty() {
            return Ok("Autonomous mode accepted.".to_owned());
        }
    }

    match filtered.first().copied() {
        Some("--list") => list_work_items(locale, verbose),
        Some("plan") => run_plan(filtered.get(1).copied(), locale, verbose),
        Some(id) if !id.starts_with('-') => run_work_item(id, locale, verbose),
        None => run_loop(locale, verbose),
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
        return Ok(i18n::run_no_items(locale).to_owned());
    }

    let heading = i18n::run_available_items(locale);
    let mut lines = vec![format!("{heading} ({}):", items.len())];
    for item in &items {
        lines.push(format!("  {} | {} | {}", item.id, item.title, item.status));
    }
    Ok(lines.join("\n"))
}

/// Autonomous loop: picks the next ready work item and executes it,
/// repeating until no more items are available or the user quits.
///
/// This is the core "Autonomous AI Dev Loop" — `ralph-engine run`
/// without arguments enters this mode.
fn run_loop(locale: &str, verbose: bool) -> Result<String, CliError> {
    dbg_log(verbose, "entering autonomous loop...");
    let cwd = current_dir_or_error(locale)?;

    // Acceptance check (once per session)
    ensure_autonomous_acceptance(&cwd, locale, verbose)?;

    let mut completed = 0usize;

    loop {
        // Reload plugins each iteration (config may change between runs)
        let (workflow_runtime, _agent_runtime) = resolve_run_plugins(locale, verbose)?;

        // Get next actionable work item
        let items = workflow_runtime
            .list_work_items(&cwd)
            .map_err(|err| CliError::new(err.to_string()))?;

        let next = items.iter().find(|item| item.actionable);

        let Some(item) = next else {
            dbg_log(verbose, "no more actionable work items");
            break;
        };

        let item_id = item.id.clone();
        let item_title = item.title.clone();

        dbg_log(
            verbose,
            &format!("next work item: {item_id} — {item_title}"),
        );

        eprintln!(
            "\n--- [{}/{}] {} — {} ---\n",
            completed + 1,
            items.len(),
            item_id,
            item_title,
        );

        // Execute the work item (TUI or headless)
        match run_work_item(&item_id, locale, verbose) {
            Ok(msg) => {
                completed += 1;
                dbg_log(verbose, &format!("completed: {item_id} — {msg}"));
            }
            Err(err) => {
                // Log error but continue to next item
                eprintln!("Error on {item_id}: {err}");
                dbg_log(verbose, &format!("failed: {item_id} — {err}"));
                break; // Stop on error — user should investigate
            }
        }
    }

    if completed == 0 {
        Ok(i18n::run_no_items(locale).to_owned())
    } else {
        Ok(format!(
            "Autonomous loop completed. {completed} work item(s) executed."
        ))
    }
}

/// Shows the execution plan without launching the agent (dry run).
fn run_plan(work_item_id: Option<&str>, locale: &str, verbose: bool) -> Result<String, CliError> {
    let work_item_id = work_item_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "run plan",
            i18n::run_work_item_id_label(locale),
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

    // Build prompt + discover tools
    dbg_log(verbose, "building prompt context...");
    let mut context = workflow_runtime
        .build_prompt_context(&resolution, &cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    let discovered_tools = catalog::collect_required_tools_from_plugins();
    dbg_log(
        verbose,
        &format!(
            "discovered {} tools from plugins: {:?}",
            discovered_tools.len(),
            discovered_tools
        ),
    );
    context.discovered_tools = discovered_tools;

    // Collect prompt contributions from plugins (e.g., findings).
    let contributions = catalog::collect_prompt_contributions_from_plugins(&cwd);
    for contrib in &contributions {
        dbg_log(
            verbose,
            &format!(
                "prompt contribution: '{}' ({} bytes)",
                contrib.label,
                contrib.content.len()
            ),
        );
        context.prompt_text.push_str("\n\n");
        context.prompt_text.push_str(&contrib.content);
        context.context_files.push(re_plugin::ContextFile {
            label: contrib.label.clone(),
            content: contrib.content.clone(),
        });
    }

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

    let workflow_label = i18n::run_workflow_label(locale);
    let agent_label = i18n::run_agent_label(locale);
    let story_label = i18n::run_work_item_label(locale);
    let prompt_label = i18n::run_prompt_size_label(locale);
    let ready_label = i18n::run_agent_ready_label(locale);

    let ready = agent_status.as_ref().is_ok_and(|r| r.ready);
    let ready_display = if ready { "[OK]" } else { "[NOT READY]" };

    let mut lines = vec![
        format!(
            "--- {}: {} ---",
            i18n::run_execution_plan(locale),
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
        lines.push(format!("{}: {path}", i18n::run_source_label(locale)));
    }

    for (key, val) in &resolution.metadata {
        lines.push(format!("  {key}: {val}"));
    }

    lines.push(format!(
        "{prompt_label}: {} bytes ({} {})",
        context.prompt_text.len(),
        context.context_files.len(),
        i18n::run_context_files_label(locale)
    ));
    lines.push(format!("{ready_label}: {ready_display}"));

    if !ready && let Err(err) = &agent_status {
        lines.push(format!("{}: {}", i18n::run_hint_label(locale), err.message));
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

    let agent_id = config
        .run
        .agent_id
        .ok_or_else(|| CliError::new(i18n::run_missing_agent_id(locale)))?;

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
            i18n::run_agent_not_ready(locale),
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
                i18n::run_work_item_not_found(locale),
                err.message,
                i18n::run_use_list_hint(locale)
            ))
        })?;

    dbg_log(
        verbose,
        &format!(
            "[step 2/5] resolved: id='{}', title='{}', source={:?}, metadata={:?}",
            resolution.canonical_id, resolution.title, resolution.source_path, resolution.metadata
        ),
    );

    // 3. Build prompt + collect discovered tools from all enabled plugins
    dbg_log(verbose, "[step 3/5] building prompt context...");
    let mut context = workflow_runtime
        .build_prompt_context(&resolution, &cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    // Auto-discover tools required by all enabled plugins.
    let discovered_tools = catalog::collect_required_tools_from_plugins();
    dbg_log(
        verbose,
        &format!(
            "[step 3/5] discovered {} tools from plugins: {:?}",
            discovered_tools.len(),
            discovered_tools
        ),
    );
    context.discovered_tools = discovered_tools;

    // Collect prompt contributions from all enabled plugins (e.g., findings).
    let contributions = catalog::collect_prompt_contributions_from_plugins(&cwd);
    for contrib in &contributions {
        dbg_log(
            verbose,
            &format!(
                "[step 3/5] prompt contribution: '{}' ({} bytes)",
                contrib.label,
                contrib.content.len()
            ),
        );
        // Insert contributions before constraints (constraints must stay last).
        context.prompt_text.push_str("\n\n");
        context.prompt_text.push_str(&contrib.content);
        context.context_files.push(re_plugin::ContextFile {
            label: contrib.label.clone(),
            content: contrib.content.clone(),
        });
    }

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

    // 4. Check for --no-tui flag
    let no_tui = std::env::args().any(|a| a == "--no-tui" || a == "--headless");

    if no_tui {
        // Headless mode: blocking launch, stream to stderr (original behavior)
        dbg_log(verbose, "[step 4/4] launching agent (headless)...");
        let result = agent_runtime
            .launch_agent(agent_id, &context, &cwd)
            .map_err(|err| CliError::new(err.to_string()))?;

        if result.success {
            Ok(format!(
                "\n--- {} ---\n{}",
                i18n::run_agent_completed(locale),
                result.message
            ))
        } else {
            let code_info = result
                .exit_code
                .map(|c| format!(" (exit code: {c})"))
                .unwrap_or_default();
            Err(CliError::new(format!(
                "{}{code_info}: {}",
                i18n::run_agent_failed(locale),
                result.message
            )))
        }
    } else {
        // TUI mode: spawn agent, run dashboard with live events
        run_with_tui(
            agent_runtime.as_ref(),
            agent_id,
            &context,
            &cwd,
            &resolution,
            locale,
        )
    }
}

// ── Private helpers ──────────────────────────────────────────────

/// Pair of workflow and agent plugin runtimes.
type PluginRuntimePair = (
    Box<dyn re_plugin::PluginRuntime>,
    Box<dyn re_plugin::PluginRuntime>,
);

/// Runs the agent with TUI dashboard.
///
/// Spawns the agent as a child process, then runs the ratatui render loop
/// reading stream-json events from stdout and displaying them in real-time.
#[cfg_attr(coverage_nightly, coverage(off))]
fn run_with_tui(
    agent_runtime: &dyn re_plugin::PluginRuntime,
    agent_id: &str,
    context: &re_plugin::PromptContext,
    cwd: &std::path::Path,
    resolution: &re_plugin::WorkItemResolution,
    locale: &str,
) -> Result<String, CliError> {
    use ratatui::crossterm::event::{self, Event, KeyEventKind};
    use std::io::BufRead as _;

    // Spawn agent (non-blocking)
    let mut spawned = agent_runtime
        .spawn_agent(agent_id, context, cwd)
        .map_err(|err| CliError::new(err.to_string()))?;

    // Set up TUI
    let tui_config = re_tui::TuiConfig {
        title: format!("{} — {}", resolution.canonical_id, resolution.title),
        agent_id: agent_id.to_owned(),
        locale: locale.to_owned(),
    };

    let mut shell = re_tui::TuiShell::new(tui_config);
    shell.set_agent_pid(spawned.pid);

    // Auto-discover: input bar from plugins
    if catalog::any_plugin_wants_input_bar() {
        shell.enable_input();
    }

    // Auto-discover: agent commands for autocomplete
    let commands: Vec<re_tui::CommandEntry> = catalog::collect_agent_commands_from_plugins(cwd)
        .into_iter()
        .map(|cmd| re_tui::CommandEntry {
            name: cmd.name,
            description: cmd.description,
            source: re_tui::CommandSource::Agent,
            source_name: cmd.plugin_id,
        })
        .collect();
    if !commands.is_empty() {
        shell.set_agent_commands(commands, "/".to_owned());
    }

    // Auto-discover: sidebar panels
    let panels: Vec<re_tui::SidebarPanel> = catalog::collect_tui_panels_from_plugins()
        .into_iter()
        .map(|(plugin_id, panel)| re_tui::SidebarPanel {
            title: panel.title,
            lines: panel.lines,
            plugin_id,
        })
        .collect();
    shell.set_sidebar_panels(panels);
    shell.push_startup_banner();

    // Non-blocking stdout reader via thread
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    if let Some(stdout) = spawned.take_stdout() {
        std::thread::spawn(move || {
            let reader = std::io::BufReader::new(stdout);
            for line in reader.lines() {
                let Ok(line) = line else { break };
                if tx.send(line).is_err() {
                    break;
                }
            }
        });
    }

    // TUI render loop
    let mut terminal = ratatui::init();
    let mut prev_state = re_tui::TuiState::Running;
    let result: Result<(), String> = (|| {
        loop {
            // Read agent events (non-blocking)
            while let Ok(line) = rx.try_recv() {
                let event = re_tui::parse_stream_line(&line);
                shell.process_event(&event);
                if event.is_terminal() {
                    shell.set_progress(100);
                }
            }

            terminal
                .draw(|frame| shell.render_frame(frame))
                .map_err(|e| format!("render: {e}"))?;

            if event::poll(std::time::Duration::from_millis(50))
                .map_err(|e| format!("poll: {e}"))?
                && let Event::Key(key) = event::read().map_err(|e| format!("read: {e}"))?
                && key.kind == KeyEventKind::Press
            {
                shell.handle_key_with_modifiers(key.code, key.modifiers);

                // Dispatch text input to agent plugin (feedback injection)
                if let Some(text) = shell.take_text_input() {
                    match agent_runtime.inject_feedback(&text, cwd) {
                        Ok(_) => {
                            shell.push_activity(">> Feedback saved.".to_owned());
                        }
                        Err(e) => {
                            shell.push_activity(format!(">> Feedback error: {}", e.message));
                        }
                    }
                }

                // Handle pause/resume state transitions via agent plugin
                let curr_state = shell.state();
                if curr_state != prev_state {
                    if let Some(pid) = shell.agent_pid() {
                        if curr_state == re_tui::TuiState::Paused {
                            let _ = agent_runtime.pause_agent(pid);
                        } else if prev_state == re_tui::TuiState::Paused
                            && curr_state == re_tui::TuiState::Running
                        {
                            let _ = agent_runtime.resume_agent(pid);
                        }
                    }
                    prev_state = curr_state;
                }
            }

            if shell.should_quit() {
                break;
            }

            // Check if agent process exited
            if let Ok(Some(_status)) = spawned.child.try_wait() {
                // Drain remaining events
                while let Ok(line) = rx.try_recv() {
                    let event = re_tui::parse_stream_line(&line);
                    shell.process_event(&event);
                }
                if shell.state() == re_tui::TuiState::Running {
                    shell.set_state(re_tui::TuiState::Complete);
                    shell.set_progress(100);
                    shell.push_activity(">> Agent process exited.".to_owned());
                }
            }
        }
        Ok(())
    })();

    ratatui::restore();
    result.map_err(CliError::new)?;

    Ok(format!("--- {} ---", i18n::run_agent_completed(locale)))
}

/// Resolves the workflow and agent plugin runtimes from project config.
fn resolve_run_plugins(locale: &str, verbose: bool) -> Result<PluginRuntimePair, CliError> {
    let config =
        load_project_config().map_err(|_| CliError::new(i18n::run_config_not_found(locale)))?;

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

    let workflow_plugin_id = config
        .run
        .workflow_plugin
        .ok_or_else(|| CliError::new(i18n::run_missing_workflow_plugin(locale)))?;

    let agent_plugin_id = config
        .run
        .agent_plugin
        .ok_or_else(|| CliError::new(i18n::run_missing_agent_plugin(locale)))?;

    dbg_log(
        verbose,
        &format!("resolving workflow runtime: '{workflow_plugin_id}'..."),
    );
    let workflow_runtime =
        catalog::official_plugin_runtime(workflow_plugin_id).ok_or_else(|| {
            CliError::new(format!(
                "{}: {workflow_plugin_id}",
                i18n::run_workflow_no_runtime(locale),
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
            i18n::run_agent_no_runtime(locale),
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

    // If stdin is not a TTY (CI, hooks, piped input), reject immediately
    // instead of blocking forever on read_line(). Rule 55.
    if !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
        return Err(CliError::new(
            "Autonomous mode requires interactive confirmation. \
             Run `ralph-engine run` in a terminal, or pre-accept with: \
             ralph-engine run --i-understand-ai-can-make-mistakes",
        ));
    }

    eprint!("{}", i18n::run_autonomous_warning(locale));
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
        return Err(CliError::new(i18n::run_autonomous_rejected(locale)));
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

/// Saves the autonomous acceptance file without interactive prompt.
fn save_autonomous_acceptance(project_root: &std::path::Path) -> Result<(), CliError> {
    let re_dir = project_root.join(".ralph-engine");
    std::fs::create_dir_all(&re_dir)
        .map_err(|err| CliError::new(format!("Failed to create .ralph-engine/: {err}")))?;
    let acceptance_path = re_dir.join(".accepted-autonomous");
    std::fs::write(
        &acceptance_path,
        format!(
            "# Autonomous mode accepted on {}\n# User confirmed: --i-understand-ai-can-make-mistakes\n",
            chrono_free_now()
        ),
    )
    .map_err(|err| CliError::new(format!("Failed to save acceptance: {err}")))?;
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
    std::env::current_dir()
        .map_err(|err| CliError::new(format!("{}: {err}", i18n::run_cwd_error(locale))))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    // Note: execute(&[], ..) and execute(&["--verbose"], ..) call run_loop()
    // which reads stdin for acceptance — cannot be unit tested (Rule 55).
    // These are covered by cli_smoke.rs with pre-accepted autonomous file.

    #[test]
    fn execute_with_unknown_flag_returns_error() {
        let result = execute(&["--unknown".to_owned()], "en");
        assert!(result.is_err());
    }

    #[test]
    fn execute_with_specific_id_without_config_returns_error() {
        let result = execute(&["5.3".to_owned()], "en");
        assert!(result.is_err());
    }
}
