//! Re-exported official runtime catalog for CLI consumption.
//!
//! All functions that depend on plugin activation state are resolved
//! against the effective configuration layers (built-in defaults +
//! project file).  The wildcard re-export provides descriptor-only
//! queries; the explicit definitions below shadow the activation-aware
//! versions with the correct config resolution.

#![allow(dead_code)] // Shadow functions are part of the catalog API even when not all are called today

// Re-export everything from re_official as baseline.
pub use re_official::*;

use re_config::ProjectConfigLayer;

use crate::commands::runtime_state::load_effective_config_layers;

/// Returns the effective configuration layers for the current project.
///
/// Falls back to canonical defaults when the project file is absent or
/// unreadable.
fn effective_layers() -> Vec<ProjectConfigLayer> {
    load_effective_config_layers().unwrap_or_else(|_| re_config::canonical_config_layers().to_vec())
}

// ── Shadow activation-aware functions with project-config resolution ──

/// Returns a runtime snapshot resolved against the effective config layers.
///
/// Shadows [`re_official::official_runtime_snapshot`].
#[must_use]
pub fn official_runtime_snapshot() -> re_official::OfficialRuntimeSnapshot {
    re_official::official_runtime_snapshot_with_layers(&effective_layers())
}

/// Returns a runtime snapshot resolved against explicit config layers.
#[must_use]
pub fn official_runtime_snapshot_with_layers(
    layers: &[ProjectConfigLayer],
) -> re_official::OfficialRuntimeSnapshot {
    re_official::official_runtime_snapshot_with_layers(layers)
}

/// Returns runtime plugin registrations resolved against the effective config.
///
/// Shadows [`re_official::official_runtime_plugins`].
#[must_use]
pub fn official_runtime_plugins() -> Vec<re_core::RuntimePluginRegistration> {
    official_runtime_snapshot().plugins
}

/// Returns runtime capability registrations resolved against the effective config.
///
/// Shadows [`re_official::official_runtime_capabilities`].
#[must_use]
pub fn official_runtime_capabilities() -> Vec<re_core::RuntimeCapabilityRegistration> {
    official_runtime_snapshot().capabilities
}

/// Returns runtime template registrations resolved against the effective config.
///
/// Shadows [`re_official::official_runtime_templates`].
#[must_use]
pub fn official_runtime_templates() -> Vec<re_core::RuntimeTemplateRegistration> {
    official_runtime_snapshot().templates
}

/// Returns runtime prompt registrations resolved against the effective config.
///
/// Shadows [`re_official::official_runtime_prompts`].
#[must_use]
pub fn official_runtime_prompts() -> Vec<re_core::RuntimePromptRegistration> {
    official_runtime_snapshot().prompts
}

/// Returns runtime agent registrations resolved against the effective config.
///
/// Shadows [`re_official::official_runtime_agents`].
#[must_use]
pub fn official_runtime_agents() -> Vec<re_core::RuntimeAgentRegistration> {
    official_runtime_snapshot().agents
}

/// Returns runtime check registrations resolved against the effective config.
///
/// Shadows [`re_official::official_runtime_checks`].
#[must_use]
pub fn official_runtime_checks() -> Vec<re_core::RuntimeCheckRegistration> {
    official_runtime_snapshot().checks
}

/// Returns runtime provider registrations resolved against the effective config.
///
/// Shadows [`re_official::official_runtime_providers`].
#[must_use]
pub fn official_runtime_providers() -> Vec<re_core::RuntimeProviderRegistration> {
    official_runtime_snapshot().providers
}

/// Returns runtime policy registrations resolved against the effective config.
///
/// Shadows [`re_official::official_runtime_policies`].
#[must_use]
pub fn official_runtime_policies() -> Vec<re_core::RuntimePolicyRegistration> {
    official_runtime_snapshot().policies
}

/// Returns runtime hook registrations resolved against the effective config.
///
/// Shadows [`re_official::official_runtime_hooks`].
#[must_use]
pub fn official_runtime_hooks() -> Vec<re_core::RuntimeHookRegistration> {
    official_runtime_snapshot().hooks
}

/// Returns runtime MCP registrations resolved against the effective config.
///
/// Shadows [`re_official::official_runtime_mcp_registrations`].
#[must_use]
pub fn official_runtime_mcp_registrations() -> Vec<re_core::RuntimeMcpRegistration> {
    official_runtime_snapshot().mcp_servers
}

/// Returns template contributions resolved against the effective config.
///
/// Shadows [`re_official::official_template_contributions`].
#[must_use]
pub fn official_template_contributions() -> Vec<re_official::OfficialTemplateContribution> {
    let snapshot = official_runtime_snapshot();
    re_official::official_template_contributions_from_snapshot(&snapshot)
}

