//! Official BMAD workflow plugin metadata and runtime.

use std::path::Path;

mod i18n;

use re_plugin::{
    AgentBootstrapResult, CheckExecutionResult, DOCTOR_CHECKS, McpRegistrationResult,
    PREPARE_CHECKS, PROMPT_FRAGMENTS, PluginCheckAsset, PluginCheckDescriptor, PluginCheckKind,
    PluginDescriptor, PluginKind, PluginLifecycleStage, PluginLoadBoundary, PluginLocalizedText,
    PluginPromptAsset, PluginPromptDescriptor, PluginRuntime, PluginRuntimeError,
    PluginRuntimeHook, PluginTemplateAsset, PluginTemplateDescriptor, PluginTrustLevel,
    PromptContext, TEMPLATE, WORKFLOW, WorkItemResolution, WorkItemSummary, WorkQueueItem,
    WorkQueueStatus,
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
            // Safety net for future PluginCheckKind variants (#[non_exhaustive]).
            // Unreachable with current 2 variants — compile-required.
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
        let (epic, story) = parse_dot_notation(work_item_id)?;

        let config_path = project_root.join(".ralph-engine/config.yaml");
        let (tracker_file, stories_path) = read_bmad_paths(&config_path);

        let tracker_path = project_root.join(&tracker_file);
        let stories_dir = project_root.join(&stories_path);

        let (title, status) = lookup_story_in_tracker(&tracker_path, epic, story, work_item_id);
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

        let items = content
            .lines()
            .filter_map(|line| parse_tracker_line_as_work_item(line.trim()))
            .collect();

        Ok(items)
    }

    fn work_item_queue(&self) -> Vec<WorkQueueItem> {
        let config_path = Path::new(".ralph-engine/config.yaml");
        let (tracker_file, _) = read_bmad_paths(config_path);
        let tracker_path = Path::new(&tracker_file);

        let Ok(content) = std::fs::read_to_string(tracker_path) else {
            return Vec::new();
        };

        let all_items: Vec<WorkItemSummary> = content
            .lines()
            .filter_map(|line| parse_tracker_line_as_work_item(line.trim()))
            .collect();

        build_work_queue_from_items(&all_items)
    }

    /// BMAD workflow plugin requires research tools (Archon RAG, Context7)
    /// and orchestration tools (Skill, Agent) for autonomous story execution.
    fn required_tools(&self) -> &[&str] {
        BMAD_REQUIRED_TOOLS
    }

    /// BMAD requires prompt.md for project context during prompt assembly.
    fn required_files(&self) -> &[&str] {
        &[".ralph-engine/prompt.md"]
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

    /// Contributes a TUI sidebar panel showing sprint progress.
    fn tui_contributions(&self) -> Vec<re_plugin::TuiPanel> {
        let cwd = std::env::current_dir().unwrap_or_default();
        let config_path = cwd.join(".ralph-engine/config.yaml");
        let (tracker_file, _) = read_bmad_paths(&config_path);
        let tracker_path = cwd.join(&tracker_file);

        let Ok(content) = std::fs::read_to_string(&tracker_path) else {
            return Vec::new();
        };

        let blocks = build_sprint_blocks(&content);

        vec![
            re_plugin::TuiPanel {
                id: "sprint-status".to_owned(),
                title: "Sprint".to_owned(),
                blocks: blocks.clone(),
                zone_hint: "sidebar".to_owned(),
            },
            re_plugin::TuiPanel {
                id: "sprint-dashboard".to_owned(),
                title: "Sprint Dashboard".to_owned(),
                blocks,
                zone_hint: "main".to_owned(),
            },
        ]
    }

    fn feed_contributions(&self) -> Vec<re_plugin::FeedContribution> {
        // Feed contributions are event-driven (called when state changes).
        // Return empty for now — will be populated when workflow hooks fire.
        Vec::new()
    }

    fn idle_hints(&self) -> Vec<re_plugin::IdleHint> {
        let cwd = std::env::current_dir().unwrap_or_default();
        let config_path = cwd.join(".ralph-engine/config.yaml");
        let has_config = config_path.exists();

        if !has_config {
            return Vec::new();
        }

        let mut hints = vec![
            re_plugin::IdleHint {
                command: "/run".to_owned(),
                description: "start autonomous loop".to_owned(),
            },
            re_plugin::IdleHint {
                command: "/list".to_owned(),
                description: "available work items".to_owned(),
            },
        ];

        // Add example with first actionable story if available
        let (tracker_file, _) = read_bmad_paths(&config_path);
        let tracker_path = cwd.join(&tracker_file);
        if let Ok(content) = std::fs::read_to_string(&tracker_path) {
            let first_ready = content
                .lines()
                .find(|l| l.trim().ends_with("ready-for-dev"))
                .and_then(|l| l.trim().split(':').next())
                .map(|s| s.trim().to_owned());

            if let Some(story_id) = first_ready {
                hints.insert(
                    1,
                    re_plugin::IdleHint {
                        command: format!("/run {story_id}"),
                        description: format!("execute story {story_id}"),
                    },
                );
            }
        }

        hints
    }
}

