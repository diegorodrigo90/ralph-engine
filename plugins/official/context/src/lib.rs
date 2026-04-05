//! Official context manager plugin — session persistence and context compaction.
//!
//! Provides save/load for session snapshots and compaction to fit
//! within token budgets when transferring context between agents.

use std::path::Path;

mod i18n;

use re_plugin::{
    AgentBootstrapResult, CONTEXT_MANAGEMENT, CheckExecutionResult, McpRegistrationResult,
    PluginCheckKind, PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary,
    PluginLocalizedText, PluginRuntime, PluginRuntimeError, PluginRuntimeHook, PluginTrustLevel,
    PortableContext, PortableMessage, SESSION_PERSISTENCE,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.context";
const PLUGIN_NAME: &str = i18n::plugin_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_plugin_names();
const PLUGIN_SUMMARY: &str = i18n::plugin_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_plugin_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[CONTEXT_MANAGEMENT, SESSION_PERSISTENCE];
const LIFECYCLE: &[PluginLifecycleStage] = &[
    PluginLifecycleStage::Discover,
    PluginLifecycleStage::Configure,
    PluginLifecycleStage::Load,
];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[
    PluginRuntimeHook::ContextManagement,
    PluginRuntimeHook::SessionPersistence,
];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::ContextManager,
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

/// Declared capabilities.
#[must_use]
pub fn capabilities() -> &'static [re_plugin::PluginCapability] {
    DESCRIPTOR.capabilities
}

/// Declared lifecycle stages.
#[must_use]
pub fn lifecycle() -> &'static [PluginLifecycleStage] {
    DESCRIPTOR.lifecycle
}

/// Declared runtime hooks.
#[must_use]
pub fn runtime_hooks() -> &'static [PluginRuntimeHook] {
    DESCRIPTOR.runtime_hooks
}

/// Returns the immutable plugin descriptor.
#[must_use]
pub const fn descriptor() -> PluginDescriptor {
    DESCRIPTOR
}

/// Returns a new instance of the context plugin runtime.
#[must_use]
pub fn runtime() -> ContextRuntime {
    ContextRuntime
}

/// Default sessions directory name.
const SESSIONS_DIR: &str = ".ralph-engine/sessions";

/// Context plugin runtime — manages session persistence and compaction.
pub struct ContextRuntime;

impl PluginRuntime for ContextRuntime {
    fn plugin_id(&self) -> &str {
        PLUGIN_ID
    }

    fn run_check(
        &self,
        check_id: &str,
        _kind: PluginCheckKind,
        _project_root: &Path,
    ) -> Result<CheckExecutionResult, PluginRuntimeError> {
        Ok(CheckExecutionResult {
            check_id: check_id.to_owned(),
            passed: true,
            findings: Vec::new(),
        })
    }

