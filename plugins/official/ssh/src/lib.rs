//! Official SSH remote-control plugin metadata.

use re_plugin::{PluginDescriptor, REMOTE_CONTROL};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.ssh";
const PLUGIN_NAME: &str = "SSH";
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[REMOTE_CONTROL];
const DESCRIPTOR: PluginDescriptor =
    PluginDescriptor::new(PLUGIN_ID, PLUGIN_NAME, PLUGIN_VERSION, CAPABILITIES);

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

#[cfg(test)]
mod tests {
    use super::{PLUGIN_ID, capabilities, descriptor};

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
        let descriptor_matches = plugin.id == PLUGIN_ID && plugin.name == "SSH";

        // Assert
        assert!(descriptor_matches);
    }
}