// ── BMAD work item helpers ──────────────────────────────────────

/// Parses BMAD dot notation "N.M" into (epic, story) string slices.
fn parse_dot_notation(work_item_id: &str) -> Result<(&str, &str), PluginRuntimeError> {
    let parts: Vec<&str> = work_item_id.split('.').collect();
    if parts.len() != 2 || parts[0].parse::<u32>().is_err() || parts[1].parse::<u32>().is_err() {
        return Err(PluginRuntimeError::new(
            "invalid_work_item_format",
            format!(
                "Expected BMAD format 'N.M' (e.g., '5.3' for epic 5, story 3), got '{work_item_id}'"
            ),
        ));
    }
    Ok((parts[0], parts[1]))
}

/// Looks up a story in the tracker file and returns (title, status).
fn lookup_story_in_tracker(
    tracker_path: &Path,
    epic: &str,
    story: &str,
    work_item_id: &str,
) -> (String, String) {
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

    let Ok(content) = std::fs::read_to_string(tracker_path) else {
        return (title, status);
    };

    for line in content.lines() {
        let trimmed = line.trim();
        let matches = prefixes.iter().any(|p| trimmed.starts_with(p.as_str()))
            || exact_keys.iter().any(|k| trimmed.starts_with(k.as_str()));
        if !matches {
            continue;
        }
        if let Some((key, val)) = trimmed.split_once(':') {
            let key = key.trim();
            status = val.trim().to_owned();
            if let Some((s, _)) = status.split_once('#') {
                status = s.trim().to_owned();
            }
            let slug = prefixes
                .iter()
                .find_map(|p| key.strip_prefix(p.as_str()))
                .unwrap_or(key);
            title = capitalize_slug(slug);
        }
        break;
    }

    (title, status)
}

/// Converts a slug like "some-feature" into "Some feature".
fn capitalize_slug(slug: &str) -> String {
    let title = slug.replace('-', " ");
    let Some(first_char) = title.chars().next() else {
        return title;
    };
    let rest: String = title.chars().skip(1).collect();
    format!("{}{rest}", first_char.to_uppercase())
}

/// Parses a single tracker line into a `WorkItemSummary`, if actionable.
fn parse_tracker_line_as_work_item(trimmed: &str) -> Option<WorkItemSummary> {
    if is_tracker_metadata_line(trimmed) {
        return None;
    }

    let (key, val) = trimmed.split_once(':')?;
    let key = key.trim();
    let mut status = val.trim().to_owned();
    if let Some((s, _)) = status.split_once('#') {
        status = s.trim().to_owned();
    }

    if !matches!(
        status.as_str(),
        "backlog" | "ready-for-dev" | "in-progress" | "review"
    ) {
        return None;
    }

    let parts: Vec<&str> = key.splitn(3, '-').collect();
    if parts.len() < 2 || parts[0].parse::<u32>().is_err() || parts[1].parse::<u32>().is_err() {
        return None;
    }

    let id = format!("{}.{}", parts[0], parts[1]);
    let slug = if parts.len() == 3 { parts[2] } else { "" };
    let title = slug.replace('-', " ");
    let actionable = matches!(status.as_str(), "ready-for-dev" | "todo" | "ready");

    Some(WorkItemSummary {
        id,
        title,
        status,
        actionable,
    })
}

/// Returns true if the line is a tracker metadata/comment line to skip.
fn is_tracker_metadata_line(trimmed: &str) -> bool {
    trimmed.is_empty()
        || trimmed.starts_with('#')
        || trimmed.starts_with("epic-")
        || trimmed.starts_with("generated")
        || trimmed.starts_with("last_updated")
        || trimmed.starts_with("project")
        || trimmed.starts_with("tracking_system")
        || trimmed.starts_with("story_location")
        || trimmed.starts_with("development_status")
}

