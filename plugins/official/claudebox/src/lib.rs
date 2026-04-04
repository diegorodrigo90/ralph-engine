//! Official Claude Box runtime plugin metadata and runtime.

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
pub const PLUGIN_ID: &str = "official.claudebox";
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
    "official.claudebox.session",
    PLUGIN_ID,
    i18n::agent_name(),
    i18n::localized_agent_names(),
    i18n::agent_summary(),
    i18n::localized_agent_summaries(),
)];
const MCP_SERVERS: &[McpServerDescriptor] = &[McpServerDescriptor::new(
    "official.claudebox.session",
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

/// Returns a new instance of the Claude Box plugin runtime.
#[must_use]
pub fn runtime() -> ClaudeBoxRuntime {
    ClaudeBoxRuntime
}

const AGENT_BINARY: &str = "claudebox";

/// Base tools always auto-approved in programmatic mode.
const BASE_ALLOWED_TOOLS: &[&str] = &["Bash", "Read", "Edit", "Write", "Glob", "Grep"];

/// Default max agent turns when not configured.
const DEFAULT_MAX_TURNS: &str = "200";

/// Claude Box plugin runtime.
pub struct ClaudeBoxRuntime;

impl PluginRuntime for ClaudeBoxRuntime {
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
                "Claude Box does not provide check '{check_id}' (kind: {})",
                kind.as_str()
            ),
        ))
    }

    fn bootstrap_agent(&self, agent_id: &str) -> Result<AgentBootstrapResult, PluginRuntimeError> {
        let found = re_plugin::probe_binary_on_path(AGENT_BINARY).is_some();
        Ok(AgentBootstrapResult {
            agent_id: agent_id.to_owned(),
            ready: found,
            message: if found {
                format!("Binary '{AGENT_BINARY}' found. Agent ready.")
            } else {
                format!("Binary '{AGENT_BINARY}' not found. Install to enable.")
            },
        })
    }

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
                format!("MCP server requires '{AGENT_BINARY}'.")
            },
        })
    }

    fn launch_agent(
        &self,
        agent_id: &str,
        context: &PromptContext,
        project_root: &Path,
    ) -> Result<AgentLaunchResult, PluginRuntimeError> {
        if re_plugin::probe_binary_on_path(AGENT_BINARY).is_none() {
            return Err(PluginRuntimeError::new(
                "agent_not_installed",
                format!(
                    "'{AGENT_BINARY}' not found on PATH.\n\
                     Install: curl -fsSL https://claude.ai/install.sh | bash"
                ),
            ));
        }

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

        let user_prompt = format!(
            "Implement work item {id}.\n\n\
             Your system prompt contains the <task>, <rules>, <context>, and <constraints> sections.\n\
             Expected outcome: all acceptance criteria implemented with tests, \
             quality gates passing, and tracking files updated.",
            id = context.work_item_id,
        );

        // Build allowed tools: base + discovered from plugins + config extras.
        let config_path = project_root.join(".ralph-engine/config.yaml");
        let config_content = std::fs::read_to_string(&config_path).unwrap_or_default();
        let config_extra = extract_run_setting(&config_content, "allowed_tools");
        let allowed_tools = merge_all_tools(
            BASE_ALLOWED_TOOLS,
            &context.discovered_tools,
            config_extra.as_deref(),
        );
        let max_turns = extract_run_setting(&config_content, "max_turns")
            .unwrap_or(DEFAULT_MAX_TURNS.to_owned());

        // Programmatic mode: -p + stream-json + allowedTools.
        // claudebox has same CLI interface as claude.
        let mut cmd = std::process::Command::new(AGENT_BINARY);
        cmd.arg("-p")
            .arg(&user_prompt)
            .arg("--append-system-prompt-file")
            .arg(&context_file)
            .arg("--allowedTools")
            .arg(&allowed_tools)
            .arg("--output-format")
            .arg("stream-json")
            .arg("--verbose")
            .arg("--max-turns")
            .arg(&max_turns);

        let autonomous = project_root
            .join(".ralph-engine/.accepted-autonomous")
            .exists();
        if autonomous {
            cmd.arg("--dangerously-skip-permissions");
        } else {
            cmd.arg("--permission-mode").arg("auto");
        }

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
}

// ── Shared helpers (same as claude plugin) ─────────────────────────

/// Merges tools from three sources: base + discovered + config extras.
fn merge_all_tools(base: &[&str], discovered: &[String], config_extra: Option<&str>) -> String {
    let mut tools: Vec<String> = base.iter().map(|s| (*s).to_owned()).collect();
    for tool in discovered {
        if !tools.contains(tool) {
            tools.push(tool.clone());
        }
    }
    if let Some(extra) = config_extra {
        for tool in extra.split(',') {
            let tool = tool.trim().to_owned();
            if !tool.is_empty() && !tools.contains(&tool) {
                tools.push(tool);
            }
        }
    }
    tools.join(",")
}

