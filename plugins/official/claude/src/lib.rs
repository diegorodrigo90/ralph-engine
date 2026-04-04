//! Official Claude runtime plugin metadata and runtime.

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
        // This injects story + instructions + project rules into the system
        // prompt WITHOUT replacing Claude Code's built-in capabilities
        // (CLAUDE.md, hooks, MCP servers, tool definitions are preserved).
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

        // User message for -p mode: outcome-based, not prescriptive.
        // The system prompt has full story + rules + constraints.
        // Research: user message is high-attention — keep it focused on
        // WHAT to achieve, not HOW (the system prompt handles HOW).
        let user_prompt = format!(
            "Implement work item {id}.\n\n\
             Your system prompt contains the <task>, <rules>, <context>, and <constraints> sections.\n\
             Expected outcome: all acceptance criteria implemented with tests, \
             quality gates passing, and tracking files updated.",
            id = context.work_item_id,
        );

        // Build the final allowed tools list by merging three sources:
        // 1. Base tools (this plugin's own requirements: Bash, Read, etc.)
        // 2. Discovered tools (from all enabled plugins via required_tools())
        // 3. Config extras (user overrides from run.allowed_tools in YAML)
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

        // Launch claude in programmatic (-p) mode:
        //
        // -p "prompt"                    Full agent loop, no TUI
        // --append-system-prompt-file    Story + rules as system context
        // --allowedTools                 Auto-approve listed tools (incl. MCP)
        // --output-format stream-json    Structured output for monitoring
        // --verbose                      Include tool calls in stream
        // --max-turns N                  Safety limit (configurable)
        //
        // The agent runs autonomously: reads files, edits code, runs
        // commands, and commits — all driven by the prompt context.
        //
        // Docs: https://code.claude.com/docs/en/cli-usage
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

        // Permission mode: if user accepted autonomous mode (first-run
        // warning saved to .accepted-autonomous), use full auto. Otherwise
        // use the ML classifier that auto-approves safe operations.
        let autonomous = project_root
            .join(".ralph-engine/.accepted-autonomous")
            .exists();
        if autonomous {
            cmd.arg("--dangerously-skip-permissions");
        } else {
            cmd.arg("--permission-mode").arg("auto");
        }

        // stdout piped (we parse stream-json events)
        // stderr inherited (Claude's own logs visible to user)
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

        // Read stream-json events from stdout and forward progress to stderr.
        let message = read_stream_json_events(child.stdout.take());

        let exit_status = child.wait().map_err(|err| {
            let _ = std::fs::remove_file(&context_file);
            PluginRuntimeError::new(
                "agent_wait_failed",
                format!("Failed to wait for '{AGENT_BINARY}': {err}"),
            )
        })?;

        // Clean up temp file
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

// ── Stream-JSON event parser ───────────────────────────────────────

/// Reads Claude Code stream-json events from stdout, forwards text
/// to stderr for user visibility, and returns the final message.
///
/// Each line is a JSON object. We match key patterns without a JSON
/// parser to avoid adding dependencies:
///
/// - `text_delta` events → extract text, print to stderr
/// - `tool_use` events → print tool name summary to stderr
/// - `result` event → extract summary
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

        // Forward text_delta content to stderr for user visibility.
        if trimmed.contains("\"text_delta\"") {
            if let Some(text) = extract_json_string_value(trimmed, "text") {
                eprint!("{text}");
                // Keep last ~500 chars as summary for the result message.
                last_text.push_str(&text);
                if last_text.len() > 2000 {
                    // Truncate to last ~1500 chars, safe for multi-byte UTF-8.
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
        }
        // Show tool usage summaries.
        else if trimmed.contains("\"tool_use\"") && trimmed.contains("\"name\"") {
            if let Some(tool_name) = extract_json_string_value(trimmed, "name") {
                eprintln!("\n[tool: {tool_name}]");
            }
        }
        // Capture the final result event.
        else if trimmed.contains("\"type\":\"result\"") {
            if trimmed.contains("\"is_error\":true") {
                eprintln!("\n[agent: error]");
            } else {
                eprintln!("\n[agent: completed]");
            }
        }
    }

    eprintln!();

    // Return a trimmed summary of what the agent did.
    let summary = last_text.trim().to_owned();
    // Truncate safely for multi-byte UTF-8 (no byte-boundary panics).
    let truncated: String = summary.chars().take(500).collect();
    if truncated.len() < summary.len() {
        format!("{truncated}...")
    } else {
        summary
    }
}

