//! Official findings plugin — feedback loop for autonomous agent sessions.
//!
//! Reads `.ralph-engine/findings.md` at runtime and contributes a
//! `<findings>` prompt fragment. This enables a feedback loop where
//! code review findings and quality gate failures accumulate across
//! runs, preventing agents from repeating the same mistakes.

use std::path::Path;

mod i18n;

use re_plugin::{
    AgentBootstrapResult, CONTEXT_PROVIDER, CheckExecutionResult, McpRegistrationResult,
    PROMPT_FRAGMENTS, PluginCheckKind, PluginDescriptor, PluginKind, PluginLifecycleStage,
    PluginLoadBoundary, PluginLocalizedText, PluginPromptAsset, PluginPromptDescriptor,
    PluginProviderDescriptor, PluginRuntime, PluginRuntimeError, PluginRuntimeHook,
    PluginTrustLevel, PromptContribution,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.findings";
const PLUGIN_NAME: &str = i18n::plugin_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_plugin_names();
const PLUGIN_SUMMARY: &str = i18n::plugin_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_plugin_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[CONTEXT_PROVIDER, PROMPT_FRAGMENTS];
const LIFECYCLE: &[PluginLifecycleStage] =
    &[PluginLifecycleStage::Discover, PluginLifecycleStage::Load];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[PluginRuntimeHook::PromptAssembly];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::ContextProvider,
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

/// The findings file read at runtime. Project-defined format.
const FINDINGS_FILE: &str = ".ralph-engine/findings.md";

/// Placeholder prompt asset for static contribution registration.
const PROMPT_ASSETS: &[PluginPromptAsset] = &[PluginPromptAsset::new(
    "prompts/findings.md",
    "<!-- Dynamic: reads .ralph-engine/findings.md at runtime -->",
)];
const PROMPTS: &[PluginPromptDescriptor] = &[PluginPromptDescriptor::new(
    "official.findings.findings",
    PLUGIN_ID,
    i18n::prompt_name(),
    i18n::localized_prompt_names(),
    i18n::prompt_summary(),
    i18n::localized_prompt_summaries(),
    PROMPT_ASSETS,
)];

/// Declared capabilities for the findings plugin.
#[must_use]
pub fn capabilities() -> &'static [re_plugin::PluginCapability] {
    DESCRIPTOR.capabilities
}

/// Declared lifecycle stages for the findings plugin.
#[must_use]
pub fn lifecycle() -> &'static [PluginLifecycleStage] {
    DESCRIPTOR.lifecycle
}

/// Declared runtime hooks for the findings plugin.
#[must_use]
pub fn runtime_hooks() -> &'static [PluginRuntimeHook] {
    DESCRIPTOR.runtime_hooks
}

/// Returns the immutable plugin descriptor.
#[must_use]
pub const fn descriptor() -> PluginDescriptor {
    DESCRIPTOR
}

/// Returns the immutable prompt contributions declared by the plugin.
#[must_use]
pub const fn prompts() -> &'static [PluginPromptDescriptor] {
    PROMPTS
}

/// Returns the (empty) provider contributions.
///
/// Findings is a `ContextProvider` by kind but contributes content
/// via `prompt_contributions()` at runtime, not static providers.
#[must_use]
pub const fn providers() -> &'static [PluginProviderDescriptor] {
    &[]
}

/// Returns a new instance of the findings plugin runtime.
#[must_use]
pub fn runtime() -> FindingsRuntime {
    FindingsRuntime
}

/// Findings plugin runtime — reads findings from the project at runtime.
pub struct FindingsRuntime;

impl FindingsRuntime {
    /// Reads the findings file from the project root.
    ///
    /// Returns `None` if the file does not exist or is empty.
    /// The format is project-defined — this plugin does not parse it.
    #[must_use]
    pub fn read_findings(project_root: &Path) -> Option<String> {
        let path = project_root.join(FINDINGS_FILE);
        let content = std::fs::read_to_string(&path).ok()?;
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return None;
        }
        Some(trimmed.to_owned())
    }

    /// Wraps findings content in the `<findings>` XML tag for prompt injection.
    #[must_use]
    pub fn format_prompt_section(content: &str) -> String {
        format!(
            "<findings>\n\
             ## Past Findings (review BEFORE implementing)\n\n\
             {content}\n\
             </findings>"
        )
    }
}

