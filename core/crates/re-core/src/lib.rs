//! Shared product metadata for Ralph Engine.

mod i18n;

use re_config::{ConfigScope, PluginActivation};
use re_mcp::McpServerDescriptor;
use re_plugin::{
    CONTEXT_PROVIDER, DATA_SOURCE, DOCTOR_CHECKS, FORGE_PROVIDER, PREPARE_CHECKS, PluginCapability,
    PluginDescriptor, PluginLoadBoundary, PluginRuntimeHook, REMOTE_CONTROL,
};

/// Public product name.
pub const PRODUCT_NAME: &str = "Ralph Engine";

/// Public one-line positioning statement.
pub const PRODUCT_TAGLINE: &str = "Open-source plugin-first runtime for agentic coding workflows.";

/// Builds the startup banner for CLI surfaces.
#[must_use]
pub fn banner() -> String {
    format!(
        "{PRODUCT_NAME}
{PRODUCT_TAGLINE}"
    )
}

/// Typed runtime phase identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimePhase {
    /// The runtime foundation is bootstrapped but not fully resolved.
    Bootstrapped,
    /// The runtime has a resolved plugin and MCP topology.
    Ready,
}

impl RuntimePhase {
    /// Returns the stable runtime-phase identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bootstrapped => "bootstrapped",
            Self::Ready => "ready",
        }
    }
}

/// Typed runtime health identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeHealth {
    /// The runtime is fully operable for its resolved topology.
    Healthy,
    /// The runtime still depends on explicit activation or operator action.
    Degraded,
}

impl RuntimeHealth {
    /// Returns the stable runtime-health identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
        }
    }
}

/// Typed runtime issue identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeIssueKind {
    /// A plugin is registered but still disabled.
    PluginDisabled,
    /// A capability provider is registered but still disabled.
    CapabilityDisabled,
    /// A template provider is registered but still disabled.
    TemplateDisabled,
    /// A prompt provider is registered but still disabled.
    PromptProviderDisabled,
    /// An agent runtime provider is registered but still disabled.
    AgentRuntimeDisabled,
    /// A runtime check is registered but still disabled.
    CheckDisabled,
    /// A provider contribution is registered but still disabled.
    ProviderDisabled,
    /// A policy provider is registered but still disabled.
    PolicyDisabled,
    /// A runtime-hook provider is registered but still disabled.
    HookDisabled,
    /// An MCP server is registered but still disabled.
    McpServerDisabled,
}

impl RuntimeIssueKind {
    /// Returns the stable runtime-issue identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PluginDisabled => "plugin_disabled",
            Self::CapabilityDisabled => "capability_disabled",
            Self::TemplateDisabled => "template_disabled",
            Self::PromptProviderDisabled => "prompt_provider_disabled",
            Self::AgentRuntimeDisabled => "agent_runtime_disabled",
            Self::CheckDisabled => "check_disabled",
            Self::ProviderDisabled => "provider_disabled",
            Self::PolicyDisabled => "policy_disabled",
            Self::HookDisabled => "hook_disabled",
            Self::McpServerDisabled => "mcp_server_disabled",
        }
    }
}

/// Typed runtime action identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeActionKind {
    /// Enable a plugin in the resolved configuration.
    EnablePlugin,
    /// Re-enable a disabled capability provider.
    EnableCapabilityProvider,
    /// Re-enable a disabled template provider.
    EnableTemplateProvider,
    /// Re-enable a disabled prompt provider.
    EnablePromptProvider,
    /// Re-enable a disabled agent runtime provider.
    EnableAgentRuntimeProvider,
    /// Re-enable a disabled runtime check provider.
    EnableCheckProvider,
    /// Re-enable a disabled provider contribution.
    EnableProvider,
    /// Re-enable a disabled policy provider.
    EnablePolicyProvider,
    /// Re-enable a disabled runtime-hook provider.
    EnableHookProvider,
    /// Opt in to a disabled MCP contribution.
    EnableMcpServer,
}

impl RuntimeActionKind {
    /// Returns the stable runtime-action identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnablePlugin => "enable_plugin",
            Self::EnableCapabilityProvider => "enable_capability_provider",
            Self::EnableTemplateProvider => "enable_template_provider",
            Self::EnablePromptProvider => "enable_prompt_provider",
            Self::EnableAgentRuntimeProvider => "enable_agent_runtime_provider",
            Self::EnableCheckProvider => "enable_check_provider",
            Self::EnableProvider => "enable_provider",
            Self::EnablePolicyProvider => "enable_policy_provider",
            Self::EnableHookProvider => "enable_hook_provider",
            Self::EnableMcpServer => "enable_mcp_server",
        }
    }
}

/// One typed plugin registration in the resolved runtime topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimePluginRegistration {
    /// Immutable plugin descriptor.
    pub descriptor: PluginDescriptor,
    /// Effective activation state for the plugin.
    pub activation: PluginActivation,
    /// Scope that supplied the effective activation result.
    pub resolved_from: ConfigScope,
}

impl RuntimePluginRegistration {
    /// Creates a new immutable runtime plugin registration.
    #[must_use]
    pub const fn new(
        descriptor: PluginDescriptor,
        activation: PluginActivation,
        resolved_from: ConfigScope,
    ) -> Self {
        Self {
            descriptor,
            activation,
            resolved_from,
        }
    }

    /// Returns whether the plugin is enabled in the resolved topology.
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self.activation, PluginActivation::Enabled)
    }
}

/// One typed MCP registration in the resolved runtime topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeMcpRegistration {
    /// Immutable MCP server descriptor.
    pub descriptor: McpServerDescriptor,
    /// Whether the server is enabled in the resolved topology.
    pub enabled: bool,
}

impl RuntimeMcpRegistration {
    /// Creates a new immutable runtime MCP registration.
    #[must_use]
    pub const fn new(descriptor: McpServerDescriptor, enabled: bool) -> Self {
        Self {
            descriptor,
            enabled,
        }
    }
}

/// One typed capability registration in the resolved runtime topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeCapabilityRegistration {
    /// Stable capability identifier.
    pub capability: PluginCapability,
    /// Plugin providing the capability.
    pub plugin_id: &'static str,
    /// Effective activation state for the provider plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the provider plugin.
    pub load_boundary: PluginLoadBoundary,
}

impl RuntimeCapabilityRegistration {
    /// Creates a new immutable runtime capability registration.
    #[must_use]
    pub const fn new(
        capability: PluginCapability,
        plugin_id: &'static str,
        activation: PluginActivation,
        load_boundary: PluginLoadBoundary,
    ) -> Self {
        Self {
            capability,
            plugin_id,
            activation,
            load_boundary,
        }
    }

    /// Returns whether the capability provider is enabled in the resolved topology.
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self.activation, PluginActivation::Enabled)
    }
}

/// One typed template registration in the resolved runtime topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeTemplateRegistration {
    /// Plugin providing the template surface.
    pub plugin_id: &'static str,
    /// Effective activation state for the provider plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the provider plugin.
    pub load_boundary: PluginLoadBoundary,
    /// Whether the provider also declares the matching runtime hook.
    pub scaffold_hook_registered: bool,
}

impl RuntimeTemplateRegistration {
    /// Creates a new immutable runtime template registration.
    #[must_use]
    pub const fn new(
        plugin_id: &'static str,
        activation: PluginActivation,
        load_boundary: PluginLoadBoundary,
        scaffold_hook_registered: bool,
    ) -> Self {
        Self {
            plugin_id,
            activation,
            load_boundary,
            scaffold_hook_registered,
        }
    }

    /// Returns whether the template provider is enabled in the resolved topology.
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self.activation, PluginActivation::Enabled)
    }
}

/// One typed prompt-provider registration in the resolved runtime topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimePromptRegistration {
    /// Plugin providing the prompt surface.
    pub plugin_id: &'static str,
    /// Effective activation state for the provider plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the provider plugin.
    pub load_boundary: PluginLoadBoundary,
    /// Whether the provider also declares the matching runtime hook.
    pub prompt_hook_registered: bool,
}

impl RuntimePromptRegistration {
    /// Creates a new immutable runtime prompt registration.
    #[must_use]
    pub const fn new(
        plugin_id: &'static str,
        activation: PluginActivation,
        load_boundary: PluginLoadBoundary,
        prompt_hook_registered: bool,
    ) -> Self {
        Self {
            plugin_id,
            activation,
            load_boundary,
            prompt_hook_registered,
        }
    }

    /// Returns whether the prompt provider is enabled in the resolved topology.
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self.activation, PluginActivation::Enabled)
    }
}

/// One typed agent-runtime registration in the resolved runtime topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeAgentRegistration {
    /// Plugin providing the agent runtime.
    pub plugin_id: &'static str,
    /// Effective activation state for the provider plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the provider plugin.
    pub load_boundary: PluginLoadBoundary,
    /// Whether the provider also declares the matching runtime hook.
    pub bootstrap_hook_registered: bool,
}

impl RuntimeAgentRegistration {
    /// Creates a new immutable runtime agent registration.
    #[must_use]
    pub const fn new(
        plugin_id: &'static str,
        activation: PluginActivation,
        load_boundary: PluginLoadBoundary,
        bootstrap_hook_registered: bool,
    ) -> Self {
        Self {
            plugin_id,
            activation,
            load_boundary,
            bootstrap_hook_registered,
        }
    }

    /// Returns whether the agent runtime provider is enabled in the resolved topology.
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self.activation, PluginActivation::Enabled)
    }
}

/// Typed runtime check-kind identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeCheckKind {
    /// Prepare-time validation contribution.
    Prepare,
    /// Doctor-time validation contribution.
    Doctor,
}

impl RuntimeCheckKind {
    /// Returns the stable runtime-check identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prepare => "prepare",
            Self::Doctor => "doctor",
        }
    }
}

/// Resolves the typed runtime-check surface declared by one plugin capability.
#[must_use]
pub fn runtime_check_kind_for_capability(capability: PluginCapability) -> Option<RuntimeCheckKind> {
    match capability {
        PREPARE_CHECKS => Some(RuntimeCheckKind::Prepare),
        DOCTOR_CHECKS => Some(RuntimeCheckKind::Doctor),
        _ => None,
    }
}

