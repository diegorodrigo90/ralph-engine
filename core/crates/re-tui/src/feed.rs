//! Block-based activity feed for the orchestration TUI.
//!
//! Each agent action (read file, edit file, run command, think) is a
//! [`FeedBlock`] — an atomic, collapsible unit in the scrollable feed.
//!
//! Blocks are created when an [`AgentEvent`] arrives and finalized when
//! the corresponding result comes back. Text between tool calls
//! accumulates into a [`BlockKind::AgentText`] block.

use std::time::Instant;

/// The kind of action a block represents.
///
/// Each kind has a display icon and default collapse behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    /// Agent read a file. Icon: `→`
    FileRead,
    /// Agent edited a file with a diff. Icon: `←`
    FileEdit,
    /// Agent ran a shell command. Icon: `$`
    Command,
    /// Agent is thinking/reasoning. Icon: `⠋` (animated in render)
    Thinking,
    /// Agent produced text output (no tool call).
    AgentText,
    /// Quality gate passed. Icon: `✓`
    GatePass,
    /// Quality gate failed. Icon: `✗`
    GateFail,
    /// System/status message. Icon: `──`
    System,
}

impl BlockKind {
    /// The icon prefix for this block kind.
    #[must_use]
    pub fn icon(self) -> &'static str {
        match self {
            Self::FileRead => "→",
            Self::FileEdit => "←",
            Self::Command => "$",
            Self::Thinking => "⠋",
            Self::AgentText => "",
            Self::GatePass => "✓",
            Self::GateFail => "✗",
            Self::System => "──",
        }
    }

    /// Whether this block kind is collapsed by default.
    #[must_use]
    pub fn default_collapsed(self) -> bool {
        match self {
            Self::FileRead | Self::Command | Self::GatePass => true,
            Self::FileEdit | Self::Thinking | Self::AgentText | Self::GateFail | Self::System => {
                false
            }
        }
    }
}

/// A single block in the activity feed.
///
/// Created on [`AgentEvent::ToolUse`], finalized on
/// [`AgentEvent::ToolResult`]. Text deltas between tool calls
/// accumulate into an [`BlockKind::AgentText`] block.
#[derive(Debug, Clone)]
pub struct FeedBlock {
    /// What kind of action this block represents.
    pub kind: BlockKind,
    /// Title line (e.g. filepath, command, gate name).
    pub title: String,
    /// Content lines (diff lines, command output, thinking text).
    pub content: Vec<String>,
    /// Whether the block is collapsed (only title visible).
    pub collapsed: bool,
    /// Whether the block is still receiving content (streaming).
    pub active: bool,
    /// Whether the tool call succeeded (set on finalization).
    pub success: Option<bool>,
    /// When the block was created (for elapsed time).
    started_at: Option<Instant>,
    /// Elapsed time in milliseconds (set on finalization).
    elapsed_ms: Option<u64>,
}

impl FeedBlock {
    /// Creates a new active block.
    #[must_use]
    pub fn new(kind: BlockKind, title: String) -> Self {
        Self {
            collapsed: kind.default_collapsed(),
            kind,
            title,
            content: Vec::new(),
            active: true,
            success: None,
            started_at: Some(Instant::now()),
            elapsed_ms: None,
        }
    }

    /// Creates a finalized (non-active) block with no timing.
    #[must_use]
    pub fn completed(kind: BlockKind, title: String) -> Self {
        Self {
            collapsed: kind.default_collapsed(),
            kind,
            title,
            content: Vec::new(),
            active: false,
            success: Some(true),
            started_at: None,
            elapsed_ms: None,
        }
    }

    /// Appends a content line to the block.
    pub fn push_content(&mut self, line: String) {
        self.content.push(line);
    }

    /// Finalizes the block: marks as inactive, records elapsed time.
    pub fn finalize(&mut self, success: bool) {
        self.active = false;
        self.success = Some(success);
        if let Some(started) = self.started_at {
            self.elapsed_ms = Some(started.elapsed().as_millis() as u64);
        }
    }

