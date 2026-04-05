//! TUI dashboard command.
//!
//! Launches the ratatui-based orchestration dashboard. When invoked
//! directly (`ralph-engine tui`) or as the default (`ralph-engine`
//! with no args), opens the interactive TUI.
//!
//! The dashboard is **functional** — slash commands typed in the input
//! bar are dispatched to the real CLI command handlers. `/run` starts
//! orchestration, `/doctor` checks health, `/plugins` lists plugins, etc.

use ratatui::crossterm::event::{self, Event, KeyEventKind, MouseEvent};

use crate::CliError;
use crate::catalog;
use crate::i18n;

/// Built-in slash commands available in the dashboard.
const DASHBOARD_COMMANDS: &[(&str, &str)] = &[
    ("run", "Start orchestration with TUI"),
    ("doctor", "Check project health"),
    ("plugins", "List installed plugins"),
    ("agents", "List available agents"),
    ("init", "Initialize project"),
    ("config", "Show configuration"),
    ("runtime", "Inspect runtime state"),
    ("help", "Show available commands"),
];

/// Executes the TUI dashboard.
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn execute(_args: &[String], locale: &str) -> Result<String, CliError> {
    let has_config = std::path::Path::new(".ralph-engine/config.yaml").exists();

    let config = re_tui::TuiConfig {
        title: if has_config {
            i18n::tui_dashboard_title(locale).to_owned()
        } else {
            i18n::tui_no_project_title(locale).to_owned()
        },
        agent_id: detect_agent_id(locale),
        locale: locale.to_owned(),
    };

    let mut shell = re_tui::TuiShell::new(config);
    shell.set_labels(build_labels(locale));
    // Dashboard starts idle (no agent running)
    shell.set_state(re_tui::TuiState::Complete);
    let cwd = std::env::current_dir().unwrap_or_default();

    // Auto-discover: enable input bar if any plugin requests it
    if catalog::any_plugin_wants_input_bar() {
        shell.enable_input();
    } else {
        // Dashboard always has input for slash commands
        shell.enable_input();
    }

    // Auto-discover: command prefix from configured agent plugin (Model B)
    let prefix = if let Ok(config) = super::runtime_state::load_project_config() {
        config
            .run
            .agent_plugin
            .map(catalog::agent_command_prefix)
            .unwrap_or_else(|| "/".to_owned())
    } else {
        "/".to_owned()
    };

    // Register built-in dashboard commands for autocomplete
    let mut commands: Vec<re_tui::CommandEntry> = DASHBOARD_COMMANDS
        .iter()
        .map(|(name, desc)| re_tui::CommandEntry {
            name: (*name).to_owned(),
            description: (*desc).to_owned(),
            source: re_tui::CommandSource::Plugin,
            source_name: "dashboard".to_owned(),
        })
        .collect();

    // Add plugin-discovered agent commands (auto-discovery)
    let agent_commands: Vec<re_tui::CommandEntry> =
        catalog::collect_agent_commands_from_plugins(&cwd)
            .into_iter()
            .map(|cmd| re_tui::CommandEntry {
                name: cmd.name.clone(),
                description: cmd.description,
                source: re_tui::CommandSource::Agent,
                source_name: cmd.plugin_id,
            })
            .collect();
    commands.extend(agent_commands);

    // Add plugin-contributed CLI commands (auto-discovery)
    let cli_commands: Vec<re_tui::CommandEntry> = catalog::collect_cli_contributions_from_plugins()
        .into_iter()
        .map(|(plugin_id, contrib)| re_tui::CommandEntry {
            name: contrib.name.clone(),
            description: contrib.description,
            source: re_tui::CommandSource::Plugin,
            source_name: plugin_id,
        })
        .collect();
    commands.extend(cli_commands);

    if !commands.is_empty() {
        shell.set_agent_commands(commands, prefix);
    }

    // Auto-discover sidebar panels from plugins
    let panels: Vec<re_tui::SidebarPanel> = catalog::collect_tui_panels_from_plugins()
        .into_iter()
        .map(|(plugin_id, panel)| re_tui::SidebarPanel {
            title: panel.title,
            lines: panel.lines,
            plugin_id,
        })
        .collect();
    shell.set_sidebar_panels(panels);

    // Auto-discover keybindings from plugins
    let keybindings = catalog::collect_tui_keybindings_from_plugins();
    shell.set_plugin_keybindings(keybindings);

    // Show project status on startup
    push_project_status(&mut shell, has_config, locale);

    // Try loading previous session (non-blocking)
    load_previous_session(&mut shell);

    let mut terminal = ratatui::init();

    // Enable bracketed paste so multi-line pastes arrive as a single Event::Paste
    let _ = ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::event::EnableBracketedPaste
    );

    let result: Result<(), String> = (|| {
        loop {
            terminal
                .draw(|frame| shell.render_frame(frame))
                .map_err(|e| format!("render: {e}"))?;

            if event::poll(std::time::Duration::from_millis(50))
                .map_err(|e| format!("poll: {e}"))?
            {
                match event::read().map_err(|e| format!("read: {e}"))? {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        let action = shell.handle_key_with_modifiers(key.code, key.modifiers);

                        // Dispatch unhandled keys to plugin runtimes
                        if action == re_tui::PluginKeyAction::NotHandled
                            && let ratatui::crossterm::event::KeyCode::Char(c) = key.code
                        {
                            let state_label = format!("{:?}", shell.state());
                            if let Some(binding) = shell.find_active_binding(c, &state_label) {
                                let plugin_id = binding.plugin_id.clone();
                                let result = catalog::dispatch_plugin_tui_key(
                                    &plugin_id,
                                    &c.to_string(),
                                    &state_label,
                                );
                                shell.apply_plugin_action(&result);
                            }
                        }

                        // Dispatch slash commands or text input
                        if let Some(text) = shell.take_text_input() {
                            handle_dashboard_command(&mut shell, &text, locale);
                        }

                        // Handle agent switching (Ctrl+A selection)
                        if let Some(target_agent) = shell.take_selected_agent() {
                            handle_agent_switch(&mut shell, &target_agent, locale);
                        }
                    }
                    Event::Mouse(MouseEvent { kind, .. }) => {
                        shell.handle_mouse(kind);
                    }
                    Event::Paste(text) => {
                        shell.handle_paste(&text);
                    }
                    _ => {}
                }
            }

            if shell.should_quit() {
                break;
            }
        }
        Ok(())
    })();

    let _ = ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::event::DisableBracketedPaste
    );
    ratatui::restore();
    result.map_err(CliError::new)?;
    Ok(String::new())
}

