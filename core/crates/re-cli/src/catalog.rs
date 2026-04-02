//! Immutable built-in catalog for official plugins, MCP contributions, and runtime topology.

use re_config::{
    ConfigScope, PluginActivation, ResolvedPluginConfig, canonical_config_layers,
    default_project_config_layer, resolve_plugin_config,
};
use re_core::{
    RuntimeAgentRegistration, RuntimeCapabilityRegistration, RuntimeCheckKind,
    RuntimeCheckRegistration, RuntimeHookRegistration, RuntimeMcpRegistration, RuntimePhase,
    RuntimePluginRegistration, RuntimePolicyRegistration, RuntimePromptRegistration,
    RuntimeProviderKind, RuntimeProviderRegistration, RuntimeTemplateRegistration, RuntimeTopology,
};
use re_mcp::McpServerDescriptor;
use re_plugin::{
    AGENT_RUNTIME, CONTEXT_PROVIDER, DATA_SOURCE, DOCTOR_CHECKS, FORGE_PROVIDER, MCP_CONTRIBUTION,
    POLICY, PREPARE_CHECKS, PROMPT_FRAGMENTS, PluginCapability, PluginDescriptor,
    PluginRuntimeHook, REMOTE_CONTROL, TEMPLATE,
};

/// Immutable owned snapshot of the official runtime catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OfficialRuntimeSnapshot {
    /// Resolved official plugin registrations.
    pub plugins: [RuntimePluginRegistration; 8],
    /// Resolved official capability registrations.
    pub capabilities: Vec<RuntimeCapabilityRegistration>,
    /// Resolved official template registrations.
    pub templates: Vec<RuntimeTemplateRegistration>,
    /// Resolved official prompt registrations.
    pub prompts: Vec<RuntimePromptRegistration>,
    /// Resolved official agent runtime registrations.
    pub agents: Vec<RuntimeAgentRegistration>,
    /// Resolved official runtime check registrations.
    pub checks: Vec<RuntimeCheckRegistration>,
    /// Resolved official provider registrations.
    pub providers: Vec<RuntimeProviderRegistration>,
    /// Resolved official policy registrations.
    pub policies: Vec<RuntimePolicyRegistration>,
    /// Resolved official runtime-hook registrations.
    pub hooks: Vec<RuntimeHookRegistration>,
    /// Resolved official MCP registrations.
    pub mcp_servers: [RuntimeMcpRegistration; 4],
}

impl OfficialRuntimeSnapshot {
    /// Returns the borrowed runtime topology view for this snapshot.
    #[must_use]
    pub fn topology(&self) -> RuntimeTopology<'_> {
        RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: default_project_config_layer().config.default_locale,
            plugins: &self.plugins,
            capabilities: &self.capabilities,
            templates: &self.templates,
            prompts: &self.prompts,
            agents: &self.agents,
            checks: &self.checks,
            providers: &self.providers,
            policies: &self.policies,
            hooks: &self.hooks,
            mcp_servers: &self.mcp_servers,
        }
    }
}

/// Returns the immutable catalog of official plugins.
#[must_use]
pub fn official_plugins() -> [PluginDescriptor; 8] {
    [
        re_plugin_basic::descriptor(),
        re_plugin_bmad::descriptor(),
        re_plugin_claude::descriptor(),
        re_plugin_claudebox::descriptor(),
        re_plugin_codex::descriptor(),
        re_plugin_github::descriptor(),
        re_plugin_ssh::descriptor(),
        re_plugin_tdd_strict::descriptor(),
    ]
}

/// Returns one immutable official plugin descriptor by identifier.
#[must_use]
pub fn find_official_plugin(plugin_id: &str) -> Option<PluginDescriptor> {
    official_plugins()
        .into_iter()
        .find(|plugin| plugin.id == plugin_id)
}

/// Returns the immutable catalog of official MCP server contributions.
#[must_use]
pub fn official_mcp_servers() -> [McpServerDescriptor; 4] {
    [
        re_plugin_claude::mcp_servers()[0],
        re_plugin_claudebox::mcp_servers()[0],
        re_plugin_codex::mcp_servers()[0],
        re_plugin_github::mcp_servers()[0],
    ]
}

/// Returns one immutable official MCP server descriptor by identifier.
#[must_use]
pub fn find_official_mcp_server(server_id: &str) -> Option<McpServerDescriptor> {
    official_mcp_servers()
        .into_iter()
        .find(|server| server.id == server_id)
}