/// One typed runtime check registration in the resolved runtime topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeCheckRegistration {
    /// Stable check kind.
    pub kind: RuntimeCheckKind,
    /// Plugin providing the check.
    pub plugin_id: &'static str,
    /// Effective activation state for the provider plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the provider plugin.
    pub load_boundary: PluginLoadBoundary,
    /// Whether the provider also declares the matching runtime hook.
    pub runtime_hook_registered: bool,
}

impl RuntimeCheckRegistration {
    /// Creates a new immutable runtime check registration.
    #[must_use]
    pub const fn new(
        kind: RuntimeCheckKind,
        plugin_id: &'static str,
        activation: PluginActivation,
        load_boundary: PluginLoadBoundary,
        runtime_hook_registered: bool,
    ) -> Self {
        Self {
            kind,
            plugin_id,
            activation,
            load_boundary,
            runtime_hook_registered,
        }
    }

    /// Returns whether the check provider is enabled in the resolved topology.
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self.activation, PluginActivation::Enabled)
    }
}

/// Typed runtime provider-kind identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeProviderKind {
    /// Data-source provider contribution.
    DataSource,
    /// Context-provider contribution.
    ContextProvider,
    /// Forge-provider contribution.
    ForgeProvider,
    /// Remote-control contribution.
    RemoteControl,
}

impl RuntimeProviderKind {
    /// Returns the stable runtime-provider identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DataSource => "data_source",
            Self::ContextProvider => "context_provider",
            Self::ForgeProvider => "forge_provider",
            Self::RemoteControl => "remote_control",
        }
    }
}

/// Resolves the typed runtime-provider surface declared by one plugin capability.
#[must_use]
pub fn runtime_provider_kind_for_capability(
    capability: PluginCapability,
) -> Option<RuntimeProviderKind> {
    match capability {
        DATA_SOURCE => Some(RuntimeProviderKind::DataSource),
        CONTEXT_PROVIDER => Some(RuntimeProviderKind::ContextProvider),
        FORGE_PROVIDER => Some(RuntimeProviderKind::ForgeProvider),
        REMOTE_CONTROL => Some(RuntimeProviderKind::RemoteControl),
        _ => None,
    }
}

/// One typed provider registration in the resolved runtime topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeProviderRegistration {
    /// Stable provider kind.
    pub kind: RuntimeProviderKind,
    /// Plugin providing the contribution.
    pub plugin_id: &'static str,
    /// Effective activation state for the provider plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the provider plugin.
    pub load_boundary: PluginLoadBoundary,
    /// Whether the provider also declares the matching runtime hook.
    pub registration_hook_registered: bool,
}

impl RuntimeProviderRegistration {
    /// Creates a new immutable runtime provider registration.
    #[must_use]
    pub const fn new(
        kind: RuntimeProviderKind,
        plugin_id: &'static str,
        activation: PluginActivation,
        load_boundary: PluginLoadBoundary,
        registration_hook_registered: bool,
    ) -> Self {
        Self {
            kind,
            plugin_id,
            activation,
            load_boundary,
            registration_hook_registered,
        }
    }

    /// Returns whether the provider is enabled in the resolved topology.
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self.activation, PluginActivation::Enabled)
    }
}

/// One typed policy registration in the resolved runtime topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimePolicyRegistration {
    /// Stable policy identifier.
    pub policy_id: &'static str,
    /// Plugin providing the policy.
    pub plugin_id: &'static str,
    /// Effective activation state for the provider plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the provider plugin.
    pub load_boundary: PluginLoadBoundary,
    /// Whether the provider also declares the policy-enforcement runtime hook.
    pub enforcement_hook_registered: bool,
}

impl RuntimePolicyRegistration {
    /// Creates a new immutable runtime policy registration.
    #[must_use]
    pub const fn new(
        policy_id: &'static str,
        plugin_id: &'static str,
        activation: PluginActivation,
        load_boundary: PluginLoadBoundary,
        enforcement_hook_registered: bool,
    ) -> Self {
        Self {
            policy_id,
            plugin_id,
            activation,
            load_boundary,
            enforcement_hook_registered,
        }
    }

    /// Returns whether the policy provider is enabled in the resolved topology.
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self.activation, PluginActivation::Enabled)
    }
}

/// One typed runtime-hook registration in the resolved runtime topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeHookRegistration {
    /// Stable hook identifier.
    pub hook: PluginRuntimeHook,
    /// Plugin providing the hook.
    pub plugin_id: &'static str,
    /// Effective activation state for the provider plugin.
    pub activation: PluginActivation,
    /// Declared load boundary for the provider plugin.
    pub load_boundary: PluginLoadBoundary,
}

impl RuntimeHookRegistration {
    /// Creates a new immutable runtime-hook registration.
    #[must_use]
    pub const fn new(
        hook: PluginRuntimeHook,
        plugin_id: &'static str,
        activation: PluginActivation,
        load_boundary: PluginLoadBoundary,
    ) -> Self {
        Self {
            hook,
            plugin_id,
            activation,
            load_boundary,
        }
    }

    /// Returns whether the runtime-hook provider is enabled in the resolved topology.
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self.activation, PluginActivation::Enabled)
    }
}

/// Immutable snapshot of the resolved runtime topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeTopology<'a> {
    /// Resolved runtime phase.
    pub phase: RuntimePhase,
    /// Effective runtime locale.
    pub locale: &'static str,
    /// Resolved plugin registrations.
    pub plugins: &'a [RuntimePluginRegistration],
    /// Resolved capability registrations.
    pub capabilities: &'a [RuntimeCapabilityRegistration],
    /// Resolved template registrations.
    pub templates: &'a [RuntimeTemplateRegistration],
    /// Resolved prompt registrations.
    pub prompts: &'a [RuntimePromptRegistration],
    /// Resolved agent-runtime registrations.
    pub agents: &'a [RuntimeAgentRegistration],
    /// Resolved runtime check registrations.
    pub checks: &'a [RuntimeCheckRegistration],
    /// Resolved provider registrations.
    pub providers: &'a [RuntimeProviderRegistration],
    /// Resolved policy registrations.
    pub policies: &'a [RuntimePolicyRegistration],
    /// Resolved runtime-hook registrations.
    pub hooks: &'a [RuntimeHookRegistration],
    /// Resolved MCP registrations.
    pub mcp_servers: &'a [RuntimeMcpRegistration],
}

/// Immutable summary of resolved runtime state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeStatus {
    /// Current runtime phase.
    pub phase: RuntimePhase,
    /// Derived runtime health.
    pub health: RuntimeHealth,
    /// Number of enabled plugins.
    pub enabled_plugins: usize,
    /// Number of disabled plugins.
    pub disabled_plugins: usize,
    /// Number of enabled capability providers.
    pub enabled_capabilities: usize,
    /// Number of disabled capability providers.
    pub disabled_capabilities: usize,
    /// Number of enabled templates.
    pub enabled_templates: usize,
    /// Number of disabled templates.
    pub disabled_templates: usize,
    /// Number of enabled prompt providers.
    pub enabled_prompts: usize,
    /// Number of disabled prompt providers.
    pub disabled_prompts: usize,
    /// Number of enabled agent runtimes.
    pub enabled_agents: usize,
    /// Number of disabled agent runtimes.
    pub disabled_agents: usize,
    /// Number of enabled runtime checks.
    pub enabled_checks: usize,
    /// Number of disabled runtime checks.
    pub disabled_checks: usize,
    /// Number of enabled providers.
    pub enabled_providers: usize,
    /// Number of disabled providers.
    pub disabled_providers: usize,
    /// Number of enabled policy providers.
    pub enabled_policies: usize,
    /// Number of disabled policy providers.
    pub disabled_policies: usize,
    /// Number of enabled runtime-hook providers.
    pub enabled_hooks: usize,
    /// Number of disabled runtime-hook providers.
    pub disabled_hooks: usize,
    /// Number of enabled MCP servers.
    pub enabled_mcp_servers: usize,
    /// Number of disabled MCP servers.
    pub disabled_mcp_servers: usize,
}

/// One typed runtime issue produced from the resolved topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeIssue {
    /// Stable issue kind.
    pub kind: RuntimeIssueKind,
    /// Stable subject identifier affected by the issue.
    pub subject: &'static str,
    /// Operator action needed to resolve the issue.
    pub recommended_action: &'static str,
}

impl RuntimeIssue {
    /// Creates a new immutable runtime issue.
    #[must_use]
    pub const fn new(
        kind: RuntimeIssueKind,
        subject: &'static str,
        recommended_action: &'static str,
    ) -> Self {
        Self {
            kind,
            subject,
            recommended_action,
        }
    }
}

/// One typed remediation action produced from runtime issues.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeAction {
    /// Stable action kind.
    pub kind: RuntimeActionKind,
    /// Stable action target.
    pub target: &'static str,
    /// Human-readable reason for the action.
    pub reason: String,
}

impl RuntimeAction {
    /// Creates a new immutable runtime action.
    #[must_use]
    pub fn new(kind: RuntimeActionKind, target: &'static str, reason: impl Into<String>) -> Self {
        Self {
            kind,
            target,
            reason: reason.into(),
        }
    }
}

/// Immutable runtime doctor report derived from one resolved topology.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeDoctorReport {
    /// Snapshot of current runtime status.
    pub status: RuntimeStatus,
    /// Unresolved runtime issues.
    pub issues: Vec<RuntimeIssue>,
    /// Recommended remediation actions.
    pub actions: Vec<RuntimeAction>,
}

impl RuntimeDoctorReport {
    /// Creates a new immutable runtime doctor report.
    #[must_use]
    pub fn new(
        status: RuntimeStatus,
        issues: Vec<RuntimeIssue>,
        actions: Vec<RuntimeAction>,
    ) -> Self {
        Self {
            status,
            issues,
            actions,
        }
    }
}

/// Renders a human-readable runtime topology summary.
#[must_use]
pub fn render_runtime_topology(topology: &RuntimeTopology<'_>) -> String {
    render_runtime_topology_for_locale(topology, "en")
}