    /// Returns the elapsed time string (e.g. `"1.2s"`, `"340ms"`).
    #[must_use]
    pub fn elapsed_label(&self) -> Option<String> {
        self.elapsed_ms.map(|ms| {
            if ms >= 1000 {
                format!("{:.1}s", ms as f64 / 1000.0)
            } else {
                format!("{ms}ms")
            }
        })
    }

    /// Number of content lines.
    #[must_use]
    pub fn content_len(&self) -> usize {
        self.content.len()
    }

    /// Toggle collapsed/expanded state.
    pub fn toggle_collapse(&mut self) {
        self.collapsed = !self.collapsed;
    }

    /// Returns how many visible lines this block occupies in the feed.
    ///
    /// Title is always 1 line. Content lines are shown only if expanded.
    #[must_use]
    pub fn visible_lines(&self) -> usize {
        if self.collapsed || self.content.is_empty() {
            1 // title only
        } else {
            1 + self.content.len() // title + content
        }
    }
}

/// A tool-to-block-kind mapping entry, provided by plugins.
///
/// Plugins classify their tools at registration time so core
/// never needs to hardcode tool names (Model B compliance).
#[derive(Debug, Clone)]
pub struct ToolKindMapping {
    /// Tool name as reported by the agent stream (e.g. "file-reader", "shell-exec").
    pub tool_name: String,
    /// Block kind to render this tool as.
    pub kind: BlockKind,
}

/// Manages the block-based activity feed.
///
/// The feed accumulates blocks from agent events. It tracks which
/// block is currently "active" (receiving streaming content) and
/// handles the transition from tool-use → tool-result.
#[derive(Debug)]
pub struct Feed {
    /// All blocks in chronological order.
    blocks: Vec<FeedBlock>,
    /// Plugin-provided tool-to-kind mappings (Model B).
    tool_mappings: Vec<ToolKindMapping>,
    /// Maximum number of blocks to keep (ring buffer behavior).
    max_blocks: usize,
    /// Scroll offset (0 = bottom, positive = scrolled up).
    scroll_offset: usize,
}

impl Feed {
    /// Creates a new empty feed.
    #[must_use]
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            tool_mappings: Vec::new(),
            max_blocks: 5_000,
            scroll_offset: 0,
        }
    }

    /// Registers tool-to-kind mappings from a plugin.
    ///
    /// Call this during setup — the active agent plugin declares
    /// how its tools should be rendered (Model B: plugins classify,
    /// core renders).
    pub fn register_tool_mappings(&mut self, mappings: Vec<ToolKindMapping>) {
        self.tool_mappings.extend(mappings);
    }

    /// Resolves a tool name to a block kind using registered mappings.
    ///
    /// Falls back to `AgentText` if no mapping exists — core never
    /// assumes what a tool name means.
    #[must_use]
    pub fn resolve_tool_kind(&self, tool_name: &str) -> BlockKind {
        self.tool_mappings
            .iter()
            .find(|m| m.tool_name == tool_name)
            .map_or(BlockKind::AgentText, |m| m.kind)
    }

    /// Returns a slice of all blocks.
    #[must_use]
    pub fn blocks(&self) -> &[FeedBlock] {
        &self.blocks
    }

    /// Returns the current scroll offset.
    #[must_use]
    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    /// Scrolls up by `n` lines.
    pub fn scroll_up(&mut self, n: usize) {
        let total = self.total_visible_lines();
        self.scroll_offset = (self.scroll_offset + n).min(total.saturating_sub(1));
    }

    /// Scrolls down by `n` lines.
    pub fn scroll_down(&mut self, n: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(n);
    }

    /// Resets scroll to bottom (follow mode).
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }

    /// Total visible lines across all blocks.
    #[must_use]
    pub fn total_visible_lines(&self) -> usize {
        self.blocks.iter().map(FeedBlock::visible_lines).sum()
    }

    /// Pushes a new block onto the feed.
    pub fn push_block(&mut self, block: FeedBlock) {
        // Finalize any active text block before starting a new one
        self.finalize_active_text_block();

        self.blocks.push(block);

        // Evict old blocks if over limit
        if self.blocks.len() > self.max_blocks {
            let drain_count = self.blocks.len() - self.max_blocks;
            self.blocks.drain(..drain_count);
        }
    }

    /// Appends content to the last active block, or creates a new
    /// [`BlockKind::AgentText`] block if no block is active.
    pub fn append_to_active(&mut self, line: String) {
        if let Some(block) = self.blocks.last_mut().filter(|b| b.active) {
            block.push_content(line);
        } else {
            // No active block — create a text block
            let mut block = FeedBlock::new(BlockKind::AgentText, String::new());
            block.push_content(line);
            self.blocks.push(block);
        }
    }

    /// Finalizes the last active block with the given success status.
    pub fn finalize_active(&mut self, success: bool) {
        if let Some(block) = self.blocks.last_mut().filter(|b| b.active) {
            block.finalize(success);
        }
    }

    /// Finalizes any active `AgentText` block (called before starting
    /// a new tool-use block to avoid mixing text with tool output).
    fn finalize_active_text_block(&mut self) {
        if let Some(block) = self
            .blocks
            .last_mut()
            .filter(|b| b.active && b.kind == BlockKind::AgentText)
        {
            block.finalize(true);
        }
    }

    /// Pushes a system message as a completed block.
    pub fn push_system(&mut self, message: String) {
        let block = FeedBlock::completed(BlockKind::System, message);
        self.blocks.push(block);
    }

    /// Toggles collapse on the block at the given index.
    pub fn toggle_block(&mut self, index: usize) {
        if let Some(block) = self.blocks.get_mut(index) {
            block.toggle_collapse();
        }
    }

    /// Number of blocks in the feed.
    #[must_use]
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Whether the feed is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
}

