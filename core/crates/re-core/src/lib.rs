//! Shared product metadata for Ralph Engine.

use re_config::{ConfigScope, PluginActivation};
use re_mcp::McpServerDescriptor;
use re_plugin::PluginDescriptor;

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

/// Immutable snapshot of the resolved runtime topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeTopology<'a> {
    /// Resolved runtime phase.
    pub phase: RuntimePhase,
    /// Effective runtime locale.
    pub locale: &'static str,
    /// Resolved plugin registrations.
    pub plugins: &'a [RuntimePluginRegistration],
    /// Resolved MCP registrations.
    pub mcp_servers: &'a [RuntimeMcpRegistration],
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

    lines.push(format!("MCP servers ({})", topology.mcp_servers.len()));

    for server in topology.mcp_servers {
        lines.push(format!(
            "- {} | enabled={} | process={} | availability={}",
            server.descriptor.id,
            server.enabled,
            server.descriptor.process_model.as_str(),
            server.descriptor.availability.as_str()
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use re_config::{ConfigScope, PluginActivation};
    use re_mcp::{McpAvailability, McpProcessModel, McpServerDescriptor, McpTransport};
    use re_plugin::{
        PluginCapability, PluginDescriptor, PluginLifecycleStage, PluginLoadBoundary,
        PluginRuntimeHook,
    };

    use super::{
        PRODUCT_NAME, PRODUCT_TAGLINE, RuntimeMcpRegistration, RuntimePhase,
        RuntimePluginRegistration, RuntimeTopology, banner, render_runtime_topology,
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
            McpProcessModel::PluginManaged,
            McpAvailability::OnDemand,
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
    fn render_runtime_topology_is_human_readable() {
        // Arrange
        let plugins = [RuntimePluginRegistration::new(
            plugin_descriptor(),
            PluginActivation::Enabled,
            ConfigScope::BuiltInDefaults,
        )];
        let mcp_servers = [RuntimeMcpRegistration::new(mcp_descriptor(), true)];
        let topology = RuntimeTopology {
            phase: RuntimePhase::Ready,
            locale: "en",
            plugins: &plugins,
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
        assert!(rendered.contains("MCP servers (1)"));
        assert!(rendered.contains("- official.codex.session | enabled=true | process=plugin_managed | availability=on_demand"));
    }

    #[test]
    fn render_runtime_topology_handles_empty_bootstrapped_state() {
        // Arrange
        let topology = RuntimeTopology {
            phase: RuntimePhase::Bootstrapped,
            locale: "en",
            plugins: &[],
            mcp_servers: &[],
        };

        // Act
        let rendered = render_runtime_topology(&topology);

        // Assert
        assert!(rendered.contains("Runtime phase: bootstrapped"));
        assert!(rendered.contains("Plugins (0)"));
        assert!(rendered.contains("MCP servers (0)"));
    }
}
