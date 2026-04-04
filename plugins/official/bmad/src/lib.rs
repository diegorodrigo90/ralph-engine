//! Official BMAD workflow plugin metadata and runtime.

use std::path::Path;

mod i18n;

use re_plugin::{
    AgentBootstrapResult, CheckExecutionResult, DOCTOR_CHECKS, McpRegistrationResult,
    PREPARE_CHECKS, PROMPT_FRAGMENTS, PluginCheckAsset, PluginCheckDescriptor, PluginCheckKind,
    PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary, PluginLocalizedText,
    PluginPromptAsset, PluginPromptDescriptor, PluginRuntime, PluginRuntimeError,
    PluginRuntimeHook, PluginTemplateAsset, PluginTemplateDescriptor, PluginTrustLevel,
    PromptContext, TEMPLATE, WORKFLOW, WorkItemResolution, WorkItemSummary,
};

/// Stable plugin identifier.
pub const PLUGIN_ID: &str = "official.bmad";
const PLUGIN_NAME: &str = i18n::plugin_name();
const LOCALIZED_NAMES: &[PluginLocalizedText] = i18n::localized_plugin_names();
const PLUGIN_SUMMARY: &str = i18n::plugin_summary();
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = i18n::localized_plugin_summaries();
const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const CAPABILITIES: &[re_plugin::PluginCapability] = &[
    TEMPLATE,
    PROMPT_FRAGMENTS,
    PREPARE_CHECKS,
    DOCTOR_CHECKS,
    WORKFLOW,
];
const LIFECYCLE: &[PluginLifecycleStage] = &[
    PluginLifecycleStage::Discover,
    PluginLifecycleStage::Configure,
    PluginLifecycleStage::Validate,
    PluginLifecycleStage::Load,
];
const RUNTIME_HOOKS: &[PluginRuntimeHook] = &[
    PluginRuntimeHook::Scaffold,
    PluginRuntimeHook::PromptAssembly,
    PluginRuntimeHook::Prepare,
    PluginRuntimeHook::Doctor,
    PluginRuntimeHook::WorkItemResolution,
];
const DESCRIPTOR: PluginDescriptor = PluginDescriptor::new(
    PLUGIN_ID,
    PluginKind::Template,
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
const TEMPLATE_ASSETS: &[PluginTemplateAsset] = &[
    PluginTemplateAsset::new(
        ".ralph-engine/README.md",
        include_str!("../template/README.md"),
    ),
    PluginTemplateAsset::new(
        ".ralph-engine/config.yaml",
        include_str!("../template/config.yaml"),
    ),
    PluginTemplateAsset::new(
        ".ralph-engine/hooks.yaml",
        include_str!("../template/hooks.yaml"),
    ),
    PluginTemplateAsset::new(
        ".ralph-engine/prompt.md",
        include_str!("../template/prompt.md"),
    ),
];
const TEMPLATES: &[PluginTemplateDescriptor] = &[PluginTemplateDescriptor::new(
    "official.bmad.starter",
    PLUGIN_ID,
    i18n::template_name(),
    i18n::localized_template_names(),
    i18n::template_summary(),
    i18n::localized_template_summaries(),
    TEMPLATE_ASSETS,
)];
const PROMPT_ASSETS: &[PluginPromptAsset] = &[PluginPromptAsset::new(
    "prompts/workflow.md",
    include_str!("../template/prompt.md"),
)];
const PROMPTS: &[PluginPromptDescriptor] = &[PluginPromptDescriptor::new(
    "official.bmad.workflow",
    PLUGIN_ID,
    i18n::prompt_name(),
    i18n::localized_prompt_names(),
    i18n::prompt_summary(),
    i18n::localized_prompt_summaries(),
    PROMPT_ASSETS,
)];
const PREPARE_CHECK_ASSETS: &[PluginCheckAsset] = &[PluginCheckAsset::new(
    "checks/prepare.md",
    include_str!("../checks/prepare.md"),
)];
const DOCTOR_CHECK_ASSETS: &[PluginCheckAsset] = &[PluginCheckAsset::new(
    "checks/doctor.md",
    include_str!("../checks/doctor.md"),
)];
const CHECKS: &[PluginCheckDescriptor] = &[
    PluginCheckDescriptor::new(
        "official.bmad.prepare",
        PLUGIN_ID,
        PluginCheckKind::Prepare,
        i18n::prepare_check_name(),
        i18n::localized_prepare_check_names(),
        i18n::prepare_check_summary(),
        i18n::localized_prepare_check_summaries(),
        PREPARE_CHECK_ASSETS,
    ),
    PluginCheckDescriptor::new(
        "official.bmad.doctor",
        PLUGIN_ID,
        PluginCheckKind::Doctor,
        i18n::doctor_check_name(),
        i18n::localized_doctor_check_names(),
        i18n::doctor_check_summary(),
        i18n::localized_doctor_check_summaries(),
        DOCTOR_CHECK_ASSETS,
    ),
];

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

/// Returns the immutable template contributions declared by the plugin.
#[must_use]
pub const fn templates() -> &'static [PluginTemplateDescriptor] {
    TEMPLATES
}

