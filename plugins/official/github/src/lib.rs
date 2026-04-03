//! Official GitHub integration plugin metadata and runtime.

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
}

#[cfg(test)]
mod tests {
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
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: official.github"));
        assert!(manifest.contains("kind: mcp_contribution"));
        assert!(manifest.contains("- data_source"));
        assert!(manifest.contains("- context_provider"));
        assert!(manifest.contains("- forge_provider"));
        assert!(manifest.contains("- mcp_contribution"));
        assert!(manifest.contains("id: official.github.data"));
        assert!(manifest.contains("id: official.github.context"));
        assert!(manifest.contains("id: official.github.forge"));
        assert!(manifest.contains("plugin_api_version: 1"));
    }
}