/// Merges tools from three sources into a deduplicated comma-separated list:
/// 1. Base tools (agent plugin's own: Bash, Read, Edit, etc.)
/// 2. Discovered tools (from all enabled plugins via `required_tools()`)
/// 3. Config extras (user overrides from `run.allowed_tools` in YAML)
fn merge_all_tools(base: &[&str], discovered: &[String], config_extra: Option<&str>) -> String {
    let mut tools: Vec<String> = base.iter().map(|s| (*s).to_owned()).collect();

    // Add tools discovered from enabled plugins (auto-discovery).
    for tool in discovered {
        if !tools.contains(tool) {
            tools.push(tool.clone());
        }
    }

    // Add user-configured extras (manual overrides).
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
///
/// Scans only lines within the `run:` block (stops at next top-level key).
fn extract_run_setting(config_content: &str, key: &str) -> Option<String> {
    let mut in_run = false;
    let prefix = format!("{key}:");

    for line in config_content.lines() {
        let trimmed = line.trim();

        // Enter run: section
        if trimmed == "run:" {
            in_run = true;
            continue;
        }

        // Exit on next top-level key (no leading whitespace)
        if in_run && !line.starts_with(' ') && !line.starts_with('\t') && !trimmed.is_empty() {
            break;
        }

        if in_run && trimmed.starts_with(&prefix) {
            let val = trimmed[prefix.len()..].trim();
            // Strip quotes and trailing comments
            let val = val.trim_matches('"').trim_matches('\'');
            let val = val.split('#').next().unwrap_or(val).trim();
            if !val.is_empty() {
                return Some(val.to_owned());
            }
        }
    }
    None
}

/// Extracts the string value for a given key from a JSON line.
///
/// Looks for `"key":"value"` and handles basic escape sequences.
/// Returns None if the pattern is not found.
fn extract_json_string_value(json_line: &str, key: &str) -> Option<String> {
    // Find the last occurrence of "key":" (last because text_delta has
    // nested objects and we want the innermost "text" value).
    let pattern = format!("\"{key}\":\"");
    let start = json_line.rfind(&pattern)? + pattern.len();
    let rest = &json_line[start..];

    // Read until unescaped closing quote.
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
    fn runtime_bootstrap_agent_returns_result() {
        let rt = super::runtime();
        let result = rt.bootstrap_agent("official.claude.session");
        assert!(result.is_ok());
    }

    #[test]
    fn runtime_register_mcp_returns_result() {
        let rt = super::runtime();
        let result = rt.register_mcp_server("official.claude.session");
        assert!(result.is_ok());
    }

    #[test]
    fn runtime_rejects_check() {
        let rt = super::runtime();
        let result = rt.run_check(
            "any.check",
            re_plugin::PluginCheckKind::Prepare,
            std::path::Path::new("/tmp"),
        );
        assert!(result.is_err());
    }

    // ── Stream-JSON parser tests ───────────────────────────────────

    #[test]
    fn extract_json_string_value_finds_text_delta() {
        let line =
            r#"{"type":"content_block_delta","delta":{"type":"text_delta","text":"Hello world"}}"#;
        let result = super::extract_json_string_value(line, "text");
        assert_eq!(result.as_deref(), Some("Hello world"));
    }

    #[test]
    fn extract_json_string_value_handles_escapes() {
        let line = r#"{"delta":{"type":"text_delta","text":"line1\nline2\t\"quoted\""}}"#;
        let result = super::extract_json_string_value(line, "text");
        assert_eq!(result.as_deref(), Some("line1\nline2\t\"quoted\""));
    }

    #[test]
    fn extract_json_string_value_finds_tool_name() {
        let line = r#"{"content_block":{"type":"tool_use","id":"abc","name":"Read","input":{}}}"#;
        let result = super::extract_json_string_value(line, "name");
        assert_eq!(result.as_deref(), Some("Read"));
    }

    #[test]
    fn extract_json_string_value_returns_none_for_missing_key() {
        let line = r#"{"type":"result","is_error":false}"#;
        let result = super::extract_json_string_value(line, "text");
        assert!(result.is_none());
    }

    // ── Config helpers tests ───────────────────────────────────────

    #[test]
    fn merge_all_tools_base_only() {
        let result = super::merge_all_tools(&["Bash", "Read"], &[], None);
        assert_eq!(result, "Bash,Read");
    }

    #[test]
    fn merge_all_tools_with_discovered() {
        let discovered = vec!["Skill".to_owned(), "mcp__archon__search".to_owned()];
        let result = super::merge_all_tools(&["Bash", "Read"], &discovered, None);
        assert_eq!(result, "Bash,Read,Skill,mcp__archon__search");
    }

    #[test]
    fn merge_all_tools_with_config_extras() {
        let result =
            super::merge_all_tools(&["Bash", "Read"], &[], Some("Skill,Agent,mcp__archon__*"));
        assert_eq!(result, "Bash,Read,Skill,Agent,mcp__archon__*");
    }

    #[test]
    fn merge_all_tools_deduplicates_across_sources() {
        let discovered = vec!["Skill".to_owned(), "Bash".to_owned()];
        let result = super::merge_all_tools(&["Bash", "Read"], &discovered, Some("Read,Agent"));
        assert_eq!(result, "Bash,Read,Skill,Agent");
    }

    #[test]
    fn extract_run_setting_finds_allowed_tools() {
        let config = "run:\n  workflow_plugin: official.bmad\n  allowed_tools: \"Skill,Agent\"\n  max_turns: 150\n";
        let result = super::extract_run_setting(config, "allowed_tools");
        assert_eq!(result.as_deref(), Some("Skill,Agent"));
    }

    #[test]
    fn extract_run_setting_finds_max_turns() {
        let config = "run:\n  max_turns: 150\n";
        let result = super::extract_run_setting(config, "max_turns");
        assert_eq!(result.as_deref(), Some("150"));
    }

    #[test]
    fn extract_run_setting_ignores_other_sections() {
        let config =
            "mcp:\n  allowed_tools: wrong\nrun:\n  allowed_tools: correct\nbudgets:\n  max: 100\n";
        let result = super::extract_run_setting(config, "allowed_tools");
        assert_eq!(result.as_deref(), Some("correct"));
    }

    #[test]
    fn extract_run_setting_returns_none_when_missing() {
        let config = "run:\n  workflow_plugin: official.bmad\n";
        let result = super::extract_run_setting(config, "allowed_tools");
        assert!(result.is_none());
    }
}