/// Renders a human-readable runtime topology summary for one locale.
#[must_use]
pub fn render_runtime_topology_for_locale(topology: &RuntimeTopology<'_>, locale: &str) -> String {
    let mut lines = vec![
        format!(
            "{}: {}",
            i18n::runtime_phase_label(locale),
            topology.phase.as_str()
        ),
        format!("{}: {}", i18n::locale_label(locale), topology.locale),
        format!(
            "{} ({})",
            i18n::plugins_label(locale),
            topology.plugins.len()
        ),
    ];

    for plugin in topology.plugins {
        lines.push(format!(
            "- {} | activation={} | scope={} | boundary={}",
            plugin.descriptor.id,
            plugin.activation.as_str(),
            plugin.resolved_from.as_str(),
            plugin.descriptor.load_boundary.as_str()
        ));
    }

    lines.push(format!(
        "{} ({})",
        i18n::capabilities_label(locale),
        topology.capabilities.len()
    ));

    for capability in topology.capabilities {
        lines.push(format!(
            "- {} | plugin={} | activation={} | boundary={}",
            capability.capability.as_str(),
            capability.plugin_id,
            capability.activation.as_str(),
            capability.load_boundary.as_str()
        ));
    }

    lines.push(format!(
        "{} ({})",
        i18n::templates_label(locale),
        topology.templates.len()
    ));

    for template in topology.templates {
        lines.push(format!(
            "- {} | activation={} | boundary={} | scaffold_hook={}",
            template.plugin_id,
            template.activation.as_str(),
            template.load_boundary.as_str(),
            template.scaffold_hook_registered
        ));
    }

    lines.push(format!(
        "{} ({})",
        i18n::prompts_label(locale),
        topology.prompts.len()
    ));

    for prompt in topology.prompts {
        lines.push(format!(
            "- {} | activation={} | boundary={} | prompt_hook={}",
            prompt.plugin_id,
            prompt.activation.as_str(),
            prompt.load_boundary.as_str(),
            prompt.prompt_hook_registered
        ));
    }

    lines.push(format!(
        "{} ({})",
        i18n::agent_runtimes_label(locale),
        topology.agents.len()
    ));

    for agent in topology.agents {
        lines.push(format!(
            "- {} | activation={} | boundary={} | bootstrap_hook={}",
            agent.plugin_id,
            agent.activation.as_str(),
            agent.load_boundary.as_str(),
            agent.bootstrap_hook_registered
        ));
    }

    lines.push(format!(
        "{} ({})",
        i18n::checks_label(locale),
        topology.checks.len()
    ));

    for check in topology.checks {
        lines.push(format!(
            "- {} | plugin={} | activation={} | boundary={} | runtime_hook={}",
            check.kind.as_str(),
            check.plugin_id,
            check.activation.as_str(),
            check.load_boundary.as_str(),
            check.runtime_hook_registered
        ));
    }

    lines.push(format!(
        "{} ({})",
        i18n::providers_label(locale),
        topology.providers.len()
    ));

    for provider in topology.providers {
        lines.push(format!(
            "- {} | plugin={} | activation={} | boundary={} | registration_hook={}",
            provider.kind.as_str(),
            provider.plugin_id,
            provider.activation.as_str(),
            provider.load_boundary.as_str(),
            provider.registration_hook_registered
        ));
    }

    lines.push(format!(
        "{} ({})",
        i18n::policies_label(locale),
        topology.policies.len()
    ));

    for policy in topology.policies {
        lines.push(format!(
            "- {} | plugin={} | activation={} | boundary={} | enforcement_hook={}",
            policy.policy_id,
            policy.plugin_id,
            policy.activation.as_str(),
            policy.load_boundary.as_str(),
            policy.enforcement_hook_registered
        ));
    }

    lines.push(format!(
        "{} ({})",
        i18n::runtime_hooks_label(locale),
        topology.hooks.len()
    ));

    for hook in topology.hooks {
        lines.push(format!(
            "- {} | plugin={} | activation={} | boundary={}",
            hook.hook.as_str(),
            hook.plugin_id,
            hook.activation.as_str(),
            hook.load_boundary.as_str()
        ));
    }

    lines.push(format!(
        "{} ({})",
        i18n::mcp_servers_label(locale),
        topology.mcp_servers.len()
    ));

    for server in topology.mcp_servers {
        lines.push(format!(
            "- {} | enabled={} | process={} | availability={}",
            server.descriptor.id,
            server.enabled,
            server.descriptor.process_model().as_str(),
            server.descriptor.availability.as_str()
        ));
    }

    lines.join("\n")
}

/// Evaluates the current runtime status from the resolved topology.
#[must_use]
pub fn evaluate_runtime_status(topology: &RuntimeTopology<'_>) -> RuntimeStatus {
    let enabled_plugins = topology
        .plugins
        .iter()
        .filter(|plugin| plugin.is_enabled())
        .count();
    let disabled_plugins = topology.plugins.len() - enabled_plugins;
    let enabled_capabilities = topology
        .capabilities
        .iter()
        .filter(|capability| capability.is_enabled())
        .count();
    let disabled_capabilities = topology.capabilities.len() - enabled_capabilities;
    let enabled_templates = topology
        .templates
        .iter()
        .filter(|template| template.is_enabled())
        .count();
    let disabled_templates = topology.templates.len() - enabled_templates;
    let enabled_prompts = topology
        .prompts
        .iter()
        .filter(|prompt| prompt.is_enabled())
        .count();
    let disabled_prompts = topology.prompts.len() - enabled_prompts;
    let enabled_agents = topology
        .agents
        .iter()
        .filter(|agent| agent.is_enabled())
        .count();
    let disabled_agents = topology.agents.len() - enabled_agents;
    let enabled_checks = topology
        .checks
        .iter()
        .filter(|check| check.is_enabled())
        .count();
    let disabled_checks = topology.checks.len() - enabled_checks;
    let enabled_providers = topology
        .providers
        .iter()
        .filter(|provider| provider.is_enabled())
        .count();
    let disabled_providers = topology.providers.len() - enabled_providers;
    let enabled_policies = topology
        .policies
        .iter()
        .filter(|policy| policy.is_enabled())
        .count();
    let disabled_policies = topology.policies.len() - enabled_policies;
    let enabled_hooks = topology
        .hooks
        .iter()
        .filter(|hook| hook.is_enabled())
        .count();
    let disabled_hooks = topology.hooks.len() - enabled_hooks;
    let enabled_mcp_servers = topology
        .mcp_servers
        .iter()
        .filter(|server| server.enabled)
        .count();
    let disabled_mcp_servers = topology.mcp_servers.len() - enabled_mcp_servers;
    let health = if disabled_plugins == 0
        && disabled_capabilities == 0
        && disabled_templates == 0
        && disabled_prompts == 0
        && disabled_agents == 0
        && disabled_checks == 0
        && disabled_providers == 0
        && disabled_policies == 0
        && disabled_hooks == 0
        && disabled_mcp_servers == 0
    {
        RuntimeHealth::Healthy
    } else {
        RuntimeHealth::Degraded
    };

    RuntimeStatus {
        phase: topology.phase,
        health,
        enabled_plugins,
        disabled_plugins,
        enabled_capabilities,
        disabled_capabilities,
        enabled_templates,
        disabled_templates,
        enabled_prompts,
        disabled_prompts,
        enabled_agents,
        disabled_agents,
        enabled_checks,
        disabled_checks,
        enabled_providers,
        disabled_providers,
        enabled_policies,
        disabled_policies,
        enabled_hooks,
        disabled_hooks,
        enabled_mcp_servers,
        disabled_mcp_servers,
    }
}

/// Renders a human-readable runtime status summary.
#[must_use]
pub fn render_runtime_status(status: &RuntimeStatus) -> String {
    render_runtime_status_for_locale(status, "en")
}

/// Renders a human-readable runtime status summary for one locale.
#[must_use]
pub fn render_runtime_status_for_locale(status: &RuntimeStatus, locale: &str) -> String {
    [
        format!(
            "{}: {}",
            i18n::runtime_phase_label(locale),
            status.phase.as_str()
        ),
        format!(
            "{}: {}",
            i18n::runtime_health_label(locale),
            status.health.as_str()
        ),
        format!(
            "{}: enabled={}, disabled={}",
            i18n::plugins_label(locale),
            status.enabled_plugins,
            status.disabled_plugins
        ),
        format!(
            "{}: enabled={}, disabled={}",
            i18n::capabilities_label(locale),
            status.enabled_capabilities,
            status.disabled_capabilities
        ),
        format!(
            "{}: enabled={}, disabled={}",
            i18n::templates_label(locale),
            status.enabled_templates,
            status.disabled_templates
        ),
        format!(
            "{}: enabled={}, disabled={}",
            i18n::prompts_label(locale),
            status.enabled_prompts,
            status.disabled_prompts
        ),
        format!(
            "{}: enabled={}, disabled={}",
            i18n::agent_runtimes_label(locale),
            status.enabled_agents,
            status.disabled_agents
        ),
        format!(
            "{}: enabled={}, disabled={}",
            i18n::checks_label(locale),
            status.enabled_checks,
            status.disabled_checks
        ),
        format!(
            "{}: enabled={}, disabled={}",
            i18n::providers_label(locale),
            status.enabled_providers,
            status.disabled_providers
        ),
        format!(
            "{}: enabled={}, disabled={}",
            i18n::policies_label(locale),
            status.enabled_policies,
            status.disabled_policies
        ),
        format!(
            "{}: enabled={}, disabled={}",
            i18n::runtime_hooks_label(locale),
            status.enabled_hooks,
            status.disabled_hooks
        ),
        format!(
            "{}: enabled={}, disabled={}",
            i18n::mcp_servers_label(locale),
            status.enabled_mcp_servers,
            status.disabled_mcp_servers
        ),
    ]
    .join("\n")
}

