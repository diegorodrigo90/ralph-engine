//! Normalized agent event stream for TUI consumption.
//!
//! Parses stream-json lines from any agent (Claude, Codex) into
//! a uniform [`AgentEvent`] enum that the TUI renders without
//! knowing agent-specific formats.
//!
//! The existing `agent_helpers::read_stream_json_events()` in re-plugin
//! prints directly to stderr — fine for headless mode. This module
//! provides a structured alternative for TUI mode.

/// A normalized event from an agent's stream-json output.
///
/// The TUI only sees these — never raw JSON. Adding a new agent
/// requires only extending [`parse_stream_line`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentEvent {
    /// Agent produced text output (streaming delta).
    TextDelta(String),
    /// Agent is invoking a tool.
    ToolUse {
        /// Tool name (e.g. "Read", "Edit", "Bash").
        name: String,
    },
    /// Agent completed a tool invocation.
    ToolResult {
        /// Tool name.
        name: String,
        /// Whether the tool succeeded.
        success: bool,
    },
    /// Agent session completed.
    Complete {
        /// Whether the session ended with an error.
        is_error: bool,
    },
    /// Agent produced a system/status message.
    System(String),
    /// Unrecognized line (preserved for debugging).
    Unknown(String),
}

impl AgentEvent {
    /// Returns a one-line summary for the TUI activity stream.
    #[must_use]
    pub fn activity_line(&self) -> String {
        match self {
            Self::TextDelta(text) => text.clone(),
            Self::ToolUse { name } => format!(">> Tool: {name}"),
            Self::ToolResult { name, success } => {
                let status = if *success { "OK" } else { "FAIL" };
                format!(">> Tool result: {name} [{status}]")
            }
            Self::Complete { is_error } => {
                if *is_error {
                    ">> Agent: error".to_owned()
                } else {
                    ">> Agent: completed".to_owned()
                }
            }
            Self::System(msg) => format!(">> {msg}"),
            Self::Unknown(line) => {
                // Truncate long unknown lines
                if line.len() > 120 {
                    let truncated: String = line.chars().take(117).collect();
                    format!("{truncated}...")
                } else {
                    line.clone()
                }
            }
        }
    }

    /// Whether this event represents the end of the agent stream.
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Complete { .. })
    }
}

/// Parses a single stream-json line into an [`AgentEvent`].
///
/// Uses pattern matching on key JSON fields without a full JSON parser
/// (same approach as `agent_helpers::read_stream_json_events`).
/// This keeps `re-tui` dependency-free from `serde_json`.
#[must_use]
pub fn parse_stream_line(line: &str) -> AgentEvent {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return AgentEvent::Unknown(String::new());
    }

    // text_delta → TextDelta
    if trimmed.contains("\"text_delta\"")
        && let Some(text) = extract_text_value(trimmed)
    {
        return AgentEvent::TextDelta(text);
    }

    // tool_use → ToolUse
    if trimmed.contains("\"tool_use\"")
        && trimmed.contains("\"name\"")
        && let Some(name) = extract_name_value(trimmed)
    {
        return AgentEvent::ToolUse { name };
    }

    // tool_result → ToolResult
    if trimmed.contains("\"tool_result\"") {
        let name = extract_name_value(trimmed).unwrap_or_default();
        let success = !trimmed.contains("\"is_error\":true");
        return AgentEvent::ToolResult { name, success };
    }

    // result (session end) → Complete
    if trimmed.contains("\"type\":\"result\"") {
        let is_error = trimmed.contains("\"is_error\":true");
        return AgentEvent::Complete { is_error };
    }

    // system messages
    if trimmed.contains("\"type\":\"system\"")
        && let Some(msg) = extract_text_value(trimmed)
    {
        return AgentEvent::System(msg);
    }

    AgentEvent::Unknown(trimmed.to_owned())
}

/// Parses multiple lines (e.g. from a buffered read) into events.
#[must_use]
pub fn parse_stream_lines(input: &str) -> Vec<AgentEvent> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_stream_line)
        .collect()
}

/// Extracts the `"text"` field value from a JSON line.
fn extract_text_value(line: &str) -> Option<String> {
    extract_json_string_value(line, "text")
}

/// Extracts the `"name"` field value from a JSON line.
fn extract_name_value(line: &str) -> Option<String> {
    extract_json_string_value(line, "name")
}

