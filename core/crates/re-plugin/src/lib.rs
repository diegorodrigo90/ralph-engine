//! Shared plugin contracts for Ralph Engine.

/// Immutable metadata for a Ralph Engine plugin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginDescriptor {
    /// Stable plugin identifier.
    pub id: &'static str,
    /// Human-readable plugin name.
    pub name: &'static str,
    /// Published plugin version.
    pub version: &'static str,
    /// Declared plugin capabilities.
    pub capabilities: &'static [&'static str],
}

impl PluginDescriptor {
    /// Creates a new immutable plugin descriptor.
    #[must_use]
    pub const fn new(
        id: &'static str,
        name: &'static str,
        version: &'static str,
        capabilities: &'static [&'static str],
    ) -> Self {
        Self {
            id,
            name,
            version,
            capabilities,
        }
    }

    /// Returns whether the plugin identifier uses a namespace prefix.
    #[must_use]
    pub fn is_namespaced(&self) -> bool {
        self.id.contains('.')
    }

    /// Returns whether the plugin declares at least one capability.
    #[must_use]
    pub fn has_capabilities(&self) -> bool {
        !self.capabilities.is_empty()
    }
}

/// Renders a human-readable plugin listing.
#[must_use]
pub fn render_plugin_listing(plugins: &[PluginDescriptor]) -> String {
    let mut lines = Vec::with_capacity(plugins.len() + 1);
    lines.push(format!("Official plugins ({})", plugins.len()));

    for plugin in plugins {
        lines.push(format!(
            "- {} | {} | v{} | {}",
            plugin.id,
            plugin.name,
            plugin.version,
            plugin.capabilities.join(", ")
        ));
    }

    lines.join("\n")
}