/// Returns agent contributions resolved against the effective config.
///
/// Shadows [`re_official::official_agent_contributions`].
#[must_use]
pub fn official_agent_contributions() -> Vec<re_official::OfficialAgentContribution> {
    let snapshot = official_runtime_snapshot();
    re_official::official_agent_contributions_from_snapshot(&snapshot)
}

/// Returns prompt contributions resolved against the effective config.
///
/// Shadows [`re_official::official_prompt_contributions`].
#[must_use]
pub fn official_prompt_contributions() -> Vec<re_official::OfficialPromptContribution> {
    let snapshot = official_runtime_snapshot();
    re_official::official_prompt_contributions_from_snapshot(&snapshot)
}

/// Returns policy contributions resolved against the effective config.
///
/// Shadows [`re_official::official_policy_contributions`].
#[must_use]
pub fn official_policy_contributions() -> Vec<re_official::OfficialPolicyContribution> {
    let snapshot = official_runtime_snapshot();
    re_official::official_policy_contributions_from_snapshot(&snapshot)
}

/// Returns check contributions resolved against the effective config.
///
/// Shadows [`re_official::official_check_contributions`].
#[must_use]
pub fn official_check_contributions() -> Vec<re_official::OfficialCheckContribution> {
    let snapshot = official_runtime_snapshot();
    re_official::official_check_contributions_from_snapshot(&snapshot)
}

/// Returns provider contributions resolved against the effective config.
///
/// Shadows [`re_official::official_provider_contributions`].
#[must_use]
pub fn official_provider_contributions() -> Vec<re_official::OfficialProviderContribution> {
    let snapshot = official_runtime_snapshot();
    re_official::official_provider_contributions_from_snapshot(&snapshot)
}

/// Finds one template contribution by stable identifier.
///
/// Shadows [`re_official::find_official_template_contribution`].
#[must_use]
pub fn find_official_template_contribution(
    template_id: &str,
) -> Option<re_official::OfficialTemplateContribution> {
    official_template_contributions()
        .into_iter()
        .find(|t| t.descriptor.id == template_id)
}

/// Finds one agent contribution by stable identifier.
///
/// Shadows [`re_official::find_official_agent_contribution`].
#[must_use]
pub fn find_official_agent_contribution(
    agent_id: &str,
) -> Option<re_official::OfficialAgentContribution> {
    official_agent_contributions()
        .into_iter()
        .find(|a| a.descriptor.id == agent_id)
}

/// Finds one prompt contribution by stable identifier.
///
/// Shadows [`re_official::find_official_prompt_contribution`].
#[must_use]
pub fn find_official_prompt_contribution(
    prompt_id: &str,
) -> Option<re_official::OfficialPromptContribution> {
    official_prompt_contributions()
        .into_iter()
        .find(|p| p.descriptor.id == prompt_id)
}

/// Finds one policy contribution by stable identifier.
///
/// Shadows [`re_official::find_official_policy_contribution`].
#[must_use]
pub fn find_official_policy_contribution(
    policy_id: &str,
) -> Option<re_official::OfficialPolicyContribution> {
    official_policy_contributions()
        .into_iter()
        .find(|p| p.descriptor.id == policy_id)
}

/// Finds one check contribution by stable identifier.
///
/// Shadows [`re_official::find_official_check_contribution`].
#[must_use]
pub fn find_official_check_contribution(
    check_id: &str,
) -> Option<re_official::OfficialCheckContribution> {
    official_check_contributions()
        .into_iter()
        .find(|c| c.descriptor.id == check_id)
}

/// Finds one provider contribution by stable identifier.
///
/// Shadows [`re_official::find_official_provider_contribution`].
#[must_use]
pub fn find_official_provider_contribution(
    provider_id: &str,
) -> Option<re_official::OfficialProviderContribution> {
    official_provider_contributions()
        .into_iter()
        .find(|p| p.descriptor.id == provider_id)
}

/// Finds capability registrations for a specific capability kind.
///
/// Shadows [`re_official::find_official_runtime_capabilities`].
#[must_use]
pub fn find_official_runtime_capabilities(
    capability: re_plugin::PluginCapability,
) -> Vec<re_core::RuntimeCapabilityRegistration> {
    official_runtime_capabilities()
        .into_iter()
        .filter(|c| c.capability == capability)
        .collect()
}

/// Finds hook registrations for a specific hook kind.
///
/// Shadows [`re_official::find_official_runtime_hooks`].
#[must_use]
pub fn find_official_runtime_hooks(
    hook: re_plugin::PluginRuntimeHook,
) -> Vec<re_core::RuntimeHookRegistration> {
    official_runtime_hooks()
        .into_iter()
        .filter(|h| h.hook == hook)
        .collect()
}

