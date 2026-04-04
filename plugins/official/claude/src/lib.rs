//! Official Claude runtime plugin metadata and runtime.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use std::path::Path;

mod i18n;

use re_mcp::{McpAvailability, McpLaunchPolicy, McpServerDescriptor, McpTransport};
use re_plugin::{
    AGENT_RUNTIME, AgentBootstrapResult, AgentLaunchResult, CheckExecutionResult, MCP_CONTRIBUTION,
    McpRegistrationResult, PluginAgentDescriptor, PluginCheckKind, PluginDescriptor, PluginKind,
    PluginLifecycleStage, PluginLoadBoundary, PluginLocalizedText, PluginRuntime,
    PluginRuntimeError, PluginRuntimeHook, PluginTrustLevel, PromptContext,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.claude";
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
    PluginRuntimeHook::AgentLaunch,
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
    "official.claude.session",
    PLUGIN_ID,
    i18n::agent_name(),
    i18n::localized_agent_names(),
    i18n::agent_summary(),
    i18n::localized_agent_summaries(),
)];
const MCP_SERVERS: &[McpServerDescriptor] = &[McpServerDescriptor::new(
    "official.claude.session",
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

/// Returns a new instance of the Claude plugin runtime.
#[must_use]
pub fn runtime() -> ClaudeRuntime {
    ClaudeRuntime
}

/// The binary name for Claude CLI.
const AGENT_BINARY: &str = "claude";

/// Base tools always auto-approved. These are the core Claude Code tools
/// needed for autonomous coding. Project-specific tools (MCP servers, etc.)
/// are added via `run.allowed_tools` in config.yaml.
const BASE_ALLOWED_TOOLS: &[&str] = &["Bash", "Read", "Edit", "Write", "Glob", "Grep"];

/// Default max agent turns when not configured.
const DEFAULT_MAX_TURNS: &str = "200";

/// Claude plugin runtime — probes for the Claude CLI binary to
/// determine agent and MCP server readiness.
pub struct ClaudeRuntime;

impl PluginRuntime for ClaudeRuntime {
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
                "Claude plugin does not provide check '{check_id}' (kind: {})",
                kind.as_str()
            ),
        ))
    }

    // Binary probe: result depends on whether `claude` is installed on the
    // host. Both branches (found/not-found) are tested — which one executes
    // depends on the machine. Tests verify both message formats.
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn bootstrap_agent(&self, agent_id: &str) -> Result<AgentBootstrapResult, PluginRuntimeError> {
        let found = re_plugin::probe_binary_on_path(AGENT_BINARY).is_some();
        Ok(AgentBootstrapResult {
            agent_id: agent_id.to_owned(),
            ready: found,
            message: if found {
                format!("Binary '{AGENT_BINARY}' found. Agent ready.")
            } else {
                format!("Binary '{AGENT_BINARY}' not found. Install Claude CLI to enable.")
            },
        })
    }

    // Binary probe: same machine-dependent branching as bootstrap_agent.
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn register_mcp_server(
        &self,
        server_id: &str,
    ) -> Result<McpRegistrationResult, PluginRuntimeError> {
        let found = re_plugin::probe_binary_on_path(AGENT_BINARY).is_some();
        Ok(McpRegistrationResult {
            server_id: server_id.to_owned(),
            ready: found,
            message: if found {
                format!("MCP server backed by '{AGENT_BINARY}' is available.")
            } else {
                format!("MCP server requires '{AGENT_BINARY}'. Install Claude CLI.")
            },
        })
    }

    // I/O boundary: spawns a real subprocess (`claude -p`), reads its
    // stdout stream, and waits for exit. All pure logic (tool merging,
    // config parsing, prompt building, command assembly) is tested via
    // `re_plugin::agent_helpers` at 100% coverage. This function is the
    // thin wrapper that does the actual OS-level spawn + wait + cleanup —
    // cannot be unit-tested without running the real agent binary.
    // Validated end-to-end by `ralph-engine run <id>` integration runs.
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn launch_agent(
        &self,
        agent_id: &str,
        context: &PromptContext,
        project_root: &Path,
    ) -> Result<AgentLaunchResult, PluginRuntimeError> {
        // Verify binary exists
        if re_plugin::probe_binary_on_path(AGENT_BINARY).is_none() {
            return Err(PluginRuntimeError::new(
                "agent_not_installed",
                format!(
                    "'{AGENT_BINARY}' not found on PATH.\n\
                     Install: curl -fsSL https://claude.ai/install.sh | bash\n\
                     Docs: https://code.claude.com/docs/en/quickstart"
                ),
            ));
        }

        // Write context to temp file for --append-system-prompt-file.
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        let context_file = std::env::temp_dir().join(format!(
            "ralph-engine-context-{}-{ts}.md",
            std::process::id()
        ));
        std::fs::write(&context_file, &context.prompt_text).map_err(|err| {
            PluginRuntimeError::new(
                "context_write_failed",
                format!("Failed to write context file: {err}"),
            )
        })?;

        // Build command config (pure logic — testable).
        let config_path = project_root.join(".ralph-engine/config.yaml");
        let config_content = std::fs::read_to_string(&config_path).unwrap_or_default();
        let autonomous = project_root
            .join(".ralph-engine/.accepted-autonomous")
            .exists();

        let agent_config = re_plugin::agent_helpers::build_agent_command_config(
            &re_plugin::agent_helpers::AgentCommandInput {
                binary: AGENT_BINARY,
                base_tools: BASE_ALLOWED_TOOLS,
                default_max_turns: DEFAULT_MAX_TURNS,
                work_item_id: &context.work_item_id,
                discovered_tools: &context.discovered_tools,
                config_content: &config_content,
                autonomous,
                context_file: context_file.clone(),
            },
        );

        // Spawn the agent process (I/O boundary).
        let mut cmd = re_plugin::agent_helpers::build_command(&agent_config);
        let mut child = cmd
            .current_dir(project_root)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .map_err(|err| {
                let _ = std::fs::remove_file(&context_file);
                PluginRuntimeError::new(
                    "agent_spawn_failed",
                    format!("Failed to spawn '{AGENT_BINARY}': {err}"),
                )
            })?;

        let message = read_stream_json_events(child.stdout.take());

        let exit_status = child.wait().map_err(|err| {
            let _ = std::fs::remove_file(&context_file);
            PluginRuntimeError::new(
                "agent_wait_failed",
                format!("Failed to wait for '{AGENT_BINARY}': {err}"),
            )
        })?;

        let _ = std::fs::remove_file(&context_file);

        let code = exit_status.code();
        let success = exit_status.success();
        Ok(AgentLaunchResult {
            agent_id: agent_id.to_owned(),
            success,
            exit_code: code,
            message: if success {
                if message.is_empty() {
                    "Agent session completed successfully.".to_owned()
                } else {
                    message
                }
            } else {
                format!(
                    "Agent exited with code {}.",
                    code.map_or("unknown".to_owned(), |c| c.to_string())
                )
            },
        })
    }

    /// Contributes a TUI sidebar panel showing agent connection status.
    fn tui_contributions(&self) -> Vec<re_plugin::TuiPanel> {
        let binary_available = re_plugin::probe_binary_on_path("claude").is_some();
        let status = if binary_available {
            "Available"
        } else {
            "Not found"
        };

        vec![re_plugin::TuiPanel {
            id: "claude-status".to_owned(),
            title: "Claude".to_owned(),
            lines: vec![format!("Binary: {status}"), "Mode: -p (prompt)".to_owned()],
            zone_hint: "sidebar".to_owned(),
        }]
    }
}