/// Returns the immutable prompt contributions declared by the plugin.
#[must_use]
pub const fn prompts() -> &'static [PluginPromptDescriptor] {
    PROMPTS
}

/// Returns the immutable check contributions declared by the plugin.
#[must_use]
pub const fn checks() -> &'static [PluginCheckDescriptor] {
    CHECKS
}

/// Returns a new instance of the BMAD plugin runtime.
#[must_use]
pub fn runtime() -> BmadRuntime {
    BmadRuntime
}

/// BMAD plugin runtime — executes prepare and doctor checks against
/// the project filesystem.
pub struct BmadRuntime;

/// Tools required by the BMAD workflow for autonomous agent sessions.
/// Archon MCP for RAG research, Context7 for library docs, and
/// Skill/Agent for BMAD agent orchestration.
const BMAD_REQUIRED_TOOLS: &[&str] = &[
    "Skill",
    "Agent",
    "WebSearch",
    "WebFetch",
    "mcp__archon__rag_search_knowledge_base",
    "mcp__archon__rag_search_code_examples",
    "mcp__archon__rag_get_available_sources",
    "mcp__archon__manage_task",
    "mcp__archon__find_tasks",
    "mcp__plugin_context7_context7__resolve-library-id",
    "mcp__plugin_context7_context7__query-docs",
];

/// Files required by the prepare check.
const PREPARE_REQUIRED: &[&str] = &[".ralph-engine/config.yaml"];

/// Files required by the doctor check (superset of prepare).
const DOCTOR_REQUIRED: &[&str] = &[".ralph-engine/config.yaml", ".ralph-engine/prompt.md"];

impl PluginRuntime for BmadRuntime {
    fn plugin_id(&self) -> &str {
        PLUGIN_ID
    }

    fn run_check(
        &self,
        check_id: &str,
        kind: PluginCheckKind,
        project_root: &Path,
    ) -> Result<CheckExecutionResult, PluginRuntimeError> {
        let required = match kind {
            PluginCheckKind::Prepare => PREPARE_REQUIRED,
            PluginCheckKind::Doctor => DOCTOR_REQUIRED,
            _ => {
                return Err(PluginRuntimeError::new(
                    "unsupported_check_kind",
                    format!("BMAD does not handle check kind '{}'", kind.as_str()),
                ));
            }
        };

        let mut findings = Vec::new();
        for path in required {
            if !project_root.join(path).exists() {
                findings.push(format!("missing: {path}"));
            }
        }

        Ok(CheckExecutionResult {
            check_id: check_id.to_owned(),
            passed: findings.is_empty(),
            findings,
        })
    }