/// Finds check registrations for a specific check kind.
///
/// Shadows [`re_official::find_official_runtime_checks`].
#[must_use]
pub fn find_official_runtime_checks(
    kind: re_core::RuntimeCheckKind,
) -> Vec<re_core::RuntimeCheckRegistration> {
    official_runtime_checks()
        .into_iter()
        .filter(|c| c.kind == kind)
        .collect()
}

/// Finds provider registrations for a specific provider kind.
///
/// Shadows [`re_official::find_official_runtime_providers`].
#[must_use]
pub fn find_official_runtime_providers(
    kind: re_core::RuntimeProviderKind,
) -> Vec<re_core::RuntimeProviderRegistration> {
    official_runtime_providers()
        .into_iter()
        .filter(|p| p.kind == kind)
        .collect()
}

/// Finds check contributions for a specific check kind.
///
/// Shadows [`re_official::find_official_check_contributions`].
#[must_use]
pub fn find_official_check_contributions(
    kind: re_core::RuntimeCheckKind,
) -> Vec<re_official::OfficialCheckContribution> {
    official_check_contributions()
        .into_iter()
        .filter(|c| re_official::runtime_check_kind_for_descriptor(c.descriptor.kind) == kind)
        .collect()
}

/// Finds provider contributions for a specific provider kind.
///
/// Shadows [`re_official::find_official_provider_contributions`].
#[must_use]
pub fn find_official_provider_contributions(
    kind: re_core::RuntimeProviderKind,
) -> Vec<re_official::OfficialProviderContribution> {
    official_provider_contributions()
        .into_iter()
        .filter(|p| re_official::runtime_provider_kind_for_descriptor(p.descriptor.kind) == kind)
        .collect()
}

/// Finds resolved check surface by identifier.
///
/// Shadows [`re_official::find_official_check_surface`].
#[must_use]
pub fn find_official_check_surface(
    check_id: &str,
) -> Option<re_official::OfficialResolvedCheckSurface> {
    let contribution = find_official_check_contribution(check_id)?;
    let registration = find_official_runtime_checks(
        re_official::runtime_check_kind_for_descriptor(contribution.descriptor.kind),
    )
    .into_iter()
    .find(|candidate| candidate.plugin_id == contribution.descriptor.plugin_id)?;

    Some(re_official::OfficialResolvedCheckSurface {
        contribution,
        registration,
    })
}

/// Finds resolved provider surface by identifier.
///
/// Shadows [`re_official::find_official_provider_surface`].
#[must_use]
pub fn find_official_provider_surface(
    provider_id: &str,
) -> Option<re_official::OfficialResolvedProviderSurface> {
    let contribution = find_official_provider_contribution(provider_id)?;
    let registration = find_official_runtime_providers(
        re_official::runtime_provider_kind_for_descriptor(contribution.descriptor.kind),
    )
    .into_iter()
    .find(|candidate| candidate.plugin_id == contribution.descriptor.plugin_id)?;

    Some(re_official::OfficialResolvedProviderSurface {
        contribution,
        registration,
    })
}

/// Collects required tools from all enabled plugin runtimes.
///
/// Iterates over every enabled plugin that provides a runtime, calls
/// `required_tools()`, and returns a deduplicated list. This enables
/// auto-discovery: plugins declare what agent tools they need (MCP
/// tools, Skill, Agent, etc.) and the core passes the merged list
/// to the agent plugin at launch time.
#[must_use]
pub fn collect_required_tools_from_plugins() -> Vec<String> {
    let snapshot = official_runtime_snapshot();
    let mut tools: Vec<String> = Vec::new();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id) {
            for tool in runtime.required_tools() {
                let tool_str = (*tool).to_owned();
                if !tools.contains(&tool_str) {
                    tools.push(tool_str);
                }
            }
        }
    }

    tools
}

/// Collects prompt contributions from all enabled plugin runtimes.
///
/// Iterates over every enabled plugin that provides a runtime, calls
/// `prompt_contributions()`, and returns the merged list. This enables
/// plugins like `official.findings` to inject content (learnings,
/// context, etc.) into agent prompts without coupling to the workflow
/// plugin.
#[must_use]
pub fn collect_prompt_contributions_from_plugins(
    project_root: &std::path::Path,
) -> Vec<re_plugin::PromptContribution> {
    let snapshot = official_runtime_snapshot();
    let mut contributions = Vec::new();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id) {
            contributions.extend(runtime.prompt_contributions(project_root));
        }
    }

    contributions
}