/// Dispatches a slash command typed in the TUI input bar.
///
/// Commands starting with `/` are parsed and dispatched to the real
/// CLI command handlers. The output is pushed to the activity feed.
fn handle_dashboard_command(shell: &mut re_tui::TuiShell, input: &str, locale: &str) {
    let trimmed = input.trim();

    // Parse slash command
    let Some(command_text) = trimmed.strip_prefix('/') else {
        // Not a slash command — try plugin text input handlers first
        let cwd = std::env::current_dir().unwrap_or_default();
        let result = catalog::dispatch_plugin_text_input(trimmed, &cwd);
        if result != re_tui::PluginKeyAction::NotHandled {
            shell.apply_plugin_action(&result);
            return;
        }
        // No plugin handled it — show "no agent" message
        let you = shell.labels().you_label.clone();
        let msg = shell.labels().no_agent_message.clone();
        shell.push_activity(format!("  ╭─ {you}: {trimmed}"));
        shell.push_activity(format!("  ╰─ {msg}"));
        return;
    };

    let parts: Vec<&str> = command_text.split_whitespace().collect();
    let Some(cmd_name) = parts.first() else {
        return;
    };

    // Show what we're running
    shell.push_activity(format!(">> /{command_text}"));

    match *cmd_name {
        "help" => {
            shell.push_activity(format!("── {} ──", i18n::tui_available_commands(locale)));
            for (name, desc) in DASHBOARD_COMMANDS {
                shell.push_activity(format!("  /{name:<12} {desc}"));
            }
        }
        "init" => {
            // Use --auto for non-interactive TUI init
            let mut args = vec!["--auto".to_owned()];
            args.extend(parts[1..].iter().map(|s| (*s).to_owned()));
            match super::dispatch_command("init", &args, locale) {
                Ok(output) => {
                    for line in output.lines() {
                        shell.push_activity(format!("  {line}"));
                    }
                }
                Err(e) => {
                    shell.push_activity(format!("  Error: {e}"));
                }
            }
        }
        #[cfg(debug_assertions)]
        "demo" => {
            populate_demo_feed(shell, locale);
        }
        "run" | "doctor" | "plugins" | "agents" | "config" | "runtime" | "checks" | "templates"
        | "prompts" | "hooks" | "mcp" | "capabilities" | "providers" | "locales" => {
            // Build args for the command handler
            let args: Vec<String> = parts[1..].iter().map(|s| (*s).to_owned()).collect();

            // Dispatch to the real command handler
            match super::dispatch_command(cmd_name, &args, locale) {
                Ok(output) => {
                    for line in output.lines() {
                        shell.push_activity(format!("  {line}"));
                    }
                }
                Err(e) => {
                    shell.push_activity(format!("  Error: {e}"));
                }
            }
        }
        other => {
            shell.push_activity(format!(
                "  {}: /{other}. {}",
                i18n::tui_unknown_command(locale),
                i18n::tui_type_help_hint(locale)
            ));
        }
    }
}