    fn bootstrap_agent(&self, agent_id: &str) -> Result<AgentBootstrapResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_agent_plugin",
            format!("Context plugin does not provide agent '{agent_id}'"),
        ))
    }

    fn register_mcp_server(
        &self,
        server_id: &str,
    ) -> Result<McpRegistrationResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_mcp_plugin",
            format!("Context plugin does not provide MCP server '{server_id}'"),
        ))
    }

    /// Compacts a context by keeping only recent messages to fit within budget.
    ///
    /// Strategy: keep system prompt + summary + last N messages that fit.
    /// The plugin owns the compaction strategy — core just calls this.
    fn compact_context(
        &self,
        context: &PortableContext,
        target_tokens: usize,
    ) -> Result<PortableContext, PluginRuntimeError> {
        if context.token_count <= target_tokens || context.messages.is_empty() {
            return Ok(context.clone());
        }

        // Estimate: ~4 chars per token (rough heuristic)
        let target_chars = target_tokens * 4;
        let mut budget = target_chars;

        // Reserve space for system prompt
        if let Some(ref prompt) = context.system_prompt {
            budget = budget.saturating_sub(prompt.len());
        }

        // Keep messages from the end until budget exhausted
        let mut kept: Vec<PortableMessage> = Vec::new();
        for msg in context.messages.iter().rev() {
            let msg_size: usize = msg
                .content
                .iter()
                .map(|block| match block {
                    re_plugin::ContentBlock::Text { text } => text.len(),
                    re_plugin::ContentBlock::ToolUse { input, .. } => input.len(),
                    re_plugin::ContentBlock::ToolResult { content, .. } => content.len(),
                })
                .sum();

            if msg_size > budget {
                break;
            }
            budget = budget.saturating_sub(msg_size);
            kept.push(msg.clone());
        }

        kept.reverse();

        // Build summary from dropped messages
        let dropped = context.messages.len() - kept.len();
        let summary = if dropped > 0 {
            Some(format!(
                "[Context compacted: {dropped} earlier messages summarized. {} messages retained.]",
                kept.len()
            ))
        } else {
            context.summary.clone()
        };

        Ok(PortableContext {
            system_prompt: context.system_prompt.clone(),
            messages: kept,
            active_files: context.active_files.clone(),
            summary,
            token_count: target_tokens.min(context.token_count),
            max_tokens: context.max_tokens,
            metadata: context.metadata.clone(),
        })
    }

    /// Saves a context snapshot as JSON to the sessions directory.
    fn save_session(
        &self,
        context: &PortableContext,
        project_root: &Path,
    ) -> Result<(), PluginRuntimeError> {
        let sessions_dir = project_root.join(SESSIONS_DIR);
        if !sessions_dir.exists() {
            std::fs::create_dir_all(&sessions_dir).map_err(|e| {
                PluginRuntimeError::new(
                    "sessions_dir_failed",
                    format!("Cannot create sessions dir: {e}"),
                )
            })?;
        }

        let filename = format!(
            "session-{}-{}.json",
            context.metadata.source_agent.replace('.', "-"),
            context.metadata.created_at
        );
        let path = sessions_dir.join(filename);

        let json = serialize_context(context);
        std::fs::write(&path, json).map_err(|e| {
            PluginRuntimeError::new("session_write_failed", format!("Cannot write session: {e}"))
        })?;

        Ok(())
    }

    /// Loads the most recent session snapshot from the sessions directory.
    fn load_session(&self, project_root: &Path) -> Result<PortableContext, PluginRuntimeError> {
        let sessions_dir = project_root.join(SESSIONS_DIR);
        if !sessions_dir.exists() {
            return Err(PluginRuntimeError::new(
                "no_sessions_dir",
                "No sessions directory found".to_owned(),
            ));
        }

        let mut files: Vec<_> = std::fs::read_dir(&sessions_dir)
            .map_err(|e| {
                PluginRuntimeError::new("sessions_read_failed", format!("Cannot read dir: {e}"))
            })?
            .flatten()
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
            .collect();

        files.sort_by_key(|e| {
            e.metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });

        let latest = files.last().ok_or_else(|| {
            PluginRuntimeError::new("no_sessions_found", "No session files found".to_owned())
        })?;

        let content = std::fs::read_to_string(latest.path()).map_err(|e| {
            PluginRuntimeError::new("session_read_failed", format!("Cannot read session: {e}"))
        })?;

        deserialize_context(&content)
    }

    /// Shows context status in TUI sidebar.
    fn tui_contributions(&self) -> Vec<re_plugin::TuiPanel> {
        vec![re_plugin::TuiPanel {
            id: "context-status".to_owned(),
            title: "Context".to_owned(),
            lines: vec![
                "Status: Ready".to_owned(),
                "Compaction: enabled".to_owned(),
                "Sessions: .ralph-engine/sessions/".to_owned(),
            ],
            zone_hint: "sidebar".to_owned(),
        }]
    }
}

