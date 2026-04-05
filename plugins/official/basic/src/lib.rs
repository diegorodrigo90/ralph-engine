//! Official basic starter plugin metadata and runtime.

use std::path::Path;

mod i18n;

use re_plugin::{
    AgentBootstrapResult, CheckExecutionResult, McpRegistrationResult, PluginCheckKind,
    PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary, PluginLocalizedText,
    PluginRuntime, PluginRuntimeError, PluginRuntimeHook, PluginTemplateAsset,
    PluginTemplateDescriptor, PluginTrustLevel, TEMPLATE,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.basic";
const PLUGIN_NAME: &str = i18n::plugin_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_plugin_names();
const PLUGIN_SUMMARY: &str = i18n::plugin_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_plugin_summaries();
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
    "official.basic.starter",
    PLUGIN_ID,
    i18n::template_name(),
    i18n::localized_template_names(),
    i18n::template_summary(),
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

/// Returns a new instance of the basic plugin runtime.
#[must_use]
pub fn runtime() -> BasicRuntime {
    BasicRuntime
}

/// Basic plugin runtime — validates project config exists.
pub struct BasicRuntime;

impl PluginRuntime for BasicRuntime {
    fn plugin_id(&self) -> &str {
        PLUGIN_ID
    }

    fn run_check(
        &self,
        check_id: &str,
        _kind: PluginCheckKind,
        project_root: &Path,
    ) -> Result<CheckExecutionResult, PluginRuntimeError> {
        let mut findings = Vec::new();
        if !project_root.join(".ralph-engine/config.yaml").exists() {
            findings.push("missing: .ralph-engine/config.yaml".to_owned());
        }
        Ok(CheckExecutionResult {
            check_id: check_id.to_owned(),
            passed: findings.is_empty(),
            findings,
        })
    }

    fn bootstrap_agent(&self, agent_id: &str) -> Result<AgentBootstrapResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_agent_plugin",
            format!("Basic does not provide agent '{agent_id}'"),
        ))
    }

    fn register_mcp_server(
        &self,
        server_id: &str,
    ) -> Result<McpRegistrationResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_mcp_plugin",
            format!("Basic does not provide MCP server '{server_id}'"),
        ))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use re_plugin::PluginRuntime;

    use super::{
        PLUGIN_ID, PLUGIN_SUMMARY, capabilities, descriptor, i18n, lifecycle, runtime_hooks,
        templates,
    };

    fn manifest_document() -> &'static str {
        include_str!("../manifest.yaml")
    }

    #[test]
    fn plugin_id_is_namespaced() {
        assert!(PLUGIN_ID.starts_with("official."));
    }

    #[test]
    fn plugin_declares_template_capability() {
        let caps = capabilities();
        assert_eq!(caps.len(), 1);
        assert!(caps.iter().any(|c| c.as_str() == "template"));
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        let plugin = descriptor();
        assert_eq!(plugin.id, PLUGIN_ID);
        assert_eq!(plugin.name, i18n::plugin_name());
        assert_eq!(plugin.display_name_for_locale("pt-br"), "Básico");
        assert_eq!(
            plugin.summary_for_locale("pt-br"),
            "Plugin base para templates iniciais."
        );
        assert_eq!(plugin.summary_for_locale("es"), PLUGIN_SUMMARY);
    }

    #[test]
    fn plugin_declares_lifecycle_stages() {
        let stages = lifecycle();
        assert_eq!(stages.len(), 3);
        assert!(stages.iter().any(|s| s.as_str() == "discover"));
        assert!(stages.iter().any(|s| s.as_str() == "configure"));
        assert!(stages.iter().any(|s| s.as_str() == "load"));
    }

    #[test]
    fn plugin_declares_runtime_hooks() {
        let hooks = runtime_hooks();
        assert_eq!(hooks.len(), 1);
        assert!(hooks.iter().any(|h| h.as_str() == "scaffold"));
    }

    #[test]
    fn plugin_declares_template_contributions() {
        let template = templates()[0];
        assert_eq!(template.id, "official.basic.starter");
        assert_eq!(template.plugin_id, PLUGIN_ID);
        assert!(template.has_assets());
        assert_eq!(template.assets[0].path, ".ralph-engine/README.md");
        assert_eq!(template.display_name_for_locale("pt-br"), "Starter básico");
        assert_eq!(template.display_name_for_locale("es"), "Basic starter");
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

    // ── Runtime tests ────────────────────────────────────────────────

    #[test]
    fn runtime_plugin_id_matches() {
        let rt = super::runtime();
        assert_eq!(rt.plugin_id(), PLUGIN_ID);
    }

    #[test]
    fn runtime_check_passes_with_config() {
        let tmp = std::env::temp_dir().join("re-basic-test-pass");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).unwrap();
        std::fs::write(tmp.join(".ralph-engine/config.yaml"), "schema_version: 1\n").unwrap();

        let rt = super::runtime();
        let result = rt
            .run_check("basic.check", re_plugin::PluginCheckKind::Prepare, &tmp)
            .unwrap();
        assert!(result.passed);
        assert!(result.findings.is_empty());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn runtime_check_fails_without_config() {
        let tmp = std::env::temp_dir().join("re-basic-test-fail");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let rt = super::runtime();
        let result = rt
            .run_check("basic.check", re_plugin::PluginCheckKind::Prepare, &tmp)
            .unwrap();
        assert!(!result.passed);
        assert!(result.findings.iter().any(|f| f.contains("config.yaml")));

        let _ = std::fs::remove_dir_all(&tmp);
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

    #[test]
    fn template_config_has_required_keys() {
        let config_yaml = include_str!("../template/config.yaml");
        // The config must contain all required keys for the core parser
        assert!(
            config_yaml.contains("schema_version:"),
            "template must have schema_version"
        );
        assert!(
            config_yaml.contains("default_locale:"),
            "template must have default_locale"
        );
        assert!(
            config_yaml.contains("plugins:"),
            "template must have plugins"
        );
        assert!(config_yaml.contains("mcp:"), "template must have mcp");
        assert!(
            config_yaml.contains("budgets:"),
            "template must have budgets"
        );
    }
}