    fn bootstrap_agent(&self, agent_id: &str) -> Result<AgentBootstrapResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_agent_plugin",
            format!("BMAD plugin does not provide agent '{agent_id}'"),
        ))
    }

    fn register_mcp_server(
        &self,
        server_id: &str,
    ) -> Result<McpRegistrationResult, PluginRuntimeError> {
        Err(PluginRuntimeError::new(
            "not_an_mcp_plugin",
            format!("BMAD plugin does not provide MCP server '{server_id}'"),
        ))
    }

    fn resolve_work_item(
        &self,
        work_item_id: &str,
        project_root: &Path,
    ) -> Result<WorkItemResolution, PluginRuntimeError> {
        // Parse BMAD dot notation: "5.3" → epic 5, story 3
        let parts: Vec<&str> = work_item_id.split('.').collect();
        if parts.len() != 2 || parts[0].parse::<u32>().is_err() || parts[1].parse::<u32>().is_err()
        {
            return Err(PluginRuntimeError::new(
                "invalid_work_item_format",
                format!(
                    "Expected BMAD format 'N.M' (e.g., '5.3' for epic 5, story 3), got '{work_item_id}'"
                ),
            ));
        }

        let epic = parts[0];
        let story = parts[1];

        // Read tracker config to find status file and stories path
        let config_path = project_root.join(".ralph-engine/config.yaml");
        let (tracker_file, stories_path) = read_bmad_paths(&config_path);

        let tracker_path = project_root.join(&tracker_file);
        let stories_dir = project_root.join(&stories_path);

        // Look up story in tracker — support multiple BMAD key formats:
        // "5-3-slug", "5-s3-slug", "5-p3-slug" (s=story, p=planning)
        let prefixes = [
            format!("{epic}-{story}-"),
            format!("{epic}-s{story}-"),
            format!("{epic}-p{story}-"),
        ];
        let exact_keys = [
            format!("{epic}-{story}:"),
            format!("{epic}-s{story}:"),
            format!("{epic}-p{story}:"),
        ];
        let mut title = format!("Story {work_item_id}");
        let mut status = "unknown".to_owned();

        if let Ok(content) = std::fs::read_to_string(&tracker_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if prefixes.iter().any(|p| trimmed.starts_with(p.as_str()))
                    || exact_keys.iter().any(|k| trimmed.starts_with(k.as_str()))
                {
                    // Extract slug as title: "5-3-some-feature: done" → "some-feature"
                    if let Some((key, val)) = trimmed.split_once(':') {
                        let key = key.trim();
                        status = val.trim().to_owned();
                        // Remove status comments: "done  # comment" → "done"
                        if let Some((s, _)) = status.split_once('#') {
                            status = s.trim().to_owned();
                        }
                        let slug = prefixes
                            .iter()
                            .find_map(|p| key.strip_prefix(p.as_str()))
                            .unwrap_or(key);
                        title = slug.replace('-', " ");
                        // Capitalize first letter
                        if let Some(first) = title.get(..1) {
                            title = format!("{}{}", first.to_uppercase(), &title[1..]);
                        }
                    }
                    break;
                }
            }
        }

        // Find story file in stories directory
        let source_path = find_story_file(&stories_dir, epic, story, work_item_id);

        Ok(WorkItemResolution {
            raw_id: work_item_id.to_owned(),
            canonical_id: work_item_id.to_owned(),
            title,
            source_path,
            metadata: vec![
                ("epic".to_owned(), epic.to_owned()),
                ("story".to_owned(), story.to_owned()),
                ("status".to_owned(), status),
            ],
        })
    }

    fn list_work_items(
        &self,
        project_root: &Path,
    ) -> Result<Vec<WorkItemSummary>, PluginRuntimeError> {
        let config_path = project_root.join(".ralph-engine/config.yaml");
        let (tracker_file, _) = read_bmad_paths(&config_path);
        let tracker_path = project_root.join(&tracker_file);

        let content = std::fs::read_to_string(&tracker_path).map_err(|err| {
            PluginRuntimeError::new(
                "tracker_not_found",
                format!(
                    "Cannot read tracker file '{}': {err}",
                    tracker_path.display()
                ),
            )
        })?;

        let mut items = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            // Skip comments, empty lines, epics, and metadata
            if trimmed.is_empty()
                || trimmed.starts_with('#')
                || trimmed.starts_with("epic-")
                || trimmed.starts_with("generated")
                || trimmed.starts_with("last_updated")
                || trimmed.starts_with("project")
                || trimmed.starts_with("tracking_system")
                || trimmed.starts_with("story_location")
                || trimmed.starts_with("development_status")
            {
                continue;
            }

            if let Some((key, val)) = trimmed.split_once(':') {
                let key = key.trim();
                let mut status = val.trim().to_owned();
                // Strip trailing comments
                if let Some((s, _)) = status.split_once('#') {
                    status = s.trim().to_owned();
                }

                // Only show actionable items
                if !matches!(
                    status.as_str(),
                    "backlog" | "ready-for-dev" | "in-progress" | "review"
                ) {
                    continue;
                }

                // Parse key: "5-3-some-feature" → id="5.3", title="some feature"
                let parts: Vec<&str> = key.splitn(3, '-').collect();
                if parts.len() >= 2
                    && parts[0].parse::<u32>().is_ok()
                    && parts[1].parse::<u32>().is_ok()
                {
                    let id = format!("{}.{}", parts[0], parts[1]);
                    let slug = if parts.len() == 3 { parts[2] } else { "" };
                    let title = slug.replace('-', " ");

                    items.push(WorkItemSummary { id, title, status });
                }
            }
        }

        Ok(items)
    }

    /// BMAD workflow plugin requires research tools (Archon RAG, Context7)
    /// and orchestration tools (Skill, Agent) for autonomous story execution.
    fn required_tools(&self) -> &[&str] {
        BMAD_REQUIRED_TOOLS
    }

    fn build_prompt_context(
        &self,
        resolution: &WorkItemResolution,
        project_root: &Path,
    ) -> Result<PromptContext, PluginRuntimeError> {
        let mut context_files = Vec::new();
        let mut prompt_parts = Vec::new();

        // Prompt structure optimized for LLM attention patterns:
        //
        // --append-system-prompt-file content goes at the END of the
        // system prompt (after the agent's built-in instructions).
        // Research shows "lost in the middle" effect: 30%+ accuracy
        // drop for content buried in the middle. Beginning and end
        // get highest attention.
        //
        // Order within our appended block:
        //   1. Task (story) — HIGH attention (start of our block)
        //   2. Context/rules — MEDIUM attention (middle, reference)
        //   3. Constraints — HIGH attention (end = very last thing)
        //
        // XML tags improve parsing and enable prompt caching.

        // ── 1. TASK: story + workflow (start = high attention) ────
        let config_path = project_root.join(".ralph-engine/config.yaml");
        if let Some(ref path) = resolution.source_path {
            let full_path = if std::path::Path::new(path).is_absolute() {
                std::path::PathBuf::from(path)
            } else {
                project_root.join(path)
            };
            if let Ok(story_content) = std::fs::read_to_string(&full_path) {
                prompt_parts.push(format!(
                    "<task>\n## Work Item: {} — {}\n\n{}\n</task>",
                    resolution.canonical_id, resolution.title, story_content
                ));
                context_files.push(re_plugin::ContextFile {
                    label: format!("story-{}", resolution.canonical_id),
                    content: story_content,
                });
            }
        } else {
            prompt_parts.push(format!(
                "<task>\n## Work Item: {} — {}\n\n\
                 No story file found. Implement based on the title and project context.\n</task>",
                resolution.canonical_id, resolution.title
            ));
        }

        // ── 2. CONTEXT: project rules + prompt (middle = reference) ──
        // Rules digest is a condensed single-file covering golden rules,
        // global ACs, coding principles, and quality gates. Useful for
        // agents that do NOT load CLAUDE.md natively (Codex, Aider, etc).
        // For Claude Code, this overlaps with .claude/rules/ but provides
        // a pre-prioritized summary.
        let digest_path = project_root.join(".ralph-engine/rules-digest.md");
        if let Ok(digest) = std::fs::read_to_string(&digest_path) {
            prompt_parts.push(format!("<rules>\n{digest}\n</rules>"));
            context_files.push(re_plugin::ContextFile {
                label: "rules-digest".to_owned(),
                content: digest,
            });
        }

        let prompt_path = project_root.join(".ralph-engine/prompt.md");
        if let Ok(project_ctx) = std::fs::read_to_string(&prompt_path) {
            prompt_parts.push(format!("<context>\n{project_ctx}\n</context>"));
            context_files.push(re_plugin::ContextFile {
                label: "project-context".to_owned(),
                content: project_ctx,
            });
        }

        // ── 3. CONSTRAINTS: workflow + tracking (end = highest attention) ──
        // The last section of the system prompt gets the most attention.
        // Put non-negotiable requirements here: workflow order, mandatory
        // tracking updates, quality gates. Outcome-based over prescriptive.
        let (tracker_file, _) = read_bmad_paths(&config_path);
        let epic = resolution
            .metadata
            .iter()
            .find(|(k, _)| k == "epic")
            .map_or("?", |(_, v)| v.as_str());
        let story = resolution
            .metadata
            .iter()
            .find(|(k, _)| k == "story")
            .map_or("?", |(_, v)| v.as_str());

        let mut constraints = Vec::new();

        // Workflow instructions from config (user-defined process).
        if let Ok(content) = std::fs::read_to_string(&config_path)
            && let Some(instructions) = extract_workflow_instructions(&content)
        {
            constraints.push(format!("## Workflow\n{instructions}"));
        }

        // Tracking — BMAD plugin-level, always injected.
        constraints.push(format!(
            "## Tracking (MANDATORY)\n\
             After implementation, update these files:\n\
             - `{tracker_file}`: change story {epic}.{story} status to `done`\n\
             - Story file: add Dev Agent Record with AC→test mapping\n\
             - Run ALL quality gates before commit (tests, type-check, build)\n\
             \n\
             ## Findings (MANDATORY — create or edit)\n\
             - BEFORE implementing: review <findings> section to avoid past mistakes\n\
             - AFTER code review: you MUST edit `.ralph-engine/findings.md`:\n\
               - If the file does NOT exist, CREATE it with a header and your findings\n\
               - If it exists, APPEND new findings at the end\n\
               - Each finding: root cause analysis, not just the symptom\n\
               - Even if CR found zero issues, note what went well as confirmation",
        ));

        prompt_parts.push(format!(
            "<constraints>\n{}\n</constraints>",
            constraints.join("\n\n")
        ));

        if prompt_parts.is_empty() {
            return Err(PluginRuntimeError::new(
                "empty_prompt",
                "No content available to build prompt (no story, no rules, no context)".to_owned(),
            ));
        }

        Ok(PromptContext {
            prompt_text: prompt_parts.join("\n\n"),
            context_files,
            work_item_id: resolution.canonical_id.clone(),
            discovered_tools: Vec::new(), // Populated by core run command
        })
    }
}

