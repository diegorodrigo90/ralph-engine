//! Official SSH remote-control plugin metadata and runtime.

use std::path::Path;

mod i18n;

use re_plugin::{
    AgentBootstrapResult, CheckExecutionResult, McpRegistrationResult, PluginCheckKind,
    PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary, PluginLocalizedText,
    PluginProviderDescriptor, PluginProviderKind, PluginRuntime, PluginRuntimeError,
    PluginRuntimeHook, PluginTrustLevel, REMOTE_CONTROL,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.ssh";
const PLUGIN_NAME: &str = i18n::plugin_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_plugin_names();
const PLUGIN_SUMMARY: &str = i18n::plugin_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_plugin_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[REMOTE_CONTROL];
const LIFECYCLE: &[PluginLifecycleStage] =
    &[PluginLifecycleStage::Discover, PluginLifecycleStage::Load];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[PluginRuntimeHook::RemoteControlBootstrap];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::RemoteControl,
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
const PROVIDERS: &[PluginProviderDescriptor] = &[PluginProviderDescriptor::new(
    "official.ssh.remote",
    PLUGIN_ID,
    PluginProviderKind::RemoteControl,
    i18n::remote_control_name(),
    i18n::localized_remote_control_names(),
    i18n::remote_control_summary(),
    i18n::localized_remote_control_summaries(),
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

/// Returns the immutable provider contributions declared by the plugin.
#[must_use]
pub const fn providers() -> &'static [PluginProviderDescriptor] {
    PROVIDERS
}

/// Returns a new instance of the SSH plugin runtime.
#[must_use]
pub fn runtime() -> SshRuntime {
    SshRuntime
}

/// SSH plugin runtime — probes for the ssh binary.
pub struct SshRuntime;

impl PluginRuntime for SshRuntime {
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
                "SSH does not provide check '{check_id}' (kind: {})",
                kind.as_str()
            ),
        ))
    }

    fn bootstrap_agent(&self, agent_id: &str) -> Result<AgentBootstrapResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_agent_plugin",
            format!("SSH does not provide agent '{agent_id}'"),
        ))
    }

    fn register_mcp_server(
        &self,
        server_id: &str,
    ) -> Result<McpRegistrationResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_mcp_plugin",
            format!("SSH does not provide MCP server '{server_id}'"),
        ))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use re_plugin::PluginRuntime;

    use super::{
        PLUGIN_ID, PLUGIN_SUMMARY, capabilities, descriptor, i18n, lifecycle, providers,
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
    fn plugin_declares_remote_control_capability() {
        let caps = capabilities();
        assert_eq!(caps.len(), 1);
        assert!(caps.iter().any(|c| c.as_str() == "remote_control"));
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        // Arrange
        let plugin = descriptor();

        // Act
        let descriptor_matches = plugin.id == PLUGIN_ID
            && plugin.name == i18n::plugin_name()
            && plugin.display_name_for_locale("pt-br") == "SSH"
            && plugin.summary_for_locale("pt-br") == "Integração de controle remoto por SSH."
            && plugin.summary_for_locale("es") == PLUGIN_SUMMARY;

        // Assert
        assert!(descriptor_matches);
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
        assert!(
            hooks
                .iter()
                .any(|h| h.as_str() == "remote_control_bootstrap")
        );
    }

    #[test]
    fn plugin_declares_provider_contributions() {
        let providers = providers();

        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].id, "official.ssh.remote");
        assert_eq!(providers[0].kind.as_str(), "remote_control");
        assert_eq!(
            providers[0].display_name_for_locale("pt-br"),
            "Controle remoto SSH"
        );
        assert_eq!(
            providers[0].summary_for_locale("es"),
            i18n::remote_control_summary()
        );
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: official.ssh"));
        assert!(manifest.contains("kind: remote_control"));
        assert!(manifest.contains("- remote_control"));
        assert!(manifest.contains("id: official.ssh.remote"));
        assert!(manifest.contains("trust_level: official"));
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
