//! Official GitHub integration plugin metadata and runtime.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use std::path::Path;

mod i18n;

use re_mcp::{
    McpAvailability, McpCommandDescriptor, McpEnvironmentPolicy, McpLaunchPolicy,
    McpServerDescriptor, McpTransport, McpWorkingDirectoryPolicy,
};
use re_plugin::{
    AgentBootstrapResult, CONTEXT_PROVIDER, CheckExecutionResult, DATA_SOURCE, FORGE_PROVIDER,
    MCP_CONTRIBUTION, McpRegistrationResult, PluginCheckKind, PluginDescriptor, PluginKind,
    PluginLifecycleStage, PluginLoadBoundary, PluginLocalizedText, PluginProviderDescriptor,
    PluginProviderKind, PluginRuntime, PluginRuntimeError, PluginRuntimeHook, PluginTrustLevel,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.github";
const PLUGIN_NAME: &str = i18n::plugin_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_plugin_names();
const PLUGIN_SUMMARY: &str = i18n::plugin_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_plugin_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[
    DATA_SOURCE,
    CONTEXT_PROVIDER,
    FORGE_PROVIDER,
    MCP_CONTRIBUTION,
];
const LIFECYCLE: &[PluginLifecycleStage] = &[
    PluginLifecycleStage::Discover,
    PluginLifecycleStage::Configure,
    PluginLifecycleStage::Load,
];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[
    PluginRuntimeHook::McpRegistration,
    PluginRuntimeHook::DataSourceRegistration,
    PluginRuntimeHook::ContextProviderRegistration,
    PluginRuntimeHook::ForgeProviderRegistration,
];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::DataSource,
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
const MCP_SERVERS: &[McpServerDescriptor] = &[McpServerDescriptor::new(
    "official.github.repository",
    PLUGIN_ID,
    i18n::mcp_server_name(),
    i18n::localized_mcp_server_names(),
    McpTransport::Stdio,
    McpLaunchPolicy::SpawnProcess(McpCommandDescriptor::new(
        "ralph-engine-github-mcp",
        &["serve"],
        McpWorkingDirectoryPolicy::ProjectRoot,
        McpEnvironmentPolicy::PluginScoped,
    )),
    McpAvailability::ExplicitOptIn,
)];
const PROVIDERS: &[PluginProviderDescriptor] = &[
    PluginProviderDescriptor::new(
        "official.github.data",
        PLUGIN_ID,
        PluginProviderKind::DataSource,
        i18n::data_source_name(),
        i18n::localized_data_source_names(),
        i18n::data_source_summary(),
        i18n::localized_data_source_summaries(),
    ),
    PluginProviderDescriptor::new(
        "official.github.context",
        PLUGIN_ID,
        PluginProviderKind::ContextProvider,
        i18n::context_provider_name(),
        i18n::localized_context_provider_names(),
        i18n::context_provider_summary(),
        i18n::localized_context_provider_summaries(),
    ),
    PluginProviderDescriptor::new(
        "official.github.forge",
        PLUGIN_ID,
        PluginProviderKind::ForgeProvider,
        i18n::forge_provider_name(),
        i18n::localized_forge_provider_names(),
        i18n::forge_provider_summary(),
        i18n::localized_forge_provider_summaries(),
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

/// Returns the immutable MCP server contributions declared by the plugin.
#[must_use]
pub const fn mcp_servers() -> &'static [McpServerDescriptor] {
    MCP_SERVERS
}

/// Returns the immutable provider contributions declared by the plugin.
#[must_use]
pub const fn providers() -> &'static [PluginProviderDescriptor] {
    PROVIDERS
}

/// Returns a new instance of the GitHub plugin runtime.
#[must_use]
pub fn runtime() -> GitHubRuntime {
    GitHubRuntime
}

const MCP_BINARY: &str = "ralph-engine-github-mcp";

/// GitHub plugin runtime — probes for the GitHub MCP binary.
pub struct GitHubRuntime;

impl PluginRuntime for GitHubRuntime {
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
                "GitHub does not provide check '{check_id}' (kind: {})",
                kind.as_str()
            ),
        ))
    }

    fn bootstrap_agent(&self, agent_id: &str) -> Result<AgentBootstrapResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_agent_plugin",
            format!("GitHub does not provide agent '{agent_id}'"),
        ))
    }

    // Binary probe: result depends on host environment.
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn register_mcp_server(
        &self,
        server_id: &str,
    ) -> Result<McpRegistrationResult, PluginRuntimeError> {
        let found = re_plugin::probe_binary_on_path(MCP_BINARY).is_some();
        Ok(McpRegistrationResult {
            server_id: server_id.to_owned(),
            ready: found,
            message: if found {
                format!("Binary '{MCP_BINARY}' found. MCP server ready.")
            } else {
                format!("Binary '{MCP_BINARY}' not found. Install to enable.")
            },
        })
    }

    fn tui_contributions(&self) -> Vec<re_plugin::TuiPanel> {
        let mcp_available = re_plugin::probe_binary_on_path(MCP_BINARY).is_some();
        let sev = if mcp_available {
            re_plugin::Severity::Success
        } else {
            re_plugin::Severity::Neutral
        };
        let status = if mcp_available {
            "Available"
        } else {
            "Not installed"
        };
        vec![re_plugin::TuiPanel {
            id: "github-status".to_owned(),
            title: "GitHub".to_owned(),
            lines: Vec::new(),
            blocks: vec![
                re_plugin::TuiBlock::indicator("MCP Server", status, sev),
                re_plugin::TuiBlock::pairs(vec![
                    ("Transport".to_owned(), "stdio".to_owned()),
                    ("Tool".to_owned(), "gh CLI".to_owned()),
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
        PLUGIN_ID, PLUGIN_SUMMARY, capabilities, descriptor, i18n, lifecycle, mcp_servers,
        providers, runtime_hooks,
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
        assert_eq!(caps.len(), 4);
        assert!(caps.iter().any(|c| c.as_str() == "data_source"));
        assert!(caps.iter().any(|c| c.as_str() == "context_provider"));
        assert!(caps.iter().any(|c| c.as_str() == "forge_provider"));
        assert!(caps.iter().any(|c| c.as_str() == "mcp_contribution"));
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        // Arrange
        let plugin = descriptor();

        // Act
        let descriptor_matches = plugin.id == PLUGIN_ID
            && plugin.name == i18n::plugin_name()
            && plugin.display_name_for_locale("pt-br") == "GitHub"
            && plugin.summary_for_locale("pt-br")
                == "Integração de dados, contexto, forge e MCP do GitHub."
            && plugin.summary_for_locale("es") == PLUGIN_SUMMARY;

        // Assert
        assert!(descriptor_matches);
    }

    #[test]
    fn plugin_declares_mcp_server_contributions() {
        // Arrange
        let servers = mcp_servers();

        // Act
        let contributes_servers = !servers.is_empty() && servers[0].plugin_id == PLUGIN_ID;

        // Assert
        assert!(contributes_servers);
        assert_eq!(
            servers[0].display_name_for_locale("pt-br"),
            "Repositório GitHub"
        );
        assert_eq!(
            servers[0].display_name_for_locale("es"),
            i18n::mcp_server_name()
        );
    }

    #[test]
    fn plugin_declares_provider_contributions() {
        let providers = providers();

        assert_eq!(providers.len(), 3);
        assert_eq!(providers[0].id, "official.github.data");
        assert_eq!(providers[0].kind.as_str(), "data_source");
        assert_eq!(
            providers[0].display_name_for_locale("pt-br"),
            "Fonte de dados GitHub"
        );
        assert_eq!(
            providers[0].summary_for_locale("es"),
            i18n::data_source_summary()
        );
        assert_eq!(providers[1].id, "official.github.context");
        assert_eq!(providers[1].kind.as_str(), "context_provider");
        assert_eq!(
            providers[1].display_name_for_locale("es"),
            i18n::context_provider_name()
        );
        assert_eq!(providers[2].id, "official.github.forge");
        assert_eq!(providers[2].kind.as_str(), "forge_provider");
        assert_eq!(
            providers[2].summary_for_locale("es"),
            i18n::forge_provider_summary()
        );
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
        assert_eq!(hooks.len(), 4);
        assert!(hooks.iter().any(|h| h.as_str() == "mcp_registration"));
        assert!(
            hooks
                .iter()
                .any(|h| h.as_str() == "data_source_registration")
        );
        assert!(
            hooks
                .iter()
                .any(|h| h.as_str() == "context_provider_registration")
        );
        assert!(
            hooks
                .iter()
                .any(|h| h.as_str() == "forge_provider_registration")
        );
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: official.github"));
        assert!(manifest.contains("kind: data_source"));
        assert!(manifest.contains("- data_source"));
        assert!(manifest.contains("- context_provider"));
        assert!(manifest.contains("- forge_provider"));
        assert!(manifest.contains("- mcp_contribution"));
        assert!(manifest.contains("id: official.github.data"));
        assert!(manifest.contains("id: official.github.context"));
        assert!(manifest.contains("id: official.github.forge"));
        assert!(manifest.contains("plugin_api_version: 1"));
    }

    // ── Runtime tests ────────────────────────────────────────────────

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
    fn runtime_register_mcp_returns_result_with_content() {
        let rt = super::runtime();
        let result = rt
            .register_mcp_server("official.github.repository")
            .unwrap();
        assert_eq!(result.server_id, "official.github.repository");
        // MCP binary may or may not be installed — verify both branches.
        if result.ready {
            assert!(result.message.contains("found"));
        } else {
            assert!(result.message.contains("not found"));
        }
    }

    #[test]
    fn runtime_default_required_tools_is_empty() {
        let rt = super::runtime();
        assert!(rt.required_tools().is_empty());
    }
}