fn resolved_plugin_entry(plugin: PluginDescriptor) -> ResolvedPluginConfig {
    resolve_plugin_config(canonical_config_layers(), plugin.id).unwrap_or(
        ResolvedPluginConfig::new(
            plugin.id,
            PluginActivation::Disabled,
            ConfigScope::BuiltInDefaults,
        ),
    )
}

fn resolved_plugin_entry_by_id(plugin_id: &'static str) -> ResolvedPluginConfig {
    find_official_plugin(plugin_id)
        .map(resolved_plugin_entry)
        .unwrap_or(ResolvedPluginConfig::new(
            plugin_id,
            PluginActivation::Disabled,
            ConfigScope::BuiltInDefaults,
        ))
}

/// Returns the resolved runtime plugin registrations for the official catalog.
#[must_use]
pub fn official_runtime_plugins() -> [RuntimePluginRegistration; 8] {
    let plugins = official_plugins();

    plugins.map(|plugin| {
        let resolved = resolved_plugin_entry(plugin);

        RuntimePluginRegistration::new(plugin, resolved.activation, resolved.resolved_from)
    })
}

/// Returns the resolved runtime MCP registrations for the official catalog.
#[must_use]
pub fn official_runtime_mcp_registrations() -> [RuntimeMcpRegistration; 4] {
    let servers = official_mcp_servers();

    servers.map(|server| {
        let resolved = resolved_plugin_entry_by_id(server.plugin_id);
        let enabled = matches!(resolved.activation, PluginActivation::Enabled);

        RuntimeMcpRegistration::new(server, enabled)
    })
}

/// Returns the resolved runtime capability registrations for the official catalog.
#[must_use]
pub fn official_runtime_capabilities() -> Vec<RuntimeCapabilityRegistration> {
    official_runtime_plugins()
        .into_iter()
        .flat_map(|plugin| {
            plugin
                .descriptor
                .capabilities
                .iter()
                .copied()
                .map(move |capability| {
                    RuntimeCapabilityRegistration::new(
                        capability,
                        plugin.descriptor.id,
                        plugin.activation,
                        plugin.descriptor.load_boundary,
                    )
                })
        })
        .collect()
}

/// Returns the resolved template registrations for the official catalog.
#[must_use]
pub fn official_runtime_templates() -> Vec<RuntimeTemplateRegistration> {
    official_runtime_plugins()
        .into_iter()
        .filter(|plugin| plugin.descriptor.capabilities.contains(&TEMPLATE))
        .map(|plugin| {
            RuntimeTemplateRegistration::new(
                plugin.descriptor.id,
                plugin.activation,
                plugin.descriptor.load_boundary,
                plugin
                    .descriptor
                    .runtime_hooks
                    .contains(&PluginRuntimeHook::Scaffold),
            )
        })
        .collect()
}

/// Returns the resolved prompt registrations for the official catalog.
#[must_use]
pub fn official_runtime_prompts() -> Vec<RuntimePromptRegistration> {
    official_runtime_plugins()
        .into_iter()
        .filter(|plugin| plugin.descriptor.capabilities.contains(&PROMPT_FRAGMENTS))
        .map(|plugin| {
            RuntimePromptRegistration::new(
                plugin.descriptor.id,
                plugin.activation,
                plugin.descriptor.load_boundary,
                plugin
                    .descriptor
                    .runtime_hooks
                    .contains(&PluginRuntimeHook::PromptAssembly),
            )
        })
        .collect()
}

/// Returns the resolved agent runtime registrations for the official catalog.
#[must_use]
pub fn official_runtime_agents() -> Vec<RuntimeAgentRegistration> {
    official_runtime_plugins()
        .into_iter()
        .filter(|plugin| plugin.descriptor.capabilities.contains(&AGENT_RUNTIME))
        .map(|plugin| {
            RuntimeAgentRegistration::new(
                plugin.descriptor.id,
                plugin.activation,
                plugin.descriptor.load_boundary,
                plugin
                    .descriptor
                    .runtime_hooks
                    .contains(&PluginRuntimeHook::AgentBootstrap),
            )
        })
        .collect()
}

/// Returns the resolved runtime-hook registrations for the official catalog.
#[must_use]
pub fn official_runtime_hooks() -> Vec<RuntimeHookRegistration> {
    official_runtime_plugins()
        .into_iter()
        .flat_map(|plugin| {
            plugin
                .descriptor
                .runtime_hooks
                .iter()
                .copied()
                .map(move |hook| {
                    RuntimeHookRegistration::new(
                        hook,
                        plugin.descriptor.id,
                        plugin.activation,
                        plugin.descriptor.load_boundary,
                    )
                })
        })
        .collect()
}

