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
const PLUGIN_NAME: &str = i18n::plugin_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_plugin_names();
const PLUGIN_SUMMARY: &str = i18n::plugin_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_plugin_summaries();
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
    i18n::template_name(),
    i18n::localized_template_names(),
    i18n::template_summary(),
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
    i18n::policy_name(),
    i18n::localized_policy_names(),
    i18n::policy_summary(),
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

    fn tui_contributions(&self) -> Vec<re_plugin::TuiPanel> {
        vec![re_plugin::TuiPanel {
            id: "tdd-status".to_owned(),
            title: "TDD Strict".to_owned(),
            blocks: vec![
                re_plugin::TuiBlock::indicator(
                    "Enforcement",
                    "Active",
                    re_plugin::Severity::Success,
                ),
                re_plugin::TuiBlock::pairs(vec![
                    ("Policy".to_owned(), "RED → GREEN → REFACTOR".to_owned()),
                    (
                        "Gate".to_owned(),
                        "tests must pass before commit".to_owned(),
                    ),
                ]),
            ],
            zone_hint: "sidebar".to_owned(),
        }]
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
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
    fn plugin_declares_expected_capabilities() {
        let caps = capabilities();
        assert_eq!(caps.len(), 2);
        assert!(caps.iter().any(|c| c.as_str() == "template"));
        assert!(caps.iter().any(|c| c.as_str() == "policy"));
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        // Arrange
        let plugin = descriptor();

        // Act
        let descriptor_matches = plugin.id == PLUGIN_ID
            && plugin.name == i18n::plugin_name()
            && plugin.display_name_for_locale("pt-br") == "TDD Estrito"
            && plugin.summary_for_locale("pt-br")
                == "Política estrita de TDD com guardrails de template."
            && plugin.summary_for_locale("es") == PLUGIN_SUMMARY;

        // Assert
        assert!(descriptor_matches);
    }

    #[test]
    fn plugin_declares_lifecycle_stages() {
        let stages = lifecycle();
        assert_eq!(stages.len(), 4);
        assert!(stages.iter().any(|s| s.as_str() == "discover"));
        assert!(stages.iter().any(|s| s.as_str() == "configure"));
        assert!(stages.iter().any(|s| s.as_str() == "validate"));
        assert!(stages.iter().any(|s| s.as_str() == "load"));
    }

    #[test]
    fn plugin_declares_runtime_hooks() {
        let hooks = runtime_hooks();
        assert_eq!(hooks.len(), 2);
        assert!(hooks.iter().any(|h| h.as_str() == "scaffold"));
        assert!(hooks.iter().any(|h| h.as_str() == "policy_enforcement"));
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
        let err = rt
            .run_check(
                "official.tdd-strict.prepare",
                re_plugin::PluginCheckKind::Prepare,
                std::path::Path::new("/tmp"),
            )
            .unwrap_err();
        assert_eq!(err.code, "not_a_check_plugin");
    }

    #[test]
    fn runtime_rejects_agent() {
        let rt = super::runtime();
        let err = rt.bootstrap_agent("any").unwrap_err();
        assert_eq!(err.code, "not_an_agent_plugin");
    }

    #[test]
    fn runtime_rejects_mcp() {
        let rt = super::runtime();
        let err = rt.register_mcp_server("any").unwrap_err();
        assert_eq!(err.code, "not_an_mcp_plugin");
    }

    #[test]
    fn runtime_default_required_tools_is_empty() {
        let rt = super::runtime();
        assert!(rt.required_tools().is_empty());
    }
}