/// Pushes project status lines to the activity feed on startup.
fn push_project_status(shell: &mut re_tui::TuiShell, has_config: bool, locale: &str) {
    if has_config {
        // Run doctor silently to get status
        match super::dispatch_command("doctor", &[], locale) {
            Ok(output) => {
                for line in output.lines() {
                    shell.push_activity(format!("  {line}"));
                }
            }
            Err(_) => {
                shell.push_activity(format!("  {}", i18n::tui_project_run_hint(locale)));
            }
        }
    } else {
        shell.push_activity(format!("  {}", i18n::tui_no_config_found(locale)));
        shell.push_activity(format!("  {}", i18n::tui_type_init_tui(locale)));
    }
}

/// Detects the configured agent ID from project config, if available.
fn detect_agent_id(locale: &str) -> String {
    if let Ok(config) = super::runtime_state::load_project_config() {
        config
            .run
            .agent_id
            .unwrap_or(i18n::tui_no_agent_label(locale))
            .to_owned()
    } else {
        i18n::tui_no_project_label(locale).to_owned()
    }
}

/// Builds localized TUI labels from the CLI i18n system.
fn build_labels(locale: &str) -> re_tui::TuiLabels {
    re_tui::TuiLabels {
        project_configured: i18n::tui_project_configured(locale).to_owned(),
        no_project_found: i18n::tui_no_project_found(locale).to_owned(),
        type_run: i18n::tui_type_run(locale).to_owned(),
        type_init: i18n::tui_type_init(locale).to_owned(),
        orchestration_runtime: i18n::tui_orchestration_runtime(locale).to_owned(),
        waiting_session: i18n::tui_waiting_session(locale).to_owned(),
        help_title: i18n::tui_help_keys_heading(locale).to_owned(),
        nav_heading: i18n::tui_help_keys_heading(locale).to_owned(),
        actions_heading: i18n::tui_help_commands_heading(locale).to_owned(),
        plugins_heading: i18n::tui_help_plugin_keys(locale).to_owned(),
        slash_hint: i18n::tui_help_type_slash(locale).to_owned(),
        press_any_key: if locale == "pt-br" {
            "Pressione qualquer tecla para fechar".to_owned()
        } else {
            "Press any key to close".to_owned()
        },
        quit_title: if locale == "pt-br" {
            "Sair".to_owned()
        } else {
            "Quit".to_owned()
        },
        quit_question: if locale == "pt-br" {
            "Sair?".to_owned()
        } else {
            "Quit?".to_owned()
        },
        modal_open_hint: if locale == "pt-br" {
            "Modal aberto — pressione uma tecla".to_owned()
        } else {
            "Modal open — press a key".to_owned()
        },
        state_running: if locale == "pt-br" {
            "EXECUTANDO"
        } else {
            "RUNNING"
        }
        .to_owned(),
        state_paused: if locale == "pt-br" {
            "PAUSADO"
        } else {
            "PAUSED"
        }
        .to_owned(),
        state_complete: if locale == "pt-br" {
            "COMPLETO"
        } else {
            "COMPLETE"
        }
        .to_owned(),
        state_error: if locale == "pt-br" { "ERRO" } else { "ERROR" }.to_owned(),
        pause_label: if locale == "pt-br" { "pausar" } else { "pause" }.to_owned(),
        help_label: if locale == "pt-br" { "ajuda" } else { "help" }.to_owned(),
        quit_label: if locale == "pt-br" { "sair" } else { "quit" }.to_owned(),
        control_state: if locale == "pt-br" { "Estado" } else { "State" }.to_owned(),
        control_work: if locale == "pt-br" { "Tarefa" } else { "Work" }.to_owned(),
        tools_label: if locale == "pt-br" {
            "Ferramentas"
        } else {
            "Tools"
        }
        .to_owned(),
        lines_label: if locale == "pt-br" { "Linhas" } else { "Lines" }.to_owned(),
        progress_label: if locale == "pt-br" {
            "Progresso"
        } else {
            "Progress"
        }
        .to_owned(),
        logo_tagline: if locale == "pt-br" {
            "Loop Autônomo de Desenvolvimento IA".to_owned()
        } else {
            "Autonomous AI Dev Loop".to_owned()
        },
        nav_keys: if locale == "pt-br" {
            vec![
                ("j/k".into(), "Focar blocos".into()),
                ("↑↓".into(), "Rolar linhas".into()),
                ("PgUp/PgDn".into(), "Rolar páginas".into()),
                ("G / End".into(), "Seguir".into()),
                ("Home".into(), "Início".into()),
            ]
        } else {
            vec![
                ("j/k".into(), "Focus blocks".into()),
                ("↑↓".into(), "Scroll lines".into()),
                ("PgUp/PgDn".into(), "Scroll pages".into()),
                ("G / End".into(), "Follow mode".into()),
                ("Home".into(), "Scroll to top".into()),
            ]
        },
        action_keys: if locale == "pt-br" {
            vec![
                ("⏎ Enter".into(), "Expandir/recolher".into()),
                ("y".into(), "Copiar bloco".into()),
                ("⎋ Esc".into(), "Limpar foco".into()),
                ("F2".into(), "Alternar sidebar".into()),
                ("Ctrl+A".into(), "Trocar agente".into()),
                ("?".into(), "Esta ajuda".into()),
                ("q".into(), "Sair".into()),
            ]
        } else {
            vec![
                ("⏎ Enter".into(), "Expand/collapse".into()),
                ("y".into(), "Copy block".into()),
                ("⎋ Esc".into(), "Clear focus".into()),
                ("F2".into(), "Toggle sidebar".into()),
                ("Ctrl+A".into(), "Agent switcher".into()),
                ("?".into(), "This help".into()),
                ("q".into(), "Quit".into()),
            ]
        },
        you_label: if locale == "pt-br" {
            "Você".to_owned()
        } else {
            "You".to_owned()
        },
        no_agent_message: if locale == "pt-br" {
            "Nenhum agente conectado. Use /run para iniciar orquestração.".to_owned()
        } else {
            "No agent connected. Use /run to start orchestration.".to_owned()
        },
    }
}

