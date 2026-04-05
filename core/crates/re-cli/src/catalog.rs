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
