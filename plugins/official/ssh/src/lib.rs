//! Official SSH remote-control plugin metadata.

mod i18n;

use re_plugin::{
    PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary, PluginLocalizedText,
    PluginProviderDescriptor, PluginProviderKind, PluginRuntimeHook, PluginTrustLevel,
    REMOTE_CONTROL,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.ssh";
const PLUGIN_NAME: &str = i18n::plugin_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_plugin_names();
const PLUGIN_SUMMARY: &str = i18n::plugin_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_plugin_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[REMOTE_CONTROL];
const LIFECYCLE: &[PluginLifecycleStage] =
    &[PluginLifecycleStage::Discover, PluginLifecycleStage::Load];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[PluginRuntimeHook::RemoteControlBootstrap];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::RemoteControl,
    PluginTrustLevel::Official,
    PLUGIN_NAME,
    LOCALIZED_NAMES,
    PLUGIN_SUMMARY,
    LOCALIZED_SUMMARIES,
    PLUGIN_VERSION,
    re_plugin::CURRENT_PLUGIN_API_VERSION,
    CAPABILITIES,
    LIFECYCLE,
    PluginLoadBoundary::InProcess,
    RUNTIME_HOOKS,
);
const PROVIDERS: &[PluginProviderDescriptor] = &[PluginProviderDescriptor::new(
    "official.ssh.remote",
    PLUGIN_ID,
    PluginProviderKind::RemoteControl,
    i18n::remote_control_name(),
    i18n::localized_remote_control_names(),
    i18n::remote_control_summary(),
    i18n::localized_remote_control_summaries(),
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

/// Returns the immutable provider contributions declared by the plugin.
#[must_use]
pub const fn providers() -> &'static [PluginProviderDescriptor] {
    PROVIDERS
}

#[cfg(test)]
mod tests {
    use super::{
        PLUGIN_ID, PLUGIN_SUMMARY, capabilities, descriptor, i18n, lifecycle, providers,
        runtime_hooks,
    };

    fn manifest_document() -> &'static str {
        include_str!("../manifest.yaml")
    }

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
        let descriptor_matches = plugin.id == PLUGIN_ID
            && plugin.name == i18n::plugin_name()
            && plugin.display_name_for_locale("pt-br") == "SSH"
            && plugin.summary_for_locale("pt-br") == "Integração de controle remoto por SSH."
            && plugin.summary_for_locale("es") == PLUGIN_SUMMARY;

        // Assert
        assert!(descriptor_matches);
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

    #[test]
    fn plugin_declares_provider_contributions() {
        let providers = providers();

        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].id, "official.ssh.remote");
        assert_eq!(providers[0].kind.as_str(), "remote_control");
        assert_eq!(
            providers[0].display_name_for_locale("pt-br"),
            "Controle remoto SSH"
        );
        assert_eq!(
            providers[0].summary_for_locale("es"),
            i18n::remote_control_summary()
        );
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: official.ssh"));
        assert!(manifest.contains("kind: remote_control"));
        assert!(manifest.contains("- remote_control"));
        assert!(manifest.contains("id: official.ssh.remote"));
        assert!(manifest.contains("trust_level: official"));
    }
}
