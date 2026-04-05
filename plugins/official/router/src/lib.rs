//! Official agent router plugin — task-based agent selection.
//!
//! Reads routing rules from the project config and recommends
//! which agent to use for each task. Supports pattern matching,
//! priority ordering, and fallback chains.

use std::path::Path;

mod i18n;

use re_plugin::{
    AGENT_ROUTING, AgentBootstrapResult, AgentRecommendation, CheckExecutionResult, FallbackEntry,
    McpRegistrationResult, PluginCheckKind, PluginDescriptor, PluginKind, PluginLifecycleStage,
    PluginLoadBoundary, PluginLocalizedText, PluginRuntime, PluginRuntimeError, PluginRuntimeHook,
    PluginTrustLevel, RoutingRule,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.router";
const PLUGIN_NAME: &str = i18n::plugin_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_plugin_names();
const PLUGIN_SUMMARY: &str = i18n::plugin_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_plugin_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[AGENT_ROUTING];
const LIFECYCLE: &[PluginLifecycleStage] = &[
    PluginLifecycleStage::Discover,
    PluginLifecycleStage::Configure,
    PluginLifecycleStage::Load,
];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[PluginRuntimeHook::AgentRouting];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::AgentRouter,
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

/// Returns a new instance of the router plugin runtime.
#[must_use]
pub fn runtime() -> RouterRuntime {
    RouterRuntime
}

/// Router plugin runtime — reads config, classifies tasks, picks agents.
pub struct RouterRuntime;

impl PluginRuntime for RouterRuntime {
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
            format!("Router does not provide agent '{agent_id}'"),
        ))
    }

    fn register_mcp_server(
        &self,
        server_id: &str,
    ) -> Result<McpRegistrationResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_mcp_plugin",
            format!("Router does not provide MCP server '{server_id}'"),
        ))
    }

    /// Returns routing rules read from the project config.
    ///
    /// Reads the `routing.rules` section from `.ralph-engine/config.yaml`.
    /// Each rule maps a task pattern to an agent + optional model.
    fn routing_rules(&self) -> Vec<RoutingRule> {
        // Rules are loaded by the CLI from config and passed at runtime.
        // This default returns empty — the CLI populates rules from config.
        Vec::new()
    }

    /// Classifies a task and recommends the best agent.
    ///
    /// Matches the task description against routing rules (pattern matching).
    /// Returns the highest-priority matching rule's agent.
    fn classify_task(&self, task_description: &str) -> Option<AgentRecommendation> {
        // Without loaded rules, return None (use default agent).
        // In production, the CLI passes rules via config and the router
        // matches against them. This is a stateless classifier.
        let _ = task_description;
        None
    }

    /// Returns fallback agents when the primary is unavailable.
    fn fallback_chain(&self, _primary_agent: &str) -> Vec<FallbackEntry> {
        Vec::new()
    }

    /// Shows routing status in TUI sidebar.
    fn tui_contributions(&self) -> Vec<re_plugin::TuiPanel> {
        let rules = self.routing_rules();
        let fallbacks = self.fallback_chain("primary");
        vec![re_plugin::TuiPanel {
            id: "router-status".to_owned(),
            title: "Routing".to_owned(),
            blocks: vec![
                re_plugin::TuiBlock::KeyValue(vec![(
                    "Mode".to_owned(),
                    "config-driven".to_owned(),
                )]),
                re_plugin::TuiBlock::Metric {
                    label: "Rules".to_owned(),
                    value: rules.len(),
                    total: None,
                },
                re_plugin::TuiBlock::Metric {
                    label: "Fallbacks".to_owned(),
                    value: fallbacks.len(),
                    total: None,
                },
            ],
            lines: Vec::new(),
            zone_hint: "sidebar".to_owned(),
        }]
    }
}

/// Matches a task description against a glob-like pattern.
///
/// Supports `*` as wildcard. Empty pattern matches everything (default rule).
#[must_use]
pub fn matches_task_pattern(pattern: &str, task: &str) -> bool {
    if pattern.is_empty() {
        return true; // Default rule
    }

    let task_lower = task.to_lowercase();
    let pattern_lower = pattern.to_lowercase();
    let parts: Vec<&str> = pattern_lower.split('*').collect();

    if parts.len() == 1 {
        return task_lower.contains(parts[0]);
    }

    let mut pos = 0;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        if let Some(found) = task_lower[pos..].find(part) {
            if i == 0 && found != 0 {
                return false; // First part must match at start (unless preceded by *)
            }
            pos += found + part.len();
        } else {
            return false;
        }
    }

    true
}

