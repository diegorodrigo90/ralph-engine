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