/// Extracts a string value for a given key from a JSON-like line.
///
/// Simple pattern: finds `"key":"value"` or `"key": "value"`.
/// Does NOT handle escaped quotes in values — sufficient for
/// tool names and short text fragments.
fn extract_json_string_value(line: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{key}\"");
    let key_pos = line.find(&pattern)?;
    let after_key = &line[(key_pos + pattern.len())..];

    // Skip optional whitespace and colon
    let after_colon = after_key.trim_start().strip_prefix(':')?;
    let after_ws = after_colon.trim_start();

    // Expect opening quote
    let after_quote = after_ws.strip_prefix('"')?;

    // Find closing quote (simple — no escaped quotes)
    let end = after_quote.find('"')?;
    Some(after_quote[..end].to_owned())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn parse_text_delta() {
        let line =
            r#"{"type":"content_block_delta","delta":{"type":"text_delta","text":"Hello world"}}"#;
        let event = parse_stream_line(line);
        assert_eq!(event, AgentEvent::TextDelta("Hello world".to_owned()));
    }

    #[test]
    fn parse_tool_use() {
        let line = r#"{"type":"content_block_start","content_block":{"type":"tool_use","name":"Read","id":"tool_123"}}"#;
        let event = parse_stream_line(line);
        assert_eq!(
            event,
            AgentEvent::ToolUse {
                name: "Read".to_owned()
            }
        );
    }

    #[test]
    fn parse_tool_result_success() {
        let line = r#"{"type":"tool_result","name":"Read","content":"file contents"}"#;
        let event = parse_stream_line(line);
        assert_eq!(
            event,
            AgentEvent::ToolResult {
                name: "Read".to_owned(),
                success: true
            }
        );
    }

    #[test]
    fn parse_tool_result_error() {
        let line = r#"{"type":"tool_result","name":"Bash","is_error":true}"#;
        let event = parse_stream_line(line);
        assert_eq!(
            event,
            AgentEvent::ToolResult {
                name: "Bash".to_owned(),
                success: false
            }
        );
    }

    #[test]
    fn parse_complete_success() {
        let line = r#"{"type":"result","subtype":"success"}"#;
        let event = parse_stream_line(line);
        assert_eq!(event, AgentEvent::Complete { is_error: false });
    }

    #[test]
    fn parse_complete_error() {
        let line = r#"{"type":"result","is_error":true}"#;
        let event = parse_stream_line(line);
        assert_eq!(event, AgentEvent::Complete { is_error: true });
    }

    #[test]
    fn parse_system_message() {
        let line = r#"{"type":"system","text":"Initializing session"}"#;
        let event = parse_stream_line(line);
        assert_eq!(event, AgentEvent::System("Initializing session".to_owned()));
    }

    #[test]
    fn parse_unknown_line() {
        let line = r#"{"type":"something_else","data":123}"#;
        let event = parse_stream_line(line);
        assert!(matches!(event, AgentEvent::Unknown(_)));
    }

    #[test]
    fn parse_empty_line() {
        assert!(matches!(
            parse_stream_line(""),
            AgentEvent::Unknown(ref s) if s.is_empty()
        ));
        assert!(matches!(
            parse_stream_line("   "),
            AgentEvent::Unknown(ref s) if s.is_empty()
        ));
    }

    #[test]
    fn activity_line_text_delta() {
        let event = AgentEvent::TextDelta("hello".to_owned());
        assert_eq!(event.activity_line(), "hello");
    }

    #[test]
    fn activity_line_tool_use() {
        let event = AgentEvent::ToolUse {
            name: "Edit".to_owned(),
        };
        assert_eq!(event.activity_line(), ">> Tool: Edit");
    }

    #[test]
    fn activity_line_complete() {
        assert_eq!(
            AgentEvent::Complete { is_error: false }.activity_line(),
            ">> Agent: completed"
        );
        assert_eq!(
            AgentEvent::Complete { is_error: true }.activity_line(),
            ">> Agent: error"
        );
    }

    #[test]
    fn is_terminal_only_for_complete() {
        assert!(AgentEvent::Complete { is_error: false }.is_terminal());
        assert!(AgentEvent::Complete { is_error: true }.is_terminal());
        assert!(!AgentEvent::TextDelta("x".to_owned()).is_terminal());
        assert!(
            !AgentEvent::ToolUse {
                name: "x".to_owned()
            }
            .is_terminal()
        );
    }

    #[test]
    fn parse_stream_lines_filters_empty() {
        let input = r#"
{"type":"content_block_delta","delta":{"type":"text_delta","text":"Hi"}}

{"type":"result","subtype":"success"}
"#;
        let events = parse_stream_lines(input);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], AgentEvent::TextDelta("Hi".to_owned()));
        assert_eq!(events[1], AgentEvent::Complete { is_error: false });
    }

    #[test]
    fn activity_line_truncates_long_unknown() {
        let long = "x".repeat(200);
        let event = AgentEvent::Unknown(long);
        let line = event.activity_line();
        assert!(line.len() < 125);
        assert!(line.ends_with("..."));
    }

    #[test]
    fn extract_json_string_value_handles_spaces() {
        let line = r#"{"name" : "Bash"}"#;
        assert_eq!(
            extract_json_string_value(line, "name"),
            Some("Bash".to_owned())
        );
    }

    #[test]
    fn extract_json_string_value_no_match() {
        let line = r#"{"other":"value"}"#;
        assert_eq!(extract_json_string_value(line, "name"), None);
    }
}
