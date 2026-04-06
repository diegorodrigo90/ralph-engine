//! Main render orchestration — state-aware layout with progressive disclosure.
//!
//! The TUI has two visual modes:
//! - **Idle**: logo + agent status + command hints (no sidebar, no tabs, no metrics)
//! - **Active**: feed blocks + condensed sidebar + tabs + live metrics
//!
//! This implements the 50% rule: only show information users need in >50% of
//! workflows. Details live in the Config tab (Level 2 disclosure).

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect, Size};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Tabs};
use ratatui_themekit::builders::ThemedSpan;
use tui_scrollview::ScrollView;

use crate::theme::ThemeExt;

use super::shell::TuiShell;
use super::style::style_content_line;
use super::types::{PanelHint, PanelSeverity};

impl TuiShell {
    /// Renders the TUI frame with responsive zone-based layout.
    pub fn render_frame(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        self.render_in(frame, area);
    }

    /// Renders the TUI into a specific sub-area of the frame.
    pub fn render_frame_in_area(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.render_in(frame, area);
    }

    /// Whether the TUI has active feed content (blocks streaming or completed).
    fn has_feed_content(&self) -> bool {
        !self.feed.is_empty()
    }

    /// Whether sidebar has visible groups (agents, sprint, findings).
    fn has_sidebar_content(&self) -> bool {
        !super::sidebar_groups::group_panels(&self.sidebar_panels).is_empty()
    }

    /// Internal render — fluid layout that adapts to available content.
    fn render_in(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.tick = self.tick.wrapping_add(1);
        self.drain_pending_block();

        let canvas = ratatui::widgets::Block::default().style(self.theme().style_base());
        frame.render_widget(canvas, area);

        if crate::layout::is_terminal_too_small(area) {
            let msg = format!(
                "Terminal too small ({}x{}). Minimum: {}x{}.",
                area.width,
                area.height,
                crate::MIN_TERMINAL_WIDTH,
                crate::MIN_TERMINAL_HEIGHT,
            );
            frame.render_widget(Paragraph::new(msg).style(self.theme().style_error()), area);
            return;
        }

        // Fluid layout — adapts to content:
        // - Feed content → full layout with tabs + metrics
        // - No feed but sidebar data → idle dashboard + sidebar
        // - No feed, no sidebar → clean idle screen
        let zones = crate::layout::compute_zones(area, self.input_enabled);
        let has_feed = self.has_feed_content();
        let has_sidebar = self.has_sidebar_content();

        // Header: idle style (clean) or active style (with metrics)
        if has_feed {
            self.render_header(frame, zones.header);
        } else {
            self.render_header_idle(frame, zones.header);
        }

        // Tab bar: only when there's feed content
        if has_feed && let Some(tab_area) = zones.tab_bar {
            self.render_tab_bar(frame, tab_area);
        }

        // Main content: feed blocks or idle dashboard
        if has_feed {
            self.render_active_tab(frame, zones.activity);
        } else {
            self.render_idle_dashboard(frame, zones.activity);
        }

        // Metrics: only when there's meaningful data
        if has_feed {
            self.render_metrics(frame, zones.metrics);
        }

        // Help bar always visible
        self.render_help(frame, &zones);

        // Input bar when enabled
        if let Some(input_area) = zones.input {
            self.render_input_bar(frame, input_area);
        }

        // Sidebar: visible when there's content AND terminal is wide enough
        if let Some(sidebar) = zones.sidebar
            && self.sidebar_visible
            && has_sidebar
        {
            self.render_sidebar(frame, sidebar);
        }

        // Control panel (wide tier)
        if let Some(control) = zones.control {
            self.render_control_panel(frame, control);
        }

        // Autocomplete overlay
        if let Some(input_area) = zones.input {
            self.render_autocomplete(frame, input_area);
        }

        // Agent switcher overlay
        if self.agent_switcher_visible {
            self.render_agent_switcher(frame, zones.activity);
        }

        // Modal overlays
        self.render_toasts(frame, area);
        if self.quit_pending {
            self.render_quit_modal(frame, area);
        }
        if self.help_modal_visible {
            self.render_help_modal(frame, area);
        }
        if self.theme_selector_visible {
            self.render_theme_selector(frame, area);
        }
    }

    // ── Header ──────────────────────────────────────────────────