/// Extracts a simple `key: value` from the `run:` section of config YAML.
fn extract_run_setting(config_content: &str, key: &str) -> Option<String> {
    let mut in_run = false;
    let prefix = format!("{key}:");
    for line in config_content.lines() {
        let trimmed = line.trim();
        if trimmed == "run:" {
            in_run = true;
            continue;
        }
        if in_run && !line.starts_with(' ') && !line.starts_with('\t') && !trimmed.is_empty() {
            break;
        }
        if in_run && trimmed.starts_with(&prefix) {
            let val = trimmed[prefix.len()..].trim();
            let val = val.trim_matches('"').trim_matches('\'');
            let val = val.split('#').next().unwrap_or(val).trim();
            if !val.is_empty() {
                return Some(val.to_owned());
            }
        }
    }
    None
}

/// Reads stream-json events from stdout, forwards text to stderr.
fn read_stream_json_events(stdout: Option<std::process::ChildStdout>) -> String {
    use std::io::BufRead as _;
    let Some(stdout) = stdout else {
        return String::new();
    };
    let reader = std::io::BufReader::new(stdout);
    let mut last_text = String::new();
    for line in reader.lines() {
        let Ok(line) = line else { break };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.contains("\"text_delta\"") {
            if let Some(text) = extract_json_string_value(trimmed, "text") {
                eprint!("{text}");
                last_text.push_str(&text);
                if last_text.len() > 2000 {
                    let keep: String = last_text
                        .chars()
                        .rev()
                        .take(1500)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .collect();
                    last_text = keep;
                }
            }
        } else if trimmed.contains("\"tool_use\"") && trimmed.contains("\"name\"") {
            if let Some(tool_name) = extract_json_string_value(trimmed, "name") {
                eprintln!("\n[tool: {tool_name}]");
            }
        } else if trimmed.contains("\"type\":\"result\"") {
            if trimmed.contains("\"is_error\":true") {
                eprintln!("\n[agent: error]");
            } else {
                eprintln!("\n[agent: completed]");
            }
        }
    }
    eprintln!();
    let summary = last_text.trim().to_owned();
    let truncated: String = summary.chars().take(500).collect();
    if truncated.len() < summary.len() {
        format!("{truncated}...")
    } else {
        summary
    }
}

/// Extracts a JSON string value for a key from a line.
fn extract_json_string_value(json_line: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{key}\":\"");
    let start = json_line.rfind(&pattern)? + pattern.len();
    let rest = &json_line[start..];
    let mut result = String::new();
    let mut chars = rest.chars();
    loop {
        match chars.next()? {
            '\\' => match chars.next()? {
                'n' => result.push('\n'),
                't' => result.push('\t'),
                'r' => result.push('\r'),
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                '/' => result.push('/'),
                other => {
                    result.push('\\');
                    result.push(other);
                }
            },
            '"' => break,
            c => result.push(c),
        }
    }
    Some(result)
}

#[cfg(test)]
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
            && plugin.display_name_for_locale("pt-br") == "Claude Box"
            && plugin.summary_for_locale("pt-br")
                == "Integração do runtime Claude Box com sessão MCP."
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
            "Sessão Claude Box"
        );
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

        assert_eq!(agent.id, "official.claudebox.session");
        assert_eq!(agent.plugin_id, PLUGIN_ID);
        assert_eq!(agent.display_name_for_locale("pt-br"), "Sessão Claude Box");
        assert_eq!(
            agent.summary_for_locale("pt-br"),
            "Sessão de runtime do Claude Box para o Ralph Engine."
        );
        assert_eq!(
            agent.summary_for_locale("es"),
            "Claude Box runtime session for Ralph Engine."
        );
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: official.claudebox"));
        assert!(manifest.contains("kind: agent_runtime"));
        assert!(manifest.contains("- agent_runtime"));
        assert!(manifest.contains("- mcp_contribution"));
        assert!(manifest.contains("id: official.claudebox.session"));
        assert!(manifest.contains("plugin_api_version: 1"));
    }

    #[test]
    fn runtime_plugin_id_matches() {
        let rt = super::runtime();
        assert_eq!(rt.plugin_id(), PLUGIN_ID);
    }

    #[test]
    fn runtime_bootstrap_agent_returns_result() {
        let rt = super::runtime();
        assert!(rt.bootstrap_agent("official.claudebox.session").is_ok());
    }
}
