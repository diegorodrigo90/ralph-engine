//! Shared product metadata for Ralph Engine.

use re_config::{ConfigScope, PluginActivation};
use re_mcp::McpServerDescriptor;
use re_plugin::{PluginCapability, PluginDescriptor, PluginLoadBoundary, PluginRuntimeHook};

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
    let mut lines = vec![
        format!("Runtime phase: {}", topology.phase.as_str()),
        format!("Locale: {}", topology.locale),
        format!("Plugins ({})", topology.plugins.len()),
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

    lines.push(format!("Capabilities ({})", topology.capabilities.len()));

    for capability in topology.capabilities {
        lines.push(format!(
            "- {} | plugin={} | activation={} | boundary={}",
            capability.capability.as_str(),
            capability.plugin_id,
            capability.activation.as_str(),
            capability.load_boundary.as_str()
        ));
    }

    lines.push(format!("Policies ({})", topology.policies.len()));

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

    lines.push(format!("Runtime hooks ({})", topology.hooks.len()));

    for hook in topology.hooks {
        lines.push(format!(
            "- {} | plugin={} | activation={} | boundary={}",
            hook.hook.as_str(),
            hook.plugin_id,
            hook.activation.as_str(),
            hook.load_boundary.as_str()
        ));
    }

    lines.push(format!("MCP servers ({})", topology.mcp_servers.len()));

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
    [
        format!("Runtime phase: {}", status.phase.as_str()),
        format!("Runtime health: {}", status.health.as_str()),
        format!(
            "Plugins: enabled={}, disabled={}",
            status.enabled_plugins, status.disabled_plugins
        ),
        format!(
            "Capabilities: enabled={}, disabled={}",
            status.enabled_capabilities, status.disabled_capabilities
        ),
        format!(
            "Policies: enabled={}, disabled={}",
            status.enabled_policies, status.disabled_policies
        ),
        format!(
            "Runtime hooks: enabled={}, disabled={}",
            status.enabled_hooks, status.disabled_hooks
        ),
        format!(
            "MCP servers: enabled={}, disabled={}",
            status.enabled_mcp_servers, status.disabled_mcp_servers
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
    if issues.is_empty() {
        return "Runtime issues (0)".to_owned();
    }

    let mut lines = vec![format!("Runtime issues ({})", issues.len())];

    for issue in issues {
        lines.push(format!(
            "- {} | subject={} | action={}",
            issue.kind.as_str(),
            issue.subject,
            issue.recommended_action
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
    if actions.is_empty() {
        return "Runtime action plan (0)".to_owned();
    }

    let mut lines = vec![format!("Runtime action plan ({})", actions.len())];

    for action in actions {
        lines.push(format!(
            "- {} | target={} | reason={}",
            action.kind.as_str(),
            action.target,
            action.reason
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
    [
        "Runtime doctor".to_owned(),
        String::new(),
        render_runtime_status(&report.status),
        String::new(),
        render_runtime_issues(&report.issues),
        String::new(),
        render_runtime_action_plan(&report.actions),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use re_config::{ConfigScope, PluginActivation};
    use re_mcp::{McpAvailability, McpLaunchPolicy, McpServerDescriptor, McpTransport};
    use re_plugin::{
        PluginCapability, PluginDescriptor, PluginLifecycleStage, PluginLoadBoundary,
        PluginRuntimeHook,
    };

    use super::{
        PRODUCT_NAME, PRODUCT_TAGLINE, RuntimeAction, RuntimeActionKind,
        RuntimeCapabilityRegistration, RuntimeDoctorReport, RuntimeHealth, RuntimeHookRegistration,
        RuntimeIssue, RuntimeIssueKind, RuntimeMcpRegistration, RuntimePhase,
        RuntimePluginRegistration, RuntimePolicyRegistration, RuntimeTopology, banner,
        build_runtime_action_plan, build_runtime_doctor_report, collect_runtime_issues,
        evaluate_runtime_status, render_runtime_action_plan, render_runtime_doctor_report,
        render_runtime_issues, render_runtime_status, render_runtime_topology,
    };

    const CAPABILITIES: &[PluginCapability] = &[PluginCapability::new("template")];
    const LIFECYCLE: &[PluginLifecycleStage] = &[PluginLifecycleStage::Discover];
    const HOOKS: &[PluginRuntimeHook] = &[PluginRuntimeHook::Scaffold];

    fn plugin_descriptor() -> PluginDescriptor {
        PluginDescriptor::new(
            "official.basic",
            "Basic",
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
    fn render_runtime_topology_is_human_readable() {
        // Arrange
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Enabled,
            ConfigScope::BuiltInDefaults,
        )];
        let capabilities = [capability_registration()];
        let policies = [policy_registration()];
        let hooks = [hook_registration()];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), true)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
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
        let policies = [policy_registration()];
        let hooks = [hook_registration()];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), true)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
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
        let policies = [policy_registration()];
        let hooks = [hook_registration()];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), true)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
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
        let policies = [policy_registration()];
        let hooks = [hook_registration()];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), true)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
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
        assert!(rendered.contains("Policies (0)"));
        assert!(rendered.contains("Runtime hooks (0)"));
        assert!(rendered.contains("MCP servers (0)"));
    }
}
