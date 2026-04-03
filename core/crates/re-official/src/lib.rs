//! Immutable built-in catalog for official plugins, MCP contributions, and runtime topology.

use re_config::{
    ConfigScope, PluginActivation, ResolvedPluginConfig, canonical_config_layers,
    default_project_config, default_project_config_layer, resolve_mcp_server_config,
    resolve_plugin_config,
};
use re_core::{
    RuntimeAgentRegistration, RuntimeCapabilityRegistration, RuntimeCheckKind,
    RuntimeCheckRegistration, RuntimeHookRegistration, RuntimeMcpRegistration, RuntimePhase,
    RuntimePluginRegistration, RuntimePolicyRegistration, RuntimePromptRegistration,
    RuntimeProviderKind, RuntimeProviderRegistration, RuntimeTemplateRegistration, RuntimeTopology,
    agent_runtime_hook, capability_activates_agent_surface, capability_activates_prompt_surface,
    capability_activates_template_surface, policy_runtime_hook, prompt_runtime_hook,
    runtime_hook_for_check, runtime_hook_for_provider, template_runtime_hook,
};
use re_mcp::{McpAvailability, McpServerDescriptor};
use re_plugin::{
    PluginAgentDescriptor, PluginCheckDescriptor, PluginCheckKind, PluginDescriptor,
    PluginPolicyDescriptor, PluginPromptDescriptor, PluginProviderDescriptor, PluginProviderKind,
    PluginRuntimeHook, PluginTemplateDescriptor,
};

/// One resolved official template contribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfficialTemplateContribution {
    /// Immutable template descriptor.
    pub descriptor: PluginTemplateDescriptor,
    /// Effective activation state for the owning plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the owning plugin.
    pub load_boundary: re_plugin::PluginLoadBoundary,
    /// Whether the owning plugin declares the scaffold hook.
    pub scaffold_hook_registered: bool,
}

/// One resolved official prompt contribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfficialPromptContribution {
    /// Immutable prompt descriptor.
    pub descriptor: PluginPromptDescriptor,
    /// Effective activation state for the owning plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the owning plugin.
    pub load_boundary: re_plugin::PluginLoadBoundary,
    /// Whether the owning plugin declares the prompt-assembly hook.
    pub prompt_hook_registered: bool,
}

/// One resolved official agent runtime contribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfficialAgentContribution {
    /// Immutable agent descriptor.
    pub descriptor: PluginAgentDescriptor,
    /// Effective activation state for the owning plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the owning plugin.
    pub load_boundary: re_plugin::PluginLoadBoundary,
    /// Whether the owning plugin declares the bootstrap hook.
    pub bootstrap_hook_registered: bool,
}

/// One resolved official policy contribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfficialPolicyContribution {
    /// Immutable policy descriptor.
    pub descriptor: PluginPolicyDescriptor,
    /// Effective activation state for the owning plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the owning plugin.
    pub load_boundary: re_plugin::PluginLoadBoundary,
    /// Whether the owning plugin declares the policy-enforcement hook.
    pub enforcement_hook_registered: bool,
}

/// One resolved official check contribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfficialCheckContribution {
    /// Immutable check descriptor.
    pub descriptor: PluginCheckDescriptor,
    /// Effective activation state for the owning plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the owning plugin.
    pub load_boundary: re_plugin::PluginLoadBoundary,
    /// Whether the owning plugin declares the matching runtime hook.
    pub runtime_hook_registered: bool,
}

/// One resolved official provider contribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfficialProviderContribution {
    /// Immutable provider descriptor.
    pub descriptor: PluginProviderDescriptor,
    /// Effective activation state for the owning plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the owning plugin.
    pub load_boundary: re_plugin::PluginLoadBoundary,
    /// Whether the owning plugin declares the matching runtime hook.
    pub registration_hook_registered: bool,
}

/// One resolved official check surface with both contract and runtime registration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfficialResolvedCheckSurface {
    /// Immutable check descriptor and plugin-owned metadata.
    pub contribution: OfficialCheckContribution,
    /// Runtime registration derived from the resolved official topology.
    pub registration: RuntimeCheckRegistration,
}