fn check_kind_for_capability(capability: PluginCapability) -> Option<RuntimeCheckKind> {
    match capability {
        PREPARE_CHECKS => Some(RuntimeCheckKind::Prepare),
        DOCTOR_CHECKS => Some(RuntimeCheckKind::Doctor),
        _ => None,
    }
}

/// Returns the dedicated runtime surface that owns one reviewed capability.
#[must_use]
#[cfg_attr(not(test), allow(dead_code))]
pub fn dedicated_runtime_surface_for_capability(
    capability: PluginCapability,
) -> Option<&'static str> {
    match capability {
        TEMPLATE => Some("templates"),
        PROMPT_FRAGMENTS => Some("prompts"),
        PREPARE_CHECKS | DOCTOR_CHECKS => Some("checks"),
        AGENT_RUNTIME => Some("agents"),
        MCP_CONTRIBUTION => Some("mcp"),
        DATA_SOURCE | CONTEXT_PROVIDER | FORGE_PROVIDER | REMOTE_CONTROL => Some("providers"),
        POLICY => Some("policies"),
        _ => None,
    }
}

fn runtime_hook_for_check(kind: RuntimeCheckKind) -> PluginRuntimeHook {
    match kind {
        RuntimeCheckKind::Prepare => PluginRuntimeHook::Prepare,
        RuntimeCheckKind::Doctor => PluginRuntimeHook::Doctor,
    }
}

/// Returns the resolved runtime check registrations for the official catalog.
#[must_use]
pub fn official_runtime_checks() -> Vec<RuntimeCheckRegistration> {
    official_runtime_plugins()
        .into_iter()
        .flat_map(|plugin| {
            plugin
                .descriptor
                .capabilities
                .iter()
                .copied()
                .filter_map(move |capability| {
                    check_kind_for_capability(capability).map(|kind| {
                        RuntimeCheckRegistration::new(
                            kind,
                            plugin.descriptor.id,
                            plugin.activation,
                            plugin.descriptor.load_boundary,
                            plugin
                                .descriptor
                                .runtime_hooks
                                .contains(&runtime_hook_for_check(kind)),
                        )
                    })
                })
        })
        .collect()
}

fn provider_kind_for_capability(capability: PluginCapability) -> Option<RuntimeProviderKind> {
    match capability {
        DATA_SOURCE => Some(RuntimeProviderKind::DataSource),
        CONTEXT_PROVIDER => Some(RuntimeProviderKind::ContextProvider),
        FORGE_PROVIDER => Some(RuntimeProviderKind::ForgeProvider),
        REMOTE_CONTROL => Some(RuntimeProviderKind::RemoteControl),
        _ => None,
    }
}

fn registration_hook_for_provider(kind: RuntimeProviderKind) -> PluginRuntimeHook {
    match kind {
        RuntimeProviderKind::DataSource => PluginRuntimeHook::DataSourceRegistration,
        RuntimeProviderKind::ContextProvider => PluginRuntimeHook::ContextProviderRegistration,
        RuntimeProviderKind::ForgeProvider => PluginRuntimeHook::ForgeProviderRegistration,
        RuntimeProviderKind::RemoteControl => PluginRuntimeHook::RemoteControlBootstrap,
    }
}

/// Returns the resolved runtime provider registrations for the official catalog.
#[must_use]
pub fn official_runtime_providers() -> Vec<RuntimeProviderRegistration> {
    official_runtime_plugins()
        .into_iter()
        .flat_map(|plugin| {
            plugin
                .descriptor
                .capabilities
                .iter()
                .copied()
                .filter_map(move |capability| {
                    provider_kind_for_capability(capability).map(|kind| {
                        RuntimeProviderRegistration::new(
                            kind,
                            plugin.descriptor.id,
                            plugin.activation,
                            plugin.descriptor.load_boundary,
                            plugin
                                .descriptor
                                .runtime_hooks
                                .contains(&registration_hook_for_provider(kind)),
                        )
                    })
                })
        })
        .collect()
}

/// Returns the resolved runtime policy registrations for the official catalog.
#[must_use]
pub fn official_runtime_policies() -> Vec<RuntimePolicyRegistration> {
    official_runtime_plugins()
        .into_iter()
        .filter(|plugin| plugin.descriptor.capabilities.contains(&POLICY))
        .map(|plugin| {
            RuntimePolicyRegistration::new(
                plugin.descriptor.id,
                plugin.descriptor.id,
                plugin.activation,
                plugin.descriptor.load_boundary,
                plugin
                    .descriptor
                    .runtime_hooks
                    .contains(&PluginRuntimeHook::PolicyEnforcement),
            )
        })
        .collect()
}

