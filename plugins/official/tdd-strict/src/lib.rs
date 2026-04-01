//! Official TDD strict policy plugin metadata.

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.tdd-strict";

/// Declared capabilities for the official plugin foundation.
#[must_use]
pub fn capabilities() -> &'static [&'static str] {
    &["template", "policy"]
}

#[cfg(test)]
mod tests {
    use super::{PLUGIN_ID, capabilities};

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
}
