//! Shared helpers for agent runtime plugins.
//!
//! Functions in this module are used by plugins that launch coding agents
//! (e.g. Claude, `ClaudeBox`, Codex). They handle tool merging, config
//! extraction, and stream-JSON parsing.

use std::io::BufRead as _;

// ── Tool merging ─────────────────────────────────────────────────────

/// Merges tools from three sources into a deduplicated comma-separated list:
/// 1. Base tools (agent plugin's own: Bash, Read, Edit, etc.)
/// 2. Discovered tools (from all enabled plugins via `required_tools()`)
/// 3. Config extras (user overrides from `run.allowed_tools` in YAML)
#[must_use]
pub fn merge_all_tools(base: &[&str], discovered: &[String], config_extra: Option<&str>) -> String {
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

// ── Config extraction ────────────────────────────────────────────────

/// Extracts a simple `key: value` from the `run:` section of config YAML.
///
/// Scans only lines within the `run:` block (stops at next top-level key).
#[must_use]
pub fn extract_run_setting(config_content: &str, key: &str) -> Option<String> {
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

// ── JSON string extraction ───────────────────────────────────────────

/// Extracts the string value for a given key from a JSON line.
///
/// Looks for `"key":"value"` and handles basic escape sequences.
/// Uses `rfind` to match the innermost value when nested objects exist
/// (e.g. `text_delta` events have nested `"text"` keys).
/// Returns `None` if the pattern is not found.
#[must_use]
pub fn extract_json_string_value(json_line: &str, key: &str) -> Option<String> {
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

// ── Stream-JSON event parser ─────────────────────────────────────────

/// Reads Claude Code stream-json events from stdout, forwards text
/// to stderr for user visibility, and returns the final message.
///
/// Each line is a JSON object. We match key patterns without a JSON
/// parser to avoid adding dependencies:
///
/// - `text_delta` events → extract text, print to stderr
/// - `tool_use` events → print tool name summary to stderr
/// - `result` event → extract summary
pub fn read_stream_json_events(stdout: Option<std::process::ChildStdout>) -> String {
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

// ── Agent command configuration ──────────────────────────────────────

/// Configuration for launching a coding agent in `-p` (programmatic) mode.
///
/// This struct captures all the arguments needed to spawn the agent
/// process. It separates the pure logic (testable) from the I/O
/// boundary (subprocess spawn).
#[derive(Debug, Clone)]
pub struct AgentCommandConfig {
    /// The binary name (e.g. "claude", "claudebox").
    pub binary: String,
    /// User-facing prompt (the `-p` argument).
    pub user_prompt: String,
    /// Path to the context file for `--append-system-prompt-file`.
    pub context_file: std::path::PathBuf,
    /// Comma-separated list of allowed tools.
    pub allowed_tools: String,
    /// Maximum agent turns.
    pub max_turns: String,
    /// Whether to use `--dangerously-skip-permissions` (true) or
    /// `--permission-mode auto` (false).
    pub autonomous: bool,
}

/// Input parameters for building an agent command configuration.
pub struct AgentCommandInput<'a> {
    /// The binary name (e.g. "claude", "claudebox").
    pub binary: &'a str,
    /// Base tools always auto-approved by this agent plugin.
    pub base_tools: &'a [&'a str],
    /// Default max turns when not configured.
    pub default_max_turns: &'a str,
    /// Work item ID (e.g. "5.3").
    pub work_item_id: &'a str,
    /// Tools discovered from all enabled plugins.
    pub discovered_tools: &'a [String],
    /// Raw config YAML content.
    pub config_content: &'a str,
    /// Whether to use `--dangerously-skip-permissions`.
    pub autonomous: bool,
    /// Path to the context file for `--append-system-prompt-file`.
    pub context_file: std::path::PathBuf,
}

/// Builds the agent command configuration from context and project state.
///
/// This is the pure-logic portion of `launch_agent()`: it reads config,
/// merges tools, builds the user prompt, and determines permission mode.
/// The caller is responsible for writing the context file and spawning
/// the process.
#[must_use]
pub fn build_agent_command_config(input: &AgentCommandInput<'_>) -> AgentCommandConfig {
    let config_extra = extract_run_setting(input.config_content, "allowed_tools");
    let allowed_tools = merge_all_tools(
        input.base_tools,
        input.discovered_tools,
        config_extra.as_deref(),
    );
    let max_turns = extract_run_setting(input.config_content, "max_turns")
        .unwrap_or_else(|| input.default_max_turns.to_owned());

    let user_prompt = format!(
        "Implement work item {}.\n\n\
         Your system prompt contains the <task>, <rules>, <context>, and <constraints> sections.\n\
         Expected outcome: all acceptance criteria implemented with tests, \
         quality gates passing, and tracking files updated.",
        input.work_item_id,
    );

    AgentCommandConfig {
        binary: input.binary.to_owned(),
        user_prompt,
        context_file: input.context_file.clone(),
        allowed_tools,
        max_turns,
        autonomous: input.autonomous,
    }
}

/// Converts an `AgentCommandConfig` into `std::process::Command` arguments.
///
/// Returns the configured Command ready for `.spawn()`.
#[must_use]
pub fn build_command(config: &AgentCommandConfig) -> std::process::Command {
    let mut cmd = std::process::Command::new(&config.binary);
    cmd.arg("-p")
        .arg(&config.user_prompt)
        .arg("--append-system-prompt-file")
        .arg(&config.context_file)
        .arg("--allowedTools")
        .arg(&config.allowed_tools)
        .arg("--output-format")
        .arg("stream-json")
        .arg("--verbose")
        .arg("--max-turns")
        .arg(&config.max_turns);

    if config.autonomous {
        cmd.arg("--dangerously-skip-permissions");
    } else {
        cmd.arg("--permission-mode").arg("auto");
    }

    cmd
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    // ── extract_json_string_value ────────────────────────────────────

    #[test]
    fn extract_json_finds_text_delta() {
        let line =
            r#"{"type":"content_block_delta","delta":{"type":"text_delta","text":"Hello world"}}"#;
        let result = extract_json_string_value(line, "text");
        assert_eq!(result.as_deref(), Some("Hello world"));
    }

    #[test]
    fn extract_json_handles_escapes() {
        let line = r#"{"delta":{"type":"text_delta","text":"line1\nline2\t\"quoted\""}}"#;
        let result = extract_json_string_value(line, "text");
        assert_eq!(result.as_deref(), Some("line1\nline2\t\"quoted\""));
    }

    #[test]
    fn extract_json_handles_slash_escape() {
        let line = r#"{"text":"path\/to\/file"}"#;
        let result = extract_json_string_value(line, "text");
        assert_eq!(result.as_deref(), Some("path/to/file"));
    }

    #[test]
    fn extract_json_handles_backslash_escape() {
        let line = r#"{"text":"C:\\Users\\test"}"#;
        let result = extract_json_string_value(line, "text");
        assert_eq!(result.as_deref(), Some("C:\\Users\\test"));
    }

    #[test]
    fn extract_json_finds_tool_name() {
        let line = r#"{"content_block":{"type":"tool_use","id":"abc","name":"Read","input":{}}}"#;
        let result = extract_json_string_value(line, "name");
        assert_eq!(result.as_deref(), Some("Read"));
    }

    #[test]
    fn extract_json_returns_none_for_missing_key() {
        let line = r#"{"type":"result","is_error":false}"#;
        let result = extract_json_string_value(line, "text");
        assert!(result.is_none());
    }

    #[test]
    fn extract_json_returns_none_for_empty_input() {
        let result = extract_json_string_value("", "text");
        assert!(result.is_none());
    }

    #[test]
    fn extract_json_handles_unknown_escape() {
        let line = r#"{"text":"hello\xworld"}"#;
        let result = extract_json_string_value(line, "text");
        assert_eq!(result.as_deref(), Some("hello\\xworld"));
    }

    // ── merge_all_tools ──────────────────────────────────────────────

    #[test]
    fn merge_base_only() {
        let result = merge_all_tools(&["Bash", "Read"], &[], None);
        assert_eq!(result, "Bash,Read");
    }

    #[test]
    fn merge_with_discovered() {
        let discovered = vec!["Skill".to_owned(), "mcp__archon__search".to_owned()];
        let result = merge_all_tools(&["Bash", "Read"], &discovered, None);
        assert_eq!(result, "Bash,Read,Skill,mcp__archon__search");
    }

    #[test]
    fn merge_with_config_extras() {
        let result = merge_all_tools(&["Bash", "Read"], &[], Some("Skill,Agent,mcp__archon__*"));
        assert_eq!(result, "Bash,Read,Skill,Agent,mcp__archon__*");
    }

    #[test]
    fn merge_deduplicates_across_sources() {
        let discovered = vec!["Skill".to_owned(), "Bash".to_owned()];
        let result = merge_all_tools(&["Bash", "Read"], &discovered, Some("Read,Agent"));
        assert_eq!(result, "Bash,Read,Skill,Agent");
    }

    #[test]
    fn merge_handles_empty_config_extras() {
        let result = merge_all_tools(&["Bash"], &[], Some(""));
        assert_eq!(result, "Bash");
    }

    #[test]
    fn merge_handles_whitespace_in_config() {
        let result = merge_all_tools(&["Bash"], &[], Some(" Skill , Agent "));
        assert_eq!(result, "Bash,Skill,Agent");
    }

    #[test]
    fn merge_handles_all_empty() {
        let result = merge_all_tools(&[], &[], None);
        assert_eq!(result, "");
    }

    // ── extract_run_setting ──────────────────────────────────────────

    #[test]
    fn extract_setting_finds_allowed_tools() {
        let config = "run:\n  workflow_plugin: official.bmad\n  allowed_tools: \"Skill,Agent\"\n  max_turns: 150\n";
        let result = extract_run_setting(config, "allowed_tools");
        assert_eq!(result.as_deref(), Some("Skill,Agent"));
    }

    #[test]
    fn extract_setting_finds_max_turns() {
        let config = "run:\n  max_turns: 150\n";
        let result = extract_run_setting(config, "max_turns");
        assert_eq!(result.as_deref(), Some("150"));
    }

    #[test]
    fn extract_setting_ignores_other_sections() {
        let config =
            "mcp:\n  allowed_tools: wrong\nrun:\n  allowed_tools: correct\nbudgets:\n  max: 100\n";
        let result = extract_run_setting(config, "allowed_tools");
        assert_eq!(result.as_deref(), Some("correct"));
    }

    #[test]
    fn extract_setting_returns_none_when_missing() {
        let config = "run:\n  workflow_plugin: official.bmad\n";
        let result = extract_run_setting(config, "allowed_tools");
        assert!(result.is_none());
    }

    #[test]
    fn extract_setting_returns_none_without_run_section() {
        let config = "mcp:\n  enabled: true\n";
        let result = extract_run_setting(config, "allowed_tools");
        assert!(result.is_none());
    }

    #[test]
    fn extract_setting_strips_trailing_comments() {
        let config = "run:\n  max_turns: 200 # default\n";
        let result = extract_run_setting(config, "max_turns");
        assert_eq!(result.as_deref(), Some("200"));
    }

    #[test]
    fn extract_setting_strips_single_quotes() {
        let config = "run:\n  agent_plugin: 'official.claude'\n";
        let result = extract_run_setting(config, "agent_plugin");
        assert_eq!(result.as_deref(), Some("official.claude"));
    }

    #[test]
    fn extract_setting_handles_tab_indentation() {
        let config = "run:\n\tmax_turns: 100\n";
        let result = extract_run_setting(config, "max_turns");
        assert_eq!(result.as_deref(), Some("100"));
    }

    // ── read_stream_json_events ──────────────────────────────────────

    #[test]
    fn read_stream_returns_empty_for_none() {
        let result = read_stream_json_events(None);
        assert!(result.is_empty());
    }

    // ── build_agent_command_config ───────────────────────────────────

    #[test]
    fn build_config_basic() {
        let config = build_agent_command_config(&AgentCommandInput {
            binary: "claude",
            base_tools: &["Bash", "Read"],
            default_max_turns: "200",
            work_item_id: "5.3",
            discovered_tools: &[],
            config_content: "",
            autonomous: false,
            context_file: std::path::PathBuf::from("/tmp/ctx.md"),
        });
        assert_eq!(config.binary, "claude");
        assert!(config.user_prompt.contains("5.3"));
        assert_eq!(config.allowed_tools, "Bash,Read");
        assert_eq!(config.max_turns, "200");
        assert!(!config.autonomous);
        assert_eq!(config.context_file, std::path::PathBuf::from("/tmp/ctx.md"));
    }

    #[test]
    fn build_config_merges_discovered_tools() {
        let discovered = vec!["Skill".to_owned(), "mcp__archon__search".to_owned()];
        let config = build_agent_command_config(&AgentCommandInput {
            binary: "claude",
            base_tools: &["Bash", "Read"],
            default_max_turns: "200",
            work_item_id: "5.3",
            discovered_tools: &discovered,
            config_content: "",
            autonomous: false,
            context_file: std::path::PathBuf::from("/tmp/ctx.md"),
        });
        assert!(config.allowed_tools.contains("Skill"));
        assert!(config.allowed_tools.contains("mcp__archon__search"));
        assert!(config.allowed_tools.contains("Bash"));
    }

    #[test]
    fn build_config_reads_max_turns_from_config() {
        let config = build_agent_command_config(&AgentCommandInput {
            binary: "claude",
            base_tools: &["Bash"],
            default_max_turns: "200",
            work_item_id: "1.1",
            discovered_tools: &[],
            config_content: "run:\n  max_turns: 300\n",
            autonomous: false,
            context_file: std::path::PathBuf::from("/tmp/ctx.md"),
        });
        assert_eq!(config.max_turns, "300");
    }

    #[test]
    fn build_config_reads_allowed_tools_from_config() {
        let config = build_agent_command_config(&AgentCommandInput {
            binary: "claude",
            base_tools: &["Bash"],
            default_max_turns: "200",
            work_item_id: "1.1",
            discovered_tools: &[],
            config_content: "run:\n  allowed_tools: \"Agent,WebSearch\"\n",
            autonomous: false,
            context_file: std::path::PathBuf::from("/tmp/ctx.md"),
        });
        assert!(config.allowed_tools.contains("Agent"));
        assert!(config.allowed_tools.contains("WebSearch"));
        assert!(config.allowed_tools.contains("Bash"));
    }

    #[test]
    fn build_config_autonomous_mode() {
        let config = build_agent_command_config(&AgentCommandInput {
            binary: "claude",
            base_tools: &["Bash"],
            default_max_turns: "200",
            work_item_id: "1.1",
            discovered_tools: &[],
            config_content: "",
            autonomous: true,
            context_file: std::path::PathBuf::from("/tmp/ctx.md"),
        });
        assert!(config.autonomous);
    }

    #[test]
    fn build_config_user_prompt_is_outcome_based() {
        let config = build_agent_command_config(&AgentCommandInput {
            binary: "claude",
            base_tools: &["Bash"],
            default_max_turns: "200",
            work_item_id: "5.3",
            discovered_tools: &[],
            config_content: "",
            autonomous: false,
            context_file: std::path::PathBuf::from("/tmp/ctx.md"),
        });
        assert!(config.user_prompt.contains("Implement work item 5.3"));
        assert!(config.user_prompt.contains("acceptance criteria"));
        assert!(config.user_prompt.contains("quality gates"));
    }

    // ── build_command ────────────────────────────────────────────────

    #[test]
    fn build_command_sets_p_mode() {
        let cfg = AgentCommandConfig {
            binary: "claude".to_owned(),
            user_prompt: "test prompt".to_owned(),
            context_file: std::path::PathBuf::from("/tmp/ctx.md"),
            allowed_tools: "Bash,Read".to_owned(),
            max_turns: "100".to_owned(),
            autonomous: false,
        };
        let cmd = build_command(&cfg);
        let args: Vec<_> = cmd
            .get_args()
            .map(|a| a.to_string_lossy().to_string())
            .collect();
        assert_eq!(args[0], "-p");
        assert_eq!(args[1], "test prompt");
        assert!(args.contains(&"--append-system-prompt-file".to_owned()));
        assert!(args.contains(&"--allowedTools".to_owned()));
        assert!(args.contains(&"Bash,Read".to_owned()));
        assert!(args.contains(&"stream-json".to_owned()));
        assert!(args.contains(&"--verbose".to_owned()));
        assert!(args.contains(&"--permission-mode".to_owned()));
        assert!(args.contains(&"auto".to_owned()));
    }

    #[test]
    fn build_command_autonomous_mode() {
        let cfg = AgentCommandConfig {
            binary: "claude".to_owned(),
            user_prompt: "test".to_owned(),
            context_file: std::path::PathBuf::from("/tmp/ctx.md"),
            allowed_tools: "Bash".to_owned(),
            max_turns: "50".to_owned(),
            autonomous: true,
        };
        let cmd = build_command(&cfg);
        let args: Vec<_> = cmd
            .get_args()
            .map(|a| a.to_string_lossy().to_string())
            .collect();
        assert!(args.contains(&"--dangerously-skip-permissions".to_owned()));
        assert!(!args.contains(&"--permission-mode".to_owned()));
    }
}