/// Handles agent switching from the Ctrl+A switcher popup.
///
/// Exports context from current agent, optionally compacts it, and
/// imports into the target agent. Uses toasts for feedback.
fn handle_agent_switch(shell: &mut re_tui::TuiShell, target_agent: &str, _locale: &str) {
    let cwd = std::env::current_dir().unwrap_or_default();

    // Try to export from current agent (may not support export — proceed without)
    let current_agent = shell.labels().you_label.clone(); // placeholder — config has real ID
    let context = catalog::export_agent_session(&current_agent, &cwd).ok();

    // Compact context if target has a smaller window
    let context = context.map(|ctx| {
        let target_window = catalog::agent_context_window_size(target_agent);
        if target_window > 0 {
            catalog::compact_session_context(&ctx, target_window)
        } else {
            ctx
        }
    });

    // Import into target agent
    if let Some(ctx) = &context {
        match catalog::import_agent_session(target_agent, ctx, &cwd) {
            Ok(()) => {
                shell.toast_success(format!("Switched to {target_agent}"));
            }
            Err(e) => {
                shell.show_error_modal("Agent switch", &e);
            }
        }
    } else {
        shell.toast_info(format!("Switched to {target_agent} (no context transfer)"));
    }

    // Save session for recovery
    if let Some(ctx) = &context {
        let _ = catalog::save_session(ctx, &cwd);
    }
}