/// One resolved official provider surface with both contract and runtime registration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfficialResolvedProviderSurface {
    /// Immutable provider descriptor and plugin-owned metadata.
    pub contribution: OfficialProviderContribution,
    /// Runtime registration derived from the resolved official topology.
    pub registration: RuntimeProviderRegistration,
}

/// Immutable owned snapshot of the official runtime catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OfficialRuntimeSnapshot {
    /// Resolved official plugin registrations.
    pub plugins: Vec<RuntimePluginRegistration>,
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
    pub mcp_servers: Vec<RuntimeMcpRegistration>,
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

#[derive(Clone, Copy)]
struct OfficialPluginBundle {
    descriptor: PluginDescriptor,
    templates: &'static [PluginTemplateDescriptor],
    prompts: &'static [PluginPromptDescriptor],
    agents: &'static [PluginAgentDescriptor],
    checks: &'static [PluginCheckDescriptor],
    providers: &'static [PluginProviderDescriptor],
    policies: &'static [PluginPolicyDescriptor],
    mcp_servers: &'static [McpServerDescriptor],
}

#[derive(Clone, Copy)]
struct ResolvedOfficialPluginBundle {
    plugin: RuntimePluginRegistration,
    bundle: OfficialPluginBundle,
}

#[allow(clippy::too_many_arguments)]
const fn official_plugin_bundle(
    descriptor: PluginDescriptor,
    templates: &'static [PluginTemplateDescriptor],
    prompts: &'static [PluginPromptDescriptor],
    agents: &'static [PluginAgentDescriptor],
    checks: &'static [PluginCheckDescriptor],
    providers: &'static [PluginProviderDescriptor],
    policies: &'static [PluginPolicyDescriptor],
    mcp_servers: &'static [McpServerDescriptor],
) -> OfficialPluginBundle {
    OfficialPluginBundle {
        descriptor,
        templates,
        prompts,
        agents,
        checks,
        providers,
        policies,
        mcp_servers,
    }
}

fn runtime_check_kind_for_descriptor(kind: PluginCheckKind) -> RuntimeCheckKind {
    match kind {
        PluginCheckKind::Prepare => RuntimeCheckKind::Prepare,
        PluginCheckKind::Doctor => RuntimeCheckKind::Doctor,
        _ => unreachable!("unexpected PluginCheckKind variant"),
    }
}

fn runtime_provider_kind_for_descriptor(kind: PluginProviderKind) -> RuntimeProviderKind {
    match kind {
        PluginProviderKind::DataSource => RuntimeProviderKind::DataSource,
        PluginProviderKind::ContextProvider => RuntimeProviderKind::ContextProvider,
        PluginProviderKind::ForgeProvider => RuntimeProviderKind::ForgeProvider,
        PluginProviderKind::RemoteControl => RuntimeProviderKind::RemoteControl,
        _ => unreachable!("unexpected PluginProviderKind variant"),
    }
}

fn official_plugin_bundles() -> Vec<OfficialPluginBundle> {
    vec![
        official_plugin_bundle(
            re_plugin_basic::descriptor(),
            re_plugin_basic::templates(),
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        ),
        official_plugin_bundle(
            re_plugin_bmad::descriptor(),
            re_plugin_bmad::templates(),
            re_plugin_bmad::prompts(),
            &[],
            re_plugin_bmad::checks(),
            &[],
            &[],
            &[],
        ),
        official_plugin_bundle(
            re_plugin_claude::descriptor(),
            &[],
            &[],
            re_plugin_claude::agents(),
            &[],
            &[],
            &[],
            re_plugin_claude::mcp_servers(),
        ),
        official_plugin_bundle(
            re_plugin_claudebox::descriptor(),
            &[],
            &[],
            re_plugin_claudebox::agents(),
            &[],
            &[],
            &[],
            re_plugin_claudebox::mcp_servers(),
        ),
        official_plugin_bundle(
            re_plugin_codex::descriptor(),
            &[],
            &[],
            re_plugin_codex::agents(),
            &[],
            &[],
            &[],
            re_plugin_codex::mcp_servers(),
        ),
        official_plugin_bundle(
            re_plugin_github::descriptor(),
            &[],
            &[],
            &[],
            &[],
            re_plugin_github::providers(),
            &[],
            re_plugin_github::mcp_servers(),
        ),
        official_plugin_bundle(
            re_plugin_ssh::descriptor(),
            &[],
            &[],
            &[],
            &[],
            re_plugin_ssh::providers(),
            &[],
            &[],
        ),
        official_plugin_bundle(
            re_plugin_tdd_strict::descriptor(),
            re_plugin_tdd_strict::templates(),
            &[],
            &[],
            &[],
            &[],
            re_plugin_tdd_strict::policies(),
            &[],
        ),
    ]
}