// Shared agent helpers (tool merging, config extraction, stream-JSON parsing).
use re_plugin::agent_helpers::read_stream_json_events;

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use re_plugin::PluginRuntime;

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
    fn plugin_declares_expected_capabilities() {
        let caps = capabilities();
        assert_eq!(caps.len(), 2);
        assert!(caps.iter().any(|c| c.as_str() == "agent_runtime"));
        assert!(caps.iter().any(|c| c.as_str() == "mcp_contribution"));
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        // Arrange
        let plugin = descriptor();

        // Act
        let descriptor_matches = plugin.id == PLUGIN_ID
            && plugin.name == i18n::plugin_name()
            && plugin.display_name_for_locale("pt-br") == "Claude"
            && plugin.summary_for_locale("pt-br")
                == "Integração do runtime de agente Claude com sessão MCP."
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
        assert_eq!(servers[0].display_name_for_locale("pt-br"), "Sessão Claude");
        assert_eq!(
            servers[0].display_name_for_locale("es"),
            i18n::mcp_server_name()
        );
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
        assert_eq!(hooks.len(), 3);
        assert!(hooks.iter().any(|h| h.as_str() == "agent_bootstrap"));
        assert!(hooks.iter().any(|h| h.as_str() == "mcp_registration"));
        assert!(hooks.iter().any(|h| h.as_str() == "agent_launch"));
    }

    #[test]
    fn plugin_declares_agent_runtime_contributions() {
        let agent = AGENTS[0];

        assert_eq!(agent.id, "official.claude.session");
        assert_eq!(agent.plugin_id, PLUGIN_ID);
        assert_eq!(agent.display_name_for_locale("pt-br"), "Sessão Claude");
        assert_eq!(
            agent.summary_for_locale("pt-br"),
            "Sessão de runtime do Claude para o Ralph Engine."
        );
        assert_eq!(
            agent.summary_for_locale("es"),
            "Claude runtime session for Ralph Engine."
        );
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: official.claude"));
        assert!(manifest.contains("kind: agent_runtime"));
        assert!(manifest.contains("- agent_runtime"));
        assert!(manifest.contains("- mcp_contribution"));
        assert!(manifest.contains("id: official.claude.session"));
        assert!(manifest.contains("plugin_api_version: 1"));
    }

    #[test]
    fn runtime_plugin_id_matches() {
        let rt = super::runtime();
        assert_eq!(rt.plugin_id(), PLUGIN_ID);
    }

    #[test]
    fn runtime_bootstrap_agent_returns_result_with_content() {
        let rt = super::runtime();
        let result = rt.bootstrap_agent("official.claude.session").unwrap();
        assert_eq!(result.agent_id, "official.claude.session");
        // Binary may or may not be installed — test both branches.
        if result.ready {
            assert!(result.message.contains("found"));
        } else {
            assert!(result.message.contains("not found"));
        }
    }

    #[test]
    fn runtime_register_mcp_returns_result_with_content() {
        let rt = super::runtime();
        let result = rt.register_mcp_server("official.claude.session").unwrap();
        assert_eq!(result.server_id, "official.claude.session");
        if result.ready {
            assert!(result.message.contains("available"));
        } else {
            assert!(result.message.contains("requires"));
        }
    }

    #[test]
    fn runtime_rejects_check() {
        let rt = super::runtime();
        let err = rt
            .run_check(
                "any.check",
                re_plugin::PluginCheckKind::Prepare,
                std::path::Path::new("/tmp"),
            )
            .unwrap_err();
        assert_eq!(err.code, "not_a_check_plugin");
    }

    #[test]
    fn runtime_launch_agent_fails_without_binary() {
        // Unless claude is actually installed, launch_agent should fail
        // with "agent_not_installed". This tests the guard + error path.
        if re_plugin::probe_binary_on_path("claude").is_some() {
            return; // Can't test the error path when binary exists
        }
        let rt = super::runtime();
        let context = re_plugin::PromptContext {
            prompt_text: "test prompt".to_owned(),
            context_files: vec![],
            work_item_id: "1.1".to_owned(),
            discovered_tools: vec![],
        };
        let err = rt
            .launch_agent(
                "official.claude.session",
                &context,
                std::path::Path::new("/tmp"),
            )
            .unwrap_err();
        assert_eq!(err.code, "agent_not_installed");
        assert!(err.message.contains("not found on PATH"));
        assert!(err.message.contains("Install"));
    }

    // Helper tests (merge_all_tools, extract_run_setting, extract_json_string_value,
    // read_stream_json_events) live in re_plugin::agent_helpers::tests.
}