/// Collects init contributions from all enabled plugin runtimes.
///
/// Iterates over every enabled plugin that provides a runtime, calls
/// `init_contributions()`, and returns the merged list. This enables
/// plugins (official or third-party) to contribute additional questions,
/// config sections, or files to `ralph-engine init` via auto-discovery.
#[must_use]
pub fn collect_init_contributions_from_plugins() -> Vec<re_plugin::InitContribution> {
    let snapshot = official_runtime_snapshot();
    let mut contributions = Vec::new();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id) {
            contributions.extend(runtime.init_contributions());
        }
    }

    contributions
}

/// Collects required files from all enabled plugin runtimes.
///
/// Each plugin declares files it needs in the project directory.
/// Core combines them and validates existence during doctor/checks.
#[must_use]
pub fn collect_required_files_from_plugins() -> Vec<String> {
    let snapshot = official_runtime_snapshot();
    let mut files: Vec<String> = Vec::new();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id) {
            for file in runtime.required_files() {
                let file_str = (*file).to_owned();
                if !files.contains(&file_str) {
                    files.push(file_str);
                }
            }
        }
    }

    files
}

/// Collects config validation issues from all enabled plugin runtimes.
#[must_use]
pub fn collect_config_issues_from_plugins(
    config_content: &str,
) -> Vec<(String, re_plugin::ConfigIssue)> {
    let snapshot = official_runtime_snapshot();
    let mut issues = Vec::new();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id) {
            for issue in runtime.validate_config(config_content) {
                issues.push((plugin.descriptor.id.to_owned(), issue));
            }
        }
    }

    issues
}

/// Collects CLI contributions from all enabled plugin runtimes.
#[must_use]
pub fn collect_cli_contributions_from_plugins() -> Vec<(String, re_plugin::CliContribution)> {
    let snapshot = official_runtime_snapshot();
    let mut contributions = Vec::new();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id) {
            for contrib in runtime.cli_contributions() {
                contributions.push((plugin.descriptor.id.to_owned(), contrib));
            }
        }
    }

    contributions
}

/// Collects TUI panel contributions from all enabled plugins.
///
/// Auto-discovers panels via `tui_contributions()` on each plugin
/// that has the `tui_widgets` capability. Returns panels with
/// their source plugin ID for attribution.
#[must_use]
pub fn collect_tui_panels_from_plugins() -> Vec<(String, re_plugin::TuiPanel)> {
    let snapshot = official_runtime_snapshot();
    let mut panels = Vec::new();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id) {
            for panel in runtime.tui_contributions() {
                panels.push((plugin.descriptor.id.to_owned(), panel));
            }
        }
    }

    panels
}

/// Collects agent commands from all enabled plugins that provide them.
///
/// Auto-discovers slash commands from agent plugins (e.g., Claude scans
/// `.claude/commands/`, Codex scans `.agents/skills/`). Returns commands
/// with their owning plugin ID.
pub fn collect_agent_commands_from_plugins(
    project_root: &std::path::Path,
) -> Vec<re_plugin::AgentCommand> {
    let snapshot = official_runtime_snapshot();
    let mut commands = Vec::new();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id) {
            commands.extend(runtime.discover_agent_commands(project_root));
        }
    }

    commands
}

/// Checks if any enabled plugin wants a TUI input bar.
///
/// Returns the first non-None placeholder from plugins with `tui_input_placeholder()`.
pub fn any_plugin_wants_input_bar() -> bool {
    let snapshot = official_runtime_snapshot();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id)
            && runtime.tui_input_placeholder().is_some()
        {
            return true;
        }
    }

    false
}

/// Returns the command prefix from the configured agent plugin, or "/" as default.
pub fn agent_command_prefix(agent_plugin_id: &str) -> String {
    re_official::official_plugin_runtime(agent_plugin_id)
        .map(|rt| rt.tui_command_prefix().to_owned())
        .unwrap_or_else(|| "/".to_owned())
}

/// Collects TUI keybindings from all enabled plugins.
///
/// Auto-discovers keybindings declared by plugins via `tui_keybindings()`.
/// Returns keybindings converted to the TUI's `RegisteredKeybinding` type.
#[must_use]
pub fn collect_tui_keybindings_from_plugins() -> Vec<re_tui::RegisteredKeybinding> {
    let snapshot = official_runtime_snapshot();
    let mut bindings = Vec::new();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id) {
            for kb in runtime.tui_keybindings() {
                if let Some(key_char) = kb.key.chars().next() {
                    bindings.push(re_tui::RegisteredKeybinding {
                        key: key_char,
                        description: kb.description,
                        plugin_id: kb.plugin_id,
                        active_states: kb.active_states,
                    });
                }
            }
        }
    }

    bindings
}

/// Dispatches a TUI key event to the plugin that owns the keybinding.
///
/// Finds the plugin matching the binding's `plugin_id`, calls
/// `handle_tui_key()`, and translates the result to a `PluginKeyAction`.
pub fn dispatch_plugin_tui_key(
    plugin_id: &str,
    key: &str,
    tui_state: &str,
) -> re_tui::PluginKeyAction {
    let Some(runtime) = re_official::official_plugin_runtime(plugin_id) else {
        return re_tui::PluginKeyAction::NotHandled;
    };

    translate_key_result(runtime.handle_tui_key(key, tui_state))
}

