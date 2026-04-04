//! Official hello-world plugin — minimal example for learning the plugin architecture.

mod i18n;

use re_plugin::{
    PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary, PluginLocalizedText,
    PluginRuntimeHook, PluginTemplateAsset, PluginTemplateDescriptor, PluginTrustLevel, TEMPLATE,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.hello-world";
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
    "official.hello-world.starter",
    PLUGIN_ID,
    i18n::template_name(),
    i18n::localized_template_names(),
    i18n::template_summary(),
    i18n::localized_template_summaries(),
    TEMPLATE_ASSETS,
)];

/// Declared capabilities for the plugin.
#[must_use]
pub fn capabilities() -> &'static [re_plugin::PluginCapability] {
    DESCRIPTOR.capabilities
}

/// Declared lifecycle stages for the plugin.
#[must_use]
pub fn lifecycle() -> &'static [PluginLifecycleStage] {
    DESCRIPTOR.lifecycle
}

/// Declared runtime hooks for the plugin.
#[must_use]
pub fn runtime_hooks() -> &'static [PluginRuntimeHook] {
    DESCRIPTOR.runtime_hooks
}

/// Returns the immutable plugin descriptor.
#[must_use]
pub const fn descriptor() -> PluginDescriptor {
    DESCRIPTOR
}

/// Returns the immutable template contributions.
#[must_use]
pub const fn templates() -> &'static [PluginTemplateDescriptor] {
    TEMPLATES
}

/// Returns a new instance of the plugin runtime.
#[must_use]
pub fn runtime() -> HelloWorldRuntime {
    HelloWorldRuntime
}

/// Hello World plugin runtime.
pub struct HelloWorldRuntime;

impl re_plugin::PluginRuntime for HelloWorldRuntime {
    fn plugin_id(&self) -> &str {
        PLUGIN_ID
    }

    fn run_check(
        &self,
        check_id: &str,
        _kind: re_plugin::PluginCheckKind,
        project_root: &std::path::Path,
    ) -> Result<re_plugin::CheckExecutionResult, re_plugin::PluginRuntimeError> {
        let mut findings = Vec::new();
        if !project_root.join(".ralph-engine/config.yaml").exists() {
            findings.push("missing: .ralph-engine/config.yaml".to_owned());
        }
        Ok(re_plugin::CheckExecutionResult {
            check_id: check_id.to_owned(),
            passed: findings.is_empty(),
            findings,
        })
    }

    fn bootstrap_agent(
        &self,
        agent_id: &str,
    ) -> Result<re_plugin::AgentBootstrapResult, re_plugin::PluginRuntimeError> {
        Err(re_plugin::PluginRuntimeError::new(
            "not_an_agent_plugin",
            format!("Hello World does not provide agent '{agent_id}'"),
        ))
    }

    fn register_mcp_server(
        &self,
        server_id: &str,
    ) -> Result<re_plugin::McpRegistrationResult, re_plugin::PluginRuntimeError> {
        Err(re_plugin::PluginRuntimeError::new(
            "not_an_mcp_plugin",
            format!("Hello World does not provide MCP server '{server_id}'"),
        ))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use re_plugin::PluginRuntime;

    use super::*;

    #[test]
    fn plugin_id_is_official() {
        assert!(PLUGIN_ID.starts_with("official."));
    }

    #[test]
    fn plugin_declares_template_capability() {
        let caps = capabilities();
        assert!(!caps.is_empty());
        assert!(caps.iter().any(|c| c.as_str() == "template"));
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
        assert!(!hooks.is_empty());
        assert!(hooks.iter().any(|h| h.as_str() == "scaffold"));
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        let d = descriptor();
        assert_eq!(d.id, PLUGIN_ID);
        assert_eq!(d.kind, PluginKind::Template);
        assert_eq!(d.trust_level, PluginTrustLevel::Official);
        assert_eq!(d.name, i18n::plugin_name());
        assert_eq!(d.display_name_for_locale("pt-br"), "Olá Mundo");
    }

    #[test]
    fn plugin_descriptor_i18n_fallback() {
        let d = descriptor();
        assert_eq!(d.summary_for_locale("es"), PLUGIN_SUMMARY);
    }

    #[test]
    fn template_id_matches() {
        assert_eq!(templates()[0].id, "official.hello-world.starter");
        assert_eq!(templates()[0].plugin_id, PLUGIN_ID);
        assert!(templates()[0].has_assets());
    }

    #[test]
    fn template_has_expected_assets() {
        let assets = templates()[0].assets;
        let paths: Vec<&str> = assets.iter().map(|a| a.path).collect();
        assert!(paths.contains(&".ralph-engine/config.yaml"));
        assert!(paths.contains(&".ralph-engine/hooks.yaml"));
        assert!(paths.contains(&".ralph-engine/prompt.md"));
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = include_str!("../manifest.yaml");
        assert!(manifest.contains("id: official.hello-world"));
        assert!(manifest.contains("kind: template"));
        assert!(manifest.contains("trust_level: official"));
        assert!(manifest.contains("id: official.hello-world.starter"));
        assert!(manifest.contains("plugin_api_version: 1"));
    }

    // ── Runtime tests ────────────────────────────────────────────────

    #[test]
    fn runtime_plugin_id_matches() {
        let rt = runtime();
        assert_eq!(rt.plugin_id(), PLUGIN_ID);
    }

    #[test]
    fn runtime_check_passes_with_config() {
        let tmp = std::env::temp_dir().join("re-hw-test-pass");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).unwrap();
        std::fs::write(tmp.join(".ralph-engine/config.yaml"), "schema_version: 1\n").unwrap();

        let rt = runtime();
        let result = rt
            .run_check("hw.check", re_plugin::PluginCheckKind::Prepare, &tmp)
            .unwrap();
        assert!(result.passed);
        assert!(result.findings.is_empty());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn runtime_check_fails_without_config() {
        let tmp = std::env::temp_dir().join("re-hw-test-fail");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let rt = runtime();
        let result = rt
            .run_check("hw.check", re_plugin::PluginCheckKind::Prepare, &tmp)
            .unwrap();
        assert!(!result.passed);
        assert!(!result.findings.is_empty());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn runtime_rejects_agent() {
        let rt = runtime();
        let err = rt.bootstrap_agent("any").unwrap_err();
        assert_eq!(err.code, "not_an_agent_plugin");
    }

    #[test]
    fn runtime_rejects_mcp() {
        let rt = runtime();
        let err = rt.register_mcp_server("any").unwrap_err();
        assert_eq!(err.code, "not_an_mcp_plugin");
    }

    #[test]
    fn runtime_default_required_tools_is_empty() {
        let rt = runtime();
        assert!(rt.required_tools().is_empty());
    }

    #[test]
    fn runtime_default_prompt_contributions_is_empty() {
        let rt = runtime();
        let contributions = rt.prompt_contributions(std::path::Path::new("/tmp"));
        assert!(contributions.is_empty());
    }
}