/// Returns the immutable catalog of official plugins.
#[must_use]
pub fn official_plugins() -> Vec<PluginDescriptor> {
    official_plugin_bundles()
        .into_iter()
        .map(|bundle| bundle.descriptor)
        .collect()
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
pub fn official_mcp_servers() -> Vec<McpServerDescriptor> {
    official_plugin_bundles()
        .into_iter()
        .flat_map(|bundle| bundle.mcp_servers.iter().copied())
        .collect()
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

fn resolved_mcp_registration(
    server: McpServerDescriptor,
    plugin_activation: PluginActivation,
) -> RuntimeMcpRegistration {
    let mcp_enabled = default_project_config().mcp.enabled;
    let default_server_enabled = match server.availability {
        McpAvailability::OnDemand => true,
        McpAvailability::ExplicitOptIn => false,
    };
    let server_enabled = resolve_mcp_server_config(canonical_config_layers(), server.id)
        .map(|entry| entry.enabled)
        .unwrap_or(default_server_enabled);
    let enabled =
        matches!(plugin_activation, PluginActivation::Enabled) && mcp_enabled && server_enabled;

    RuntimeMcpRegistration::new(server, enabled)
}

fn resolved_official_plugin_bundles() -> Vec<ResolvedOfficialPluginBundle> {
    official_plugin_bundles()
        .into_iter()
        .map(|bundle| {
            let resolved = resolved_plugin_entry(bundle.descriptor);
            let plugin = RuntimePluginRegistration::new(
                bundle.descriptor,
                resolved.activation,
                resolved.resolved_from,
            );

            ResolvedOfficialPluginBundle { plugin, bundle }
        })
        .collect()
}

/// Returns the resolved runtime plugin registrations for the official catalog.
#[must_use]
pub fn official_runtime_plugins() -> Vec<RuntimePluginRegistration> {
    resolved_official_plugin_bundles()
        .into_iter()
        .map(|bundle| bundle.plugin)
        .collect()
}

/// Returns the resolved runtime MCP registrations for the official catalog.
#[must_use]
pub fn official_runtime_mcp_registrations() -> Vec<RuntimeMcpRegistration> {
    resolved_official_plugin_bundles()
        .into_iter()
        .flat_map(|bundle| {
            bundle
                .bundle
                .mcp_servers
                .iter()
                .copied()
                .map(move |server| resolved_mcp_registration(server, bundle.plugin.activation))
        })
        .collect()
}

/// Returns the resolved runtime capability registrations for the official catalog.
#[must_use]
pub fn official_runtime_capabilities() -> Vec<RuntimeCapabilityRegistration> {
    resolved_official_plugin_bundles()
        .into_iter()
        .flat_map(|bundle| {
            bundle
                .plugin
                .descriptor
                .capabilities
                .iter()
                .copied()
                .map(move |capability| {
                    RuntimeCapabilityRegistration::new(
                        capability,
                        bundle.plugin.descriptor.id,
                        bundle.plugin.activation,
                        bundle.plugin.descriptor.load_boundary,
                    )
                })
        })
        .collect()
}

/// Returns the resolved template registrations for the official catalog.
#[must_use]
pub fn official_runtime_templates() -> Vec<RuntimeTemplateRegistration> {
    resolved_official_plugin_bundles()
        .into_iter()
        .filter(|bundle| {
            bundle
                .plugin
                .descriptor
                .capabilities
                .iter()
                .copied()
                .any(capability_activates_template_surface)
        })
        .map(|bundle| {
            RuntimeTemplateRegistration::new(
                bundle.plugin.descriptor.id,
                bundle.plugin.activation,
                bundle.plugin.descriptor.load_boundary,
                bundle
                    .plugin
                    .descriptor
                    .runtime_hooks
                    .contains(&template_runtime_hook()),
            )
        })
        .collect()
}

/// Returns the resolved template contributions for the official catalog.
#[must_use]
pub fn official_template_contributions() -> Vec<OfficialTemplateContribution> {
    resolved_official_plugin_bundles()
        .into_iter()
        .flat_map(|bundle| {
            bundle
                .bundle
                .templates
                .iter()
                .copied()
                .map(move |descriptor| OfficialTemplateContribution {
                    descriptor,
                    activation: bundle.plugin.activation,
                    load_boundary: bundle.plugin.descriptor.load_boundary,
                    scaffold_hook_registered: bundle
                        .plugin
                        .descriptor
                        .runtime_hooks
                        .contains(&template_runtime_hook()),
                })
        })
        .collect()
}

/// Returns the resolved prompt registrations for the official catalog.
#[must_use]
pub fn official_runtime_prompts() -> Vec<RuntimePromptRegistration> {
    resolved_official_plugin_bundles()
        .into_iter()
        .filter(|bundle| {
            bundle
                .plugin
                .descriptor
                .capabilities
                .iter()
                .copied()
                .any(capability_activates_prompt_surface)
        })
        .map(|bundle| {
            RuntimePromptRegistration::new(
                bundle.plugin.descriptor.id,
                bundle.plugin.activation,
                bundle.plugin.descriptor.load_boundary,
                bundle
                    .plugin
                    .descriptor
                    .runtime_hooks
                    .contains(&prompt_runtime_hook()),
            )
        })
        .collect()
}

/// Returns the resolved prompt contributions for the official catalog.
#[must_use]
pub fn official_prompt_contributions() -> Vec<OfficialPromptContribution> {
    resolved_official_plugin_bundles()
        .into_iter()
        .flat_map(|bundle| {
            bundle
                .bundle
                .prompts
                .iter()
                .copied()
                .map(move |descriptor| OfficialPromptContribution {
                    descriptor,
                    activation: bundle.plugin.activation,
                    load_boundary: bundle.plugin.descriptor.load_boundary,
                    prompt_hook_registered: bundle
                        .plugin
                        .descriptor
                        .runtime_hooks
                        .contains(&prompt_runtime_hook()),
                })
        })
        .collect()
}

/// Returns the resolved agent runtime registrations for the official catalog.
#[must_use]
pub fn official_runtime_agents() -> Vec<RuntimeAgentRegistration> {
    resolved_official_plugin_bundles()
        .into_iter()
        .filter(|bundle| {
            bundle
                .plugin
                .descriptor
                .capabilities
                .iter()
                .copied()
                .any(capability_activates_agent_surface)
        })
        .map(|bundle| {
            RuntimeAgentRegistration::new(
                bundle.bundle.agents[0].id,
                bundle.plugin.descriptor.id,
                bundle.plugin.activation,
                bundle.plugin.descriptor.load_boundary,
                bundle
                    .plugin
                    .descriptor
                    .runtime_hooks
                    .contains(&agent_runtime_hook()),
            )
        })
        .collect()
}

/// Returns the resolved agent runtime contributions for the official catalog.
#[must_use]
pub fn official_agent_contributions() -> Vec<OfficialAgentContribution> {
    resolved_official_plugin_bundles()
        .into_iter()
        .flat_map(|bundle| {
            bundle
                .bundle
                .agents
                .iter()
                .copied()
                .map(move |descriptor| OfficialAgentContribution {
                    descriptor,
                    activation: bundle.plugin.activation,
                    load_boundary: bundle.plugin.descriptor.load_boundary,
                    bootstrap_hook_registered: bundle
                        .plugin
                        .descriptor
                        .runtime_hooks
                        .contains(&agent_runtime_hook()),
                })
        })
        .collect()
}

/// Returns the resolved check contributions for the official catalog.
#[must_use]
pub fn official_check_contributions() -> Vec<OfficialCheckContribution> {
    resolved_official_plugin_bundles()
        .into_iter()
        .flat_map(|bundle| {
            bundle
                .bundle
                .checks
                .iter()
                .copied()
                .map(move |descriptor| OfficialCheckContribution {
                    descriptor,
                    activation: bundle.plugin.activation,
                    load_boundary: bundle.plugin.descriptor.load_boundary,
                    runtime_hook_registered: bundle.plugin.descriptor.runtime_hooks.contains(
                        &runtime_hook_for_check(runtime_check_kind_for_descriptor(descriptor.kind)),
                    ),
                })
        })
        .collect()
}

/// Returns the resolved provider contributions for the official catalog.
#[must_use]
pub fn official_provider_contributions() -> Vec<OfficialProviderContribution> {
    resolved_official_plugin_bundles()
        .into_iter()
        .flat_map(|bundle| {
            bundle
                .bundle
                .providers
                .iter()
                .copied()
                .map(move |descriptor| OfficialProviderContribution {
                    descriptor,
                    activation: bundle.plugin.activation,
                    load_boundary: bundle.plugin.descriptor.load_boundary,
                    registration_hook_registered: bundle.plugin.descriptor.runtime_hooks.contains(
                        &runtime_hook_for_provider(runtime_provider_kind_for_descriptor(
                            descriptor.kind,
                        )),
                    ),
                })
        })
        .collect()
}

/// Returns the resolved runtime-hook registrations for the official catalog.
#[must_use]
pub fn official_runtime_hooks() -> Vec<RuntimeHookRegistration> {
    resolved_official_plugin_bundles()
        .into_iter()
        .flat_map(|bundle| {
            bundle
                .plugin
                .descriptor
                .runtime_hooks
                .iter()
                .copied()
                .map(move |hook| {
                    RuntimeHookRegistration::new(
                        hook,
                        bundle.plugin.descriptor.id,
                        bundle.plugin.activation,
                        bundle.plugin.descriptor.load_boundary,
                    )
                })
        })
        .collect()
}

/// Returns the resolved runtime check registrations for the official catalog.
#[must_use]
pub fn official_runtime_checks() -> Vec<RuntimeCheckRegistration> {
    official_check_contributions()
        .into_iter()
        .map(|check| {
            RuntimeCheckRegistration::new(
                runtime_check_kind_for_descriptor(check.descriptor.kind),
                check.descriptor.plugin_id,
                check.activation,
                check.load_boundary,
                check.runtime_hook_registered,
            )
        })
        .collect()
}

/// Returns the resolved runtime provider registrations for the official catalog.
#[must_use]
pub fn official_runtime_providers() -> Vec<RuntimeProviderRegistration> {
    official_provider_contributions()
        .into_iter()
        .map(|provider| {
            RuntimeProviderRegistration::new(
                provider.descriptor.id,
                runtime_provider_kind_for_descriptor(provider.descriptor.kind),
                provider.descriptor.plugin_id,
                provider.activation,
                provider.load_boundary,
                provider.registration_hook_registered,
            )
        })
        .collect()
}

/// Returns the resolved runtime policy registrations for the official catalog.
#[must_use]
pub fn official_runtime_policies() -> Vec<RuntimePolicyRegistration> {
    resolved_official_plugin_bundles()
        .into_iter()
        .flat_map(|bundle| {
            bundle.bundle.policies.iter().copied().map(move |policy| {
                RuntimePolicyRegistration::new(
                    policy.id,
                    bundle.plugin.descriptor.id,
                    bundle.plugin.activation,
                    bundle.plugin.descriptor.load_boundary,
                    bundle
                        .plugin
                        .descriptor
                        .runtime_hooks
                        .contains(&policy_runtime_hook()),
                )
            })
        })
        .collect()
}

/// Returns the resolved policy contributions for the official catalog.
#[must_use]
pub fn official_policy_contributions() -> Vec<OfficialPolicyContribution> {
    resolved_official_plugin_bundles()
        .into_iter()
        .flat_map(|bundle| {
            bundle
                .bundle
                .policies
                .iter()
                .copied()
                .map(move |descriptor| OfficialPolicyContribution {
                    descriptor,
                    activation: bundle.plugin.activation,
                    load_boundary: bundle.plugin.descriptor.load_boundary,
                    enforcement_hook_registered: bundle
                        .plugin
                        .descriptor
                        .runtime_hooks
                        .contains(&policy_runtime_hook()),
                })
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

fn registrations_for_key<T: Copy, K: Copy + Eq>(
    registrations: Vec<T>,
    key: K,
    key_of: fn(T) -> K,
) -> Vec<T> {
    registrations
        .into_iter()
        .filter(|registration| key_of(*registration) == key)
        .collect()
}

/// Returns one resolved template contribution by stable identifier.
#[must_use]
pub fn find_official_template_contribution(
    template_id: &str,
) -> Option<OfficialTemplateContribution> {
    official_template_contributions()
        .into_iter()
        .find(|template| template.descriptor.id == template_id)
}

/// Returns one resolved agent contribution by stable identifier.
#[must_use]
pub fn find_official_agent_contribution(agent_id: &str) -> Option<OfficialAgentContribution> {
    official_agent_contributions()
        .into_iter()
        .find(|agent| agent.descriptor.id == agent_id)
}

/// Returns one resolved prompt contribution by stable identifier.
#[must_use]
pub fn find_official_prompt_contribution(prompt_id: &str) -> Option<OfficialPromptContribution> {
    official_prompt_contributions()
        .into_iter()
        .find(|prompt| prompt.descriptor.id == prompt_id)
}

/// Returns one resolved policy contribution by stable identifier.
#[must_use]
pub fn find_official_policy_contribution(policy_id: &str) -> Option<OfficialPolicyContribution> {
    official_policy_contributions()
        .into_iter()
        .find(|policy| policy.descriptor.id == policy_id)
}

/// Returns one resolved check contribution by stable identifier.
#[must_use]
pub fn find_official_check_contribution(check_id: &str) -> Option<OfficialCheckContribution> {
    official_check_contributions()
        .into_iter()
        .find(|check| check.descriptor.id == check_id)
}

/// Returns one resolved provider contribution by stable identifier.
#[must_use]
pub fn find_official_provider_contribution(
    provider_id: &str,
) -> Option<OfficialProviderContribution> {
    official_provider_contributions()
        .into_iter()
        .find(|provider| provider.descriptor.id == provider_id)
}

/// Returns one resolved check surface by stable identifier.
#[must_use]
pub fn find_official_check_surface(check_id: &str) -> Option<OfficialResolvedCheckSurface> {
    let contribution = find_official_check_contribution(check_id)?;
    let registration = find_official_runtime_checks(runtime_check_kind_for_descriptor(
        contribution.descriptor.kind,
    ))
    .into_iter()
    .find(|candidate| candidate.plugin_id == contribution.descriptor.plugin_id)?;

    Some(OfficialResolvedCheckSurface {
        contribution,
        registration,
    })
}

/// Returns one resolved provider surface by stable identifier.
#[must_use]
pub fn find_official_provider_surface(
    provider_id: &str,
) -> Option<OfficialResolvedProviderSurface> {
    let contribution = find_official_provider_contribution(provider_id)?;
    let registration = find_official_runtime_providers(runtime_provider_kind_for_descriptor(
        contribution.descriptor.kind,
    ))
    .into_iter()
    .find(|candidate| candidate.plugin_id == contribution.descriptor.plugin_id)?;

    Some(OfficialResolvedProviderSurface {
        contribution,
        registration,
    })
}

/// Returns the resolved capability registrations for one reviewed capability.
#[must_use]
pub fn find_official_runtime_capabilities(
    capability: re_plugin::PluginCapability,
) -> Vec<RuntimeCapabilityRegistration> {
    registrations_for_key(
        official_runtime_capabilities(),
        capability,
        |registration| registration.capability,
    )
}

/// Returns the resolved runtime-hook registrations for one typed hook.
#[must_use]
pub fn find_official_runtime_hooks(hook: PluginRuntimeHook) -> Vec<RuntimeHookRegistration> {
    registrations_for_key(official_runtime_hooks(), hook, |registration| {
        registration.hook
    })
}

/// Returns the resolved runtime check registrations for one typed kind.
#[must_use]
pub fn find_official_runtime_checks(kind: RuntimeCheckKind) -> Vec<RuntimeCheckRegistration> {
    registrations_for_key(official_runtime_checks(), kind, |registration| {
        registration.kind
    })
}

/// Returns the resolved check contributions for one typed kind.
#[must_use]
pub fn find_official_check_contributions(kind: RuntimeCheckKind) -> Vec<OfficialCheckContribution> {
    official_check_contributions()
        .into_iter()
        .filter(|check| runtime_check_kind_for_descriptor(check.descriptor.kind) == kind)
        .collect()
}

/// Returns the resolved runtime provider registrations for one typed kind.
#[must_use]
pub fn find_official_runtime_providers(
    kind: RuntimeProviderKind,
) -> Vec<RuntimeProviderRegistration> {
    registrations_for_key(official_runtime_providers(), kind, |registration| {
        registration.kind
    })
}

/// Returns the resolved provider contributions for one typed kind.
#[must_use]
pub fn find_official_provider_contributions(
    kind: RuntimeProviderKind,
) -> Vec<OfficialProviderContribution> {
    official_provider_contributions()
        .into_iter()
        .filter(|provider| runtime_provider_kind_for_descriptor(provider.descriptor.kind) == kind)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        find_official_agent_contribution, find_official_check_surface, find_official_mcp_server,
        find_official_plugin, find_official_policy_contribution, find_official_prompt_contribution,
        find_official_provider_surface, find_official_runtime_capabilities,
        find_official_runtime_checks, find_official_runtime_hooks, find_official_runtime_providers,
        find_official_template_contribution, official_plugin_bundles, official_plugins,
        official_runtime_agents, official_runtime_checks, official_runtime_mcp_registrations,
        official_runtime_policies, official_runtime_prompts, official_runtime_providers,
        official_runtime_snapshot, official_runtime_templates,
    };
    use re_core::{
        RuntimeCheckKind, RuntimeProviderKind, runtime_check_kind_for_capability,
        runtime_provider_kind_for_capability,
    };
    use re_plugin::{
        ALL_PLUGIN_CAPABILITIES, PluginCapability, PluginRuntimeHook,
        runtime_surface_for_capability,
    };

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
                runtime_surface_for_capability(PluginCapability::new(capability))
                    .map(re_plugin::PluginRuntimeSurface::as_str)
                    .is_none()
            })
            .collect::<Vec<_>>();

        assert_eq!(uncovered, Vec::<&'static str>::new());
    }

    #[test]
    fn official_plugin_capabilities_are_covered_by_runtime_surfaces() {
        for plugin in official_plugins() {
            for capability in plugin.capabilities {
                assert!(runtime_surface_for_capability(*capability).is_some());
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
    fn official_plugin_bundles_stay_descriptor_aligned() {
        let bundles = official_plugin_bundles();

        assert_eq!(bundles.len(), official_plugins().len());

        for bundle in bundles {
            for template in bundle.templates {
                assert_eq!(template.plugin_id, bundle.descriptor.id);
            }

            for prompt in bundle.prompts {
                assert_eq!(prompt.plugin_id, bundle.descriptor.id);
            }

            for agent in bundle.agents {
                assert_eq!(agent.plugin_id, bundle.descriptor.id);
            }

            for check in bundle.checks {
                assert_eq!(check.plugin_id, bundle.descriptor.id);
            }

            for provider in bundle.providers {
                assert_eq!(provider.plugin_id, bundle.descriptor.id);
            }

            for policy in bundle.policies {
                assert_eq!(policy.plugin_id, bundle.descriptor.id);
            }

            for server in bundle.mcp_servers {
                assert_eq!(server.plugin_id, bundle.descriptor.id);
            }
        }
    }

    #[test]
    fn unknown_capabilities_do_not_map_to_runtime_surfaces() {
        let unknown = PluginCapability::new("unknown_surface");

        assert_eq!(runtime_surface_for_capability(unknown), None);
        assert_eq!(runtime_check_kind_for_capability(unknown), None);
        assert_eq!(runtime_provider_kind_for_capability(unknown), None);
    }

    #[test]
    fn unknown_plugin_and_mcp_identifiers_are_rejected() {
        assert_eq!(find_official_plugin("fixture.missing"), None);
        assert_eq!(find_official_mcp_server("fixture.missing"), None);
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

    #[test]
    fn contribution_helpers_reject_unknown_identifiers() {
        assert_eq!(find_official_template_contribution("fixture.missing"), None);
        assert_eq!(find_official_prompt_contribution("fixture.missing"), None);
        assert_eq!(find_official_agent_contribution("fixture.missing"), None);
        assert_eq!(find_official_policy_contribution("fixture.missing"), None);
        assert_eq!(find_official_check_surface("fixture.missing"), None);
        assert_eq!(find_official_provider_surface("fixture.missing"), None);
    }

    #[test]
    fn resolved_surface_helpers_pair_contributions_with_runtime_registrations() {
        let Some(check_surface) = find_official_check_surface("official.bmad.prepare") else {
            unreachable!("known check should resolve");
        };
        let Some(provider_surface) = find_official_provider_surface("official.github.data") else {
            unreachable!("known provider should resolve");
        };

        assert_eq!(
            check_surface.contribution.descriptor.plugin_id,
            check_surface.registration.plugin_id
        );
        assert_eq!(
            provider_surface.contribution.descriptor.plugin_id,
            provider_surface.registration.plugin_id
        );
    }

    #[test]
    fn grouped_surface_helpers_filter_typed_keys() {
        let capabilities = find_official_runtime_capabilities(PluginCapability::new("template"));
        let hooks = find_official_runtime_hooks(PluginRuntimeHook::Scaffold);
        let checks = find_official_runtime_checks(RuntimeCheckKind::Doctor);
        let providers = find_official_runtime_providers(RuntimeProviderKind::RemoteControl);

        assert!(!capabilities.is_empty());
        assert!(
            capabilities
                .iter()
                .all(|registration| registration.capability == PluginCapability::new("template"))
        );
        assert!(!hooks.is_empty());
        assert!(
            hooks
                .iter()
                .all(|registration| registration.hook == PluginRuntimeHook::Scaffold)
        );
        assert!(!checks.is_empty());
        assert!(
            checks
                .iter()
                .all(|registration| registration.kind == RuntimeCheckKind::Doctor)
        );
        assert!(!providers.is_empty());
        assert!(
            providers
                .iter()
                .all(|registration| registration.kind == RuntimeProviderKind::RemoteControl)
        );
        assert!(find_official_policy_contribution("official.tdd-strict.guardrails").is_some());
        assert_eq!(find_official_policy_contribution("fixture.missing"), None);
    }

    #[test]
    fn all_official_plugins_pass_validation() {
        for plugin in official_plugins() {
            let errors = plugin.validate();
            assert!(
                errors.is_empty(),
                "Plugin '{}' failed validation: {:?}",
                plugin.id,
                errors
            );
        }
    }

    #[test]
    fn all_official_plugins_are_api_compatible() {
        for plugin in official_plugins() {
            assert!(
                plugin.is_api_compatible(),
                "Plugin '{}' has api_version {} but runtime supports {}",
                plugin.id,
                plugin.plugin_api_version,
                re_plugin::CURRENT_PLUGIN_API_VERSION
            );
        }
    }
}