// ── BMAD config helpers (plugin-owned sections) ──────────────────

/// Default tracker file path.
const DEFAULT_TRACKER_FILE: &str = "sprint-status.yaml";
/// Default stories directory.
const DEFAULT_STORIES_PATH: &str = "stories/";

/// Reads BMAD-specific paths from the config YAML.
/// Falls back to defaults when the config is unreadable.
fn read_bmad_paths(config_path: &Path) -> (String, String) {
    let content = std::fs::read_to_string(config_path).unwrap_or_default();

    let tracker = extract_yaml_scalar(&content, "status_file")
        .unwrap_or(DEFAULT_TRACKER_FILE)
        .to_owned();
    let stories = extract_yaml_scalar(&content, "stories")
        .unwrap_or(DEFAULT_STORIES_PATH)
        .to_owned();

    (tracker, stories)
}

/// Extracts a simple `key: value` from YAML content.
fn extract_yaml_scalar<'a>(content: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    content.lines().find_map(|line| {
        let trimmed = line.trim();
        trimmed.strip_prefix(&prefix).map(|rest| {
            let val = rest.trim();
            // Strip trailing comments
            val.split('#').next().unwrap_or(val).trim()
        })
    })
}

/// Extracts the `workflow.instructions` multiline value from config.
fn extract_workflow_instructions(content: &str) -> Option<String> {
    let mut in_instructions = false;
    let mut lines = Vec::new();

    for line in content.lines() {
        if line.trim().starts_with("instructions:") {
            // Check for inline value: "instructions: some text"
            let after = line.trim().strip_prefix("instructions:")?.trim();
            if after == "|" || after.is_empty() {
                in_instructions = true;
                continue;
            }
            return Some(after.to_owned());
        }

        if in_instructions {
            // Multi-line block: indented lines until next key at same or lower indent
            if line.starts_with("    ") || line.starts_with('\t') {
                lines.push(line.trim_start());
            } else if line.trim().is_empty() {
                lines.push("");
            } else {
                break;
            }
        }
    }

    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n").trim().to_owned())
    }
}

