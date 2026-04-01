//! Shared product metadata for Ralph Engine.

use re_config::{ConfigScope, PluginActivation};
use re_mcp::McpServerDescriptor;
use re_plugin::{PluginCapability, PluginDescriptor, PluginLoadBoundary};

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
    /// Number of enabled MCP servers.
    pub enabled_mcp_servers: usize,
    /// Number of disabled MCP servers.
    pub disabled_mcp_servers: usize,
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
    let enabled_mcp_servers = topology
        .mcp_servers
        .iter()
        .filter(|server| server.enabled)
        .count();
    let disabled_mcp_servers = topology.mcp_servers.len() - enabled_mcp_servers;
    let health = if disabled_plugins == 0 && disabled_mcp_servers == 0 {
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
            "MCP servers: enabled={}, disabled={}",
            status.enabled_mcp_servers, status.disabled_mcp_servers
        ),
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
        PRODUCT_NAME, PRODUCT_TAGLINE, RuntimeCapabilityRegistration, RuntimeHealth,
        RuntimeMcpRegistration, RuntimePhase, RuntimePluginRegistration, RuntimeTopology, banner,
        evaluate_runtime_status, render_runtime_status, render_runtime_topology,
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
    fn render_runtime_topology_is_human_readable() {
        // Arrange
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Enabled,
            ConfigScope::BuiltInDefaults,
        )];
        let capabilities = [capability_registration()];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), true)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
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
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), true)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
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
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), false)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Bootstrapped,
            locale: "en",
            plugins: &plugins,
            capabilities: &capabilities,
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
        assert!(rendered.contains("MCP servers: enabled=0, disabled=1"));
    }

    #[test]
    fn render_runtime_topology_handles_empty_bootstrapped_state() {
        // Arrange
        let topology = RuntimeTopology {
            phase: RuntimePhase::Bootstrapped,
            locale: "en",
            plugins: &[],
            capabilities: &[],
            mcp_servers: &[],
        };

        // Act
        let rendered = render_runtime_topology(&topology);

        // Assert
        assert!(rendered.contains("Runtime phase: bootstrapped"));
        assert!(rendered.contains("Plugins (0)"));
        assert!(rendered.contains("Capabilities (0)"));
        assert!(rendered.contains("MCP servers (0)"));
    }
}