/// Collects typed runtime issues from the resolved topology.
#[must_use]
pub fn collect_runtime_issues(topology: &RuntimeTopology<'_>) -> Vec<RuntimeIssue> {
    let mut issues = Vec::new();

    for plugin in topology.plugins {
        if !plugin.is_enabled() {
            issues.push(RuntimeIssue::new(
                RuntimeIssueKind::PluginDisabled,
                plugin.descriptor.id,
                "enable the plugin in typed project configuration",
            ));
        }
    }

    for capability in topology.capabilities {
        if !capability.is_enabled() {
            issues.push(RuntimeIssue::new(
                RuntimeIssueKind::CapabilityDisabled,
                capability.capability.as_str(),
                "enable the provider plugin that owns this capability",
            ));
        }
    }

    for template in topology.templates {
        if !template.is_enabled() {
            issues.push(RuntimeIssue::new(
                RuntimeIssueKind::TemplateDisabled,
                template.plugin_id,
                "enable the provider plugin that owns this template surface",
            ));
        }
    }

    for prompt in topology.prompts {
        if !prompt.is_enabled() {
            issues.push(RuntimeIssue::new(
                RuntimeIssueKind::PromptProviderDisabled,
                prompt.plugin_id,
                "enable the provider plugin that owns this prompt surface",
            ));
        }
    }

    for agent in topology.agents {
        if !agent.is_enabled() {
            issues.push(RuntimeIssue::new(
                RuntimeIssueKind::AgentRuntimeDisabled,
                agent.plugin_id,
                "enable the provider plugin that owns this agent runtime",
            ));
        }
    }

    for check in topology.checks {
        if !check.is_enabled() {
            issues.push(RuntimeIssue::new(
                RuntimeIssueKind::CheckDisabled,
                check.kind.as_str(),
                "enable the provider plugin that owns this runtime check",
            ));
        }
    }

    for provider in topology.providers {
        if !provider.is_enabled() {
            issues.push(RuntimeIssue::new(
                RuntimeIssueKind::ProviderDisabled,
                provider.kind.as_str(),
                "enable the provider plugin that owns this contribution",
            ));
        }
    }

    for policy in topology.policies {
        if !policy.is_enabled() {
            issues.push(RuntimeIssue::new(
                RuntimeIssueKind::PolicyDisabled,
                policy.policy_id,
                "enable the provider plugin that owns this policy",
            ));
        }
    }

    for hook in topology.hooks {
        if !hook.is_enabled() {
            issues.push(RuntimeIssue::new(
                RuntimeIssueKind::HookDisabled,
                hook.hook.as_str(),
                "enable the provider plugin that owns this runtime hook",
            ));
        }
    }

    for server in topology.mcp_servers {
        if !server.enabled {
            issues.push(RuntimeIssue::new(
                RuntimeIssueKind::McpServerDisabled,
                server.descriptor.id,
                "enable the owning plugin or opt in to the MCP server",
            ));
        }
    }

    issues
}

/// Renders a human-readable runtime issue summary.
#[must_use]
pub fn render_runtime_issues(issues: &[RuntimeIssue]) -> String {
    render_runtime_issues_for_locale(issues, "en")
}

/// Renders a human-readable runtime issue summary for one locale.
#[must_use]
pub fn render_runtime_issues_for_locale(issues: &[RuntimeIssue], locale: &str) -> String {
    if issues.is_empty() {
        return format!("{} (0)", i18n::runtime_issues_label(locale));
    }

    let mut lines = vec![format!(
        "{} ({})",
        i18n::runtime_issues_label(locale),
        issues.len()
    )];

    for issue in issues {
        lines.push(format!(
            "- {} | subject={} | action={}",
            issue.kind.as_str(),
            issue.subject,
            translate_runtime_reason(locale, issue.recommended_action)
        ));
    }

    lines.join("\n")
}

/// Builds a typed remediation plan from the resolved runtime topology.
#[must_use]
pub fn build_runtime_action_plan(topology: &RuntimeTopology<'_>) -> Vec<RuntimeAction> {
    let mut actions = Vec::new();

    for plugin in topology.plugins {
        if !plugin.is_enabled() {
            actions.push(RuntimeAction::new(
                RuntimeActionKind::EnablePlugin,
                plugin.descriptor.id,
                "the plugin is registered but disabled",
            ));
        }
    }

    for capability in topology.capabilities {
        if !capability.is_enabled() {
            actions.push(RuntimeAction::new(
                RuntimeActionKind::EnableCapabilityProvider,
                capability.plugin_id,
                format!(
                    "the provider still disables capability {}",
                    capability.capability.as_str()
                ),
            ));
        }
    }

    for template in topology.templates {
        if !template.is_enabled() {
            actions.push(RuntimeAction::new(
                RuntimeActionKind::EnableTemplateProvider,
                template.plugin_id,
                "the provider still disables the template surface",
            ));
        }
    }

    for prompt in topology.prompts {
        if !prompt.is_enabled() {
            actions.push(RuntimeAction::new(
                RuntimeActionKind::EnablePromptProvider,
                prompt.plugin_id,
                "the provider still disables the prompt surface",
            ));
        }
    }

    for agent in topology.agents {
        if !agent.is_enabled() {
            actions.push(RuntimeAction::new(
                RuntimeActionKind::EnableAgentRuntimeProvider,
                agent.plugin_id,
                "the provider still disables the agent runtime",
            ));
        }
    }

    for check in topology.checks {
        if !check.is_enabled() {
            actions.push(RuntimeAction::new(
                RuntimeActionKind::EnableCheckProvider,
                check.plugin_id,
                format!(
                    "the provider still disables runtime check {}",
                    check.kind.as_str()
                ),
            ));
        }
    }

    for provider in topology.providers {
        if !provider.is_enabled() {
            actions.push(RuntimeAction::new(
                RuntimeActionKind::EnableProvider,
                provider.plugin_id,
                format!(
                    "the provider still disables contribution {}",
                    provider.kind.as_str()
                ),
            ));
        }
    }

    for policy in topology.policies {
        if !policy.is_enabled() {
            actions.push(RuntimeAction::new(
                RuntimeActionKind::EnablePolicyProvider,
                policy.plugin_id,
                format!("the provider still disables policy {}", policy.policy_id),
            ));
        }
    }

    for hook in topology.hooks {
        if !hook.is_enabled() {
            actions.push(RuntimeAction::new(
                RuntimeActionKind::EnableHookProvider,
                hook.plugin_id,
                format!(
                    "the provider still disables runtime hook {}",
                    hook.hook.as_str()
                ),
            ));
        }
    }

    for server in topology.mcp_servers {
        if !server.enabled {
            actions.push(RuntimeAction::new(
                RuntimeActionKind::EnableMcpServer,
                server.descriptor.id,
                "the MCP contribution is registered but disabled",
            ));
        }
    }

    actions
}

/// Renders a human-readable runtime remediation plan.
#[must_use]
pub fn render_runtime_action_plan(actions: &[RuntimeAction]) -> String {
    render_runtime_action_plan_for_locale(actions, "en")
}

/// Renders a human-readable runtime remediation plan for one locale.
#[must_use]
pub fn render_runtime_action_plan_for_locale(actions: &[RuntimeAction], locale: &str) -> String {
    if actions.is_empty() {
        return format!("{} (0)", i18n::runtime_action_plan_label(locale));
    }

    let mut lines = vec![format!(
        "{} ({})",
        i18n::runtime_action_plan_label(locale),
        actions.len()
    )];

    for action in actions {
        lines.push(format!(
            "- {} | target={} | reason={}",
            action.kind.as_str(),
            action.target,
            translate_runtime_reason(locale, action.reason.as_str())
        ));
    }

    lines.join("\n")
}

/// Builds a typed runtime doctor report from the resolved topology.
#[must_use]
pub fn build_runtime_doctor_report(topology: &RuntimeTopology<'_>) -> RuntimeDoctorReport {
    let status = evaluate_runtime_status(topology);
    let issues = collect_runtime_issues(topology);
    let actions = build_runtime_action_plan(topology);

    RuntimeDoctorReport::new(status, issues, actions)
}

/// Renders a human-readable runtime doctor report.
#[must_use]
pub fn render_runtime_doctor_report(report: &RuntimeDoctorReport) -> String {
    render_runtime_doctor_report_for_locale(report, "en")
}

/// Renders a human-readable runtime doctor report for one locale.
#[must_use]
pub fn render_runtime_doctor_report_for_locale(
    report: &RuntimeDoctorReport,
    locale: &str,
) -> String {
    [
        i18n::runtime_doctor_label(locale).to_owned(),
        String::new(),
        render_runtime_status_for_locale(&report.status, locale),
        String::new(),
        render_runtime_issues_for_locale(&report.issues, locale),
        String::new(),
        render_runtime_action_plan_for_locale(&report.actions, locale),
    ]
    .join("\n")
}

fn translate_runtime_reason(locale: &str, reason: &str) -> String {
    i18n::translate_runtime_reason(locale, reason)
}

#[cfg(test)]
mod tests {
    use re_config::{ConfigScope, PluginActivation};
    use re_mcp::{McpAvailability, McpLaunchPolicy, McpServerDescriptor, McpTransport};
    use re_plugin::{
        PluginCapability, PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary,
        PluginLocalizedText, PluginRuntimeHook, PluginTrustLevel,
    };

    use super::{
        PRODUCT_NAME, PRODUCT_TAGLINE, RuntimeAction, RuntimeActionKind, RuntimeAgentRegistration,
        RuntimeCapabilityRegistration, RuntimeCheckKind, RuntimeCheckRegistration,
        RuntimeDoctorReport, RuntimeHealth, RuntimeHookRegistration, RuntimeIssue,
        RuntimeIssueKind, RuntimeMcpRegistration, RuntimePhase, RuntimePluginRegistration,
        RuntimePolicyRegistration, RuntimePromptRegistration, RuntimeProviderKind,
        RuntimeProviderRegistration, RuntimeStatus, RuntimeTemplateRegistration, RuntimeTopology,
        banner, build_runtime_action_plan, build_runtime_doctor_report, collect_runtime_issues,
        evaluate_runtime_status, render_runtime_action_plan, render_runtime_action_plan_for_locale,
        render_runtime_doctor_report, render_runtime_doctor_report_for_locale,
        render_runtime_issues, render_runtime_issues_for_locale, render_runtime_status,
        render_runtime_status_for_locale, render_runtime_topology,
        render_runtime_topology_for_locale,
    };

