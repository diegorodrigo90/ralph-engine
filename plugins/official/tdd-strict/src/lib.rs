//! Official TDD strict policy plugin metadata and runtime.

use std::path::Path;

mod i18n;

use re_plugin::{
    AgentBootstrapResult, CheckExecutionResult, McpRegistrationResult, POLICY, PluginCheckKind,
    PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary, PluginLocalizedText,
    PluginPolicyAsset, PluginPolicyDescriptor, PluginRuntime, PluginRuntimeError,
    PluginRuntimeHook, PluginTemplateAsset, PluginTemplateDescriptor, PluginTrustLevel, TEMPLATE,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.tdd-strict";
const PLUGIN_NAME: &str = i18n::default_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_names();
const PLUGIN_SUMMARY: &str = i18n::default_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[TEMPLATE, POLICY];
const LIFECYCLE: &[PluginLifecycleStage] = &[
    PluginLifecycleStage::Discover,
    PluginLifecycleStage::Configure,
    PluginLifecycleStage::Validate,
    PluginLifecycleStage::Load,
];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[
    PluginRuntimeHook::Scaffold,
    PluginRuntimeHook::PolicyEnforcement,
];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::Policy,
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
    "official.tdd-strict.starter",
    PLUGIN_ID,
    i18n::default_template_name(),
    i18n::localized_template_names(),
    i18n::default_template_summary(),
    i18n::localized_template_summaries(),
    TEMPLATE_ASSETS,
)];
const POLICY_ASSETS: &[PluginPolicyAsset] = &[PluginPolicyAsset::new(
    "policies/guardrails.md",
    include_str!("../policies/guardrails.md"),
)];
const POLICIES: &[PluginPolicyDescriptor] = &[PluginPolicyDescriptor::new(
    "official.tdd-strict.guardrails",
    PLUGIN_ID,
    i18n::default_policy_name(),
    i18n::localized_policy_names(),
    i18n::default_policy_summary(),
    i18n::localized_policy_summaries(),
    POLICY_ASSETS,
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

/// Returns the immutable policy contributions declared by the plugin.
#[must_use]
pub const fn policies() -> &'static [PluginPolicyDescriptor] {
    POLICIES
}

/// Returns a new instance of the TDD strict plugin runtime.
#[must_use]
pub fn runtime() -> TddStrictRuntime {
    TddStrictRuntime
}

/// TDD strict plugin runtime — verifies that policy assets are
/// materialized in the project directory.
pub struct TddStrictRuntime;

impl PluginRuntime for TddStrictRuntime {
    fn plugin_id(&self) -> &str {
        PLUGIN_ID
    }

    fn run_check(
        &self,
        check_id: &str,
        kind: PluginCheckKind,
        _project_root: &Path,
    ) -> Result<CheckExecutionResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_a_check_plugin",
            format!(
                "TDD strict plugin does not provide check '{check_id}' (kind: {})",
                kind.as_str()
            ),
        ))
    }

    fn bootstrap_agent(&self, agent_id: &str) -> Result<AgentBootstrapResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_agent_plugin",
            format!("TDD strict plugin does not provide agent '{agent_id}'"),
        ))
    }

    fn register_mcp_server(
        &self,
        server_id: &str,
    ) -> Result<McpRegistrationResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_mcp_plugin",
            format!("TDD strict plugin does not provide MCP server '{server_id}'"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use re_plugin::PluginRuntime;

    use super::{
        PLUGIN_ID, PLUGIN_SUMMARY, capabilities, descriptor, i18n, lifecycle, policies,
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

        assert_eq!(template.id, "official.tdd-strict.starter");
        assert_eq!(template.plugin_id, PLUGIN_ID);
        assert!(template.has_assets());
        assert_eq!(template.assets.len(), 4);
        assert_eq!(
            template.display_name_for_locale("pt-br"),
            "Starter TDD estrito"
        );
        assert_eq!(
            template.summary_for_locale("pt-br"),
            "Template inicial com guardrails estritos de TDD ativados."
        );
        assert_eq!(template.display_name_for_locale("es"), "TDD strict starter");
        assert!(!template.assets[0].contents.contains("Placeholder"));
    }

    #[test]
    fn plugin_declares_policy_contributions() {
        let policy = policies()[0];

        assert_eq!(policy.id, "official.tdd-strict.guardrails");
        assert_eq!(policy.plugin_id, PLUGIN_ID);
        assert_eq!(
            policy.display_name_for_locale("pt-br"),
            "Guardrails TDD estrito"
        );
        assert_eq!(
            policy.summary_for_locale("pt-br"),
            "Política oficial com guardrails estritos de TDD."
        );
        assert_eq!(
            policy.summary_for_locale("es"),
            "Official policy with strict TDD guardrails."
        );
        assert!(policy.has_assets());
        assert_eq!(policy.assets[0].path, "policies/guardrails.md");
        assert!(
            policy.assets[0]
                .contents
                .contains("# TDD Strict Guardrails")
        );
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: official.tdd-strict"));
        assert!(manifest.contains("kind: policy"));
        assert!(manifest.contains("- template"));
        assert!(manifest.contains("- policy"));
        assert!(manifest.contains("id: official.tdd-strict.starter"));
        assert!(manifest.contains("id: official.tdd-strict.guardrails"));
    }

    #[test]
    fn runtime_plugin_id_matches_descriptor() {
        let rt = super::runtime();
        assert_eq!(rt.plugin_id(), PLUGIN_ID);
    }

    #[test]
    fn runtime_rejects_check_execution() {
        let rt = super::runtime();
        let result = rt.run_check(
            "official.tdd-strict.prepare",
            re_plugin::PluginCheckKind::Prepare,
            std::path::Path::new("/tmp"),
        );
        assert!(result.is_err());
    }
}
