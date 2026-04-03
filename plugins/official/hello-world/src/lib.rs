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
mod tests {
    use super::*;

    #[test]
    fn plugin_id_is_official() {
        assert!(PLUGIN_ID.starts_with("official."));
    }

    #[test]
    fn plugin_declares_template_capability() {
        assert!(!capabilities().is_empty());
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        let d = descriptor();
        assert_eq!(d.id, PLUGIN_ID);
        assert_eq!(d.kind, PluginKind::Template);
        assert_eq!(d.trust_level, PluginTrustLevel::Official);
    }

    #[test]
    fn template_id_matches() {
        assert_eq!(templates()[0].id, "official.hello-world.starter");
        assert_eq!(templates()[0].plugin_id, PLUGIN_ID);
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = include_str!("../manifest.yaml");
        assert!(manifest.contains("id: official.hello-world"));
        assert!(manifest.contains("kind: template"));
        assert!(manifest.contains("trust_level: official"));
        assert!(manifest.contains("id: official.hello-world.starter"));
    }
}