/// Builds a bounded work queue from a full list of work items.
///
/// Selects up to 5 most recent done items, the current in-progress item,
/// and up to 4 upcoming items. Maps BMAD statuses to queue statuses:
/// - `"done"` or `"review"` → Done
/// - `"in-progress"` → Running
/// - `"ready-for-dev"` → Next (first one only), rest → Queued
/// - `"backlog"` → Queued
fn build_work_queue_from_items(items: &[WorkItemSummary]) -> Vec<WorkQueueItem> {
    let mut queue: Vec<WorkQueueItem> = Vec::new();

    // Collect items by status category
    let mut done_items: Vec<&WorkItemSummary> = Vec::new();
    let mut running_items: Vec<&WorkItemSummary> = Vec::new();
    let mut ready_items: Vec<&WorkItemSummary> = Vec::new();
    let mut queued_items: Vec<&WorkItemSummary> = Vec::new();

    for item in items {
        match item.status.as_str() {
            "done" | "review" => done_items.push(item),
            "in-progress" => running_items.push(item),
            "ready-for-dev" | "todo" | "ready" => ready_items.push(item),
            _ => queued_items.push(item),
        }
    }

    // Last 5 done items
    let done_start = done_items.len().saturating_sub(5);
    for item in &done_items[done_start..] {
        queue.push(WorkQueueItem {
            id: item.id.clone(),
            title: item.title.clone(),
            status: WorkQueueStatus::Done,
        });
    }

    // Current running item(s)
    for item in &running_items {
        queue.push(WorkQueueItem {
            id: item.id.clone(),
            title: item.title.clone(),
            status: WorkQueueStatus::Running,
        });
    }

    // First ready = Next, rest = Queued
    let mut first_ready = true;
    for item in &ready_items {
        let status = if first_ready {
            first_ready = false;
            WorkQueueStatus::Next
        } else {
            WorkQueueStatus::Queued
        };
        queue.push(WorkQueueItem {
            id: item.id.clone(),
            title: item.title.clone(),
            status,
        });
    }

    // Up to 4 backlog/queued items
    for item in queued_items.iter().take(4) {
        queue.push(WorkQueueItem {
            id: item.id.clone(),
            title: item.title.clone(),
            status: WorkQueueStatus::Queued,
        });
    }

    // Limit total to ~10 items
    queue.truncate(10);
    queue
}

/// Builds TUI blocks from tracker content for sprint progress display.
fn build_sprint_blocks(content: &str) -> Vec<re_plugin::TuiBlock> {
    let (done, doing, todo) = count_sprint_statuses(content);
    let total = done + doing + todo;
    let pct = if total > 0 {
        (done * 100 / total) as u8
    } else {
        0
    };

    let mut blocks = vec![
        re_plugin::TuiBlock::bar("Progress", u32::from(pct)),
        re_plugin::TuiBlock::metric("Done", done as u32, Some(total as u32)),
        re_plugin::TuiBlock::metric("Doing", doing as u32, None),
        re_plugin::TuiBlock::metric("Todo", todo as u32, None),
    ];

    let active_stories = stories_with_suffix(content, "in-progress");
    if !active_stories.is_empty() {
        blocks.push(re_plugin::TuiBlock::separator());
        for story in &active_stories {
            blocks.push(re_plugin::TuiBlock::indicator(
                "Active",
                story.clone(),
                re_plugin::Severity::Warning,
            ));
        }
    }

    let ready_stories: Vec<String> = stories_with_suffix(content, "ready-for-dev")
        .into_iter()
        .take(3)
        .collect();
    if !ready_stories.is_empty() {
        blocks.push(re_plugin::TuiBlock::separator());
        blocks.push(re_plugin::TuiBlock::list(ready_stories));
    }

    blocks
}

/// Counts (done, doing, todo) from tracker content lines.
fn count_sprint_statuses(content: &str) -> (usize, usize, usize) {
    let mut todo = 0usize;
    let mut doing = 0usize;
    let mut done = 0usize;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if trimmed.ends_with("done") {
            done += 1;
        } else if trimmed.ends_with("in-progress") {
            doing += 1;
        } else {
            todo += 1;
        }
    }
    (done, doing, todo)
}

/// Collects story keys whose lines end with the given status suffix.
fn stories_with_suffix(content: &str, suffix: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.ends_with(suffix) {
                trimmed.split(':').next().map(|s| s.trim().to_owned())
            } else {
                None
            }
        })
        .collect()
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
    let mut iter = content.lines();

    // Find the "instructions:" line.
    let after = loop {
        let line = iter.next()?;
        if let Some(rest) = line.trim().strip_prefix("instructions:") {
            break rest.trim();
        }
    };

    // Inline value (not a block scalar).
    if !after.is_empty() && after != "|" {
        return Some(after.to_owned());
    }

    // Multi-line block: collect indented lines.
    let lines = collect_indented_block(&mut iter);
    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n").trim().to_owned())
    }
}

