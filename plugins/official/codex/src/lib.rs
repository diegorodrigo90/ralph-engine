//! Official Codex runtime plugin metadata.

mod i18n;

use re_mcp::{McpAvailability, McpLaunchPolicy, McpServerDescriptor, McpTransport};
use re_plugin::{
    AGENT_RUNTIME, MCP_CONTRIBUTION, PluginAgentDescriptor, PluginDescriptor, PluginKind,
    PluginLifecycleStage, PluginLoadBoundary, PluginLocalizedText, PluginRuntimeHook,
    PluginTrustLevel,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.codex";
const PLUGIN_NAME: &str = i18n::plugin_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_plugin_names();
const PLUGIN_SUMMARY: &str = i18n::plugin_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_plugin_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[AGENT_RUNTIME, MCP_CONTRIBUTION];
const LIFECYCLE: &[PluginLifecycleStage] =
    &[PluginLifecycleStage::Discover, PluginLifecycleStage::Load];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[
    PluginRuntimeHook::AgentBootstrap,
    PluginRuntimeHook::McpRegistration,
];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::AgentRuntime,
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
const AGENTS: &[PluginAgentDescriptor] = &[PluginAgentDescriptor::new(
    "official.codex.session",
    PLUGIN_ID,
    i18n::agent_name(),
    i18n::localized_agent_names(),
    i18n::agent_summary(),
    i18n::localized_agent_summaries(),
)];
const MCP_SERVERS: &[McpServerDescriptor] = &[McpServerDescriptor::new(
    "official.codex.session",
    PLUGIN_ID,
    i18n::mcp_server_name(),
    i18n::localized_mcp_server_names(),
    McpTransport::Stdio,
    McpLaunchPolicy::PluginRuntime,
    McpAvailability::OnDemand,
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

/// Returns the immutable agent runtime contributions declared by the plugin.
#[must_use]
pub const fn agents() -> &'static [PluginAgentDescriptor] {
    AGENTS
}

/// Returns the immutable MCP server contributions declared by the plugin.
#[must_use]
pub const fn mcp_servers() -> &'static [McpServerDescriptor] {
    MCP_SERVERS
}

#[cfg(test)]
mod tests {
    use super::{
        AGENTS, PLUGIN_ID, PLUGIN_SUMMARY, capabilities, descriptor, i18n, lifecycle, mcp_servers,
        runtime_hooks,
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
            && plugin.display_name_for_locale("pt-br") == "Codex"
            && plugin.summary_for_locale("pt-br") == "Integração do runtime Codex com sessão MCP."
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
        assert_eq!(servers[0].display_name_for_locale("pt-br"), "Sessão Codex");
        assert_eq!(
            servers[0].display_name_for_locale("es"),
            i18n::mcp_server_name()
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
    fn plugin_declares_agent_runtime_contributions() {
        let agent = AGENTS[0];

        assert_eq!(agent.id, "official.codex.session");
        assert_eq!(agent.plugin_id, PLUGIN_ID);
        assert_eq!(agent.display_name_for_locale("pt-br"), "Sessão Codex");
        assert_eq!(
            agent.summary_for_locale("pt-br"),
            "Sessão de runtime do Codex para o Ralph Engine."
        );
        assert_eq!(
            agent.summary_for_locale("es"),
            "Codex runtime session for Ralph Engine."
        );
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: official.codex"));
        assert!(manifest.contains("kind: agent_runtime"));
        assert!(manifest.contains("- agent_runtime"));
        assert!(manifest.contains("- mcp_contribution"));
        assert!(manifest.contains("id: official.codex.session"));
        assert!(manifest.contains("plugin_api_version: 1"));
    }
}
