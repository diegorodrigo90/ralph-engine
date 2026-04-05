//! Official guided-mode plugin — adds chat input bar to the TUI dashboard.
//!
//! When enabled, the TUI shows a persistent input bar (`> ▌`) where
//! users can type feedback for the running agent. The CLI layer
//! dispatches submitted text to the active agent plugin's
//! `inject_feedback()` method. Without this plugin, the TUI is
//! a read-only dashboard.

use std::path::Path;

mod i18n;

use re_plugin::{
    AgentBootstrapResult, CheckExecutionResult, McpRegistrationResult, PluginCheckKind,
    PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary, PluginLocalizedText,
    PluginRuntime, PluginRuntimeError, PluginRuntimeHook, PluginTrustLevel, TUI_WIDGETS, TuiPanel,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.guided";
const PLUGIN_NAME: &str = i18n::plugin_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_plugin_names();
const PLUGIN_SUMMARY: &str = i18n::plugin_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_plugin_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[TUI_WIDGETS];
const LIFECYCLE: &[PluginLifecycleStage] = &[
    PluginLifecycleStage::Discover,
    PluginLifecycleStage::Configure,
    PluginLifecycleStage::Load,
];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[PluginRuntimeHook::TuiContribution];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::TuiExtension,
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

/// Declared capabilities for the guided plugin.
#[must_use]
pub fn capabilities() -> &'static [re_plugin::PluginCapability] {
    DESCRIPTOR.capabilities
}

/// Declared lifecycle stages for the guided plugin.
#[must_use]
pub fn lifecycle() -> &'static [PluginLifecycleStage] {
    DESCRIPTOR.lifecycle
}

/// Declared runtime hooks for the guided plugin.
#[must_use]
pub fn runtime_hooks() -> &'static [PluginRuntimeHook] {
    DESCRIPTOR.runtime_hooks
}

/// Returns the immutable plugin descriptor.
#[must_use]
pub const fn descriptor() -> PluginDescriptor {
    DESCRIPTOR
}

/// Returns a new instance of the guided plugin runtime.
#[must_use]
pub fn runtime() -> GuidedRuntime {
    GuidedRuntime
}

/// Guided plugin runtime — adds interactive controls to TUI.
pub struct GuidedRuntime;

impl PluginRuntime for GuidedRuntime {
    fn plugin_id(&self) -> &str {
        PLUGIN_ID
    }

    fn run_check(
        &self,
        check_id: &str,
        _kind: PluginCheckKind,
        _project_root: &Path,
    ) -> Result<CheckExecutionResult, PluginRuntimeError> {
        Ok(CheckExecutionResult {
            check_id: check_id.to_owned(),
            passed: true,
            findings: Vec::new(),
        })
    }

    fn bootstrap_agent(&self, agent_id: &str) -> Result<AgentBootstrapResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_agent_plugin",
            format!("Guided does not provide agent '{agent_id}'"),
        ))
    }

    fn register_mcp_server(
        &self,
        server_id: &str,
    ) -> Result<McpRegistrationResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_mcp_plugin",
            format!("Guided does not provide MCP server '{server_id}'"),
        ))
    }

    fn tui_contributions(&self) -> Vec<TuiPanel> {
        vec![TuiPanel {
            id: "guided-controls".to_owned(),
            title: "Guided Mode".to_owned(),
            blocks: vec![
                re_plugin::TuiBlock::Status {
                    label: "Status".to_owned(),
                    value: "Active".to_owned(),
                    status: re_plugin::TuiStatus::Ok,
                },
                re_plugin::TuiBlock::KeyValue(vec![
                    ("Input".to_owned(), "enabled".to_owned()),
                    ("Feedback".to_owned(), "type to send".to_owned()),
                ]),
            ],
            lines: Vec::new(),
            zone_hint: "sidebar".to_owned(),
        }]
    }

    fn tui_input_placeholder(&self) -> Option<String> {
        Some("send feedback to the agent...".to_owned())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use re_plugin::PluginRuntime;

    use super::{PLUGIN_ID, capabilities, descriptor, i18n, lifecycle, runtime_hooks};

    fn manifest_document() -> &'static str {
        include_str!("../manifest.yaml")
    }

    #[test]
    fn plugin_id_is_namespaced() {
        assert!(PLUGIN_ID.starts_with("official."));
    }

    #[test]
    fn plugin_declares_tui_widgets_capability() {
        let caps = capabilities();
        assert_eq!(caps.len(), 1);
        assert!(caps.iter().any(|c| c.as_str() == "tui_widgets"));
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        let plugin = descriptor();
        assert_eq!(plugin.id, PLUGIN_ID);
        assert_eq!(plugin.name, i18n::plugin_name());
        assert_eq!(plugin.display_name_for_locale("pt-br"), "Modo Guiado");
        assert_eq!(
            plugin.summary_for_locale("pt-br"),
            "Modo TUI interativo com controles de pausa, feedback e retomada."
        );
    }

    #[test]
    fn plugin_declares_lifecycle_stages() {
        let stages = lifecycle();
        assert_eq!(stages.len(), 3);
    }

    #[test]
    fn plugin_declares_runtime_hooks() {
        let hooks = runtime_hooks();
        assert_eq!(hooks.len(), 1);
        assert!(hooks.iter().any(|h| h.as_str() == "tui_contribution"));
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();
        assert!(manifest.contains("id: official.guided"));
        assert!(manifest.contains("kind: tui_extension"));
        assert!(manifest.contains("trust_level: official"));
        assert!(manifest.contains("- tui_widgets"));
    }

    #[test]
    fn runtime_plugin_id_matches() {
        let rt = super::runtime();
        assert_eq!(rt.plugin_id(), PLUGIN_ID);
    }

    #[test]
    fn runtime_tui_contributions_returns_panel() {
        let rt = super::runtime();
        let panels = rt.tui_contributions();
        assert_eq!(panels.len(), 1);
        assert_eq!(panels[0].id, "guided-controls");
        assert!(!panels[0].blocks.is_empty());
    }

    #[test]
    fn runtime_declares_input_placeholder() {
        let rt = super::runtime();
        let placeholder = rt.tui_input_placeholder();
        assert!(placeholder.is_some());
        assert!(placeholder.unwrap().contains("feedback"));
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
}