/// Collects indented continuation lines from a YAML block scalar.
fn collect_indented_block<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Vec<&'a str> {
    let mut lines = Vec::new();
    for line in iter {
        if line.starts_with("    ") || line.starts_with('\t') {
            lines.push(line.trim_start());
        } else if line.trim().is_empty() {
            lines.push("");
        } else {
            break;
        }
    }
    lines
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
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use re_plugin::{PluginCheckKind, PluginRuntime, WorkItemSummary, WorkQueueStatus};

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
    fn plugin_declares_expected_capabilities() {
        let caps = capabilities();
        assert_eq!(caps.len(), 5);
        assert!(caps.iter().any(|c| c.as_str() == "template"));
        assert!(caps.iter().any(|c| c.as_str() == "prompt_fragments"));
        assert!(caps.iter().any(|c| c.as_str() == "prepare_checks"));
        assert!(caps.iter().any(|c| c.as_str() == "doctor_checks"));
        assert!(caps.iter().any(|c| c.as_str() == "workflow"));
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
        let stages = lifecycle();
        assert_eq!(stages.len(), 4);
        assert!(stages.iter().any(|s| s.as_str() == "discover"));
        assert!(stages.iter().any(|s| s.as_str() == "configure"));
        assert!(stages.iter().any(|s| s.as_str() == "validate"));
        assert!(stages.iter().any(|s| s.as_str() == "load"));
    }

    #[test]
    fn plugin_declares_runtime_hooks() {
        let hooks = runtime_hooks();
        assert_eq!(hooks.len(), 5);
        assert!(hooks.iter().any(|h| h.as_str() == "scaffold"));
        assert!(hooks.iter().any(|h| h.as_str() == "prompt_assembly"));
        assert!(hooks.iter().any(|h| h.as_str() == "prepare"));
        assert!(hooks.iter().any(|h| h.as_str() == "doctor"));
        assert!(hooks.iter().any(|h| h.as_str() == "work_item_resolution"));
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
        let err = rt.bootstrap_agent("official.bmad.session").unwrap_err();
        assert_eq!(err.code, "not_an_agent_plugin");
    }

    #[test]
    fn runtime_rejects_mcp_registration() {
        let rt = super::runtime();
        let err = rt.register_mcp_server("official.bmad.server").unwrap_err();
        assert_eq!(err.code, "not_an_mcp_plugin");
    }

    #[test]
    fn runtime_plugin_id_matches() {
        let rt = super::runtime();
        assert_eq!(rt.plugin_id(), PLUGIN_ID);
    }

    // ── resolve_work_item tests ──────────────────────────────────────

    #[test]
    fn resolve_work_item_parses_dot_notation() {
        let tmp = std::env::temp_dir().join("re-bmad-resolve-ok");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(
            tmp.join(".ralph-engine/config.yaml"),
            "tracker:\n  status_file: tracker.yaml\npaths:\n  stories: stories/\n",
        )
        .ok();
        std::fs::write(
            tmp.join("tracker.yaml"),
            "5-3-fix-search: ready-for-dev\n5-4-add-auth: done\n",
        )
        .ok();
        std::fs::create_dir_all(tmp.join("stories")).ok();
        std::fs::write(tmp.join("stories/5-3-fix-search.md"), "# Fix Search\nAC1").ok();

        let rt = super::runtime();
        let result = rt.resolve_work_item("5.3", &tmp);
        assert!(result.is_ok());
        let resolution = result.unwrap();
        assert_eq!(resolution.canonical_id, "5.3");
        assert!(resolution.title.contains("fix") || resolution.title.contains("Fix"));
        assert!(resolution.source_path.is_some());
        let meta_status = resolution
            .metadata
            .iter()
            .find(|(k, _)| k == "status")
            .map(|(_, v)| v.as_str());
        assert_eq!(meta_status, Some("ready-for-dev"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn resolve_work_item_rejects_invalid_format() {
        let tmp = std::env::temp_dir().join("re-bmad-resolve-bad");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).ok();

        let rt = super::runtime();
        assert!(rt.resolve_work_item("abc", &tmp).is_err());
        assert!(rt.resolve_work_item("5", &tmp).is_err());
        assert!(rt.resolve_work_item("5.3.1", &tmp).is_err());
        assert!(rt.resolve_work_item("a.b", &tmp).is_err());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn resolve_work_item_handles_missing_tracker() {
        let tmp = std::env::temp_dir().join("re-bmad-resolve-notracker");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(tmp.join(".ralph-engine/config.yaml"), "schema_version: 1\n").ok();

        let rt = super::runtime();
        let result = rt.resolve_work_item("5.3", &tmp);
        assert!(result.is_ok());
        let resolution = result.unwrap();
        assert_eq!(resolution.canonical_id, "5.3");
        // Status unknown when tracker missing
        let meta_status = resolution
            .metadata
            .iter()
            .find(|(k, _)| k == "status")
            .map(|(_, v)| v.as_str());
        assert_eq!(meta_status, Some("unknown"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // ── list_work_items tests ────────────────────────────────────────

    #[test]
    fn list_work_items_filters_actionable() {
        let tmp = std::env::temp_dir().join("re-bmad-list-ok");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(
            tmp.join(".ralph-engine/config.yaml"),
            "tracker:\n  status_file: tracker.yaml\n",
        )
        .ok();
        std::fs::write(
            tmp.join("tracker.yaml"),
            "# Sprint 5\n\
             5-1-login-page: done\n\
             5-2-search-fix: ready-for-dev\n\
             5-3-auth-module: in-progress\n\
             5-4-review-system: backlog\n\
             5-5-admin-panel: review\n",
        )
        .ok();

        let rt = super::runtime();
        let result = rt.list_work_items(&tmp);
        assert!(result.is_ok());
        let items = result.unwrap();
        // Should include: ready-for-dev, in-progress, backlog, review (4 items)
        // Should exclude: done
        assert_eq!(items.len(), 4);
        assert!(items.iter().all(|i| i.status != "done"));
        assert!(items.iter().any(|i| i.id == "5.2"));
        assert!(items.iter().any(|i| i.id == "5.3"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn list_work_items_errors_on_missing_tracker() {
        let tmp = std::env::temp_dir().join("re-bmad-list-nofile");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(tmp.join(".ralph-engine/config.yaml"), "schema_version: 1\n").ok();

        let rt = super::runtime();
        let result = rt.list_work_items(&tmp);
        assert!(result.is_err());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // ── build_prompt_context tests ───────────────────────────────────

    #[test]
    fn build_prompt_context_includes_xml_tags() {
        let tmp = std::env::temp_dir().join("re-bmad-prompt-xml");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::create_dir_all(tmp.join("stories")).ok();
        std::fs::write(
            tmp.join(".ralph-engine/config.yaml"),
            "tracker:\n  status_file: tracker.yaml\npaths:\n  stories: stories/\n\
             workflow:\n  instructions: |\n    Use BMAD agents for implementation.\n",
        )
        .ok();
        std::fs::write(
            tmp.join("stories/5-3-fix-search.md"),
            "# Fix Search\n\nAC1: search works",
        )
        .ok();
        std::fs::write(tmp.join(".ralph-engine/prompt.md"), "Project context here").ok();

        let resolution = re_plugin::WorkItemResolution {
            raw_id: "5.3".to_owned(),
            canonical_id: "5.3".to_owned(),
            title: "Fix search".to_owned(),
            source_path: Some(
                tmp.join("stories/5-3-fix-search.md")
                    .to_string_lossy()
                    .to_string(),
            ),
            metadata: vec![
                ("epic".to_owned(), "5".to_owned()),
                ("story".to_owned(), "3".to_owned()),
                ("status".to_owned(), "ready-for-dev".to_owned()),
            ],
        };

        let rt = super::runtime();
        let result = rt.build_prompt_context(&resolution, &tmp);
        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert!(ctx.prompt_text.contains("<task>"));
        assert!(ctx.prompt_text.contains("</task>"));
        assert!(ctx.prompt_text.contains("<context>"));
        assert!(ctx.prompt_text.contains("</context>"));
        assert!(ctx.prompt_text.contains("<constraints>"));
        assert!(ctx.prompt_text.contains("</constraints>"));
        assert!(ctx.prompt_text.contains("Fix Search"));
        assert!(ctx.prompt_text.contains("Project context here"));
        assert!(ctx.prompt_text.contains("BMAD agents"));
        assert_eq!(ctx.work_item_id, "5.3");

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn build_prompt_context_handles_missing_story_file() {
        let tmp = std::env::temp_dir().join("re-bmad-prompt-nostory");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(tmp.join(".ralph-engine/config.yaml"), "schema_version: 1\n").ok();

        let resolution = re_plugin::WorkItemResolution {
            raw_id: "5.3".to_owned(),
            canonical_id: "5.3".to_owned(),
            title: "Fix search".to_owned(),
            source_path: None,
            metadata: vec![
                ("epic".to_owned(), "5".to_owned()),
                ("story".to_owned(), "3".to_owned()),
            ],
        };

        let rt = super::runtime();
        let result = rt.build_prompt_context(&resolution, &tmp);
        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert!(ctx.prompt_text.contains("No story file found"));
        assert!(ctx.prompt_text.contains("<task>"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // ── Helper function tests ────────────────────────────────────────

    #[test]
    fn extract_yaml_scalar_finds_value() {
        let content = "tracker:\n  status_file: sprint.yaml\npaths:\n  stories: my-stories/\n";
        assert_eq!(
            super::extract_yaml_scalar(content, "status_file"),
            Some("sprint.yaml")
        );
        assert_eq!(
            super::extract_yaml_scalar(content, "stories"),
            Some("my-stories/")
        );
    }

    #[test]
    fn extract_yaml_scalar_strips_comments() {
        let content = "status_file: sprint.yaml # path to tracker\n";
        assert_eq!(
            super::extract_yaml_scalar(content, "status_file"),
            Some("sprint.yaml")
        );
    }

    #[test]
    fn extract_yaml_scalar_returns_none_for_missing() {
        let content = "status_file: sprint.yaml\n";
        assert!(super::extract_yaml_scalar(content, "stories").is_none());
    }

    #[test]
    fn read_bmad_paths_uses_defaults_when_no_config() {
        let nonexistent = std::path::Path::new("/tmp/re-bmad-nopath/config.yaml");
        let (tracker, stories) = super::read_bmad_paths(nonexistent);
        assert_eq!(tracker, super::DEFAULT_TRACKER_FILE);
        assert_eq!(stories, super::DEFAULT_STORIES_PATH);
    }

    #[test]
    fn extract_workflow_instructions_inline() {
        let content = "workflow:\n  instructions: Use BMAD agents.\n";
        let result = super::extract_workflow_instructions(content);
        assert_eq!(result.as_deref(), Some("Use BMAD agents."));
    }

    #[test]
    fn extract_workflow_instructions_multiline() {
        let content = "workflow:\n  instructions: |\n    Step 1: validate\n    Step 2: implement\nnext_key: val\n";
        let result = super::extract_workflow_instructions(content);
        assert!(result.is_some());
        let text = result.unwrap();
        assert!(text.contains("Step 1"));
        assert!(text.contains("Step 2"));
    }

    #[test]
    fn extract_workflow_instructions_returns_none_when_missing() {
        let content = "workflow:\n  other: value\n";
        let result = super::extract_workflow_instructions(content);
        assert!(result.is_none());
    }

    #[test]
    fn find_story_file_matches_patterns() {
        let tmp = std::env::temp_dir().join("re-bmad-findstory");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).ok();
        std::fs::write(tmp.join("5-3-fix-search.md"), "story").ok();

        let result = super::find_story_file(&tmp, "5", "3", "5.3");
        assert!(result.is_some());
        assert!(result.unwrap().contains("5-3-fix-search.md"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn find_story_file_returns_none_when_missing() {
        let tmp = std::env::temp_dir().join("re-bmad-findstory-none");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).ok();

        let result = super::find_story_file(&tmp, "99", "99", "99.99");
        assert!(result.is_none());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    // Note: BMAD run_check wildcard match `_ =>` covers future PluginCheckKind
    // variants (enum is #[non_exhaustive]). Cannot unit-test it because only
    // Prepare and Doctor variants exist today. The branch is compile-required.

    #[test]
    fn resolve_work_item_handles_exact_key_format() {
        let tmp = std::env::temp_dir().join("re-bmad-resolve-exact");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(
            tmp.join(".ralph-engine/config.yaml"),
            "tracker:\n  status_file: t.yaml\n",
        )
        .ok();
        std::fs::write(tmp.join("t.yaml"), "5-s3: in-progress # comment\n").ok();

        let rt = super::runtime();
        let result = rt.resolve_work_item("5.3", &tmp);
        assert!(result.is_ok());
        let resolution = result.unwrap();
        let meta_status = resolution
            .metadata
            .iter()
            .find(|(k, _)| k == "status")
            .map(|(_, v)| v.as_str());
        assert_eq!(meta_status, Some("in-progress"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn list_work_items_skips_metadata_lines() {
        let tmp = std::env::temp_dir().join("re-bmad-list-meta");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(
            tmp.join(".ralph-engine/config.yaml"),
            "tracker:\n  status_file: t.yaml\n",
        )
        .ok();
        std::fs::write(
            tmp.join("t.yaml"),
            "# header\ngenerated: 2026-04-01\nlast_updated: today\nproject: test\n\
             tracking_system: yaml\nstory_location: stories/\ndevelopment_status: active\n\
             epic-5: backlog\n5-1-login: ready-for-dev\n",
        )
        .ok();

        let rt = super::runtime();
        let result = rt.list_work_items(&tmp);
        assert!(result.is_ok());
        let items = result.unwrap();
        // Only "5-1-login" should appear (metadata lines + epic + done filtered out)
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "5.1");

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn list_work_items_strips_status_comments() {
        let tmp = std::env::temp_dir().join("re-bmad-list-comments");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(
            tmp.join(".ralph-engine/config.yaml"),
            "tracker:\n  status_file: t.yaml\n",
        )
        .ok();
        std::fs::write(tmp.join("t.yaml"), "5-1-fix: review # needs attention\n").ok();

        let rt = super::runtime();
        let result = rt.list_work_items(&tmp);
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].status, "review");

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn build_prompt_context_includes_rules_digest() {
        let tmp = std::env::temp_dir().join("re-bmad-prompt-digest");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(tmp.join(".ralph-engine/config.yaml"), "schema_version: 1\n").ok();
        std::fs::write(
            tmp.join(".ralph-engine/rules-digest.md"),
            "# Golden Rules\n- Rule 1\n",
        )
        .ok();

        let resolution = re_plugin::WorkItemResolution {
            raw_id: "1.1".to_owned(),
            canonical_id: "1.1".to_owned(),
            title: "Test".to_owned(),
            source_path: None,
            metadata: vec![
                ("epic".to_owned(), "1".to_owned()),
                ("story".to_owned(), "1".to_owned()),
            ],
        };

        let rt = super::runtime();
        let result = rt.build_prompt_context(&resolution, &tmp);
        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert!(ctx.prompt_text.contains("<rules>"));
        assert!(ctx.prompt_text.contains("Golden Rules"));
        assert!(ctx.prompt_text.contains("</rules>"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn find_story_file_matches_story_prefix_pattern() {
        let tmp = std::env::temp_dir().join("re-bmad-findstory-prefix");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).ok();
        std::fs::write(tmp.join("story-5.3-fix-search.md"), "story").ok();

        let result = super::find_story_file(&tmp, "5", "3", "5.3");
        assert!(result.is_some());
        assert!(result.unwrap().contains("story-5.3"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn find_story_file_matches_dotnotation_prefix() {
        let tmp = std::env::temp_dir().join("re-bmad-findstory-dot");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).ok();
        std::fs::write(tmp.join("5.3-fix-search.md"), "story").ok();

        let result = super::find_story_file(&tmp, "5", "3", "5.3");
        assert!(result.is_some());
        assert!(result.unwrap().contains("5.3-fix"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn find_story_file_falls_back_to_parent_dir() {
        let tmp = std::env::temp_dir().join("re-bmad-findstory-parent");
        let _ = std::fs::remove_dir_all(&tmp);
        let subdir = tmp.join("stories");
        std::fs::create_dir_all(&subdir).ok();
        // Story file in parent (tmp), not in subdir
        std::fs::write(tmp.join("5-3-fix-search.md"), "story").ok();

        let result = super::find_story_file(&subdir, "5", "3", "5.3");
        assert!(result.is_some());
        assert!(result.unwrap().contains("5-3-fix-search.md"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn resolve_work_item_strips_status_comment() {
        let tmp = std::env::temp_dir().join("re-bmad-resolve-comment");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(
            tmp.join(".ralph-engine/config.yaml"),
            "tracker:\n  status_file: t.yaml\n",
        )
        .ok();
        std::fs::write(
            tmp.join("t.yaml"),
            "5-3-search: done # completed yesterday\n",
        )
        .ok();

        let rt = super::runtime();
        let result = rt.resolve_work_item("5.3", &tmp).unwrap();
        let status = result
            .metadata
            .iter()
            .find(|(k, _)| k == "status")
            .map(|(_, v)| v.as_str());
        assert_eq!(status, Some("done"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn list_work_items_handles_key_without_slug() {
        let tmp = std::env::temp_dir().join("re-bmad-list-noslug");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::write(
            tmp.join(".ralph-engine/config.yaml"),
            "tracker:\n  status_file: t.yaml\n",
        )
        .ok();
        // Key "5-1" has no third part (no slug)
        std::fs::write(tmp.join("t.yaml"), "5-1: ready-for-dev\n").ok();

        let rt = super::runtime();
        let items = rt.list_work_items(&tmp).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "5.1");
        assert_eq!(items[0].title, ""); // No slug → empty title

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn build_prompt_context_with_relative_source_path() {
        let tmp = std::env::temp_dir().join("re-bmad-prompt-relpath");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join(".ralph-engine")).ok();
        std::fs::create_dir_all(tmp.join("stories")).ok();
        std::fs::write(tmp.join(".ralph-engine/config.yaml"), "schema_version: 1\n").ok();
        std::fs::write(tmp.join("stories/5-3-fix.md"), "# Fix\nAC: works").ok();

        let resolution = re_plugin::WorkItemResolution {
            raw_id: "5.3".to_owned(),
            canonical_id: "5.3".to_owned(),
            title: "Fix".to_owned(),
            source_path: Some("stories/5-3-fix.md".to_owned()), // relative
            metadata: vec![
                ("epic".to_owned(), "5".to_owned()),
                ("story".to_owned(), "3".to_owned()),
            ],
        };

        let rt = super::runtime();
        let ctx = rt.build_prompt_context(&resolution, &tmp).unwrap();
        assert!(ctx.prompt_text.contains("AC: works"));
        assert!(ctx.prompt_text.contains("<task>"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn extract_workflow_instructions_multiline_with_empty_lines() {
        let content = "workflow:\n  instructions: |\n    Step 1\n\n    Step 2\nnext: val\n";
        let result = super::extract_workflow_instructions(content);
        assert!(result.is_some());
        let text = result.unwrap();
        assert!(text.contains("Step 1"));
        assert!(text.contains("Step 2"));
    }

    #[test]
    fn template_config_has_required_keys() {
        let config_yaml = include_str!("../template/config.yaml");
        assert!(
            config_yaml.contains("schema_version:"),
            "template must have schema_version"
        );
        assert!(
            config_yaml.contains("default_locale:"),
            "template must have default_locale"
        );
        assert!(
            config_yaml.contains("plugins:"),
            "template must have plugins"
        );
        assert!(config_yaml.contains("mcp:"), "template must have mcp");
        assert!(
            config_yaml.contains("budgets:"),
            "template must have budgets"
        );
        assert!(
            config_yaml.contains("run:"),
            "bmad template must have run section"
        );
    }

    #[test]
    fn idle_hints_empty_without_config() {
        // Without .ralph-engine/config.yaml, no hints
        let runtime = super::BmadRuntime;
        let hints = runtime.idle_hints();
        // In test env, no config file exists → empty
        assert!(hints.is_empty(), "no hints without config");
    }

    // ── build_work_queue_from_items ────────────────────────────────

    #[test]
    fn build_work_queue_maps_statuses() {
        let items = vec![
            WorkItemSummary {
                id: "5.1".into(),
                title: "search".into(),
                status: "done".into(),
                actionable: false,
            },
            WorkItemSummary {
                id: "5.2".into(),
                title: "pagination".into(),
                status: "in-progress".into(),
                actionable: false,
            },
            WorkItemSummary {
                id: "5.3".into(),
                title: "cursor".into(),
                status: "ready-for-dev".into(),
                actionable: true,
            },
            WorkItemSummary {
                id: "5.4".into(),
                title: "sort".into(),
                status: "ready-for-dev".into(),
                actionable: true,
            },
            WorkItemSummary {
                id: "5.5".into(),
                title: "analytics".into(),
                status: "backlog".into(),
                actionable: false,
            },
        ];

        let queue = super::build_work_queue_from_items(&items);

        assert_eq!(queue.len(), 5);
        assert_eq!(queue[0].status, WorkQueueStatus::Done);
        assert_eq!(queue[0].id, "5.1");
        assert_eq!(queue[1].status, WorkQueueStatus::Running);
        assert_eq!(queue[1].id, "5.2");
        assert_eq!(queue[2].status, WorkQueueStatus::Next);
        assert_eq!(queue[2].id, "5.3");
        assert_eq!(queue[3].status, WorkQueueStatus::Queued);
        assert_eq!(queue[3].id, "5.4");
        assert_eq!(queue[4].status, WorkQueueStatus::Queued);
        assert_eq!(queue[4].id, "5.5");
    }

    #[test]
    fn build_work_queue_limits_done_to_5() {
        let items: Vec<WorkItemSummary> = (1..=8)
            .map(|i| WorkItemSummary {
                id: format!("1.{i}"),
                title: format!("story {i}"),
                status: "done".into(),
                actionable: false,
            })
            .collect();

        let queue = super::build_work_queue_from_items(&items);
        let done_count = queue
            .iter()
            .filter(|q| q.status == WorkQueueStatus::Done)
            .count();
        assert_eq!(done_count, 5);
        // Should keep the last 5 (1.4 through 1.8)
        assert_eq!(queue[0].id, "1.4");
    }

    #[test]
    fn build_work_queue_empty_input_returns_empty() {
        let queue = super::build_work_queue_from_items(&[]);
        assert!(queue.is_empty());
    }

    #[test]
    fn work_item_queue_empty_without_config() {
        // Without .ralph-engine/config.yaml, returns empty
        let runtime = super::BmadRuntime;
        let queue = runtime.work_item_queue();
        assert!(queue.is_empty());
    }
}