    /// Idle header — version + primary agent status only.
    fn render_header_idle(&self, frame: &mut Frame<'_>, area: Rect) {
        let version = env!("CARGO_PKG_VERSION");
        let t = self.theme();

        // Find primary agent status from sidebar panels
        let agent_status = self.primary_agent_status();

        let mut spans = vec![t.fg_accent(format!(" ◎ RE v{version}")).bold().build()];

        if let Some((name, status, sev)) = agent_status {
            let (icon, color) = match sev {
                PanelSeverity::Success => ("●", t.success()),
                PanelSeverity::Error => ("✗", t.error()),
                _ => ("○", t.text_dim()),
            };
            spans.push(t.fg_border("  │  ").build());
            spans.push(
                ThemedSpan::with_color(format!("{icon} "), color)
                    .bold()
                    .build(),
            );
            spans.push(t.fg_bright(format!("{name} {status}")).build());
        }

        frame.render_widget(Paragraph::new(Line::from(spans)), area);
    }

    /// Active header — version, agent, state badge, live metrics.
    fn render_header(&self, frame: &mut Frame<'_>, area: Rect) {
        let state_label = self.localized_state_label();
        let state_color = self.state.color(self.theme());
        let version = env!("CARGO_PKG_VERSION");
        let t = self.theme();

        let sep = t.fg_border(" │ ").build();

        let spans = vec![
            t.fg_accent(format!(" ◎ RE v{version}")).bold().build(),
            sep.clone(),
            t.fg_bright(self.config.agent_id.as_str()).build(),
            sep.clone(),
            t.badge(format!(" {state_label} "), state_color).build(),
        ];

        let mut right_spans: Vec<Span<'_>> = Vec::new();

        if self.token_count > 0 {
            let tok = if self.token_count >= 1000 {
                format!("⚡{}k", self.token_count / 1000)
            } else {
                format!("⚡{}", self.token_count)
            };
            right_spans.push(t.fg_dim(tok).build());
        }

        if self.tool_count > 0 {
            if !right_spans.is_empty() {
                right_spans.push(t.fg_border(" │ ").build());
            }
            right_spans.push(t.fg_dim(format!("⚙ {}", self.tool_count)).build());
        }

        if let Some(ref cost) = self.cost_label {
            if !right_spans.is_empty() {
                right_spans.push(t.fg_border(" │ ").build());
            }
            if self.extra_usage {
                right_spans.push(t.fg_warning(cost.as_str()).build());
            } else {
                right_spans.push(t.fg_dim(cost.as_str()).build());
            }
        }

        if self.extra_usage {
            right_spans.push(
                t.fg_warning(format!(" ⚠ {}", self.labels.extra_usage_label))
                    .bold()
                    .build(),
            );
        }

        if self.progress > 0 {
            if !right_spans.is_empty() {
                right_spans.push(t.fg_border(" │ ").build());
            }
            right_spans
                .push(ThemedSpan::with_color(format!("{}%", self.progress), state_color).build());
        }
        right_spans.push(t.fg_text(" ").build());

        let left = Line::from(spans);
        let right = Line::from(right_spans);

        if area.width > 50 {
            let cols = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(area);
            frame.render_widget(Paragraph::new(left), cols[0]);
            frame.render_widget(
                Paragraph::new(right).alignment(ratatui::layout::Alignment::Right),
                cols[1],
            );
        } else {
            frame.render_widget(Paragraph::new(left), area);
        }
    }

    // ── Feed ────────────────────────────────────────────────────

    /// Renders the activity stream — feed blocks or idle dashboard.
    fn render_activity(&mut self, frame: &mut Frame<'_>, area: Rect) {
        if !self.feed.is_empty() {
            self.render_feed_blocks(frame, area);
        } else {
            self.render_idle_dashboard(frame, area);
        }
    }

