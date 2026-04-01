//! Official GitHub integration plugin metadata.

use re_mcp::{McpServerDescriptor, McpTransport};
use re_plugin::{
    CONTEXT_PROVIDER, DATA_SOURCE, FORGE_PROVIDER, MCP_CONTRIBUTION, PluginDescriptor,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.github";
const PLUGIN_NAME: &str = "GitHub";
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[
    DATA_SOURCE,
    CONTEXT_PROVIDER,
    FORGE_PROVIDER,
    MCP_CONTRIBUTION,
];
const DESCRIPTOR: PluginDescriptor =
    PluginDescriptor::new(PLUGIN_ID, PLUGIN_NAME, PLUGIN_VERSION, CAPABILITIES);
const MCP_SERVERS: &[McpServerDescriptor] = &[McpServerDescriptor::new(
    "official.github.repository",
    PLUGIN_ID,
    "GitHub Repository",
    McpTransport::Stdio,
)];

/// Declared capabilities for the official plugin foundation.
#[must_use]
pub fn capabilities() -> &'static [re_plugin::PluginCapability] {
    DESCRIPTOR.capabilities
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
    use super::{PLUGIN_ID, capabilities, descriptor, mcp_servers};

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
        let descriptor_matches = plugin.id == PLUGIN_ID && plugin.name == "GitHub";

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
}
