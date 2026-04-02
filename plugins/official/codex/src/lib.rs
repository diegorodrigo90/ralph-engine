//! Official Codex runtime plugin metadata.

use re_mcp::{McpAvailability, McpLaunchPolicy, McpServerDescriptor, McpTransport};
use re_plugin::{
    AGENT_RUNTIME, MCP_CONTRIBUTION, PluginDescriptor, PluginKind, PluginLifecycleStage,
    PluginLoadBoundary, PluginRuntimeHook,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.codex";
const PLUGIN_NAME: &str = "Codex";
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[AGENT_RUNTIME, MCP_CONTRIBUTION];
const LIFECYCLE: &[PluginLifecycleStage] =
    &[PluginLifecycleStage::Discover, PluginLifecycleStage::Load];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[
    PluginRuntimeHook::AgentBootstrap,
    PluginRuntimeHook::McpRegistration,
];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::AgentRuntime,
    PLUGIN_NAME,
    PLUGIN_VERSION,
    CAPABILITIES,
    LIFECYCLE,
    PluginLoadBoundary::InProcess,
    RUNTIME_HOOKS,
);
const MCP_SERVERS: &[McpServerDescriptor] = &[McpServerDescriptor::new(
    "official.codex.session",
    PLUGIN_ID,
    "Codex Session",
    McpTransport::Stdio,
    McpLaunchPolicy::PluginRuntime,
    McpAvailability::OnDemand,
)];

/// Declared capabilities for the official plugin foundation.
#[must_use]
pub fn capabilities() -> &'static [re_plugin::PluginCapability] {
    DESCRIPTOR.capabilities
}

/// Declared lifecycle stages for the official plugin foundation.
#[must_use]
pub fn lifecycle() -> &'static [PluginLifecycleStage] {
    DESCRIPTOR.lifecycle
}

/// Declared runtime hooks for the official plugin foundation.
#[must_use]
pub fn runtime_hooks() -> &'static [PluginRuntimeHook] {
    DESCRIPTOR.runtime_hooks
}

/// Returns the immutable plugin descriptor.
#[must_use]
pub const fn descriptor() -> PluginDescriptor {
    DESCRIPTOR
}

/// Returns the immutable MCP server contributions declared by the plugin.
#[must_use]
pub const fn mcp_servers() -> &'static [McpServerDescriptor] {
    MCP_SERVERS
}

#[cfg(test)]
mod tests {
    use super::{PLUGIN_ID, capabilities, descriptor, lifecycle, mcp_servers, runtime_hooks};

    #[test]
    fn plugin_id_is_namespaced() {
        // Arrange
        let plugin_id = PLUGIN_ID;

        // Act
        let is_namespaced = plugin_id.starts_with("official.");

        // Assert
        assert!(is_namespaced);
    }

    #[test]
    fn plugin_declares_at_least_one_capability() {
        // Arrange
        let declared_capabilities = capabilities();

        // Act
        let has_capabilities = !declared_capabilities.is_empty();

        // Assert
        assert!(has_capabilities);
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        // Arrange
        let plugin = descriptor();

        // Act
        let descriptor_matches = plugin.id == PLUGIN_ID && plugin.name == "Codex";

        // Assert
        assert!(descriptor_matches);
    }

    #[test]
    fn plugin_declares_mcp_server_contributions() {
        // Arrange
        let servers = mcp_servers();

        // Act
        let contributes_servers = !servers.is_empty() && servers[0].plugin_id == PLUGIN_ID;

        // Assert
        assert!(contributes_servers);
    }

    #[test]
    fn plugin_declares_lifecycle_stages() {
        // Arrange
        let declared_lifecycle = lifecycle();

        // Act
        let has_lifecycle = !declared_lifecycle.is_empty();

        // Assert
        assert!(has_lifecycle);
    }

    #[test]
    fn plugin_declares_runtime_hooks() {
        // Arrange
        let declared_hooks = runtime_hooks();

        // Act
        let has_hooks = !declared_hooks.is_empty();

        // Assert
        assert!(has_hooks);
    }
}