/// Finds a story file matching the epic.story pattern in the stories directory.
fn find_story_file(
    stories_dir: &Path,
    epic: &str,
    story: &str,
    work_item_id: &str,
) -> Option<String> {
    // Try common patterns: "5-3-*.md", "story-5.3*.md", "5.3-*.md"
    let patterns = [
        format!("{epic}-{story}-"),
        format!("story-{work_item_id}"),
        format!("{work_item_id}-"),
    ];

    if let Ok(entries) = std::fs::read_dir(stories_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.ends_with(".md")
                && patterns.iter().any(|p| name_str.starts_with(p.as_str()))
            {
                // Return path relative to project root
                return Some(entry.path().to_string_lossy().to_string());
            }
        }
    }

    // Also try parent directory (CP keeps some stories at implementation-artifacts root)
    if let Some(parent) = stories_dir.parent()
        && let Ok(entries) = std::fs::read_dir(parent)
    {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.ends_with(".md")
                && patterns.iter().any(|p| name_str.starts_with(p.as_str()))
            {
                return Some(entry.path().to_string_lossy().to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use re_plugin::{PluginCheckKind, PluginRuntime};

    use super::{
        PLUGIN_ID, PLUGIN_SUMMARY, capabilities, checks, descriptor, i18n, lifecycle, prompts,
        runtime_hooks, templates,
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
            && plugin.display_name_for_locale("pt-br") == "BMAD"
            && plugin.summary_for_locale("pt-br")
                == "Plugin de workflow para scaffolding e prompts do BMAD."
            && plugin.summary_for_locale("es") == PLUGIN_SUMMARY;

        // Assert
        assert!(descriptor_matches);
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
    fn plugin_declares_template_contributions() {
        let template = templates()[0];

        assert_eq!(template.id, "official.bmad.starter");
        assert_eq!(template.plugin_id, PLUGIN_ID);
        assert!(template.has_assets());
        assert_eq!(template.assets[3].path, ".ralph-engine/prompt.md");
        assert_eq!(template.display_name_for_locale("pt-br"), "Starter BMAD");
        assert_eq!(
            template.summary_for_locale("pt-br"),
            "Template inicial para projetos Ralph Engine guiados por BMAD."
        );
        assert_eq!(template.display_name_for_locale("es"), "BMAD starter");
        assert!(!template.assets[0].contents.contains("Placeholder"));
    }

    #[test]
    fn plugin_declares_prompt_contributions() {
        let prompt = prompts()[0];

        assert_eq!(prompt.id, "official.bmad.workflow");
        assert_eq!(prompt.plugin_id, PLUGIN_ID);
        assert!(prompt.has_assets());
        assert_eq!(prompt.assets[0].path, "prompts/workflow.md");
        assert_eq!(
            prompt.display_name_for_locale("pt-br"),
            "Prompt de workflow BMAD"
        );
        assert_eq!(
            prompt.summary_for_locale("pt-br"),
            "Pacote de prompts para montar workflows BMAD."
        );
        assert_eq!(
            prompt.summary_for_locale("es"),
            "Prompt bundle for BMAD workflow assembly."
        );
        assert_eq!(prompt.display_name_for_locale("es"), "BMAD workflow prompt");
    }

    #[test]
    fn plugin_declares_check_contributions() {
        let checks = checks();

        assert_eq!(checks.len(), 2);
        assert_eq!(checks[0].id, "official.bmad.prepare");
        assert_eq!(checks[0].kind.as_str(), "prepare");
        assert_eq!(
            checks[0].display_name_for_locale("pt-br"),
            "Verificação de preparo BMAD"
        );
        assert_eq!(checks[1].id, "official.bmad.doctor");
        assert_eq!(checks[1].kind.as_str(), "doctor");
        assert_eq!(
            checks[1].summary_for_locale("pt-br"),
            "Executa validação tipada de diagnóstico para workflows BMAD."
        );
        assert_eq!(
            checks[0].summary_for_locale("es"),
            "Runs typed prepare-time validation for BMAD workflows."
        );
        assert_eq!(checks[1].display_name_for_locale("es"), "BMAD doctor check");
        assert!(checks[0].has_assets());
        assert_eq!(checks[0].assets[0].path, "checks/prepare.md");
        assert!(
            checks[0].assets[0]
                .contents
                .contains("# BMAD Prepare Check")
        );
        assert!(checks[1].has_assets());
        assert_eq!(checks[1].assets[0].path, "checks/doctor.md");
        assert!(checks[1].assets[0].contents.contains("# BMAD Doctor Check"));
    }

    #[test]
    fn plugin_manifest_matches_typed_contract_surface() {
        let manifest = manifest_document();

        assert!(manifest.contains("id: official.bmad"));
        assert!(manifest.contains("kind: template"));
        assert!(manifest.contains("- template"));
        assert!(manifest.contains("- prompt_fragments"));
        assert!(manifest.contains("- prepare_checks"));
        assert!(manifest.contains("- doctor_checks"));
        assert!(manifest.contains("id: official.bmad.starter"));
        assert!(manifest.contains("id: official.bmad.workflow"));
        assert!(manifest.contains("id: official.bmad.prepare"));
        assert!(manifest.contains("id: official.bmad.doctor"));
    }

    #[test]
    fn runtime_prepare_check_passes_when_config_exists() {
        let tmp = std::env::temp_dir().join("re-bmad-runtime-prepare-ok");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(tmp.join(".ralph-engine/config.yaml"), "# test").ok();

        let rt = super::runtime();
        let result = rt.run_check("official.bmad.prepare", PluginCheckKind::Prepare, &tmp);

        assert!(result.is_ok());
        let Ok(output) = result else { return };
        assert!(output.passed);
        assert!(output.findings.is_empty());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn runtime_prepare_check_fails_when_config_missing() {
        let tmp = std::env::temp_dir().join("re-bmad-runtime-prepare-fail");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).ok();

        let rt = super::runtime();
        let result = rt.run_check("official.bmad.prepare", PluginCheckKind::Prepare, &tmp);

        assert!(result.is_ok());
        let Ok(output) = result else { return };
        assert!(!output.passed);
        assert!(output.findings[0].contains("config.yaml"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn runtime_doctor_check_validates_both_files() {
        let tmp = std::env::temp_dir().join("re-bmad-runtime-doctor");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(tmp.join(".ralph-engine/config.yaml"), "# test").ok();

        let rt = super::runtime();
        let result = rt.run_check("official.bmad.doctor", PluginCheckKind::Doctor, &tmp);

        assert!(result.is_ok());
        let Ok(output) = result else { return };
        assert!(!output.passed);
        assert!(output.findings[0].contains("prompt.md"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn runtime_declares_required_tools() {
        let rt = super::runtime();
        let tools = rt.required_tools();
        assert!(!tools.is_empty());
        assert!(tools.contains(&"Skill"));
        assert!(tools.contains(&"mcp__archon__rag_search_knowledge_base"));
        assert!(tools.contains(&"mcp__plugin_context7_context7__query-docs"));
    }

    #[test]
    fn runtime_rejects_agent_bootstrap() {
        let rt = super::runtime();
        let result = rt.bootstrap_agent("official.bmad.session");
        assert!(result.is_err());
    }

    #[test]
    fn runtime_rejects_mcp_registration() {
        let rt = super::runtime();
        let result = rt.register_mcp_server("official.bmad.server");
        assert!(result.is_err());
    }
}