impl PluginRuntime for FindingsRuntime {
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
                "Findings plugin does not provide check '{check_id}' (kind: {})",
                kind.as_str()
            ),
        ))
    }

    fn bootstrap_agent(&self, agent_id: &str) -> Result<AgentBootstrapResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_agent_plugin",
            format!("Findings plugin does not provide agent '{agent_id}'"),
        ))
    }

    fn register_mcp_server(
        &self,
        server_id: &str,
    ) -> Result<McpRegistrationResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_mcp_plugin",
            format!("Findings plugin does not provide MCP server '{server_id}'"),
        ))
    }

    /// Reads `.ralph-engine/findings.md` and contributes it as a
    /// `<findings>` prompt section for agent sessions.
    fn prompt_contributions(&self, project_root: &Path) -> Vec<PromptContribution> {
        let Some(content) = Self::read_findings(project_root) else {
            return Vec::new();
        };
        vec![PromptContribution {
            label: "findings".to_owned(),
            content: Self::format_prompt_section(&content),
        }]
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use re_plugin::PluginRuntime;

    use super::{
        PLUGIN_ID, PLUGIN_SUMMARY, capabilities, descriptor, i18n, lifecycle, prompts,
        runtime_hooks,
    };

    fn manifest_document() -> &'static str {
        include_str!("../manifest.yaml")
    }

    #[test]
    fn plugin_id_is_namespaced() {
        assert!(PLUGIN_ID.starts_with("official."));
    }

    #[test]
    fn plugin_declares_expected_capabilities() {
        let caps = capabilities();
        assert_eq!(caps.len(), 2);
        assert!(caps.iter().any(|c| c.as_str() == "context_provider"));
        assert!(caps.iter().any(|c| c.as_str() == "prompt_fragments"));
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        let plugin = descriptor();
        assert_eq!(plugin.id, PLUGIN_ID);
        assert_eq!(plugin.name, i18n::plugin_name());
        assert_eq!(plugin.display_name_for_locale("pt-br"), "Achados");
        assert_eq!(
            plugin.summary_for_locale("pt-br"),
            "Plugin de feedback loop — injeta achados passados nos prompts de agentes."
        );
        assert_eq!(plugin.summary_for_locale("es"), PLUGIN_SUMMARY);
    }

    #[test]
    fn plugin_declares_lifecycle_stages() {
        let stages = lifecycle();
        assert_eq!(stages.len(), 2);
        assert!(stages.iter().any(|s| s.as_str() == "discover"));
        assert!(stages.iter().any(|s| s.as_str() == "load"));
    }

    #[test]
    fn plugin_declares_runtime_hooks() {
        let hooks = runtime_hooks();
        assert_eq!(hooks.len(), 1);
        assert!(hooks.iter().any(|h| h.as_str() == "prompt_assembly"));
    }

    #[test]
    fn plugin_declares_prompt_contributions() {
        let prompt = prompts()[0];
        assert_eq!(prompt.id, "official.findings.findings");
        assert_eq!(prompt.plugin_id, PLUGIN_ID);
        assert!(prompt.has_assets());
        assert_eq!(prompt.display_name_for_locale("pt-br"), "Prompt de achados");
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();
        assert!(manifest.contains("id: official.findings"));
        assert!(manifest.contains("kind: context_provider"));
        assert!(manifest.contains("- context_provider"));
        assert!(manifest.contains("- prompt_fragments"));
        assert!(manifest.contains("plugin_api_version: 1"));
    }

    #[test]
    fn runtime_plugin_id_matches() {
        let rt = super::runtime();
        assert_eq!(rt.plugin_id(), PLUGIN_ID);
    }

    #[test]
    fn runtime_rejects_check() {
        let rt = super::runtime();
        let err = rt
            .run_check(
                "any",
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
    fn read_findings_returns_none_for_missing_file() {
        let tmp = std::env::temp_dir().join("re-findings-test-missing");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let result = super::FindingsRuntime::read_findings(&tmp);
        assert!(result.is_none());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn read_findings_returns_none_for_empty_file() {
        let tmp = std::env::temp_dir().join("re-findings-test-empty");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).unwrap();
        std::fs::write(tmp.join(".ralph-engine/findings.md"), "   \n  ").unwrap();

        let result = super::FindingsRuntime::read_findings(&tmp);
        assert!(result.is_none());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn read_findings_returns_content_when_present() {
        let tmp = std::env::temp_dir().join("re-findings-test-content");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).unwrap();
        std::fs::write(
            tmp.join(".ralph-engine/findings.md"),
            "# Learnings\n- **[PERF]** Use bulk insert for >100 rows.",
        )
        .unwrap();

        let result = super::FindingsRuntime::read_findings(&tmp);
        assert!(result.is_some());
        let content = result.unwrap();
        assert!(content.contains("[PERF]"));
        assert!(content.contains("bulk insert"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn format_prompt_section_wraps_in_xml_tags() {
        let content = "- Use bulk insert for >100 rows.";
        let result = super::FindingsRuntime::format_prompt_section(content);
        assert!(result.starts_with("<findings>"));
        assert!(result.ends_with("</findings>"));
        assert!(result.contains("Past Findings"));
        assert!(result.contains("bulk insert"));
    }

    // ── prompt_contributions (PluginRuntime trait method) ─────────────

    #[test]
    fn prompt_contributions_returns_empty_when_no_file() {
        let tmp = std::env::temp_dir().join("re-findings-contrib-none");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let rt = super::runtime();
        let contributions = rt.prompt_contributions(&tmp);
        assert!(contributions.is_empty());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn prompt_contributions_returns_findings_when_present() {
        let tmp = std::env::temp_dir().join("re-findings-contrib-ok");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).unwrap();
        std::fs::write(
            tmp.join(".ralph-engine/findings.md"),
            "- **[PERF]** Use bulk insert for >100 rows.",
        )
        .unwrap();

        let rt = super::runtime();
        let contributions = rt.prompt_contributions(&tmp);
        assert_eq!(contributions.len(), 1);
        assert_eq!(contributions[0].label, "findings");
        assert!(contributions[0].content.contains("<findings>"));
        assert!(contributions[0].content.contains("[PERF]"));
        assert!(contributions[0].content.contains("</findings>"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn prompt_contributions_returns_empty_for_whitespace_only() {
        let tmp = std::env::temp_dir().join("re-findings-contrib-ws");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).unwrap();
        std::fs::write(tmp.join(".ralph-engine/findings.md"), "  \n  \n").unwrap();

        let rt = super::runtime();
        let contributions = rt.prompt_contributions(&tmp);
        assert!(contributions.is_empty());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn runtime_default_required_tools_is_empty() {
        let rt = super::runtime();
        assert!(rt.required_tools().is_empty());
    }
}