impl Default for Feed {
    fn default() -> Self {
        Self::new()
    }
}

/// Converts an [`AgentEvent`] into feed operations.
///
/// This is the bridge between the agent-agnostic event stream and
/// the block-based feed. Call this for each incoming event.
pub fn process_agent_event(feed: &mut Feed, event: &crate::events::AgentEvent) {
    use crate::events::AgentEvent;

    match event {
        AgentEvent::ToolUse { name } => {
            let kind = feed.resolve_tool_kind(name);
            let title = name.clone();
            feed.push_block(FeedBlock::new(kind, title));
        }
        AgentEvent::ToolResult { name, success } => {
            // If the last block matches this tool, finalize it.
            // Otherwise create a completed block.
            let finalized = if let Some(block) = feed.blocks.last_mut().filter(|b| b.active) {
                block.finalize(*success);
                true
            } else {
                false
            };

            if !finalized {
                let kind = feed.resolve_tool_kind(name);
                let mut block = FeedBlock::completed(kind, name.clone());
                block.success = Some(*success);
                feed.blocks.push(block);
            }
        }
        AgentEvent::TextDelta(text) => {
            feed.append_to_active(text.clone());
        }
        AgentEvent::Complete { is_error } => {
            feed.finalize_active(!is_error);
            let msg = if *is_error {
                "Agent finished with error".to_owned()
            } else {
                "Agent completed".to_owned()
            };
            feed.push_system(msg);
        }
        AgentEvent::System(msg) => {
            feed.push_system(msg.clone());
        }
        AgentEvent::Unknown(line) => {
            if !line.is_empty() {
                feed.append_to_active(line.clone());
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::events::AgentEvent;

    // ── BlockKind ──

    #[test]
    fn block_kind_icons() {
        assert_eq!(BlockKind::FileRead.icon(), "→");
        assert_eq!(BlockKind::FileEdit.icon(), "←");
        assert_eq!(BlockKind::Command.icon(), "$");
        assert_eq!(BlockKind::Thinking.icon(), "⠋");
        assert_eq!(BlockKind::AgentText.icon(), "");
        assert_eq!(BlockKind::GatePass.icon(), "✓");
        assert_eq!(BlockKind::GateFail.icon(), "✗");
        assert_eq!(BlockKind::System.icon(), "──");
    }

    #[test]
    fn block_kind_default_collapsed() {
        assert!(BlockKind::FileRead.default_collapsed());
        assert!(BlockKind::Command.default_collapsed());
        assert!(BlockKind::GatePass.default_collapsed());
        assert!(!BlockKind::FileEdit.default_collapsed());
        assert!(!BlockKind::AgentText.default_collapsed());
        assert!(!BlockKind::GateFail.default_collapsed());
    }

    // ── FeedBlock ──

    #[test]
    fn new_block_is_active() {
        let block = FeedBlock::new(BlockKind::FileRead, "src/main.rs".into());
        assert!(block.active);
        assert!(block.success.is_none());
        assert!(block.started_at.is_some());
        assert_eq!(block.kind, BlockKind::FileRead);
        assert_eq!(block.title, "src/main.rs");
    }

    #[test]
    fn completed_block_is_not_active() {
        let block = FeedBlock::completed(BlockKind::GatePass, "test".into());
        assert!(!block.active);
        assert_eq!(block.success, Some(true));
    }

    #[test]
    fn finalize_sets_elapsed() {
        let mut block = FeedBlock::new(BlockKind::Command, "cargo test".into());
        std::thread::sleep(std::time::Duration::from_millis(5));
        block.finalize(true);
        assert!(!block.active);
        assert_eq!(block.success, Some(true));
        assert!(block.elapsed_ms.is_some());
        assert!(block.elapsed_ms.unwrap() >= 4); // at least ~5ms
    }

    #[test]
    fn elapsed_label_formatting() {
        let mut block = FeedBlock::new(BlockKind::Command, "test".into());
        block.elapsed_ms = Some(340);
        assert_eq!(block.elapsed_label(), Some("340ms".to_owned()));

        block.elapsed_ms = Some(1500);
        assert_eq!(block.elapsed_label(), Some("1.5s".to_owned()));

        block.elapsed_ms = None;
        assert_eq!(block.elapsed_label(), None);
    }

    #[test]
    fn visible_lines_collapsed_vs_expanded() {
        let mut block = FeedBlock::new(BlockKind::FileRead, "file.rs".into());
        block.push_content("line 1".into());
        block.push_content("line 2".into());

        block.collapsed = true;
        assert_eq!(block.visible_lines(), 1);

        block.collapsed = false;
        assert_eq!(block.visible_lines(), 3); // title + 2 content
    }

    #[test]
    fn visible_lines_no_content() {
        let block = FeedBlock::new(BlockKind::System, "msg".into());
        assert_eq!(block.visible_lines(), 1); // just title, regardless of collapsed
    }

    #[test]
    fn toggle_collapse() {
        let mut block = FeedBlock::new(BlockKind::FileRead, "f".into());
        let initial = block.collapsed;
        block.toggle_collapse();
        assert_ne!(block.collapsed, initial);
        block.toggle_collapse();
        assert_eq!(block.collapsed, initial);
    }

    // ── Feed ──

    #[test]
    fn feed_starts_empty() {
        let feed = Feed::new();
        assert!(feed.is_empty());
        assert_eq!(feed.len(), 0);
    }

    #[test]
    fn push_block_adds_to_feed() {
        let mut feed = Feed::new();
        feed.push_block(FeedBlock::new(BlockKind::FileRead, "a.rs".into()));
        assert_eq!(feed.len(), 1);
        assert_eq!(feed.blocks()[0].title, "a.rs");
    }

    #[test]
    fn append_to_active_adds_content() {
        let mut feed = Feed::new();
        feed.push_block(FeedBlock::new(BlockKind::Thinking, "".into()));
        feed.append_to_active("line 1".into());
        feed.append_to_active("line 2".into());
        assert_eq!(feed.blocks()[0].content_len(), 2);
    }

    #[test]
    fn append_creates_text_block_when_no_active() {
        let mut feed = Feed::new();
        feed.append_to_active("orphan text".into());
        assert_eq!(feed.len(), 1);
        assert_eq!(feed.blocks()[0].kind, BlockKind::AgentText);
        assert_eq!(feed.blocks()[0].content[0], "orphan text");
    }

    #[test]
    fn finalize_active_marks_last_block() {
        let mut feed = Feed::new();
        feed.push_block(FeedBlock::new(BlockKind::Command, "test".into()));
        feed.finalize_active(true);
        assert!(!feed.blocks()[0].active);
        assert_eq!(feed.blocks()[0].success, Some(true));
    }

    #[test]
    fn push_block_finalizes_active_text() {
        let mut feed = Feed::new();
        feed.append_to_active("thinking...".into());
        assert!(feed.blocks()[0].active);

        // Pushing a new block should finalize the text block
        feed.push_block(FeedBlock::new(BlockKind::FileRead, "src.rs".into()));
        assert!(!feed.blocks()[0].active); // text block finalized
        assert!(feed.blocks()[1].active); // new block is active
    }

    #[test]
    fn evicts_old_blocks_over_limit() {
        let mut feed = Feed::new();
        feed.max_blocks = 3;
        for i in 0..5 {
            feed.push_block(FeedBlock::completed(BlockKind::System, format!("msg {i}")));
        }
        assert_eq!(feed.len(), 3);
        assert_eq!(feed.blocks()[0].title, "msg 2");
    }

    #[test]
    fn scroll_up_and_down() {
        let mut feed = Feed::new();
        for i in 0..10 {
            feed.push_block(FeedBlock::completed(BlockKind::System, format!("msg {i}")));
        }
        assert_eq!(feed.scroll_offset(), 0);

        feed.scroll_up(3);
        assert_eq!(feed.scroll_offset(), 3);

        feed.scroll_down(1);
        assert_eq!(feed.scroll_offset(), 2);

        feed.scroll_to_bottom();
        assert_eq!(feed.scroll_offset(), 0);
    }

    #[test]
    fn scroll_up_capped_at_total() {
        let mut feed = Feed::new();
        feed.push_block(FeedBlock::completed(BlockKind::System, "one".into()));
        feed.scroll_up(100);
        // Should not exceed total visible lines - 1
        assert!(feed.scroll_offset() <= feed.total_visible_lines());
    }

    #[test]
    fn toggle_block_at_index() {
        let mut feed = Feed::new();
        feed.push_block(FeedBlock::completed(BlockKind::FileRead, "a.rs".into()));
        let initial = feed.blocks()[0].collapsed;
        feed.toggle_block(0);
        assert_ne!(feed.blocks()[0].collapsed, initial);
    }

    #[test]
    fn push_system_creates_completed_block() {
        let mut feed = Feed::new();
        feed.push_system("hello".into());
        assert_eq!(feed.len(), 1);
        assert_eq!(feed.blocks()[0].kind, BlockKind::System);
        assert!(!feed.blocks()[0].active);
    }

    // ── Tool kind mapping (Model B) ──

    #[test]
    fn resolve_tool_kind_uses_registered_mappings() {
        let mut feed = Feed::new();
        feed.register_tool_mappings(vec![
            ToolKindMapping {
                tool_name: "tool-a".into(),
                kind: BlockKind::FileRead,
            },
            ToolKindMapping {
                tool_name: "tool-b".into(),
                kind: BlockKind::Command,
            },
        ]);
        assert_eq!(feed.resolve_tool_kind("tool-a"), BlockKind::FileRead);
        assert_eq!(feed.resolve_tool_kind("tool-b"), BlockKind::Command);
    }

    #[test]
    fn resolve_tool_kind_falls_back_to_agent_text() {
        let feed = Feed::new();
        assert_eq!(feed.resolve_tool_kind("unknown-tool"), BlockKind::AgentText);
    }

    // ── process_agent_event ──

    /// Creates a feed with test tool mappings (simulates plugin registration).
    fn feed_with_test_mappings() -> Feed {
        let mut feed = Feed::new();
        feed.register_tool_mappings(vec![
            ToolKindMapping {
                tool_name: "file-reader".into(),
                kind: BlockKind::FileRead,
            },
            ToolKindMapping {
                tool_name: "file-writer".into(),
                kind: BlockKind::FileEdit,
            },
            ToolKindMapping {
                tool_name: "shell".into(),
                kind: BlockKind::Command,
            },
        ]);
        feed
    }

    #[test]
    fn tool_use_creates_block_with_registered_kind() {
        let mut feed = feed_with_test_mappings();
        process_agent_event(
            &mut feed,
            &AgentEvent::ToolUse {
                name: "file-reader".into(),
            },
        );
        assert_eq!(feed.len(), 1);
        assert_eq!(feed.blocks()[0].kind, BlockKind::FileRead);
        assert!(feed.blocks()[0].active);
    }

    #[test]
    fn tool_use_unknown_falls_back_to_agent_text() {
        let mut feed = Feed::new(); // no mappings
        process_agent_event(
            &mut feed,
            &AgentEvent::ToolUse {
                name: "custom-tool".into(),
            },
        );
        assert_eq!(feed.blocks()[0].kind, BlockKind::AgentText);
    }

    #[test]
    fn tool_result_finalizes_block() {
        let mut feed = feed_with_test_mappings();
        process_agent_event(
            &mut feed,
            &AgentEvent::ToolUse {
                name: "shell".into(),
            },
        );
        process_agent_event(
            &mut feed,
            &AgentEvent::ToolResult {
                name: "shell".into(),
                success: true,
            },
        );
        assert_eq!(feed.len(), 1);
        assert!(!feed.blocks()[0].active);
        assert_eq!(feed.blocks()[0].success, Some(true));
    }

    #[test]
    fn text_delta_appends_to_active_block() {
        let mut feed = feed_with_test_mappings();
        process_agent_event(
            &mut feed,
            &AgentEvent::ToolUse {
                name: "file-reader".into(),
            },
        );
        process_agent_event(&mut feed, &AgentEvent::TextDelta("file content".into()));
        assert_eq!(feed.blocks()[0].content_len(), 1);
    }

    #[test]
    fn text_delta_without_active_creates_text_block() {
        let mut feed = Feed::new();
        process_agent_event(&mut feed, &AgentEvent::TextDelta("thinking...".into()));
        assert_eq!(feed.len(), 1);
        assert_eq!(feed.blocks()[0].kind, BlockKind::AgentText);
    }

    #[test]
    fn complete_event_adds_system_block() {
        let mut feed = Feed::new();
        process_agent_event(&mut feed, &AgentEvent::Complete { is_error: false });
        assert_eq!(feed.len(), 1);
        assert_eq!(feed.blocks()[0].kind, BlockKind::System);
        assert!(feed.blocks()[0].title.contains("completed"));
    }

    #[test]
    fn complete_error_event() {
        let mut feed = Feed::new();
        process_agent_event(&mut feed, &AgentEvent::Complete { is_error: true });
        assert!(feed.blocks()[0].title.contains("error"));
    }

    #[test]
    fn system_event_creates_system_block() {
        let mut feed = Feed::new();
        process_agent_event(&mut feed, &AgentEvent::System("init".into()));
        assert_eq!(feed.blocks()[0].kind, BlockKind::System);
        assert_eq!(feed.blocks()[0].title, "init");
    }

    #[test]
    fn full_tool_cycle_with_registered_mappings() {
        let mut feed = feed_with_test_mappings();

        // Agent reads a file
        process_agent_event(
            &mut feed,
            &AgentEvent::ToolUse {
                name: "file-reader".into(),
            },
        );
        process_agent_event(
            &mut feed,
            &AgentEvent::ToolResult {
                name: "file-reader".into(),
                success: true,
            },
        );

        // Agent thinks
        process_agent_event(&mut feed, &AgentEvent::TextDelta("Analyzing...".into()));

        // Agent writes a file
        process_agent_event(
            &mut feed,
            &AgentEvent::ToolUse {
                name: "file-writer".into(),
            },
        );
        process_agent_event(&mut feed, &AgentEvent::TextDelta("- old line".into()));
        process_agent_event(&mut feed, &AgentEvent::TextDelta("+ new line".into()));
        process_agent_event(
            &mut feed,
            &AgentEvent::ToolResult {
                name: "file-writer".into(),
                success: true,
            },
        );

        // Agent completes
        process_agent_event(&mut feed, &AgentEvent::Complete { is_error: false });

        assert_eq!(feed.len(), 4);
        assert_eq!(feed.blocks()[0].kind, BlockKind::FileRead);
        assert_eq!(feed.blocks()[1].kind, BlockKind::AgentText);
        assert_eq!(feed.blocks()[2].kind, BlockKind::FileEdit);
        assert_eq!(feed.blocks()[2].content_len(), 2);
        assert_eq!(feed.blocks()[3].kind, BlockKind::System);
    }
}