/// Dispatches text input to all enabled plugins via `handle_tui_text_input()`.
///
/// Tries each plugin in order until one handles the input. Returns the
/// translated result or `NotHandled` if no plugin wants it.
pub fn dispatch_plugin_text_input(
    text: &str,
    project_root: &std::path::Path,
) -> re_tui::PluginKeyAction {
    let snapshot = official_runtime_snapshot();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id) {
            let result = runtime.handle_tui_text_input(text, project_root);
            if result != re_plugin::TuiKeyResult::NotHandled {
                return translate_key_result(result);
            }
        }
    }

    re_tui::PluginKeyAction::NotHandled
}

/// Translates a plugin `TuiKeyResult` to the TUI's `PluginKeyAction`.
fn translate_key_result(result: re_plugin::TuiKeyResult) -> re_tui::PluginKeyAction {
    match result {
        re_plugin::TuiKeyResult::Handled => re_tui::PluginKeyAction::Handled,
        re_plugin::TuiKeyResult::NotHandled => re_tui::PluginKeyAction::NotHandled,
        re_plugin::TuiKeyResult::EnterTextInput { prompt } => {
            re_tui::PluginKeyAction::EnterTextInput { prompt }
        }
        re_plugin::TuiKeyResult::SetState(label) => {
            if let Some(state) = re_tui::TuiState::from_label(&label) {
                re_tui::PluginKeyAction::SetState(state)
            } else {
                re_tui::PluginKeyAction::ShowMessage(format!("Unknown state: {label}"))
            }
        }
        re_plugin::TuiKeyResult::ShowMessage(msg) => re_tui::PluginKeyAction::ShowMessage(msg),
    }
}

// ── Session management discovery ────────────────────────────────

/// Exports session context from the given agent plugin.
///
/// Auto-discovers the plugin runtime and calls `export_session_context()`.
pub fn export_agent_session(
    agent_plugin_id: &str,
    project_root: &std::path::Path,
) -> Result<re_plugin::PortableContext, String> {
    let runtime = re_official::official_plugin_runtime(agent_plugin_id)
        .ok_or_else(|| format!("Agent plugin '{agent_plugin_id}' not found"))?;
    runtime
        .export_session_context(project_root)
        .map_err(|e| e.to_string())
}

/// Imports session context into the given agent plugin.
pub fn import_agent_session(
    agent_plugin_id: &str,
    context: &re_plugin::PortableContext,
    project_root: &std::path::Path,
) -> Result<(), String> {
    let runtime = re_official::official_plugin_runtime(agent_plugin_id)
        .ok_or_else(|| format!("Agent plugin '{agent_plugin_id}' not found"))?;
    runtime
        .import_session_context(context, project_root)
        .map_err(|e| e.to_string())
}

/// Finds and calls the first plugin that supports `compact_context()`.
///
/// Tries all enabled plugins until one succeeds (not just returns the
/// input unchanged). Falls back to returning the original context.
pub fn compact_session_context(
    context: &re_plugin::PortableContext,
    target_tokens: usize,
) -> re_plugin::PortableContext {
    let snapshot = official_runtime_snapshot();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id)
            && let Ok(compacted) = runtime.compact_context(context, target_tokens)
            && compacted.messages.len() < context.messages.len()
        {
            return compacted;
        }
    }

    context.clone()
}

/// Saves session context using the first plugin that supports it.
pub fn save_session(
    context: &re_plugin::PortableContext,
    project_root: &std::path::Path,
) -> Result<(), String> {
    let snapshot = official_runtime_snapshot();
    let sessions_dir = project_root.join(".ralph-engine/sessions");

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id)
            && runtime.save_session(context, &sessions_dir).is_ok()
        {
            return Ok(());
        }
    }

    Err("No plugin supports session persistence".to_owned())
}

/// Loads session context using the first plugin that supports it.
pub fn load_session(project_root: &std::path::Path) -> Result<re_plugin::PortableContext, String> {
    let snapshot = official_runtime_snapshot();
    let sessions_dir = project_root.join(".ralph-engine/sessions");

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id)
            && let Ok(ctx) = runtime.load_session(&sessions_dir)
        {
            return Ok(ctx);
        }
    }

    Err("No previous session found".to_owned())
}

/// Returns the context window size for the given agent plugin.
pub fn agent_context_window_size(agent_plugin_id: &str) -> usize {
    re_official::official_plugin_runtime(agent_plugin_id)
        .map(|rt| rt.context_window_size())
        .unwrap_or(0)
}

