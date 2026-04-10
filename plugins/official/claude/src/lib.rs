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

    /// Spawns the agent process for TUI integration (non-blocking).
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn spawn_agent(
        &self,
        _agent_id: &str,
        context: &PromptContext,
        project_root: &Path,
    ) -> Result<re_plugin::SpawnedAgent, PluginRuntimeError> {
        if re_plugin::probe_binary_on_path(AGENT_BINARY).is_none() {
            return Err(PluginRuntimeError::new(
                "agent_not_installed",
                format!("'{AGENT_BINARY}' not found on PATH."),
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

        let mut cmd = re_plugin::agent_helpers::build_command(&agent_config);
        let mut child = cmd
            .current_dir(project_root)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|err| {
                let _ = std::fs::remove_file(&context_file);
                PluginRuntimeError::new(
                    "agent_spawn_failed",
                    format!("Failed to spawn '{AGENT_BINARY}': {err}"),
                )
            })?;

        let pid = child.id();
        let stdout = child.stdout.take();

        Ok(re_plugin::SpawnedAgent {
            pid,
            stdout,
            child,
            context_file: Some(context_file),
        })
    }

    /// Discovers slash commands from Claude Code's filesystem.
    ///
    /// Scans `.claude/commands/*.md` and `.claude/skills/*/SKILL.md`
    /// in the project, plus built-in commands. The plugin doesn't
    /// hardcode commands — it reads what the agent has.
    fn discover_agent_commands(&self, project_root: &Path) -> Vec<re_plugin::AgentCommand> {
        let mut commands = Vec::new();

        // Scan .claude/commands/*.md
        let commands_dir = project_root.join(".claude/commands");
        if let Ok(entries) = std::fs::read_dir(&commands_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "md")
                    && let Some(name) = path.file_stem().and_then(|s| s.to_str())
                {
                    let description =
                        read_first_heading(&path).unwrap_or_else(|| format!("Command: {name}"));
                    commands.push(re_plugin::AgentCommand {
                        name: name.to_owned(),
                        description,
                        plugin_id: PLUGIN_ID.to_owned(),
                    });
                }
            }
        }

        // Scan .claude/skills/*/SKILL.md
        let skills_dir = project_root.join(".claude/skills");
        if let Ok(entries) = std::fs::read_dir(&skills_dir) {
            for entry in entries.flatten() {
                let skill_file = entry.path().join("SKILL.md");
                if skill_file.exists()
                    && let Some(name) = entry.file_name().to_str()
                {
                    let description =
                        read_first_heading(&skill_file).unwrap_or_else(|| format!("Skill: {name}"));
                    commands.push(re_plugin::AgentCommand {
                        name: name.to_owned(),
                        description,
                        plugin_id: PLUGIN_ID.to_owned(),
                    });
                }
            }
        }

        commands.sort_by(|a, b| a.name.cmp(&b.name));
        commands
    }

    /// Claude Code supports up to 1M tokens context (Opus 4.6).
    fn context_window_size(&self) -> usize {
        1_000_000
    }

    /// Exports the current Claude Code session as portable context.
    ///
    /// Reads the most recent `.jsonl` session file from
    /// `~/.claude/projects/<hash>/` and converts to `PortableContext`.
    fn export_session_context(
        &self,
        project_root: &Path,
    ) -> Result<re_plugin::PortableContext, re_plugin::PluginRuntimeError> {
        export_claude_session(project_root)
    }

    /// Imports a portable context into Claude by writing it as a
    /// system prompt file that gets passed via `--append-system-prompt-file`.
    fn import_session_context(
        &self,
        context: &re_plugin::PortableContext,
        project_root: &Path,
    ) -> Result<(), re_plugin::PluginRuntimeError> {
        let import_path = project_root.join(".ralph-engine/.imported-context.md");
        let mut content = String::new();

        if let Some(ref summary) = context.summary {
            content.push_str("## Previous Session Summary\n\n");
            content.push_str(summary);
            content.push_str("\n\n");
        }

        if let Some(ref system) = context.system_prompt {
            content.push_str("## Previous System Prompt\n\n");
            content.push_str(system);
            content.push_str("\n\n");
        }

        // Include recent messages as context
        content.push_str("## Previous Conversation\n\n");
        for msg in &context.messages {
            let role = msg.role.as_str();
            for block in &msg.content {
                if let re_plugin::ContentBlock::Text { text } = block {
                    content.push_str(&format!("**{role}**: {text}\n\n"));
                }
            }
        }

        if let Some(dir) = import_path.parent()
            && !dir.exists()
        {
            std::fs::create_dir_all(dir).map_err(|e| {
                re_plugin::PluginRuntimeError::new(
                    "import_dir_failed",
                    format!("Failed to create dir: {e}"),
                )
            })?;
        }

        std::fs::write(&import_path, content).map_err(|e| {
            re_plugin::PluginRuntimeError::new(
                "import_write_failed",
                format!("Failed to write imported context: {e}"),
            )
        })?;

        Ok(())
    }

    /// Contributes a TUI sidebar panel showing agent connection status.
    fn tui_contributions(&self) -> Vec<re_plugin::TuiPanel> {
        let binary_available = re_plugin::probe_binary_on_path("claude").is_some();
        let status = if binary_available {
            "Available"
        } else {
            "Not found"
        };

        let sev = if binary_available {
            re_plugin::Severity::Success
        } else {
            re_plugin::Severity::Error
        };

        vec![re_plugin::TuiPanel {
            id: "claude-status".to_owned(),
            title: "Claude".to_owned(),
            blocks: vec![
                re_plugin::TuiBlock::indicator("Binary", status, sev),
                re_plugin::TuiBlock::pairs(vec![
                    ("Mode".to_owned(), "-p (prompt)".to_owned()),
                    ("Model".to_owned(), "claude-opus-4-6".to_owned()),
                    ("Stream".to_owned(), "JSON".to_owned()),
                ]),
            ],
            zone_hint: "sidebar".to_owned(),
        }]
    }

    /// Reports current session usage from Claude API responses.
    ///
    /// Claude's API returns `usage.input_tokens` and `usage.output_tokens`
    /// in each message response. Cost is calculated here using known
    /// model pricing — core never knows about Anthropic pricing.
    fn report_usage(&self) -> Option<re_plugin::UsageReport> {
        // In real use, this reads from the active session's accumulated
        // token counts (parsed from stream-json events). For now, returns
        // None until the stream parser wires usage events.
        None
    }

    fn thinking_message(&self, tick: usize) -> Option<String> {
        const MESSAGES: &[&str] = &[
            "Thinking...",
            "Reasoning deeply...",
            "Analyzing the codebase...",
            "Considering approaches...",
            "Reading between the lines...",
            "Connecting the dots...",
            "Weighing trade-offs...",
            "Crafting a solution...",
            "Almost there...",
            "Exploring possibilities...",
            "Mapping dependencies...",
            "Planning the implementation...",
        ];
        // Rotate every ~4 seconds (80 ticks at 50ms)
        let idx = (tick / 80) % MESSAGES.len();
        Some(MESSAGES[idx].to_owned())
    }
}