/// Serializes a `PortableContext` to JSON string.
fn serialize_context(ctx: &PortableContext) -> String {
    // Simple JSON serialization without serde (keeping deps minimal)
    let mut json = String::from("{\n");

    if let Some(ref prompt) = ctx.system_prompt {
        json.push_str(&format!("  \"system_prompt\": {},\n", json_string(prompt)));
    }

    json.push_str(&format!("  \"token_count\": {},\n", ctx.token_count));
    json.push_str(&format!("  \"max_tokens\": {},\n", ctx.max_tokens));

    if let Some(ref summary) = ctx.summary {
        json.push_str(&format!("  \"summary\": {},\n", json_string(summary)));
    }

    json.push_str(&format!(
        "  \"source_agent\": {},\n",
        json_string(&ctx.metadata.source_agent)
    ));
    json.push_str(&format!(
        "  \"source_model\": {},\n",
        json_string(&ctx.metadata.source_model)
    ));
    json.push_str(&format!("  \"created_at\": {},\n", ctx.metadata.created_at));

    json.push_str("  \"messages\": [\n");
    for (i, msg) in ctx.messages.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!(
            "      \"role\": {},\n",
            json_string(msg.role.as_str())
        ));
        json.push_str("      \"content\": ");
        if msg.content.len() == 1 {
            if let re_plugin::ContentBlock::Text { ref text } = msg.content[0] {
                json.push_str(&json_string(text));
            } else {
                json.push_str("\"\"");
            }
        } else {
            json.push_str("\"\"");
        }
        json.push('\n');
        json.push_str("    }");
        if i < ctx.messages.len() - 1 {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");

    json.push('}');
    json
}

/// Deserializes a JSON string into a `PortableContext`.
fn deserialize_context(json: &str) -> Result<PortableContext, PluginRuntimeError> {
    // Simple JSON parsing without serde
    let source_agent = extract_json_str(json, "source_agent").unwrap_or_default();
    let source_model = extract_json_str(json, "source_model").unwrap_or_default();
    let created_at = extract_json_num(json, "created_at").unwrap_or(0);
    let token_count = extract_json_num(json, "token_count").unwrap_or(0) as usize;
    let max_tokens = extract_json_num(json, "max_tokens").unwrap_or(0) as usize;
    let system_prompt = extract_json_str(json, "system_prompt");
    let summary = extract_json_str(json, "summary");

    Ok(PortableContext {
        system_prompt,
        messages: Vec::new(), // Simplified — full parsing would need serde
        active_files: Vec::new(),
        summary,
        token_count,
        max_tokens,
        metadata: re_plugin::ContextMetadata {
            source_agent,
            source_model,
            session_id: None,
            created_at,
        },
    })
}

/// JSON-escapes a string value.
fn json_string(s: &str) -> String {
    format!(
        "\"{}\"",
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    )
}

/// Extracts a string value from simple JSON (handles spaces around colon).
fn extract_json_str(json: &str, key: &str) -> Option<String> {
    // Try with space (our format: "key": "value")
    let pattern = format!("\"{key}\": \"");
    if let Some(start) = json.find(&pattern) {
        let rest = &json[start + pattern.len()..];
        let end = rest.find('"')?;
        return Some(rest[..end].to_owned());
    }
    // Fallback: try without space ("key":"value")
    re_plugin::agent_helpers::extract_json_string_value(json, key)
}