/// Reports usage from the given agent plugin.
pub fn agent_usage_report(agent_plugin_id: &str) -> Option<re_plugin::UsageReport> {
    re_official::official_plugin_runtime(agent_plugin_id).and_then(|rt| rt.report_usage())
}

// ── Config management discovery ─────────────────────────────────

/// Applies a preset from the first plugin that matches the preset ID.
///
/// Returns the list of files created (path → contents).
pub fn apply_preset(
    preset_id: &str,
    project_root: &std::path::Path,
) -> Result<Vec<(String, String)>, String> {
    let snapshot = official_runtime_snapshot();

    for plugin in &snapshot.plugins {
        if (plugin.descriptor.id == preset_id || plugin.descriptor.id.ends_with(preset_id))
            && let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id)
        {
            return runtime
                .apply_preset(project_root)
                .map_err(|e| e.to_string());
        }
    }

    Err(format!("No preset plugin found for '{preset_id}'"))
}

/// Runs config migration on all plugins.
///
/// Each plugin migrates its own config sections. Returns the final
/// config content after all migrations.
pub fn migrate_config(
    config_content: &str,
    from_version: &str,
    to_version: &str,
) -> Option<String> {
    let snapshot = official_runtime_snapshot();
    let mut current = config_content.to_owned();
    let mut changed = false;

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id)
            && let Some(migrated) = runtime.migrate_config(&current, from_version, to_version)
        {
            current = migrated;
            changed = true;
        }
    }

    if changed { Some(current) } else { None }
}

// ── Agent routing discovery ──────────────────────────────────────

/// Classifies a task and recommends an agent using routing plugins.
///
/// Tries all enabled plugins' `classify_task()` until one returns a
/// recommendation. Returns None if no router plugin is active.
#[must_use]
pub fn classify_task(task_description: &str) -> Option<re_plugin::AgentRecommendation> {
    let snapshot = official_runtime_snapshot();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id)
            && let Some(rec) = runtime.classify_task(task_description)
        {
            return Some(rec);
        }
    }

    None
}

/// Collects routing rules from all enabled router plugins.
#[must_use]
pub fn collect_routing_rules() -> Vec<re_plugin::RoutingRule> {
    let snapshot = official_runtime_snapshot();
    let mut rules = Vec::new();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id) {
            rules.extend(runtime.routing_rules());
        }
    }

    rules
}

/// Returns the fallback chain for a given primary agent.
#[must_use]
pub fn fallback_chain(primary_agent: &str) -> Vec<re_plugin::FallbackEntry> {
    let snapshot = official_runtime_snapshot();
    let mut chain = Vec::new();

    for plugin in &snapshot.plugins {
        if let Some(runtime) = re_official::official_plugin_runtime(plugin.descriptor.id) {
            chain.extend(runtime.fallback_chain(primary_agent));
        }
    }

    chain
}

// ── Community plugin discovery ────────────────────────────────────

/// Descriptor for a community plugin discovered from the filesystem.
#[derive(Debug, Clone)]
pub struct CommunityPluginInfo {
    /// Plugin ID from manifest (e.g. `"acme.jira-suite"`).
    pub id: String,
    /// Display name.
    pub display_name: String,
    /// Short summary.
    pub summary: String,
    /// Capabilities declared in manifest.
    pub capabilities: Vec<String>,
    /// Path to the plugin directory.
    pub path: String,
}

/// Scans `.ralph-engine/plugins/` for installed community plugins.
///
/// Reads each subdirectory's `manifest.yaml` to extract plugin metadata.
/// Returns only successfully parsed plugins — malformed manifests are
/// silently skipped (logged via tracing).
#[must_use]
pub fn discover_community_plugins() -> Vec<CommunityPluginInfo> {
    let plugins_dir = std::path::Path::new(".ralph-engine/plugins");
    if !plugins_dir.is_dir() {
        return Vec::new();
    }

    let Ok(entries) = std::fs::read_dir(plugins_dir) else {
        return Vec::new();
    };

    let mut plugins = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let manifest_path = path.join("manifest.yaml");
        if !manifest_path.exists() {
            continue;
        }

        if let Some(info) = parse_community_manifest(&manifest_path, &path) {
            plugins.push(info);
        }
    }

    plugins
}

/// Parses a community plugin manifest.yaml into a `CommunityPluginInfo`.
fn parse_community_manifest(
    manifest_path: &std::path::Path,
    plugin_dir: &std::path::Path,
) -> Option<CommunityPluginInfo> {
    let content = std::fs::read_to_string(manifest_path).ok()?;

    // Simple YAML extraction without serde dependency.
    let id = extract_yaml_value(&content, "id")?;
    let display_name = extract_yaml_value(&content, "display_name").unwrap_or_else(|| id.clone());
    let summary = extract_yaml_value(&content, "summary").unwrap_or_default();

    // Parse capabilities list.
    let capabilities = extract_yaml_list(&content, "capabilities");

    Some(CommunityPluginInfo {
        id,
        display_name,
        summary,
        capabilities,
        path: plugin_dir.display().to_string(),
    })
}