/// Loads previous session on TUI startup (if available).
fn load_previous_session(shell: &mut re_tui::TuiShell) {
    let cwd = std::env::current_dir().unwrap_or_default();

    match catalog::load_session(&cwd) {
        Ok(ctx) => {
            let msg_count = ctx.messages.len();
            let agent = if ctx.metadata.source_agent.is_empty() {
                "unknown"
            } else {
                &ctx.metadata.source_agent
            };
            shell.push_activity(format!(
                "  Previous session: {agent} ({msg_count} messages)"
            ));
        }
        Err(_) => {
            // No previous session — that's fine
        }
    }
}

/// Populates the feed with a realistic implementation demo.
///
/// Simulates a full story implementation cycle: read → analyze → edit →
/// test → fix → test again → commit. Enough content to fill scroll.
/// Only available in debug builds (`cargo run`), stripped from release.
#[cfg(debug_assertions)]
#[allow(clippy::too_many_lines)]
fn populate_demo_feed(shell: &mut re_tui::TuiShell, locale: &str) {
    use re_tui::feed::{BlockKind, FeedBlock};

    shell.set_state(re_tui::TuiState::Running);
    let mut blocks: Vec<FeedBlock> = Vec::new();

    // ── Phase 1: Start ──────────────────────────────────────────────
    let mut sys =
        FeedBlock::completed(BlockKind::System, i18n::demo_story_title(locale).to_owned());
    sys.collapsed = false;
    sys.push_content(i18n::demo_workflow_info(locale).to_owned());
    sys.push_content(i18n::demo_prompt_info(locale).to_owned());
    blocks.push(sys);

    let mut think1 = FeedBlock::completed(
        BlockKind::Thinking,
        i18n::demo_think_planning(locale).to_owned(),
    );
    think1.collapsed = false;
    think1.push_content(i18n::demo_think_planning_1(locale).to_owned());
    think1.push_content(i18n::demo_think_planning_2(locale).to_owned());
    think1.push_content(i18n::demo_think_planning_3(locale).to_owned());
    think1.push_content(i18n::demo_think_planning_4(locale).to_owned());
    blocks.push(think1);

    // ── Phase 2: Read existing code ─────────────────────────────────
    let mut r1 = FeedBlock::completed(
        BlockKind::FileRead,
        "src/modules/search/search.service.ts".to_owned(),
    );
    r1.push_content("@Injectable()".to_owned());
    r1.push_content("export class SearchService {".to_owned());
    r1.push_content("  constructor(private readonly prisma: PrismaService) {}".to_owned());
    r1.push_content("".to_owned());
    r1.push_content(
        "  async searchEvents(query: string, page: number, limit: number) {".to_owned(),
    );
    r1.push_content("    const offset = (page - 1) * limit;".to_owned());
    r1.push_content("    const events = await this.prisma.event.findMany({".to_owned());
    r1.push_content("      where: { title: { contains: query, mode: 'insensitive' } },".to_owned());
    r1.push_content("      skip: offset,".to_owned());
    r1.push_content("      take: limit,".to_owned());
    r1.push_content("      orderBy: { date: 'desc' },".to_owned());
    r1.push_content("    });".to_owned());
    r1.push_content("    const total = await this.prisma.event.count({ where: ... });".to_owned());
    r1.push_content("    return { events, total, page, limit };".to_owned());
    r1.push_content("  }".to_owned());
    r1.push_content("}".to_owned());
    r1.elapsed_ms_override(95);
    blocks.push(r1);

    let mut r2 = FeedBlock::completed(
        BlockKind::FileRead,
        "src/modules/search/search.resolver.ts".to_owned(),
    );
    r2.push_content("@Resolver()".to_owned());
    r2.push_content("export class SearchResolver {".to_owned());
    r2.push_content("  @Query(() => SearchResult)".to_owned());
    r2.push_content("  async searchEvents(@Args() args: SearchArgs) { ... }".to_owned());
    r2.push_content("}".to_owned());
    r2.elapsed_ms_override(78);
    blocks.push(r2);

    let mut r3 = FeedBlock::completed(
        BlockKind::FileRead,
        "src/modules/search/search.service.spec.ts".to_owned(),
    );
    r3.push_content("describe('SearchService', () => {".to_owned());
    r3.push_content("  it('should paginate results with offset', async () => { ... });".to_owned());
    r3.push_content("  it('should return total count', async () => { ... });".to_owned());
    r3.push_content("});".to_owned());
    r3.elapsed_ms_override(62);
    blocks.push(r3);

    // ── Phase 3: Edit service ───────────────────────────────────────
    let mut think2 = FeedBlock::completed(
        BlockKind::Thinking,
        i18n::demo_think_cursor(locale).to_owned(),
    );
    think2.collapsed = false;
    think2.push_content(i18n::demo_think_cursor_1(locale).to_owned());
    think2.push_content(i18n::demo_think_cursor_2(locale).to_owned());
    blocks.push(think2);

    let mut e1 = FeedBlock::completed(
        BlockKind::FileEdit,
        "src/modules/search/search.service.ts".to_owned(),
    );
    e1.collapsed = false;
    e1.push_content("@@ -5,10 +5,22 @@".to_owned());
    e1.push_content(
        "-  async searchEvents(query: string, page: number, limit: number) {".to_owned(),
    );
    e1.push_content("-    const offset = (page - 1) * limit;".to_owned());
    e1.push_content("-    const events = await this.prisma.event.findMany({".to_owned());
    e1.push_content(
        "-      where: { title: { contains: query, mode: 'insensitive' } },".to_owned(),
    );
    e1.push_content("-      skip: offset,".to_owned());
    e1.push_content("-      take: limit,".to_owned());
    e1.push_content(
        "+  async searchEvents(query: string, first: number, after?: string) {".to_owned(),
    );
    e1.push_content("+    const cursor = after ? { id: after } : undefined;".to_owned());
    e1.push_content("+    const events = await this.prisma.event.findMany({".to_owned());
    e1.push_content(
        "+      where: { title: { contains: query, mode: 'insensitive' } },".to_owned(),
    );
    e1.push_content("+      take: first + 1,".to_owned());
    e1.push_content("+      cursor,".to_owned());
    e1.push_content("+      skip: cursor ? 1 : 0,".to_owned());
    e1.push_content("       orderBy: { date: 'desc' },".to_owned());
    e1.push_content("     });".to_owned());
    e1.push_content("-    const total = await this.prisma.event.count({ where: ... });".to_owned());
    e1.push_content("-    return { events, total, page, limit };".to_owned());
    e1.push_content("+    const hasNextPage = events.length > first;".to_owned());
    e1.push_content("+    const edges = events.slice(0, first);".to_owned());
    e1.push_content("+    return {".to_owned());
    e1.push_content("+      edges: edges.map(e => ({ node: e, cursor: e.id })),".to_owned());
    e1.push_content("+      pageInfo: {".to_owned());
    e1.push_content("+        hasNextPage,".to_owned());
    e1.push_content("+        endCursor: edges.at(-1)?.id ?? null,".to_owned());
    e1.push_content("+      },".to_owned());
    e1.push_content("+    };".to_owned());
    e1.elapsed_ms_override(140);
    blocks.push(e1);

    // ── Phase 4: Edit resolver ──────────────────────────────────────
    let mut e2 = FeedBlock::completed(
        BlockKind::FileEdit,
        "src/modules/search/search.resolver.ts".to_owned(),
    );
    e2.collapsed = false;
    e2.push_content("@@ -3,4 +3,8 @@".to_owned());
    e2.push_content("-  @Query(() => SearchResult)".to_owned());
    e2.push_content("-  async searchEvents(@Args() args: SearchArgs) { ... }".to_owned());
    e2.push_content("+  @Query(() => SearchConnection)".to_owned());
    e2.push_content("+  async searchEvents(".to_owned());
    e2.push_content("+    @Args('query') query: string,".to_owned());
    e2.push_content("+    @Args('first', { defaultValue: 20 }) first: number,".to_owned());
    e2.push_content("+    @Args('after', { nullable: true }) after?: string,".to_owned());
    e2.push_content("+  ) {".to_owned());
    e2.push_content("+    return this.searchService.searchEvents(query, first, after);".to_owned());
    e2.push_content("+  }".to_owned());
    e2.elapsed_ms_override(95);
    blocks.push(e2);

    // ── Phase 5: Update tests ───────────────────────────────────────
    let mut e3 = FeedBlock::completed(
        BlockKind::FileEdit,
        "src/modules/search/search.service.spec.ts".to_owned(),
    );
    e3.collapsed = false;
    e3.push_content("@@ -1,5 +1,14 @@".to_owned());
    e3.push_content(" describe('SearchService', () => {".to_owned());
    e3.push_content(
        "-  it('should paginate results with offset', async () => { ... });".to_owned(),
    );
    e3.push_content("-  it('should return total count', async () => { ... });".to_owned());
    e3.push_content("+  it('should return first N results with cursor', async () => {".to_owned());
    e3.push_content("+    const result = await service.searchEvents('marathon', 10);".to_owned());
    e3.push_content("+    expect(result.edges).toHaveLength(10);".to_owned());
    e3.push_content("+    expect(result.pageInfo.hasNextPage).toBe(true);".to_owned());
    e3.push_content("+  });".to_owned());
    e3.push_content("+".to_owned());
    e3.push_content("+  it('should paginate with after cursor', async () => {".to_owned());
    e3.push_content("+    const page1 = await service.searchEvents('run', 5);".to_owned());
    e3.push_content("+    const cursor = page1.pageInfo.endCursor;".to_owned());
    e3.push_content("+    const page2 = await service.searchEvents('run', 5, cursor);".to_owned());
    e3.push_content(
        "+    expect(page2.edges[0].node.id).not.toBe(page1.edges[0].node.id);".to_owned(),
    );
    e3.push_content("+  });".to_owned());
    e3.elapsed_ms_override(110);
    blocks.push(e3);

    // ── Phase 6: First test run — fails ─────────────────────────────
    let mut bash1 = FeedBlock::completed(
        BlockKind::Command,
        "pnpm test -- --testPathPattern=search".to_owned(),
    );
    bash1.push_content("FAIL src/modules/search/search.service.spec.ts".to_owned());
    bash1.push_content("  SearchService".to_owned());
    bash1.push_content("    ✓ should return first N results with cursor (45ms)".to_owned());
    bash1.push_content("    ✗ should paginate with after cursor (12ms)".to_owned());
    bash1.push_content("".to_owned());
    bash1
        .push_content("  TypeError: Cannot read properties of undefined (reading 'id')".to_owned());
    bash1.push_content("    at Object.<anonymous> (search.service.spec.ts:14:52)".to_owned());
    bash1.push_content("".to_owned());
    bash1.push_content("Tests:  1 failed, 1 passed, 2 total".to_owned());
    bash1.active = false;
    bash1.success = Some(false);
    bash1.elapsed_ms_override(4800);
    blocks.push(bash1);

    let mut fail = FeedBlock::completed(
        BlockKind::GateFail,
        i18n::demo_gate_tests_fail(locale).to_owned(),
    );
    fail.success = Some(false);
    blocks.push(fail);

    // ── Phase 7: Fix the bug ────────────────────────────────────────
    let mut think3 = FeedBlock::completed(
        BlockKind::Thinking,
        i18n::demo_think_debug(locale).to_owned(),
    );
    think3.collapsed = false;
    think3.push_content(i18n::demo_think_debug_1(locale).to_owned());
    think3.push_content(i18n::demo_think_debug_2(locale).to_owned());
    blocks.push(think3);

    let mut e4 = FeedBlock::completed(
        BlockKind::FileEdit,
        "src/modules/search/search.service.ts".to_owned(),
    );
    e4.collapsed = false;
    e4.push_content("@@ -15,1 +15,2 @@".to_owned());
    e4.push_content("-        endCursor: edges.at(-1)?.id ?? null,".to_owned());
    e4.push_content(
        "+        endCursor: edges.length > 0 ? edges[edges.length - 1].id : null,".to_owned(),
    );
    e4.push_content("+        hasPreviousPage: !!cursor,".to_owned());
    e4.elapsed_ms_override(60);
    blocks.push(e4);

    // ── Phase 8: Second test run — passes ───────────────────────────
    let mut bash2 = FeedBlock::completed(
        BlockKind::Command,
        "pnpm test -- --testPathPattern=search".to_owned(),
    );
    bash2.push_content("PASS src/modules/search/search.service.spec.ts".to_owned());
    bash2.push_content("  SearchService".to_owned());
    bash2.push_content("    ✓ should return first N results with cursor (38ms)".to_owned());
    bash2.push_content("    ✓ should paginate with after cursor (22ms)".to_owned());
    bash2.push_content("".to_owned());
    bash2.push_content("Tests:  2 passed, 2 total".to_owned());
    bash2.elapsed_ms_override(3100);
    blocks.push(bash2);

    let pass1 = FeedBlock::completed(
        BlockKind::GatePass,
        i18n::demo_gate_tests_pass(locale).to_owned(),
    );
    blocks.push(pass1);

    // ── Phase 9: Quality gates ──────────────────────────────────────
    let mut bash3 = FeedBlock::completed(BlockKind::Command, "pnpm type-check".to_owned());
    bash3.push_content("No errors found.".to_owned());
    bash3.elapsed_ms_override(8500);
    blocks.push(bash3);

    let pass2 = FeedBlock::completed(
        BlockKind::GatePass,
        i18n::demo_gate_typecheck(locale).to_owned(),
    );
    blocks.push(pass2);

    let mut bash4 = FeedBlock::completed(BlockKind::Command, "pnpm build".to_owned());
    bash4.push_content("apps/api: build succeeded in 12.3s".to_owned());
    bash4.push_content("apps/web: build succeeded in 18.7s".to_owned());
    bash4.push_content("packages/ui: build succeeded in 4.1s".to_owned());
    bash4.elapsed_ms_override(35200);
    blocks.push(bash4);

    let pass3 = FeedBlock::completed(
        BlockKind::GatePass,
        i18n::demo_gate_build(locale).to_owned(),
    );
    blocks.push(pass3);

    // ── Phase 10: Commit ────────────────────────────────────────────
    let mut bash5 = FeedBlock::completed(BlockKind::Command, "git add -A && git commit".to_owned());
    bash5.push_content(
        "[main a1b2c3d] feat(search): migrate to cursor-based pagination (5.3)".to_owned(),
    );
    bash5.push_content(" 3 files changed, 42 insertions(+), 12 deletions(-)".to_owned());
    bash5.elapsed_ms_override(1200);
    blocks.push(bash5);

    // ── Phase 11: Summary ───────────────────────────────────────────
    let mut text = FeedBlock::completed(BlockKind::AgentText, String::new());
    text.collapsed = false;
    text.push_content(i18n::demo_summary_1(locale).to_owned());
    text.push_content(i18n::demo_summary_2(locale).to_owned());
    text.push_content(i18n::demo_summary_3(locale).to_owned());
    blocks.push(text);

    let mut done =
        FeedBlock::completed(BlockKind::System, i18n::demo_done_title(locale).to_owned());
    done.collapsed = false;
    done.push_content(i18n::demo_done_info(locale).to_owned());
    blocks.push(done);

    // Active: starting next story
    let active = FeedBlock::new(BlockKind::Thinking, i18n::demo_next(locale).to_owned());
    blocks.push(active);

    // Enqueue all blocks — they'll appear one by one with cadence
    shell.enqueue_blocks(blocks);
    shell.toast_info(i18n::demo_toast(locale).to_owned());
}