/// Exports the most recent Claude Code session as `PortableContext`.
///
/// Claude stores sessions as JSONL in `~/.claude/projects/<hash>/<uuid>.jsonl`.
/// Each line is a JSON event. We parse user/assistant messages and tool calls.
fn export_claude_session(
    project_root: &Path,
) -> Result<re_plugin::PortableContext, re_plugin::PluginRuntimeError> {
    // Find the Claude project directory
    let project_dir = find_claude_project_dir(project_root)?;

    // Find the most recent .jsonl file
    let mut sessions: Vec<_> = std::fs::read_dir(&project_dir)
        .map_err(|e| {
            re_plugin::PluginRuntimeError::new(
                "session_dir_read_failed",
                format!("Cannot read Claude sessions: {e}"),
            )
        })?
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "jsonl"))
        .collect();

    sessions.sort_by_key(|e| {
        e.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });

    let latest = sessions.last().ok_or_else(|| {
        re_plugin::PluginRuntimeError::new(
            "no_session_found",
            "No Claude Code session files found".to_owned(),
        )
    })?;

    // Parse JSONL
    let content = std::fs::read_to_string(latest.path()).map_err(|e| {
        re_plugin::PluginRuntimeError::new(
            "session_read_failed",
            format!("Cannot read session file: {e}"),
        )
    })?;

    let mut messages = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Simple JSONL parsing — extract role and content
        if let Some(msg) = parse_jsonl_message(line) {
            messages.push(msg);
        }
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    Ok(re_plugin::PortableContext {
        system_prompt: None,
        messages,
        active_files: Vec::new(),
        summary: None,
        token_count: 0, // Would need tiktoken for accurate count
        max_tokens: 1_000_000,
        metadata: re_plugin::ContextMetadata {
            source_agent: PLUGIN_ID.to_owned(),
            source_model: "claude".to_owned(),
            session_id: latest
                .path()
                .file_stem()
                .and_then(|s| s.to_str())
                .map(String::from),
            created_at: now,
        },
    })
}