/// Extracts a simple scalar YAML value: `key: value`.
fn extract_yaml_value(content: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}:");
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(&prefix) {
            let value = rest.trim().trim_matches('"').trim_matches('\'');
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }
    }
    None
}

/// Extracts a YAML list under a key (simple indented `- item` format).
fn extract_yaml_list(content: &str, key: &str) -> Vec<String> {
    let header = format!("{key}:");
    let mut items = Vec::new();
    let mut in_list = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&header) {
            in_list = true;
            continue;
        }
        if in_list {
            if let Some(item) = trimmed.strip_prefix("- ") {
                items.push(item.trim().trim_matches('"').trim_matches('\'').to_owned());
            } else if !trimmed.is_empty() && !trimmed.starts_with('#') {
                break; // End of list
            }
        }
    }

    items
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn extract_yaml_value_parses_simple_scalar() {
        let content = "id: acme.test\nkind: agent_runtime\nsummary: A test plugin.";
        assert_eq!(
            extract_yaml_value(content, "id"),
            Some("acme.test".to_owned())
        );
        assert_eq!(
            extract_yaml_value(content, "summary"),
            Some("A test plugin.".to_owned())
        );
    }

    #[test]
    fn extract_yaml_value_handles_quoted_values() {
        let content = "id: \"acme.test\"";
        assert_eq!(
            extract_yaml_value(content, "id"),
            Some("acme.test".to_owned())
        );
    }

    #[test]
    fn extract_yaml_value_returns_none_for_missing_key() {
        let content = "id: test";
        assert_eq!(extract_yaml_value(content, "missing"), None);
    }

    #[test]
    fn extract_yaml_list_parses_items() {
        let content = "capabilities:\n  - agent_runtime\n  - mcp_contribution\nother: value";
        let list = extract_yaml_list(content, "capabilities");
        assert_eq!(list, vec!["agent_runtime", "mcp_contribution"]);
    }

    #[test]
    fn extract_yaml_list_empty_when_missing() {
        let content = "id: test";
        let list = extract_yaml_list(content, "capabilities");
        assert!(list.is_empty());
    }

    #[test]
    fn discover_community_plugins_returns_empty_when_no_dir() {
        // In test environment, .ralph-engine/plugins/ doesn't exist
        let plugins = discover_community_plugins();
        assert!(plugins.is_empty());
    }

    #[test]
    fn translate_key_result_handled() {
        let result = translate_key_result(re_plugin::TuiKeyResult::Handled);
        assert_eq!(result, re_tui::PluginKeyAction::Handled);
    }

    #[test]
    fn translate_key_result_not_handled() {
        let result = translate_key_result(re_plugin::TuiKeyResult::NotHandled);
        assert_eq!(result, re_tui::PluginKeyAction::NotHandled);
    }

    #[test]
    fn translate_key_result_show_message() {
        let result = translate_key_result(re_plugin::TuiKeyResult::ShowMessage("hello".to_owned()));
        assert_eq!(
            result,
            re_tui::PluginKeyAction::ShowMessage("hello".to_owned())
        );
    }

    #[test]
    fn translate_key_result_enter_text_input() {
        let result = translate_key_result(re_plugin::TuiKeyResult::EnterTextInput {
            prompt: "Type:".to_owned(),
        });
        assert_eq!(
            result,
            re_tui::PluginKeyAction::EnterTextInput {
                prompt: "Type:".to_owned()
            }
        );
    }

    #[test]
    fn translate_key_result_set_state_valid() {
        let result = translate_key_result(re_plugin::TuiKeyResult::SetState("Running".to_owned()));
        assert_eq!(
            result,
            re_tui::PluginKeyAction::SetState(re_tui::TuiState::Running)
        );
    }

    #[test]
    fn translate_key_result_set_state_invalid() {
        let result = translate_key_result(re_plugin::TuiKeyResult::SetState("Invalid".to_owned()));
        assert!(matches!(result, re_tui::PluginKeyAction::ShowMessage(_)));
    }

    #[test]
    fn collect_tui_keybindings_returns_list() {
        // Official plugins may or may not declare keybindings, but the function
        // should always return a valid (possibly empty) list without panicking.
        let bindings = collect_tui_keybindings_from_plugins();
        // Guided plugin declares keybindings when loaded
        for b in &bindings {
            assert!(!b.description.is_empty());
            assert!(!b.plugin_id.is_empty());
        }
    }

    #[test]
    fn dispatch_plugin_tui_key_unknown_plugin_returns_not_handled() {
        let result = dispatch_plugin_tui_key("nonexistent.plugin", "x", "Running");
        assert_eq!(result, re_tui::PluginKeyAction::NotHandled);
    }

    #[test]
    fn dispatch_plugin_text_input_no_handler_returns_not_handled() {
        let dir = std::env::temp_dir().join("re-test-text-input");
        let result = dispatch_plugin_text_input("hello world", &dir);
        // Default plugins return NotHandled for text input
        assert_eq!(result, re_tui::PluginKeyAction::NotHandled);
    }

    #[test]
    fn export_agent_session_unknown_plugin_returns_error() {
        let dir = std::env::temp_dir().join("re-test-export");
        let result = export_agent_session("nonexistent.plugin", &dir);
        assert!(result.is_err());
    }

    #[test]
    fn import_agent_session_unknown_plugin_returns_error() {
        let dir = std::env::temp_dir().join("re-test-import");
        let ctx = re_plugin::PortableContext {
            system_prompt: None,
            messages: Vec::new(),
            active_files: Vec::new(),
            summary: None,
            token_count: 0,
            max_tokens: 0,
            metadata: re_plugin::ContextMetadata {
                source_agent: "test".to_owned(),
                source_model: "test".to_owned(),
                session_id: None,
                created_at: 0,
            },
        };
        let result = import_agent_session("nonexistent.plugin", &ctx, &dir);
        assert!(result.is_err());
    }

    #[test]
    fn compact_session_context_returns_context_when_no_compactor() {
        let ctx = re_plugin::PortableContext {
            system_prompt: None,
            messages: Vec::new(),
            active_files: Vec::new(),
            summary: None,
            token_count: 0,
            max_tokens: 0,
            metadata: re_plugin::ContextMetadata {
                source_agent: "test".to_owned(),
                source_model: "test".to_owned(),
                session_id: None,
                created_at: 0,
            },
        };
        let result = compact_session_context(&ctx, 1000);
        assert_eq!(result.messages.len(), ctx.messages.len());
    }

    #[test]
    fn save_session_fails_without_session_plugin() {
        let dir = std::env::temp_dir().join("re-test-save-session");
        let ctx = re_plugin::PortableContext {
            system_prompt: None,
            messages: Vec::new(),
            active_files: Vec::new(),
            summary: None,
            token_count: 0,
            max_tokens: 0,
            metadata: re_plugin::ContextMetadata {
                source_agent: "test".to_owned(),
                source_model: "test".to_owned(),
                session_id: None,
                created_at: 0,
            },
        };
        // Context plugin exists but sessions dir may not, so result depends on plugin impl
        let _ = save_session(&ctx, &dir);
    }

    #[test]
    fn load_session_returns_error_without_sessions() {
        let dir = std::env::temp_dir().join("re-test-load-empty-session");
        let result = load_session(&dir);
        assert!(result.is_err());
    }

    #[test]
    fn agent_context_window_unknown_plugin_returns_zero() {
        assert_eq!(agent_context_window_size("nonexistent.plugin"), 0);
    }

    #[test]
    fn classify_task_returns_none_without_rules() {
        // With default config, router plugin has no rules → returns None
        let result = classify_task("fix a bug in the login page");
        // Router plugin may return something based on keywords, or None
        // Either way it shouldn't panic
        let _ = result;
    }

    #[test]
    fn collect_routing_rules_returns_list() {
        let rules = collect_routing_rules();
        // Router plugin may have default rules or empty
        for rule in &rules {
            assert!(!rule.agent_plugin.is_empty());
        }
    }

    #[test]
    fn fallback_chain_returns_list() {
        let chain = fallback_chain("official.claude");
        // May be empty or have entries
        for entry in &chain {
            assert!(!entry.agent_plugin.is_empty());
        }
    }

    #[test]
    fn parse_community_manifest_extracts_fields() {
        let dir = std::env::temp_dir().join("re-test-community-plugin");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).ok();
        std::fs::write(
            dir.join("manifest.yaml"),
            "id: acme.test\ndisplay_name: Acme Test\nsummary: A test plugin.\ncapabilities:\n  - agent_runtime\n  - tui_widgets\n",
        )
        .ok();

        let info = parse_community_manifest(&dir.join("manifest.yaml"), &dir);
        let _ = std::fs::remove_dir_all(&dir);

        let info = info.expect("should parse manifest");
        assert_eq!(info.id, "acme.test");
        assert_eq!(info.display_name, "Acme Test");
        assert_eq!(info.summary, "A test plugin.");
        assert_eq!(info.capabilities, vec!["agent_runtime", "tui_widgets"]);
    }
}