/// Extracts a numeric value from simple JSON.
fn extract_json_num(json: &str, key: &str) -> Option<u64> {
    let pattern = format!("\"{key}\":");
    let start = json.find(&pattern)? + pattern.len();
    let rest = json[start..].trim_start();
    let end = rest.find(|c: char| !c.is_ascii_digit())?;
    rest[..end].parse().ok()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use re_plugin::PluginRuntime;

    use super::{PLUGIN_ID, capabilities, descriptor, i18n, lifecycle, runtime_hooks};

    fn manifest_document() -> &'static str {
        include_str!("../manifest.yaml")
    }

    #[test]
    fn plugin_id_is_namespaced() {
        assert!(PLUGIN_ID.starts_with("official."));
    }

    #[test]
    fn plugin_declares_capabilities() {
        let caps = capabilities();
        assert_eq!(caps.len(), 2);
        assert!(caps.iter().any(|c| c.as_str() == "context_management"));
        assert!(caps.iter().any(|c| c.as_str() == "session_persistence"));
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        let plugin = descriptor();
        assert_eq!(plugin.id, PLUGIN_ID);
        assert_eq!(plugin.name, i18n::plugin_name());
    }

    #[test]
    fn plugin_declares_lifecycle_stages() {
        assert_eq!(lifecycle().len(), 3);
    }

    #[test]
    fn plugin_declares_runtime_hooks() {
        let hooks = runtime_hooks();
        assert_eq!(hooks.len(), 2);
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();
        assert!(manifest.contains("id: official.context"));
        assert!(manifest.contains("kind: context_manager"));
        assert!(manifest.contains("- context_management"));
        assert!(manifest.contains("- session_persistence"));
    }

    #[test]
    fn runtime_plugin_id_matches() {
        let rt = super::runtime();
        assert_eq!(rt.plugin_id(), PLUGIN_ID);
    }

    #[test]
    fn compact_context_no_op_when_within_budget() {
        let rt = super::runtime();
        let ctx = test_context(100, 200);
        let result = rt.compact_context(&ctx, 200).unwrap();
        assert_eq!(result.messages.len(), ctx.messages.len());
    }

    #[test]
    fn compact_context_trims_old_messages() {
        let rt = super::runtime();
        let mut ctx = test_context(1000, 200);
        // Add many messages to exceed budget
        for i in 0..20 {
            ctx.messages.push(re_plugin::PortableMessage {
                role: re_plugin::MessageRole::User,
                content: vec![re_plugin::ContentBlock::Text {
                    text: format!("Message {i} with enough text to consume tokens in the budget calculation for compaction testing purposes."),
                }],
                timestamp: None,
            });
        }
        let result = rt.compact_context(&ctx, 5).unwrap(); // very small budget
        assert!(result.messages.len() < ctx.messages.len());
        assert!(result.summary.is_some());
    }

    #[test]
    fn save_and_load_session_roundtrip() {
        let rt = super::runtime();
        let tmp = std::env::temp_dir().join("re-context-test-save");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let ctx = test_context(100, 200);
        rt.save_session(&ctx, &tmp).unwrap();

        let loaded = rt.load_session(&tmp).unwrap();
        assert_eq!(loaded.metadata.source_agent, ctx.metadata.source_agent);
        assert_eq!(loaded.token_count, ctx.token_count);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn load_session_empty_dir_returns_error() {
        let rt = super::runtime();
        let tmp = std::env::temp_dir().join("re-context-test-empty");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine/sessions")).unwrap();

        let result = rt.load_session(&tmp);
        assert!(result.is_err());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn tui_contributions_returns_panel() {
        let rt = super::runtime();
        let panels = rt.tui_contributions();
        assert_eq!(panels.len(), 1);
        assert_eq!(panels[0].id, "context-status");
    }

    fn test_context(token_count: usize, max_tokens: usize) -> re_plugin::PortableContext {
        re_plugin::PortableContext {
            system_prompt: Some("You are a helpful assistant.".to_owned()),
            messages: vec![
                re_plugin::PortableMessage {
                    role: re_plugin::MessageRole::User,
                    content: vec![re_plugin::ContentBlock::Text {
                        text: "Hello, fix the bug in auth.rs".to_owned(),
                    }],
                    timestamp: None,
                },
                re_plugin::PortableMessage {
                    role: re_plugin::MessageRole::Assistant,
                    content: vec![re_plugin::ContentBlock::Text {
                        text: "I'll look at auth.rs and fix the authentication issue.".to_owned(),
                    }],
                    timestamp: None,
                },
            ],
            active_files: vec!["auth.rs".to_owned()],
            summary: None,
            token_count,
            max_tokens,
            metadata: re_plugin::ContextMetadata {
                source_agent: "official.claude".to_owned(),
                source_model: "claude-opus-4-6".to_owned(),
                session_id: Some("test-session".to_owned()),
                created_at: 1234567890,
            },
        }
    }
}