    const CAPABILITIES: &[PluginCapability] = &[PluginCapability::new("template")];
    const LIFECYCLE: &[PluginLifecycleStage] = &[PluginLifecycleStage::Discover];
    const HOOKS: &[PluginRuntimeHook] = &[PluginRuntimeHook::Scaffold];
    const LOCALIZED_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new("pt-br", "Básico")];
    const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
        "pt-br",
        "Plugin base para templates iniciais.",
    )];

    fn plugin_descriptor() -> PluginDescriptor {
        PluginDescriptor::new(
            "official.basic",
            PluginKind::Template,
            PluginTrustLevel::Official,
            "Basic",
            LOCALIZED_NAMES,
            "Foundation plugin for starter templates.",
            LOCALIZED_SUMMARIES,
            "0.2.0-alpha.1",
            CAPABILITIES,
            LIFECYCLE,
            PluginLoadBoundary::InProcess,
            HOOKS,
        )
    }

    fn mcp_descriptor() -> McpServerDescriptor {
        McpServerDescriptor::new(
            "official.codex.session",
            "official.codex",
            "Codex Session",
            McpTransport::Stdio,
            McpLaunchPolicy::PluginRuntime,
            McpAvailability::OnDemand,
        )
    }

    fn capability_registration() -> RuntimeCapabilityRegistration {
        RuntimeCapabilityRegistration::new(
            PluginCapability::new("template"),
            "official.basic",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
        )
    }

    fn template_registration() -> RuntimeTemplateRegistration {
        RuntimeTemplateRegistration::new(
            "official.basic",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )
    }

    fn prompt_registration() -> RuntimePromptRegistration {
        RuntimePromptRegistration::new(
            "official.bmad",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )
    }

    fn agent_registration() -> RuntimeAgentRegistration {
        RuntimeAgentRegistration::new(
            "official.codex",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )
    }

    fn hook_registration() -> RuntimeHookRegistration {
        RuntimeHookRegistration::new(
            PluginRuntimeHook::Scaffold,
            "official.basic",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
        )
    }

    fn policy_registration() -> RuntimePolicyRegistration {
        RuntimePolicyRegistration::new(
            "official.basic",
            "official.basic",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )
    }

    fn provider_registration() -> RuntimeProviderRegistration {
        RuntimeProviderRegistration::new(
            RuntimeProviderKind::DataSource,
            "official.basic",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )
    }

    fn check_registration() -> RuntimeCheckRegistration {
        RuntimeCheckRegistration::new(
            RuntimeCheckKind::Prepare,
            "official.bmad",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )
    }

    #[test]
    fn banner_includes_name_and_tagline() {
        // Arrange
        let expected = format!(
            "{PRODUCT_NAME}
{PRODUCT_TAGLINE}"
        );

        // Act
        let actual = banner();

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn runtime_phase_as_str_is_stable() {
        // Arrange
        let phases = [RuntimePhase::Bootstrapped, RuntimePhase::Ready];

        // Act
        let rendered = phases
            .into_iter()
            .map(RuntimePhase::as_str)
            .collect::<Vec<_>>();

        // Assert
        assert_eq!(rendered, vec!["bootstrapped", "ready"]);
    }

    #[test]
    fn runtime_health_as_str_is_stable() {
        // Arrange
        let values = [RuntimeHealth::Healthy, RuntimeHealth::Degraded];

        // Act
        let rendered = values
            .into_iter()
            .map(RuntimeHealth::as_str)
            .collect::<Vec<_>>();

        // Assert
        assert_eq!(rendered, vec!["healthy", "degraded"]);
    }

    #[test]
    fn runtime_issue_kind_as_str_is_stable() {
        // Arrange
        let values = [
            RuntimeIssueKind::PluginDisabled,
            RuntimeIssueKind::CapabilityDisabled,
            RuntimeIssueKind::TemplateDisabled,
            RuntimeIssueKind::PromptProviderDisabled,
            RuntimeIssueKind::AgentRuntimeDisabled,
            RuntimeIssueKind::CheckDisabled,
            RuntimeIssueKind::ProviderDisabled,
            RuntimeIssueKind::PolicyDisabled,
            RuntimeIssueKind::HookDisabled,
            RuntimeIssueKind::McpServerDisabled,
        ];

        // Act
        let rendered = values
            .into_iter()
            .map(RuntimeIssueKind::as_str)
            .collect::<Vec<_>>();

        // Assert
        assert_eq!(
            rendered,
            vec![
                "plugin_disabled",
                "capability_disabled",
                "template_disabled",
                "prompt_provider_disabled",
                "agent_runtime_disabled",
                "check_disabled",
                "provider_disabled",
                "policy_disabled",
                "hook_disabled",
                "mcp_server_disabled"
            ]
        );
    }

    #[test]
    fn runtime_action_kind_as_str_is_stable() {
        // Arrange
        let values = [
            RuntimeActionKind::EnablePlugin,
            RuntimeActionKind::EnableCapabilityProvider,
            RuntimeActionKind::EnableTemplateProvider,
            RuntimeActionKind::EnablePromptProvider,
            RuntimeActionKind::EnableAgentRuntimeProvider,
            RuntimeActionKind::EnableCheckProvider,
            RuntimeActionKind::EnableProvider,
            RuntimeActionKind::EnablePolicyProvider,
            RuntimeActionKind::EnableHookProvider,
            RuntimeActionKind::EnableMcpServer,
        ];

        // Act
        let rendered = values
            .into_iter()
            .map(RuntimeActionKind::as_str)
            .collect::<Vec<_>>();

        // Assert
        assert_eq!(
            rendered,
            vec![
                "enable_plugin",
                "enable_capability_provider",
                "enable_template_provider",
                "enable_prompt_provider",
                "enable_agent_runtime_provider",
                "enable_check_provider",
                "enable_provider",
                "enable_policy_provider",
                "enable_hook_provider",
                "enable_mcp_server",
            ]
        );
    }

    #[test]
    fn runtime_plugin_registration_tracks_enabled_state() {
        // Arrange
        let registration = RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Enabled,
            ConfigScope::BuiltInDefaults,
        );

        // Act
        let enabled = registration.is_enabled();

        // Assert
        assert!(enabled);
    }

    #[test]
    fn runtime_plugin_registration_tracks_disabled_state() {
        // Arrange
        let registration = RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Disabled,
            ConfigScope::BuiltInDefaults,
        );

        // Act
        let enabled = registration.is_enabled();

        // Assert
        assert!(!enabled);
    }

    #[test]
    fn runtime_capability_registration_tracks_enabled_state() {
        // Arrange
        let registration = capability_registration();

        // Act
        let enabled = registration.is_enabled();

        // Assert
        assert!(enabled);
    }

    #[test]
    fn runtime_hook_registration_tracks_enabled_state() {
        // Arrange
        let registration = hook_registration();

        // Act
        let enabled = registration.is_enabled();

        // Assert
        assert!(enabled);
    }

    #[test]
    fn runtime_policy_registration_tracks_enabled_state() {
        // Arrange
        let registration = policy_registration();

        // Act
        let enabled = registration.is_enabled();

        // Assert
        assert!(enabled);
    }

    #[test]
    fn runtime_provider_registration_tracks_enabled_state() {
        // Arrange
        let registration = provider_registration();

        // Act
        let enabled = registration.is_enabled();

        // Assert
        assert!(enabled);
    }

    #[test]
    fn runtime_check_registration_tracks_enabled_state() {
        // Arrange
        let registration = check_registration();

        // Act
        let enabled = registration.is_enabled();

        // Assert
        assert!(enabled);
    }

    #[test]
    fn render_runtime_topology_is_human_readable() {
        // Arrange
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Enabled,
            ConfigScope::BuiltInDefaults,
        )];
        let capabilities = [capability_registration()];
        let templates = [template_registration()];
        let prompts = [prompt_registration()];
        let agents = [agent_registration()];
        let checks = [check_registration()];
        let providers = [provider_registration()];
        let policies = [policy_registration()];
        let hooks = [hook_registration()];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), true)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
            templates: &templates,
            prompts: &prompts,
            agents: &agents,
            checks: &checks,
            providers: &providers,
            policies: &policies,
            hooks: &hooks,
            mcp_servers: &mcp_servers,
        };

        // Act
        let rendered = render_runtime_topology(&topology);

        // Assert
        assert!(rendered.contains("Runtime phase: ready"));
        assert!(rendered.contains("Locale: en"));
        assert!(rendered.contains("Plugins (1)"));
        assert!(rendered.contains(
            "- official.basic | activation=enabled | scope=built_in_defaults | boundary=in_process"
        ));
        assert!(rendered.contains("Capabilities (1)"));
        assert!(rendered.contains(
            "- template | plugin=official.basic | activation=enabled | boundary=in_process"
        ));
        assert!(rendered.contains("Templates (1)"));
        assert!(rendered.contains(
            "- official.basic | activation=enabled | boundary=in_process | scaffold_hook=true"
        ));
        assert!(rendered.contains("Prompts (1)"));
        assert!(rendered.contains(
            "- official.bmad | activation=enabled | boundary=in_process | prompt_hook=true"
        ));
        assert!(rendered.contains("Agent runtimes (1)"));
        assert!(rendered.contains(
            "- official.codex | activation=enabled | boundary=in_process | bootstrap_hook=true"
        ));
        assert!(rendered.contains("Checks (1)"));
        assert!(rendered.contains(
            "- prepare | plugin=official.bmad | activation=enabled | boundary=in_process | runtime_hook=true"
        ));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- data_source | plugin=official.basic | activation=enabled | boundary=in_process | registration_hook=true"
        ));
        assert!(rendered.contains("Policies (1)"));
        assert!(rendered.contains(
            "- official.basic | plugin=official.basic | activation=enabled | boundary=in_process | enforcement_hook=true"
        ));
        assert!(rendered.contains("Runtime hooks (1)"));
        assert!(rendered.contains(
            "- scaffold | plugin=official.basic | activation=enabled | boundary=in_process"
        ));
        assert!(rendered.contains("MCP servers (1)"));
        assert!(rendered.contains("- official.codex.session | enabled=true | process=plugin_managed | availability=on_demand"));
    }

    #[test]
    fn evaluate_runtime_status_reports_healthy_runtime() {
        // Arrange
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Enabled,
            ConfigScope::BuiltInDefaults,
        )];
        let capabilities = [capability_registration()];
        let templates = [template_registration()];
        let prompts = [prompt_registration()];
        let agents = [agent_registration()];
        let checks = [check_registration()];
        let providers = [provider_registration()];
        let policies = [policy_registration()];
        let hooks = [hook_registration()];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), true)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
            templates: &templates,
            prompts: &prompts,
            agents: &agents,
            checks: &checks,
            providers: &providers,
            policies: &policies,
            hooks: &hooks,
            mcp_servers: &mcp_servers,
        };

        // Act
        let status = evaluate_runtime_status(&topology);

        // Assert
        assert_eq!(status.health, RuntimeHealth::Healthy);
        assert_eq!(status.enabled_plugins, 1);
        assert_eq!(status.disabled_plugins, 0);
        assert_eq!(status.enabled_capabilities, 1);
        assert_eq!(status.disabled_capabilities, 0);
        assert_eq!(status.enabled_templates, 1);
        assert_eq!(status.disabled_templates, 0);
        assert_eq!(status.enabled_prompts, 1);
        assert_eq!(status.disabled_prompts, 0);
        assert_eq!(status.enabled_agents, 1);
        assert_eq!(status.disabled_agents, 0);
        assert_eq!(status.enabled_checks, 1);
        assert_eq!(status.disabled_checks, 0);
        assert_eq!(status.enabled_providers, 1);
        assert_eq!(status.disabled_providers, 0);
        assert_eq!(status.enabled_policies, 1);
        assert_eq!(status.disabled_policies, 0);
        assert_eq!(status.enabled_hooks, 1);
        assert_eq!(status.disabled_hooks, 0);
        assert_eq!(status.enabled_mcp_servers, 1);
        assert_eq!(status.disabled_mcp_servers, 0);
    }

    #[test]
    fn render_runtime_status_reports_degraded_runtime() {
        // Arrange
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Disabled,
            ConfigScope::BuiltInDefaults,
        )];
        let capabilities = [RuntimeCapabilityRegistration::new(
            PluginCapability::new("template"),
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
        )];
        let templates = [RuntimeTemplateRegistration::new(
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let prompts = [RuntimePromptRegistration::new(
            "official.bmad",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let checks = [RuntimeCheckRegistration::new(
            RuntimeCheckKind::Prepare,
            "official.bmad",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let agents = [RuntimeAgentRegistration::new(
            "official.codex",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let providers = [RuntimeProviderRegistration::new(
            RuntimeProviderKind::DataSource,
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let policies = [RuntimePolicyRegistration::new(
            "official.basic",
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let hooks = [RuntimeHookRegistration::new(
            PluginRuntimeHook::Scaffold,
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
        )];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), false)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Bootstrapped,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
            templates: &templates,
            prompts: &prompts,
            agents: &agents,
            checks: &checks,
            providers: &providers,
            policies: &policies,
            hooks: &hooks,
            mcp_servers: &mcp_servers,
        };

        // Act
        let status = evaluate_runtime_status(&topology);
        let rendered = render_runtime_status(&status);

        // Assert
        assert_eq!(status.health, RuntimeHealth::Degraded);
        assert!(rendered.contains("Runtime phase: bootstrapped"));
        assert!(rendered.contains("Runtime health: degraded"));
        assert!(rendered.contains("Plugins: enabled=0, disabled=1"));
        assert!(rendered.contains("Capabilities: enabled=0, disabled=1"));
        assert!(rendered.contains("Templates: enabled=0, disabled=1"));
        assert!(rendered.contains("Prompts: enabled=0, disabled=1"));
        assert!(rendered.contains("Agent runtimes: enabled=0, disabled=1"));
        assert!(rendered.contains("Checks: enabled=0, disabled=1"));
        assert!(rendered.contains("Providers: enabled=0, disabled=1"));
        assert!(rendered.contains("Policies: enabled=0, disabled=1"));
        assert!(rendered.contains("Runtime hooks: enabled=0, disabled=1"));
        assert!(rendered.contains("MCP servers: enabled=0, disabled=1"));
    }

    #[test]
    fn render_runtime_status_reports_degraded_runtime_with_enabled_plugins() {
        // Arrange
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Enabled,
            ConfigScope::BuiltInDefaults,
        )];
        let capabilities = [RuntimeCapabilityRegistration::new(
            PluginCapability::new("template"),
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
        )];
        let templates = [RuntimeTemplateRegistration::new(
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let prompts = [RuntimePromptRegistration::new(
            "official.bmad",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let checks = [RuntimeCheckRegistration::new(
            RuntimeCheckKind::Prepare,
            "official.bmad",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let agents = [RuntimeAgentRegistration::new(
            "official.codex",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let providers = [RuntimeProviderRegistration::new(
            RuntimeProviderKind::DataSource,
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let policies = [RuntimePolicyRegistration::new(
            "official.basic",
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let hooks = [hook_registration()];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), false)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
            templates: &templates,
            prompts: &prompts,
            agents: &agents,
            checks: &checks,
            providers: &providers,
            policies: &policies,
            hooks: &hooks,
            mcp_servers: &mcp_servers,
        };

        // Act
        let status = evaluate_runtime_status(&topology);
        let rendered = render_runtime_status(&status);

        // Assert
        assert_eq!(status.health, RuntimeHealth::Degraded);
        assert_eq!(status.enabled_plugins, 1);
        assert_eq!(status.disabled_plugins, 0);
        assert_eq!(status.enabled_capabilities, 0);
        assert_eq!(status.disabled_capabilities, 1);
        assert_eq!(status.enabled_templates, 0);
        assert_eq!(status.disabled_templates, 1);
        assert_eq!(status.enabled_prompts, 0);
        assert_eq!(status.disabled_prompts, 1);
        assert_eq!(status.enabled_agents, 0);
        assert_eq!(status.disabled_agents, 1);
        assert_eq!(status.enabled_checks, 0);
        assert_eq!(status.disabled_checks, 1);
        assert_eq!(status.enabled_providers, 0);
        assert_eq!(status.disabled_providers, 1);
        assert_eq!(status.enabled_policies, 0);
        assert_eq!(status.disabled_policies, 1);
        assert_eq!(status.enabled_hooks, 1);
        assert_eq!(status.disabled_hooks, 0);
        assert_eq!(status.enabled_mcp_servers, 0);
        assert_eq!(status.disabled_mcp_servers, 1);
        assert!(rendered.contains("Runtime phase: ready"));
        assert!(rendered.contains("Runtime health: degraded"));
        assert!(rendered.contains("Plugins: enabled=1, disabled=0"));
        assert!(rendered.contains("Capabilities: enabled=0, disabled=1"));
        assert!(rendered.contains("Templates: enabled=0, disabled=1"));
        assert!(rendered.contains("Prompts: enabled=0, disabled=1"));
        assert!(rendered.contains("Agent runtimes: enabled=0, disabled=1"));
        assert!(rendered.contains("Checks: enabled=0, disabled=1"));
        assert!(rendered.contains("Providers: enabled=0, disabled=1"));
        assert!(rendered.contains("Policies: enabled=0, disabled=1"));
        assert!(rendered.contains("Runtime hooks: enabled=1, disabled=0"));
        assert!(rendered.contains("MCP servers: enabled=0, disabled=1"));
    }

    #[test]
    fn collect_runtime_issues_reports_disabled_runtime_parts() {
        // Arrange
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Disabled,
            ConfigScope::BuiltInDefaults,
        )];
        let capabilities = [RuntimeCapabilityRegistration::new(
            PluginCapability::new("template"),
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
        )];
        let templates = [RuntimeTemplateRegistration::new(
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let prompts = [RuntimePromptRegistration::new(
            "official.bmad",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let checks = [RuntimeCheckRegistration::new(
            RuntimeCheckKind::Prepare,
            "official.bmad",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let agents = [RuntimeAgentRegistration::new(
            "official.codex",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let providers = [RuntimeProviderRegistration::new(
            RuntimeProviderKind::DataSource,
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let policies = [RuntimePolicyRegistration::new(
            "official.basic",
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let hooks = [RuntimeHookRegistration::new(
            PluginRuntimeHook::Scaffold,
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
        )];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), false)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Bootstrapped,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
            templates: &templates,
            prompts: &prompts,
            agents: &agents,
            checks: &checks,
            providers: &providers,
            policies: &policies,
            hooks: &hooks,
            mcp_servers: &mcp_servers,
        };

        // Act
        let issues = collect_runtime_issues(&topology);

        // Assert
        assert_eq!(
            issues,
            vec![
                RuntimeIssue::new(
                    RuntimeIssueKind::PluginDisabled,
                    "official.basic",
                    "enable the plugin in typed project configuration",
                ),
                RuntimeIssue::new(
                    RuntimeIssueKind::CapabilityDisabled,
                    "template",
                    "enable the provider plugin that owns this capability",
                ),
                RuntimeIssue::new(
                    RuntimeIssueKind::TemplateDisabled,
                    "official.basic",
                    "enable the provider plugin that owns this template surface",
                ),
                RuntimeIssue::new(
                    RuntimeIssueKind::PromptProviderDisabled,
                    "official.bmad",
                    "enable the provider plugin that owns this prompt surface",
                ),
                RuntimeIssue::new(
                    RuntimeIssueKind::AgentRuntimeDisabled,
                    "official.codex",
                    "enable the provider plugin that owns this agent runtime",
                ),
                RuntimeIssue::new(
                    RuntimeIssueKind::CheckDisabled,
                    "prepare",
                    "enable the provider plugin that owns this runtime check",
                ),
                RuntimeIssue::new(
                    RuntimeIssueKind::ProviderDisabled,
                    "data_source",
                    "enable the provider plugin that owns this contribution",
                ),
                RuntimeIssue::new(
                    RuntimeIssueKind::PolicyDisabled,
                    "official.basic",
                    "enable the provider plugin that owns this policy",
                ),
                RuntimeIssue::new(
                    RuntimeIssueKind::HookDisabled,
                    "scaffold",
                    "enable the provider plugin that owns this runtime hook",
                ),
                RuntimeIssue::new(
                    RuntimeIssueKind::McpServerDisabled,
                    "official.codex.session",
                    "enable the owning plugin or opt in to the MCP server",
                ),
            ]
        );
    }

    #[test]
    fn collect_runtime_issues_skips_enabled_runtime_parts() {
        // Arrange
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Enabled,
            ConfigScope::BuiltInDefaults,
        )];
        let capabilities = [capability_registration()];
        let templates = [template_registration()];
        let prompts = [prompt_registration()];
        let agents = [agent_registration()];
        let checks = [check_registration()];
        let providers = [provider_registration()];
        let policies = [policy_registration()];
        let hooks = [hook_registration()];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), true)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
            templates: &templates,
            prompts: &prompts,
            agents: &agents,
            checks: &checks,
            providers: &providers,
            policies: &policies,
            hooks: &hooks,
            mcp_servers: &mcp_servers,
        };

        // Act
        let issues = collect_runtime_issues(&topology);

        // Assert
        assert!(issues.is_empty());
    }

    #[test]
    fn render_runtime_issues_handles_empty_sets() {
        // Arrange
        let issues = [];

        // Act
        let rendered = render_runtime_issues(&issues);

        // Assert
        assert_eq!(rendered, "Runtime issues (0)");
    }

    #[test]
    fn render_runtime_issues_is_human_readable() {
        // Arrange
        let issues = [RuntimeIssue::new(
            RuntimeIssueKind::PluginDisabled,
            "official.github",
            "enable the plugin in typed project configuration",
        )];

        // Act
        let rendered = render_runtime_issues(&issues);

        // Assert
        assert!(rendered.contains("Runtime issues (1)"));
        assert!(rendered.contains(
            "- plugin_disabled | subject=official.github | action=enable the plugin in typed project configuration"
        ));
    }

    #[test]
    fn build_runtime_action_plan_maps_issues_to_typed_actions() {
        // Arrange
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Disabled,
            ConfigScope::BuiltInDefaults,
        )];
        let capabilities = [RuntimeCapabilityRegistration::new(
            PluginCapability::new("template"),
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
        )];
        let templates = [RuntimeTemplateRegistration::new(
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let prompts = [RuntimePromptRegistration::new(
            "official.bmad",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let checks = [RuntimeCheckRegistration::new(
            RuntimeCheckKind::Prepare,
            "official.bmad",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let agents = [RuntimeAgentRegistration::new(
            "official.codex",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let providers = [RuntimeProviderRegistration::new(
            RuntimeProviderKind::DataSource,
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let policies = [RuntimePolicyRegistration::new(
            "official.basic",
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let hooks = [RuntimeHookRegistration::new(
            PluginRuntimeHook::Scaffold,
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
        )];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), false)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Bootstrapped,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
            templates: &templates,
            prompts: &prompts,
            agents: &agents,
            checks: &checks,
            providers: &providers,
            policies: &policies,
            hooks: &hooks,
            mcp_servers: &mcp_servers,
        };

        // Act
        let actions = build_runtime_action_plan(&topology);

        // Assert
        assert_eq!(
            actions,
            vec![
                RuntimeAction::new(
                    RuntimeActionKind::EnablePlugin,
                    "official.basic",
                    "the plugin is registered but disabled",
                ),
                RuntimeAction::new(
                    RuntimeActionKind::EnableCapabilityProvider,
                    "official.basic",
                    "the provider still disables capability template",
                ),
                RuntimeAction::new(
                    RuntimeActionKind::EnableTemplateProvider,
                    "official.basic",
                    "the provider still disables the template surface",
                ),
                RuntimeAction::new(
                    RuntimeActionKind::EnablePromptProvider,
                    "official.bmad",
                    "the provider still disables the prompt surface",
                ),
                RuntimeAction::new(
                    RuntimeActionKind::EnableAgentRuntimeProvider,
                    "official.codex",
                    "the provider still disables the agent runtime",
                ),
                RuntimeAction::new(
                    RuntimeActionKind::EnableCheckProvider,
                    "official.bmad",
                    "the provider still disables runtime check prepare",
                ),
                RuntimeAction::new(
                    RuntimeActionKind::EnableProvider,
                    "official.basic",
                    "the provider still disables contribution data_source",
                ),
                RuntimeAction::new(
                    RuntimeActionKind::EnablePolicyProvider,
                    "official.basic",
                    "the provider still disables policy official.basic",
                ),
                RuntimeAction::new(
                    RuntimeActionKind::EnableHookProvider,
                    "official.basic",
                    "the provider still disables runtime hook scaffold",
                ),
                RuntimeAction::new(
                    RuntimeActionKind::EnableMcpServer,
                    "official.codex.session",
                    "the MCP contribution is registered but disabled",
                ),
            ]
        );
    }

    #[test]
    fn build_runtime_action_plan_skips_enabled_runtime_parts() {
        // Arrange
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Enabled,
            ConfigScope::BuiltInDefaults,
        )];
        let capabilities = [capability_registration()];
        let templates = [template_registration()];
        let prompts = [prompt_registration()];
        let agents = [agent_registration()];
        let checks = [check_registration()];
        let providers = [provider_registration()];
        let policies = [policy_registration()];
        let hooks = [hook_registration()];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), true)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
            templates: &templates,
            prompts: &prompts,
            agents: &agents,
            checks: &checks,
            providers: &providers,
            policies: &policies,
            hooks: &hooks,
            mcp_servers: &mcp_servers,
        };

        // Act
        let actions = build_runtime_action_plan(&topology);

        // Assert
        assert!(actions.is_empty());
    }

    #[test]
    fn render_runtime_action_plan_handles_empty_sets() {
        // Arrange
        let actions = [];

        // Act
        let rendered = render_runtime_action_plan(&actions);

        // Assert
        assert_eq!(rendered, "Runtime action plan (0)");
    }

    #[test]
    fn render_runtime_action_plan_is_human_readable() {
        // Arrange
        let actions = [RuntimeAction::new(
            RuntimeActionKind::EnablePlugin,
            "official.github",
            "the plugin is registered but disabled",
        )];

        // Act
        let rendered = render_runtime_action_plan(&actions);

        // Assert
        assert!(rendered.contains("Runtime action plan (1)"));
        assert!(rendered.contains(
            "- enable_plugin | target=official.github | reason=the plugin is registered but disabled"
        ));
    }

    #[test]
    fn build_runtime_doctor_report_collects_status_issues_and_actions() {
        // Arrange
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Disabled,
            ConfigScope::BuiltInDefaults,
        )];
        let capabilities = [RuntimeCapabilityRegistration::new(
            PluginCapability::new("template"),
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
        )];
        let templates = [RuntimeTemplateRegistration::new(
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let prompts = [RuntimePromptRegistration::new(
            "official.bmad",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let checks = [RuntimeCheckRegistration::new(
            RuntimeCheckKind::Prepare,
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let agents = [RuntimeAgentRegistration::new(
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let providers = [RuntimeProviderRegistration::new(
            RuntimeProviderKind::DataSource,
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let policies = [RuntimePolicyRegistration::new(
            "official.basic",
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];
        let hooks = [RuntimeHookRegistration::new(
            PluginRuntimeHook::Scaffold,
            "official.basic",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
        )];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), false)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
            templates: &templates,
            prompts: &prompts,
            agents: &agents,
            checks: &checks,
            providers: &providers,
            policies: &policies,
            hooks: &hooks,
            mcp_servers: &mcp_servers,
        };

        // Act
        let report = build_runtime_doctor_report(&topology);

        // Assert
        assert_eq!(
            report,
            RuntimeDoctorReport::new(
                evaluate_runtime_status(&topology),
                collect_runtime_issues(&topology),
                build_runtime_action_plan(&topology),
            )
        );
    }

    #[test]
    fn render_runtime_doctor_report_is_human_readable() {
        // Arrange
        let report = RuntimeDoctorReport::new(
            super::RuntimeStatus {
                phase: RuntimePhase::Ready,
                health: RuntimeHealth::Degraded,
                enabled_plugins: 1,
                disabled_plugins: 1,
                enabled_capabilities: 1,
                disabled_capabilities: 1,
                enabled_templates: 0,
                disabled_templates: 1,
                enabled_prompts: 0,
                disabled_prompts: 1,
                enabled_agents: 0,
                disabled_agents: 1,
                enabled_checks: 0,
                disabled_checks: 1,
                enabled_providers: 0,
                disabled_providers: 1,
                enabled_policies: 0,
                disabled_policies: 1,
                enabled_hooks: 1,
                disabled_hooks: 1,
                enabled_mcp_servers: 0,
                disabled_mcp_servers: 1,
            },
            vec![RuntimeIssue::new(
                RuntimeIssueKind::PluginDisabled,
                "official.github",
                "enable the plugin in typed project configuration",
            )],
            vec![RuntimeAction::new(
                RuntimeActionKind::EnablePlugin,
                "official.github",
                "the plugin is registered but disabled",
            )],
        );

        // Act
        let rendered = render_runtime_doctor_report(&report);

        // Assert
        assert!(rendered.contains("Runtime doctor"));
        assert!(rendered.contains("Runtime health: degraded"));
        assert!(rendered.contains("Runtime issues (1)"));
        assert!(rendered.contains("Runtime action plan (1)"));
    }

    #[test]
    fn render_runtime_topology_handles_empty_bootstrapped_state() {
        // Arrange
        let topology = RuntimeTopology {
            phase: RuntimePhase::Bootstrapped,
            locale: "en",
            plugins: &[],
            capabilities: &[],
            templates: &[],
            prompts: &[],
            agents: &[],
            checks: &[],
            providers: &[],
            policies: &[],
            hooks: &[],
            mcp_servers: &[],
        };

        // Act
        let rendered = render_runtime_topology(&topology);

        // Assert
        assert!(rendered.contains("Runtime phase: bootstrapped"));
        assert!(rendered.contains("Plugins (0)"));
        assert!(rendered.contains("Capabilities (0)"));
        assert!(rendered.contains("Templates (0)"));
        assert!(rendered.contains("Prompts (0)"));
        assert!(rendered.contains("Checks (0)"));
        assert!(rendered.contains("Providers (0)"));
        assert!(rendered.contains("Policies (0)"));
        assert!(rendered.contains("Runtime hooks (0)"));
        assert!(rendered.contains("MCP servers (0)"));
    }

    #[test]
    fn render_runtime_topology_supports_pt_br() {
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Enabled,
            ConfigScope::BuiltInDefaults,
        )];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "pt-br",
            plugins: &plugins,
            capabilities: &[],
            templates: &[],
            prompts: &[],
            agents: &[],
            checks: &[],
            providers: &[],
            policies: &[],
            hooks: &[],
            mcp_servers: &[],
        };

        let rendered = render_runtime_topology_for_locale(&topology, "pt-br");

        assert!(rendered.contains("Fase do runtime: ready"));
        assert!(rendered.contains("Idioma: pt-br"));
        assert!(rendered.contains("Plugins (1)"));
        assert!(rendered.contains("Runtimes de agente (0)"));
        assert!(rendered.contains("Provedores (0)"));
        assert!(rendered.contains("Hooks de runtime (0)"));
        assert!(rendered.contains("Servidores MCP (0)"));
    }

    #[test]
    fn render_runtime_status_supports_pt_br() {
        let status = RuntimeStatus {
            phase: RuntimePhase::Ready,
            health: RuntimeHealth::Degraded,
            enabled_plugins: 1,
            disabled_plugins: 2,
            enabled_capabilities: 0,
            disabled_capabilities: 1,
            enabled_templates: 0,
            disabled_templates: 1,
            enabled_prompts: 0,
            disabled_prompts: 1,
            enabled_agents: 0,
            disabled_agents: 1,
            enabled_checks: 0,
            disabled_checks: 1,
            enabled_providers: 0,
            disabled_providers: 1,
            enabled_policies: 0,
            disabled_policies: 1,
            enabled_hooks: 0,
            disabled_hooks: 1,
            enabled_mcp_servers: 0,
            disabled_mcp_servers: 1,
        };

        let rendered = render_runtime_status_for_locale(&status, "pt-br");

        assert!(rendered.contains("Fase do runtime: ready"));
        assert!(rendered.contains("Saúde do runtime: degraded"));
        assert!(rendered.contains("Runtimes de agente: enabled=0, disabled=1"));
        assert!(rendered.contains("Provedores: enabled=0, disabled=1"));
        assert!(rendered.contains("Hooks de runtime: enabled=0, disabled=1"));
        assert!(rendered.contains("Servidores MCP: enabled=0, disabled=1"));
    }

    #[test]
    fn render_runtime_issues_supports_pt_br() {
        let issues = [
            RuntimeIssue::new(
                RuntimeIssueKind::PluginDisabled,
                "official.github",
                "enable the plugin in typed project configuration",
            ),
            RuntimeIssue::new(
                RuntimeIssueKind::McpServerDisabled,
                "official.codex.session",
                "enable the owning plugin or opt in to the MCP server",
            ),
        ];

        let rendered = render_runtime_issues_for_locale(&issues, "pt-br");

        assert!(rendered.contains("Problemas do runtime (2)"));
        assert!(rendered.contains("action=ative o plugin na configuração tipada do projeto"));
        assert!(
            rendered.contains("action=ative o plugin responsável ou faça opt-in no servidor MCP")
        );
    }

    #[test]
    fn render_runtime_issues_handles_empty_sets_in_pt_br() {
        let issues = [];

        let rendered = render_runtime_issues_for_locale(&issues, "pt-br");

        assert_eq!(rendered, "Problemas do runtime (0)");
    }

    #[test]
    fn render_runtime_action_plan_supports_pt_br_for_dynamic_and_static_reasons() {
        let actions = [
            RuntimeAction::new(
                RuntimeActionKind::EnablePlugin,
                "official.github",
                "the plugin is registered but disabled",
            ),
            RuntimeAction::new(
                RuntimeActionKind::EnableCapabilityProvider,
                "official.basic",
                "the provider still disables capability template",
            ),
            RuntimeAction::new(
                RuntimeActionKind::EnableCheckProvider,
                "official.bmad",
                "the provider still disables runtime check prepare",
            ),
            RuntimeAction::new(
                RuntimeActionKind::EnableProvider,
                "official.basic",
                "the provider still disables contribution data_source",
            ),
            RuntimeAction::new(
                RuntimeActionKind::EnablePolicyProvider,
                "official.basic",
                "the provider still disables policy official.basic",
            ),
            RuntimeAction::new(
                RuntimeActionKind::EnableHookProvider,
                "official.basic",
                "the provider still disables runtime hook scaffold",
            ),
        ];

        let rendered = render_runtime_action_plan_for_locale(&actions, "pt-br");

        assert!(rendered.contains("Plano de ação do runtime (6)"));
        assert!(rendered.contains("reason=o plugin está registrado, mas desabilitado"));
        assert!(rendered.contains("reason=o provedor ainda desabilita a capacidade template"));
        assert!(
            rendered
                .contains("reason=o provedor ainda desabilita a verificação de runtime prepare")
        );
        assert!(rendered.contains("reason=o provedor ainda desabilita a contribuição data_source"));
        assert!(rendered.contains("reason=o provedor ainda desabilita a política official.basic"));
        assert!(rendered.contains("reason=o provedor ainda desabilita o hook de runtime scaffold"));
    }

    #[test]
    fn render_runtime_action_plan_handles_empty_sets_in_pt_br() {
        let actions = [];

        let rendered = render_runtime_action_plan_for_locale(&actions, "pt-br");

        assert_eq!(rendered, "Plano de ação do runtime (0)");
    }

    #[test]
    fn render_runtime_issues_supports_all_pt_br_fixed_recommendations() {
        let issues = [
            RuntimeIssue::new(
                RuntimeIssueKind::CapabilityDisabled,
                "template",
                "enable the provider plugin that owns this capability",
            ),
            RuntimeIssue::new(
                RuntimeIssueKind::TemplateDisabled,
                "official.basic",
                "enable the provider plugin that owns this template surface",
            ),
            RuntimeIssue::new(
                RuntimeIssueKind::PromptProviderDisabled,
                "official.bmad",
                "enable the provider plugin that owns this prompt surface",
            ),
            RuntimeIssue::new(
                RuntimeIssueKind::AgentRuntimeDisabled,
                "official.codex",
                "enable the provider plugin that owns this agent runtime",
            ),
            RuntimeIssue::new(
                RuntimeIssueKind::CheckDisabled,
                "prepare",
                "enable the provider plugin that owns this runtime check",
            ),
            RuntimeIssue::new(
                RuntimeIssueKind::ProviderDisabled,
                "data_source",
                "enable the provider plugin that owns this contribution",
            ),
            RuntimeIssue::new(
                RuntimeIssueKind::PolicyDisabled,
                "official.tdd-strict",
                "enable the provider plugin that owns this policy",
            ),
            RuntimeIssue::new(
                RuntimeIssueKind::HookDisabled,
                "scaffold",
                "enable the provider plugin that owns this runtime hook",
            ),
        ];

        let rendered = render_runtime_issues_for_locale(&issues, "pt-br");

        assert!(
            rendered.contains("action=ative o plugin provedor responsável por esta capacidade")
        );
        assert!(rendered.contains(
            "action=ative o plugin provedor responsável por esta superfície de template"
        ));
        assert!(
            rendered.contains(
                "action=ative o plugin provedor responsável por esta superfície de prompt"
            )
        );
        assert!(
            rendered
                .contains("action=ative o plugin provedor responsável por este runtime de agente")
        );
        assert!(rendered.contains(
            "action=ative o plugin provedor responsável por esta verificação de runtime"
        ));
        assert!(
            rendered.contains("action=ative o plugin provedor responsável por esta contribuição")
        );
        assert!(rendered.contains("action=ative o plugin provedor responsável por esta política"));
        assert!(
            rendered
                .contains("action=ative o plugin provedor responsável por este hook de runtime")
        );
    }

    #[test]
    fn render_runtime_action_plan_supports_remaining_pt_br_static_reasons() {
        let actions = [
            RuntimeAction::new(
                RuntimeActionKind::EnableTemplateProvider,
                "official.basic",
                "the provider still disables the template surface",
            ),
            RuntimeAction::new(
                RuntimeActionKind::EnablePromptProvider,
                "official.bmad",
                "the provider still disables the prompt surface",
            ),
            RuntimeAction::new(
                RuntimeActionKind::EnableAgentRuntimeProvider,
                "official.codex",
                "the provider still disables the agent runtime",
            ),
            RuntimeAction::new(
                RuntimeActionKind::EnableMcpServer,
                "official.codex.session",
                "the MCP contribution is registered but disabled",
            ),
        ];

        let rendered = render_runtime_action_plan_for_locale(&actions, "pt-br");

        assert!(rendered.contains("reason=o provedor ainda desabilita a superfície de template"));
        assert!(rendered.contains("reason=o provedor ainda desabilita a superfície de prompt"));
        assert!(rendered.contains("reason=o provedor ainda desabilita o runtime de agente"));
        assert!(rendered.contains("reason=a contribuição MCP está registrada, mas desabilitada"));
    }

    #[test]
    fn render_runtime_action_plan_falls_back_to_original_reason_when_no_translation_exists() {
        let actions = [RuntimeAction::new(
            RuntimeActionKind::EnablePlugin,
            "official.github",
            "custom untranslated reason",
        )];

        let rendered = render_runtime_action_plan_for_locale(&actions, "pt-br");

        assert!(rendered.contains("reason=custom untranslated reason"));
    }

    #[test]
    fn render_runtime_doctor_report_supports_pt_br() {
        let report = RuntimeDoctorReport::new(
            RuntimeStatus {
                phase: RuntimePhase::Ready,
                health: RuntimeHealth::Degraded,
                enabled_plugins: 0,
                disabled_plugins: 1,
                enabled_capabilities: 0,
                disabled_capabilities: 1,
                enabled_templates: 0,
                disabled_templates: 0,
                enabled_prompts: 0,
                disabled_prompts: 0,
                enabled_agents: 0,
                disabled_agents: 0,
                enabled_checks: 0,
                disabled_checks: 0,
                enabled_providers: 0,
                disabled_providers: 0,
                enabled_policies: 0,
                disabled_policies: 0,
                enabled_hooks: 0,
                disabled_hooks: 0,
                enabled_mcp_servers: 0,
                disabled_mcp_servers: 0,
            },
            vec![RuntimeIssue::new(
                RuntimeIssueKind::PluginDisabled,
                "official.github",
                "enable the plugin in typed project configuration",
            )],
            vec![RuntimeAction::new(
                RuntimeActionKind::EnablePlugin,
                "official.github",
                "the plugin is registered but disabled",
            )],
        );

        let rendered = render_runtime_doctor_report_for_locale(&report, "pt-br");

        assert!(rendered.contains("Diagnóstico do runtime"));
        assert!(rendered.contains("Problemas do runtime (1)"));
        assert!(rendered.contains("Plano de ação do runtime (1)"));
    }
}