/// Finds the Claude project directory for the given project root.
fn find_claude_project_dir(
    project_root: &Path,
) -> Result<std::path::PathBuf, re_plugin::PluginRuntimeError> {
    let home = std::env::var("HOME").unwrap_or_default();
    let claude_dir = std::path::PathBuf::from(&home).join(".claude/projects");

    if !claude_dir.exists() {
        return Err(re_plugin::PluginRuntimeError::new(
            "no_claude_dir",
            "~/.claude/projects/ not found".to_owned(),
        ));
    }

    // Check if the project root path hash matches a directory name.
    if let Some(dir) = find_dir_by_path_hash(&claude_dir, project_root) {
        return Ok(dir);
    }

    // Fall back to the directory with the most recently modified session file.
    find_dir_by_recent_session(&claude_dir).ok_or_else(|| {
        re_plugin::PluginRuntimeError::new(
            "no_claude_project",
            "No Claude Code project directory found with session files".to_owned(),
        )
    })
}

/// Finds a project directory whose name contains the path-derived hash prefix.
fn find_dir_by_path_hash(claude_dir: &Path, project_root: &Path) -> Option<std::path::PathBuf> {
    let canonical = project_root.to_string_lossy().to_string();
    let hash_prefix = format!("-{}-", canonical.replace('/', "-"));

    let entries = std::fs::read_dir(claude_dir).ok()?;
    for entry in entries.flatten() {
        if entry
            .file_name()
            .to_str()
            .is_some_and(|name| name.contains(&hash_prefix))
        {
            return Some(entry.path());
        }
    }
    None
}

/// Finds the project directory with the most recently modified `.jsonl` session file.
fn find_dir_by_recent_session(claude_dir: &Path) -> Option<std::path::PathBuf> {
    let mut best_dir = None;
    let mut best_time = std::time::SystemTime::UNIX_EPOCH;

    let entries = std::fs::read_dir(claude_dir).ok()?;
    for entry in entries.flatten() {
        if !entry.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }
        if let Some(modified) = most_recent_jsonl_time(&entry.path())
            && modified > best_time
        {
            best_time = modified;
            best_dir = Some(entry.path());
        }
    }
    best_dir
}

/// Returns the most recent modification time of any `.jsonl` file in a directory.
fn most_recent_jsonl_time(dir: &Path) -> Option<std::time::SystemTime> {
    let files = std::fs::read_dir(dir).ok()?;
    let mut best = None;
    for file in files.flatten() {
        if file.path().extension().is_some_and(|e| e == "jsonl")
            && let Ok(meta) = file.metadata()
            && let Ok(modified) = meta.modified()
            && best.is_none_or(|b| modified > b)
        {
            best = Some(modified);
        }
    }
    best
}

/// Parses one JSONL line from a Claude session into a `PortableMessage`.
fn parse_jsonl_message(line: &str) -> Option<re_plugin::PortableMessage> {
    // Extract "role" and "content" from JSON
    let role_str = re_plugin::agent_helpers::extract_json_string_value(line, "role")?;
    let role = match role_str.as_str() {
        "user" => re_plugin::MessageRole::User,
        "assistant" => re_plugin::MessageRole::Assistant,
        "system" => re_plugin::MessageRole::System,
        _ => return None,
    };

    // Try to extract text content
    let mut blocks = Vec::new();
    if let Some(content) = re_plugin::agent_helpers::extract_json_string_value(line, "content") {
        blocks.push(re_plugin::ContentBlock::Text { text: content });
    }

    if blocks.is_empty() {
        return None;
    }

    Some(re_plugin::PortableMessage {
        role,
        content: blocks,
        timestamp: None,
    })
}

/// Reads the first markdown heading from a file as description.
fn read_first_heading(path: &Path) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(heading) = trimmed.strip_prefix("# ") {
            return Some(heading.trim().to_owned());
        }
    }
    None
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