/// Returns one immutable owned snapshot of the official runtime catalog.
#[must_use]
pub fn official_runtime_snapshot() -> OfficialRuntimeSnapshot {
    let plugins = official_runtime_plugins();
    let capabilities = official_runtime_capabilities();
    let templates = official_runtime_templates();
    let prompts = official_runtime_prompts();
    let agents = official_runtime_agents();
    let checks = official_runtime_checks();
    let providers = official_runtime_providers();
    let policies = official_runtime_policies();
    let hooks = official_runtime_hooks();
    let mcp_servers = official_runtime_mcp_registrations();

    OfficialRuntimeSnapshot {
        plugins,
        capabilities,
        templates,
        prompts,
        agents,
        checks,
        providers,
        policies,
        hooks,
        mcp_servers,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        dedicated_runtime_surface_for_capability, find_official_mcp_server, find_official_plugin,
        official_plugins, official_runtime_agents, official_runtime_checks,
        official_runtime_mcp_registrations, official_runtime_policies, official_runtime_prompts,
        official_runtime_providers, official_runtime_snapshot, official_runtime_templates,
    };
    use re_plugin::{ALL_PLUGIN_CAPABILITIES, PluginCapability};

    fn capability_names(capabilities: &[PluginCapability]) -> Vec<&'static str> {
        capabilities
            .iter()
            .map(|capability| capability.as_str())
            .collect()
    }

    #[test]
    fn all_reviewed_capabilities_have_dedicated_runtime_surfaces() {
        let uncovered = capability_names(ALL_PLUGIN_CAPABILITIES)
            .into_iter()
            .filter(|capability| {
                dedicated_runtime_surface_for_capability(PluginCapability::new(capability))
                    .is_none()
            })
            .collect::<Vec<_>>();

        assert_eq!(uncovered, Vec::<&'static str>::new());
    }

    #[test]
    fn official_plugin_capabilities_are_covered_by_runtime_surfaces() {
        for plugin in official_plugins() {
            for capability in plugin.capabilities {
                assert!(dedicated_runtime_surface_for_capability(*capability).is_some());
            }
        }
    }

    #[test]
    fn dedicated_runtime_surfaces_cover_official_catalog() {
        let surface_sizes = [
            ("templates", official_runtime_templates().len()),
            ("prompts", official_runtime_prompts().len()),
            ("agents", official_runtime_agents().len()),
            ("checks", official_runtime_checks().len()),
            ("providers", official_runtime_providers().len()),
            ("policies", official_runtime_policies().len()),
            ("mcp", official_runtime_mcp_registrations().len()),
        ];

        for (surface, size) in surface_sizes {
            let _ = surface;
            assert!(size > 0);
        }
    }

    #[test]
    fn unknown_capabilities_do_not_map_to_runtime_surfaces() {
        let unknown = PluginCapability::new("unknown_surface");

        assert_eq!(dedicated_runtime_surface_for_capability(unknown), None);
        assert_eq!(super::check_kind_for_capability(unknown), None);
        assert_eq!(super::provider_kind_for_capability(unknown), None);
    }

    #[test]
    fn unknown_plugin_and_mcp_identifiers_are_rejected() {
        assert_eq!(find_official_plugin("official.missing"), None);
        assert_eq!(find_official_mcp_server("official.missing"), None);
    }

    #[test]
    fn runtime_snapshot_topology_stays_coherent() {
        let snapshot = official_runtime_snapshot();
        let topology = snapshot.topology();

        assert_eq!(topology.plugins.len(), snapshot.plugins.len());
        assert_eq!(topology.capabilities.len(), snapshot.capabilities.len());
        assert_eq!(topology.templates.len(), snapshot.templates.len());
        assert_eq!(topology.prompts.len(), snapshot.prompts.len());
        assert_eq!(topology.agents.len(), snapshot.agents.len());
        assert_eq!(topology.checks.len(), snapshot.checks.len());
        assert_eq!(topology.providers.len(), snapshot.providers.len());
        assert_eq!(topology.policies.len(), snapshot.policies.len());
        assert_eq!(topology.hooks.len(), snapshot.hooks.len());
        assert_eq!(topology.mcp_servers.len(), snapshot.mcp_servers.len());
    }
}