    /// Renders the block-based feed using `tui-scrollview`.
    fn render_feed_blocks(&mut self, frame: &mut Frame<'_>, area: Rect) {
        use crate::feed::BlockKind;

        let t = self.theme.as_ref();

        self.feed.clear_dirty();

        let focused = self.focused_block;
        let mut all_lines: Vec<Line<'_>> = Vec::new();
        let mut focused_line: Option<u16> = None;

        for (block_idx, block) in self.feed.blocks().iter().enumerate() {
            if block_idx > 0 {
                all_lines.push(Line::raw(""));
            }

            let is_focused = focused == Some(block_idx);
            if is_focused {
                focused_line = Some(all_lines.len() as u16);
            }

            let block_color = match block.kind {
                BlockKind::FileRead => t.block_file_read(),
                BlockKind::FileEdit => t.block_file_edit(),
                BlockKind::Command => t.block_command(),
                BlockKind::Thinking => t.block_thinking(),
                BlockKind::AgentText => t.text_dim(),
                BlockKind::GatePass => t.block_pass(),
                BlockKind::GateFail => t.block_fail(),
                BlockKind::System => t.block_system(),
            };

            let icon = block.kind.icon();
            let kind_label = match block.kind {
                BlockKind::FileRead => "Read",
                BlockKind::FileEdit => "Edit",
                BlockKind::Command => "Bash",
                BlockKind::Thinking => "Think",
                BlockKind::AgentText => "Text",
                BlockKind::GatePass => "Pass",
                BlockKind::GateFail => "Fail",
                BlockKind::System => "System",
            };

            let mut header_spans: Vec<Span<'_>> = Vec::new();

            if is_focused {
                header_spans.push(t.fg_accent("▸ ").bold().build());
            }

            header_spans.push(ThemedSpan::with_color("╭─ ", block_color).build());
            header_spans.push(
                ThemedSpan::with_color(format!("{icon} {kind_label}"), block_color)
                    .bold()
                    .build(),
            );

            if !block.title.is_empty() {
                header_spans.push(t.fg_border(" │ ").build());
                header_spans.push(t.fg_text(block.title.as_str()).bold().build());
            }

            if block.collapsed && !block.content.is_empty() {
                header_spans.push(
                    t.fg_dim(format!(" ({} lines)", block.content.len()))
                        .build(),
                );
            }

            if let Some(elapsed) = block.elapsed_label() {
                header_spans.push(t.fg_dim(format!("  {elapsed}")).build());
            }

            if block.active {
                const SPINNER: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
                let idx = self.tick / 2 % SPINNER.len();
                header_spans.push(t.fg_warning(format!(" {}", SPINNER[idx])).build());
            }

            match block.success {
                Some(true) => header_spans.push(t.fg_success(" ✓").build()),
                Some(false) => header_spans.push(t.fg_error(" ✗").build()),
                None => {}
            }

            let mut header = Line::from(header_spans);
            if is_focused {
                header = header.style(t.style_surface());
            }
            all_lines.push(header);

            if !block.collapsed {
                let max_lines = crate::theme::MAX_COLLAPSED_LINES;
                let total_content = block.content.len();
                let truncated = total_content > max_lines && !is_focused;

                let show_count = if truncated { max_lines } else { total_content };
                for content_line in block.content.iter().take(show_count) {
                    let content_spans = style_content_line(content_line, block.kind, t);
                    let mut spans = vec![ThemedSpan::with_color("│ ", block_color).build()];
                    spans.extend(content_spans);
                    let mut bordered = Line::from(spans);
                    if is_focused {
                        bordered = bordered.style(t.style_surface());
                    }
                    all_lines.push(bordered);
                }

                if truncated {
                    let remaining = total_content - max_lines;
                    all_lines.push(Line::from(vec![
                        ThemedSpan::with_color("│ ", block_color).build(),
                        t.fg_dim(format!("… +{remaining} lines (Enter to expand)"))
                            .italic()
                            .build(),
                    ]));
                }
            }

            all_lines.push(Line::from(
                ThemedSpan::with_color(format!("╰{}", "─".repeat(40)), block_color).build(),
            ));
        }

        all_lines.push(Line::raw(""));
        all_lines.push(Line::raw(""));

        let content_height = all_lines.len() as u16;
        let content_width = area.width.saturating_sub(1);

        if self.follow_mode {
            let target_y = content_height.saturating_sub(area.height);
            let current_y = self.feed_scroll.offset().y;
            if current_y < target_y {
                let step = 4.min(target_y - current_y);
                self.feed_scroll
                    .set_offset(Position::new(0, current_y + step));
            }
        }

        if let Some(line_y) = focused_line {
            let current_y = self.feed_scroll.offset().y;
            let visible_h = area.height;
            if line_y < current_y {
                self.feed_scroll.set_offset(Position::new(0, line_y));
            } else if line_y >= current_y + visible_h {
                self.feed_scroll
                    .set_offset(Position::new(0, line_y.saturating_sub(visible_h / 2)));
            }
        }

        let scrolled_up = self.feed_scroll.offset().y > 0;
        if scrolled_up {
            let indicator = Line::from(t.fg_warning(" ↑ more above ").bold().build());
            let [indicator_area, feed_area] =
                Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);
            frame.render_widget(Paragraph::new(indicator), indicator_area);

            let base = self.theme().style_base();
            let mut scroll_view = ScrollView::new(Size::new(content_width, content_height));
            scroll_view.render_widget(
                Paragraph::new(all_lines).style(base),
                Rect::new(0, 0, content_width, content_height),
            );
            frame.render_stateful_widget(scroll_view, feed_area, &mut self.feed_scroll);
        } else {
            let base = self.theme().style_base();
            let mut scroll_view = ScrollView::new(Size::new(content_width, content_height));
            scroll_view.render_widget(
                Paragraph::new(all_lines).style(base),
                Rect::new(0, 0, content_width, content_height),
            );
            frame.render_stateful_widget(scroll_view, area, &mut self.feed_scroll);
        }
    }

    // ── Metrics ─────────────────────────────────────────────────

    /// Renders the metrics bar — only when there's meaningful data.
    fn render_metrics(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme.as_ref();

        if !self.indicator_panel.is_empty() {
            let indicator_bar = self.indicator_panel.render_bar(t);
            frame.render_widget(Paragraph::new(indicator_bar), area);
        } else if self.tool_count > 0 || !self.feed.is_empty() {
            let metrics = t
                .status_line()
                .kv(&self.labels.tools_label, format!("{}", self.tool_count))
                .kv(
                    &self.labels.lines_label,
                    format!("{}", self.feed.total_visible_lines()),
                )
                .kv(&self.labels.progress_label, format!("{}%", self.progress))
                .separator(" │ ")
                .build();
            frame.render_widget(Paragraph::new(metrics), area);
        }
        // When idle with no data, metrics bar is empty (just themed background)
    }

    // ── Tabs ────────────────────────────────────────────────────

    /// Renders the tab bar with item counts.
    fn render_tab_bar(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.tab_bar_area = area;
        let t = self.theme();
        let ts = t.tab_styles();
        let active_index = super::types::TuiTab::ALL
            .iter()
            .position(|tab| *tab == self.active_tab)
            .unwrap_or(0);

        let tab_labels: Vec<String> = super::types::TuiTab::ALL
            .iter()
            .map(|tab| {
                let count = match tab {
                    super::types::TuiTab::Feed => self.feed.blocks().len(),
                    super::types::TuiTab::Files => self.touched_files.len(),
                    super::types::TuiTab::Log => self.log_lines.len(),
                    super::types::TuiTab::Config => 0,
                };
                if count > 0 {
                    format!("{} ({})", tab.label(), count)
                } else {
                    tab.label().to_owned()
                }
            })
            .collect();

        let tabs = Tabs::new(tab_labels)
            .style(ts.inactive)
            .highlight_style(ts.active)
            .select(active_index)
            .divider(t.fg_border(" │ ").build());

        frame.render_widget(tabs, area);
    }

    /// Routes body rendering to the active tab's content.
    fn render_active_tab(&mut self, frame: &mut Frame<'_>, area: Rect) {
        match self.active_tab {
            super::types::TuiTab::Feed => self.render_activity(frame, area),
            super::types::TuiTab::Files => self.render_files_tab(frame, area),
            super::types::TuiTab::Log => self.render_log_tab(frame, area),
            super::types::TuiTab::Config => self.render_config_tab(frame, area),
        }
    }

    // ── Tab content ─────────────────────────────────────────────

    /// Files tab — tools used with type icons.
    fn render_files_tab(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();

        if self.touched_files.is_empty() {
            let msg = t.line().dim("  No files touched yet.").build();
            frame.render_widget(Paragraph::new(msg), area);
            return;
        }

        let mut lines: Vec<Line<'_>> = Vec::new();
        let total = self.touched_files.len();
        lines.push(
            t.line()
                .dim("  ")
                .bright(format!("{total} tools used"))
                .build(),
        );
        lines.push(Line::raw(""));

        for path in &self.touched_files {
            let (icon, color) = match path.as_str() {
                "Edit" | "Write" | "NotebookEdit" => ("✎", t.block_file_edit()),
                "Read" | "NotebookRead" => ("→", t.block_file_read()),
                "Bash" => ("$", t.block_command()),
                _ => ("○", t.text_dim()),
            };
            lines.push(Line::from(vec![
                ThemedSpan::with_color(format!("  {icon} "), color)
                    .bold()
                    .build(),
                t.fg_text(path.as_str()).build(),
            ]));
        }

        frame.render_widget(Paragraph::new(lines), area);
    }

    /// Log tab — raw agent output with line numbers.
    fn render_log_tab(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();

        if self.log_lines.is_empty() {
            let msg = t.line().dim("  No log output yet.").build();
            frame.render_widget(Paragraph::new(msg), area);
            return;
        }

        let visible = area.height as usize;
        let total = self.log_lines.len();
        let start = total.saturating_sub(visible.saturating_sub(1));

        let mut lines: Vec<Line<'_>> = Vec::with_capacity(visible);
        lines.push(t.line().dim(format!("  {total} lines")).build());

        for (i, line) in self.log_lines[start..].iter().enumerate() {
            let line_num = start + i + 1;
            let styled = if line.starts_with(">> Tool") {
                Line::from(vec![
                    t.fg_dim(format!("  {line_num:>4} ")).build(),
                    t.fg_info(line.as_str()).build(),
                ])
            } else if line.starts_with(">> Agent") || line.starts_with(">> State") {
                Line::from(vec![
                    t.fg_dim(format!("  {line_num:>4} ")).build(),
                    t.fg_warning(line.as_str()).build(),
                ])
            } else {
                Line::from(vec![
                    t.fg_dim(format!("  {line_num:>4} ")).build(),
                    t.fg_dim(line.as_str()).build(),
                ])
            };
            lines.push(styled);
        }

        frame.render_widget(Paragraph::new(lines), area);
    }

    /// Config tab — full plugin detail view (Level 2 disclosure).
    ///
    /// Shows ALL data from ALL plugins — Pairs, Text, Indicators, everything.
    /// This is where Mode, Model, Transport, Router rules, TDD policy live.
    fn render_config_tab(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();

        let mut lines = vec![
            t.line()
                .colored(" Session ", t.accent())
                .border("\u{2500}".repeat(20))
                .build(),
            t.line()
                .dim("  Agent:  ")
                .bright(self.config.agent_id.as_str())
                .build(),
            t.line()
                .dim("  Locale: ")
                .bright(self.config.locale.as_str())
                .build(),
            t.line().dim("  Theme:  ").bright(self.theme.id()).build(),
        ];

        // Full plugin details — show ALL items from every plugin
        if !self.sidebar_panels.is_empty() {
            lines.push(Line::raw(""));
            let plugin_count = self.sidebar_panels.len();
            lines.push(
                t.line()
                    .colored(format!(" Plugins ({plugin_count}) "), t.info())
                    .border("\u{2500}".repeat(16))
                    .build(),
            );

            for panel in &self.sidebar_panels {
                // Plugin header with status dot
                let (status_icon, icon_color) = self.plugin_status_icon(panel);
                lines.push(Line::from(vec![
                    ThemedSpan::with_color(format!("  {status_icon} "), icon_color)
                        .bold()
                        .build(),
                    t.fg_bright(panel.plugin_id.as_str()).bold().build(),
                ]));

                // Render ALL items — no filtering here (this is Level 2)
                for item in &panel.items {
                    super::style::render_panel_item(item, icon_color, t, &mut lines);
                }
            }
        }

        // Keybindings section
        if !self.plugin_keybindings.is_empty() {
            lines.push(Line::raw(""));
            lines.push(
                t.line()
                    .colored(" Keybindings ", t.success())
                    .border("\u{2500}".repeat(16))
                    .build(),
            );
            for binding in &self.plugin_keybindings {
                lines.push(
                    t.line()
                        .accent(format!("  [{:>4}] ", binding.key))
                        .dim(format!("{} ({})", binding.description, binding.plugin_id))
                        .build(),
                );
            }
        }

        frame.render_widget(Paragraph::new(lines), area);
    }

    // ── Helpers ──────────────────────────────────────────────────

    /// Extracts the primary agent's status from sidebar panels.
    fn primary_agent_status(&self) -> Option<(String, String, PanelSeverity)> {
        for panel in &self.sidebar_panels {
            let is_agent = matches!(
                panel.plugin_id.as_str(),
                "official.claude" | "official.claudebox" | "official.codex"
            );
            if !is_agent {
                continue;
            }
            for item in &panel.items {
                if item.hint == PanelHint::Indicator {
                    let name = panel.title.to_lowercase();
                    let status = item.value.as_deref().unwrap_or("—").to_lowercase();
                    return Some((name, status, item.severity));
                }
            }
        }
        None
    }

    /// Returns a status icon and color for a plugin panel.
    pub(super) fn plugin_status_icon(
        &self,
        panel: &super::types::SidebarPanel,
    ) -> (&'static str, ratatui::style::Color) {
        let t = self.theme();
        for item in &panel.items {
            if item.hint == PanelHint::Indicator {
                return match item.severity {
                    PanelSeverity::Success => ("●", t.success()),
                    PanelSeverity::Error => ("✗", t.error()),
                    PanelSeverity::Warning => ("◆", t.warning()),
                    PanelSeverity::Neutral => ("○", t.text_dim()),
                };
            }
        }
        ("○", t.text_dim())
    }
}
