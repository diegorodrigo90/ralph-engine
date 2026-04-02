//! Official basic starter plugin metadata.

mod i18n;

use re_plugin::{
    PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary, PluginLocalizedText,
    PluginRuntimeHook, PluginTemplateAsset, PluginTemplateDescriptor, PluginTrustLevel, TEMPLATE,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.basic";
const PLUGIN_NAME: &str = i18n::default_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_names();
const PLUGIN_SUMMARY: &str = i18n::default_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[TEMPLATE];
const LIFECYCLE: &[PluginLifecycleStage] = &[
    PluginLifecycleStage::Discover,
    PluginLifecycleStage::Configure,
    PluginLifecycleStage::Load,
];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[PluginRuntimeHook::Scaffold];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::Template,
    PluginTrustLevel::Official,
    PLUGIN_NAME,
    LOCALIZED_NAMES,
    PLUGIN_SUMMARY,
    LOCALIZED_SUMMARIES,
    PLUGIN_VERSION,
    CAPABILITIES,
    LIFECYCLE,
    PluginLoadBoundary::InProcess,
    RUNTIME_HOOKS,
);
const TEMPLATE_ASSETS: &[PluginTemplateAsset] = &[
    PluginTemplateAsset::new(
        ".ralph-engine/README.md",
        include_str!("../template/README.md"),
    ),
    PluginTemplateAsset::new(
        ".ralph-engine/config.yaml",
        include_str!("../template/config.yaml"),
    ),
    PluginTemplateAsset::new(
        ".ralph-engine/hooks.yaml",
        include_str!("../template/hooks.yaml"),
    ),
    PluginTemplateAsset::new(
        ".ralph-engine/prompt.md",
        include_str!("../template/prompt.md"),
    ),
];
const TEMPLATES: &[PluginTemplateDescriptor] = &[PluginTemplateDescriptor::new(
    "official.basic.starter",
    PLUGIN_ID,
    i18n::default_template_name(),
    i18n::localized_template_names(),
    i18n::default_template_summary(),
    i18n::localized_template_summaries(),
    TEMPLATE_ASSETS,
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

/// Returns the immutable template contributions declared by the plugin.
#[must_use]
pub const fn templates() -> &'static [PluginTemplateDescriptor] {
    TEMPLATES
}

#[cfg(test)]
mod tests {
    use super::{
        PLUGIN_ID, PLUGIN_SUMMARY, capabilities, descriptor, i18n, lifecycle, runtime_hooks,
        templates,
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
            && plugin.name == i18n::en::PLUGIN_LOCALE.plugin_name
            && plugin.display_name_for_locale("pt-br") == i18n::pt_br::PLUGIN_LOCALE.plugin_name
            && plugin.summary_for_locale("pt-br") == i18n::pt_br::PLUGIN_LOCALE.plugin_summary
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
    fn plugin_declares_template_contributions() {
        let template = templates()[0];

        assert_eq!(template.id, "official.basic.starter");
        assert_eq!(template.plugin_id, PLUGIN_ID);
        assert!(template.has_assets());
        assert_eq!(template.assets[0].path, ".ralph-engine/README.md");
        assert_eq!(template.display_name_for_locale("pt-br"), "Starter básico");
        assert_eq!(
            template.summary_for_locale("pt-br"),
            "Template inicial para novos projetos Ralph Engine."
        );
        assert_eq!(
            template.summary_for_locale("es"),
            "Starter template for new Ralph Engine projects."
        );
        assert_eq!(template.display_name_for_locale("es"), "Basic starter");
        assert!(!template.assets[0].contents.contains("Placeholder"));
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: official.basic"));
        assert!(manifest.contains("kind: template"));
        assert!(manifest.contains("trust_level: official"));
        assert!(manifest.contains("- template"));
        assert!(manifest.contains("id: official.basic.starter"));
    }
}