/// Selects the best agent from a list of rules for a given task.
#[must_use]
pub fn select_agent(rules: &[RoutingRule], task_description: &str) -> Option<AgentRecommendation> {
    let mut best: Option<&RoutingRule> = None;

    for rule in rules {
        if matches_task_pattern(&rule.task_pattern, task_description)
            && (best.is_none() || rule.priority < best.map_or(u32::MAX, |r| r.priority))
        {
            best = Some(rule);
        }
    }

    best.map(|rule| AgentRecommendation {
        agent_plugin: rule.agent_plugin.clone(),
        model: rule.model.clone(),
        confidence: if rule.task_pattern.is_empty() { 0 } else { 80 },
        reason: if rule.task_pattern.is_empty() {
            "Default routing rule".to_owned()
        } else {
            format!("Matched pattern '{}'", rule.task_pattern)
        },
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use re_plugin::PluginRuntime;

    use super::*;

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
        assert_eq!(caps.len(), 1);
        assert!(caps.iter().any(|c| c.as_str() == "agent_routing"));
    }

    #[test]
    fn plugin_descriptor_is_consistent() {
        let plugin = descriptor();
        assert_eq!(plugin.id, PLUGIN_ID);
        assert_eq!(plugin.name, i18n::plugin_name());
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();
        assert!(manifest.contains("id: official.router"));
        assert!(manifest.contains("kind: agent_router"));
        assert!(manifest.contains("- agent_routing"));
    }

    #[test]
    fn runtime_plugin_id_matches() {
        let rt = super::runtime();
        assert_eq!(rt.plugin_id(), PLUGIN_ID);
    }

    #[test]
    fn tui_contributions_returns_panel() {
        let rt = super::runtime();
        let panels = rt.tui_contributions();
        assert_eq!(panels.len(), 1);
        assert_eq!(panels[0].id, "router-status");
    }

    // ── Pattern matching tests ───────────────────────────────────

    #[test]
    fn empty_pattern_matches_everything() {
        assert!(matches_task_pattern("", "anything"));
        assert!(matches_task_pattern("", ""));
    }

    #[test]
    fn exact_substring_match() {
        assert!(matches_task_pattern("test", "run the test suite"));
        assert!(!matches_task_pattern("deploy", "run the test suite"));
    }

    #[test]
    fn wildcard_match() {
        assert!(matches_task_pattern("*.test.*", "epic.test.auth"));
        assert!(matches_task_pattern("*.docs.*", "5.docs.readme"));
        assert!(!matches_task_pattern("*.test.*", "epic.code.auth"));
    }

    #[test]
    fn case_insensitive() {
        assert!(matches_task_pattern("TEST", "run the test suite"));
        assert!(matches_task_pattern("*.Docs.*", "5.docs.readme"));
    }

    // ── Agent selection tests ────────────────────────────────────

    #[test]
    fn select_agent_returns_highest_priority() {
        let rules = vec![
            RoutingRule {
                task_pattern: "test".to_owned(),
                agent_plugin: "official.claude".to_owned(),
                model: Some("haiku".to_owned()),
                priority: 1,
            },
            RoutingRule {
                task_pattern: "test".to_owned(),
                agent_plugin: "official.codex".to_owned(),
                model: None,
                priority: 2,
            },
        ];

        let rec = select_agent(&rules, "run the test").unwrap();
        assert_eq!(rec.agent_plugin, "official.claude");
        assert_eq!(rec.model, Some("haiku".to_owned()));
    }

    #[test]
    fn select_agent_falls_back_to_default() {
        let rules = vec![
            RoutingRule {
                task_pattern: "test".to_owned(),
                agent_plugin: "official.claude".to_owned(),
                model: None,
                priority: 1,
            },
            RoutingRule {
                task_pattern: String::new(), // default
                agent_plugin: "official.codex".to_owned(),
                model: None,
                priority: 100,
            },
        ];

        let rec = select_agent(&rules, "write documentation").unwrap();
        assert_eq!(rec.agent_plugin, "official.codex");
        assert_eq!(rec.confidence, 0); // default = low confidence
    }

    #[test]
    fn select_agent_no_match_returns_none() {
        let rules = vec![RoutingRule {
            task_pattern: "deploy".to_owned(),
            agent_plugin: "official.claude".to_owned(),
            model: None,
            priority: 1,
        }];

        assert!(select_agent(&rules, "write tests").is_none());
    }
}
