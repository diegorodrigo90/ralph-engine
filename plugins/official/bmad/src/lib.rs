//! Official BMAD workflow plugin metadata.

mod i18n;

use re_plugin::{
    DOCTOR_CHECKS, PREPARE_CHECKS, PROMPT_FRAGMENTS, PluginCheckAsset, PluginCheckDescriptor,
    PluginCheckKind, PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary,
    PluginLocalizedText, PluginPromptAsset, PluginPromptDescriptor, PluginRuntimeHook,
    PluginTemplateAsset, PluginTemplateDescriptor, PluginTrustLevel, TEMPLATE,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.bmad";
const PLUGIN_NAME: &str = i18n::default_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_names();
const PLUGIN_SUMMARY: &str = i18n::default_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] =
    &[TEMPLATE, PROMPT_FRAGMENTS, PREPARE_CHECKS, DOCTOR_CHECKS];
const LIFECYCLE: &[PluginLifecycleStage] = &[
    PluginLifecycleStage::Discover,
    PluginLifecycleStage::Configure,
    PluginLifecycleStage::Validate,
    PluginLifecycleStage::Load,
];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[
    PluginRuntimeHook::Scaffold,
    PluginRuntimeHook::PromptAssembly,
    PluginRuntimeHook::Prepare,
    PluginRuntimeHook::Doctor,
];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::Template,
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
    "official.bmad.starter",
    PLUGIN_ID,
    i18n::default_template_name(),
    i18n::localized_template_names(),
    i18n::default_template_summary(),
    i18n::localized_template_summaries(),
    TEMPLATE_ASSETS,
)];
const PROMPT_ASSETS: &[PluginPromptAsset] = &[PluginPromptAsset::new(
    "prompts/workflow.md",
    include_str!("../template/prompt.md"),
)];
const PROMPTS: &[PluginPromptDescriptor] = &[PluginPromptDescriptor::new(
    "official.bmad.workflow",
    PLUGIN_ID,
    i18n::default_prompt_name(),
    i18n::localized_prompt_names(),
    i18n::default_prompt_summary(),
    i18n::localized_prompt_summaries(),
    PROMPT_ASSETS,
)];
const PREPARE_CHECK_ASSETS: &[PluginCheckAsset] = &[PluginCheckAsset::new(
    "checks/prepare.md",
    include_str!("../checks/prepare.md"),
)];
const DOCTOR_CHECK_ASSETS: &[PluginCheckAsset] = &[PluginCheckAsset::new(
    "checks/doctor.md",
    include_str!("../checks/doctor.md"),
)];
const CHECKS: &[PluginCheckDescriptor] = &[
    PluginCheckDescriptor::new(
        "official.bmad.prepare",
        PLUGIN_ID,
        PluginCheckKind::Prepare,
        i18n::default_prepare_check_name(),
        i18n::localized_prepare_check_names(),
        i18n::default_prepare_check_summary(),
        i18n::localized_prepare_check_summaries(),
        PREPARE_CHECK_ASSETS,
    ),
    PluginCheckDescriptor::new(
        "official.bmad.doctor",
        PLUGIN_ID,
        PluginCheckKind::Doctor,
        i18n::default_doctor_check_name(),
        i18n::localized_doctor_check_names(),
        i18n::default_doctor_check_summary(),
        i18n::localized_doctor_check_summaries(),
        DOCTOR_CHECK_ASSETS,
    ),
];

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

/// Returns the immutable prompt contributions declared by the plugin.
#[must_use]
pub const fn prompts() -> &'static [PluginPromptDescriptor] {
    PROMPTS
}

/// Returns the immutable check contributions declared by the plugin.
#[must_use]
pub const fn checks() -> &'static [PluginCheckDescriptor] {
    CHECKS
}

#[cfg(test)]
mod tests {
    use super::{
        PLUGIN_ID, PLUGIN_SUMMARY, capabilities, checks, descriptor, i18n, lifecycle, prompts,
        runtime_hooks, templates,
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

        assert_eq!(template.id, "official.bmad.starter");
        assert_eq!(template.plugin_id, PLUGIN_ID);
        assert!(template.has_assets());
        assert_eq!(template.assets[3].path, ".ralph-engine/prompt.md");
        assert_eq!(template.display_name_for_locale("pt-br"), "Starter BMAD");
        assert_eq!(
            template.summary_for_locale("pt-br"),
            "Template inicial para projetos Ralph Engine guiados por BMAD."
        );
        assert_eq!(template.display_name_for_locale("es"), "BMAD starter");
        assert!(!template.assets[0].contents.contains("Placeholder"));
    }

    #[test]
    fn plugin_declares_prompt_contributions() {
        let prompt = prompts()[0];

        assert_eq!(prompt.id, "official.bmad.workflow");
        assert_eq!(prompt.plugin_id, PLUGIN_ID);
        assert!(prompt.has_assets());
        assert_eq!(prompt.assets[0].path, "prompts/workflow.md");
        assert_eq!(
            prompt.display_name_for_locale("pt-br"),
            "Prompt de workflow BMAD"
        );
        assert_eq!(
            prompt.summary_for_locale("pt-br"),
            "Pacote de prompts para montar workflows BMAD."
        );
        assert_eq!(
            prompt.summary_for_locale("es"),
            "Prompt bundle for BMAD workflow assembly."
        );
        assert_eq!(prompt.display_name_for_locale("es"), "BMAD workflow prompt");
    }

    #[test]
    fn plugin_declares_check_contributions() {
        let checks = checks();

        assert_eq!(checks.len(), 2);
        assert_eq!(checks[0].id, "official.bmad.prepare");
        assert_eq!(checks[0].kind.as_str(), "prepare");
        assert_eq!(
            checks[0].display_name_for_locale("pt-br"),
            "Verificação de preparo BMAD"
        );
        assert_eq!(checks[1].id, "official.bmad.doctor");
        assert_eq!(checks[1].kind.as_str(), "doctor");
        assert_eq!(
            checks[1].summary_for_locale("pt-br"),
            "Executa validação tipada de diagnóstico para workflows BMAD."
        );
        assert_eq!(
            checks[0].summary_for_locale("es"),
            "Runs typed prepare-time validation for BMAD workflows."
        );
        assert_eq!(checks[1].display_name_for_locale("es"), "BMAD doctor check");
        assert!(checks[0].has_assets());
        assert_eq!(checks[0].assets[0].path, "checks/prepare.md");
        assert!(
            checks[0].assets[0]
                .contents
                .contains("# BMAD Prepare Check")
        );
        assert!(checks[1].has_assets());
        assert_eq!(checks[1].assets[0].path, "checks/doctor.md");
        assert!(checks[1].assets[0].contents.contains("# BMAD Doctor Check"));
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: official.bmad"));
        assert!(manifest.contains("kind: template"));
        assert!(manifest.contains("- template"));
        assert!(manifest.contains("- prompt_fragments"));
        assert!(manifest.contains("- prepare_checks"));
        assert!(manifest.contains("- doctor_checks"));
        assert!(manifest.contains("id: official.bmad.starter"));
        assert!(manifest.contains("id: official.bmad.workflow"));
        assert!(manifest.contains("id: official.bmad.prepare"));
        assert!(manifest.contains("id: official.bmad.doctor"));
    }
}
