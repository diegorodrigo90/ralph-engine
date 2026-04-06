//! Main render orchestration — header, activity, feed, metrics.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect, Size};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Tabs};
use ratatui_themekit::builders::ThemedSpan;
use tui_scrollview::ScrollView;

use crate::theme::ThemeExt;

use super::shell::TuiShell;
use super::style::style_content_line;

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

    /// Internal render implementation for a given area.
    fn render_in(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.tick = self.tick.wrapping_add(1);
        self.drain_pending_block();

        // Paint the full-area canvas with the theme background + default text.
        // All subsequent widgets inherit this bg automatically (ratatui's
        // style system is patch-based). Plugins can override bg on their
        // own widgets by setting `.style(Style::default().bg(custom))`.
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

        let zones = crate::layout::compute_zones(area, self.input_enabled);

        self.render_header(frame, zones.header);

        if let Some(tab_area) = zones.tab_bar {
            self.render_tab_bar(frame, tab_area);
        }

        self.render_active_tab(frame, zones.activity);
        self.render_metrics(frame, zones.metrics);
        self.render_help(frame, &zones);

        if let Some(input_area) = zones.input {
            self.render_input_bar(frame, input_area);
        }

        if let Some(sidebar) = zones.sidebar
            && self.sidebar_visible
        {
            self.render_sidebar(frame, sidebar);
        }

        if let Some(control) = zones.control {
            self.render_control_panel(frame, control);
        }

        if let Some(input_area) = zones.input {
            self.render_autocomplete(frame, input_area);
        }

        if self.agent_switcher_visible {
            self.render_agent_switcher(frame, zones.activity);
        }

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

    /// Renders the header bar with version, agent, tokens, state badge, and progress.
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

    /// Renders the activity stream (main viewport).
    fn render_activity(&mut self, frame: &mut Frame<'_>, area: Rect) {
        if !self.feed.is_empty() {
            self.render_feed_blocks(frame, area);
            return;
        }

        if self.activity_lines.is_empty() {
            if !self.main_panels.is_empty() {
                self.render_main_panels(frame, area);
            } else {
                self.render_idle_dashboard(frame, area);
            }
            return;
        }

        let visible_lines = area.height as usize;
        let t = self.theme();
        let logo_color = Some(self.state.color(t));
        let logo_lines =
            crate::logo::build_logo_lines(area.width, t, logo_color, &self.labels.logo_tagline);
        let logo_count = logo_lines.len();

        let activity: Vec<Line<'_>> = self
            .activity_lines
            .iter()
            .map(|s| {
                if s.starts_with(">> Tool") {
                    Line::from(t.fg_info(s.as_str()).build())
                } else if s.starts_with(">> State:") || s.starts_with(">> Agent") {
                    Line::from(t.fg_warning(s.as_str()).build())
                } else if s.starts_with(">> Quit") {
                    Line::from(t.fg_error(s.as_str()).build())
                } else if s.starts_with(">> Keys:") {
                    Line::from(t.fg_dim(s.as_str()).build())
                } else {
                    Line::from(t.fg_text(s.as_str()).build())
                }
            })
            .collect();

        let total = logo_count + activity.len();
        let start = total.saturating_sub(visible_lines);

        let mut all_lines: Vec<Line<'_>> = Vec::with_capacity(visible_lines);

        for (i, line) in logo_lines.into_iter().enumerate() {
            if i >= start {
                all_lines.push(line);
            }
        }

        let activity_start = start.saturating_sub(logo_count);
        for line in activity.into_iter().skip(activity_start) {
            if all_lines.len() >= visible_lines {
                break;
            }
            all_lines.push(line);
        }

        frame.render_widget(Paragraph::new(all_lines), area);
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

    /// Renders main-zone panels in the activity area (when idle).
    ///
    /// Main panels are rendered with the same design system as sidebar
    /// panels but using the full activity width. Used when no agent is
    /// running and plugins contribute main-zone content (dashboards,
    /// summaries, etc.).
    fn render_main_panels(&self, frame: &mut Frame<'_>, area: Rect) {
        use super::style::{render_panel_item, style_sidebar_line};

        let t = self.theme();
        let panel_colors = [t.info(), t.accent(), t.success(), t.warning()];

        let panel_count = self.main_panels.len();
        let constraints: Vec<Constraint> = (0..panel_count)
            .map(|i| {
                if i < panel_count - 1 {
                    Constraint::Ratio(1, panel_count as u32)
                } else {
                    Constraint::Fill(1)
                }
            })
            .collect();

        let panel_areas = Layout::vertical(constraints).split(area);

        for (i, panel) in self.main_panels.iter().enumerate() {
            let color = panel_colors[i % panel_colors.len()];
            let separator = Line::from(vec![
                ThemedSpan::with_color(format!(" {} ", panel.title), color)
                    .bold()
                    .build(),
                t.fg_border(
                    "\u{2500}".repeat(
                        panel_areas[i]
                            .width
                            .saturating_sub(panel.title.len() as u16 + 3)
                            as usize,
                    ),
                )
                .build(),
            ]);

            let mut lines: Vec<Line<'_>> = vec![separator];

            if !panel.items.is_empty() {
                for item in &panel.items {
                    render_panel_item(item, color, t, &mut lines);
                }
            } else {
                for s in &panel.lines {
                    lines.push(style_sidebar_line(s, color, t));
                }
            }

            frame.render_widget(Paragraph::new(lines), panel_areas[i]);
        }
    }

    /// Renders the metrics bar.
    fn render_metrics(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme.as_ref();

        if !self.indicator_panel.is_empty() {
            let indicator_bar = self.indicator_panel.render_bar(t);
            frame.render_widget(Paragraph::new(indicator_bar), area);
        } else {
            let metrics = t
                .status_line()
                .kv(&self.labels.tools_label, format!("{}", self.tool_count))
                .kv(
                    &self.labels.lines_label,
                    format!("{}", self.activity_lines.len()),
                )
                .kv(&self.labels.progress_label, format!("{}%", self.progress))
                .separator(" │ ")
                .build();
            frame.render_widget(Paragraph::new(metrics), area);
        }
    }

    /// Renders the tab bar (Standard + Wide tiers).
    fn render_tab_bar(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.tab_bar_area = area;
        let t = self.theme();
        let ts = t.tab_styles();
        let active_index = super::types::TuiTab::ALL
            .iter()
            .position(|tab| *tab == self.active_tab)
            .unwrap_or(0);

        let tab_labels: Vec<&str> = super::types::TuiTab::ALL
            .iter()
            .map(|tab| tab.label())
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

    /// Renders the Files tab — list of files touched this session.
    fn render_files_tab(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();

        if self.touched_files.is_empty() {
            let msg = t.line().dim("  No files touched yet.").build();
            frame.render_widget(Paragraph::new(msg), area);
            return;
        }

        let lines: Vec<Line<'_>> = self
            .touched_files
            .iter()
            .map(|path| {
                let icon = if path.contains("edit") || path.contains("write") {
                    "←"
                } else {
                    "→"
                };
                t.line()
                    .dim(format!("  {icon} "))
                    .text(path.as_str())
                    .build()
            })
            .collect();

        frame.render_widget(Paragraph::new(lines), area);
    }

    /// Renders the Log tab — raw agent output lines.
    fn render_log_tab(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();

        if self.log_lines.is_empty() {
            let msg = t.line().dim("  No log output yet.").build();
            frame.render_widget(Paragraph::new(msg), area);
            return;
        }

        let visible = area.height as usize;
        let start = self.log_lines.len().saturating_sub(visible);
        let lines: Vec<Line<'_>> = self.log_lines[start..]
            .iter()
            .map(|line| Line::from(t.fg_dim(format!("  {line}")).build()))
            .collect();

        frame.render_widget(Paragraph::new(lines), area);
    }

    /// Renders the Config tab — active plugins, hooks, agent flags.
    fn render_config_tab(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();

        let mut lines = vec![
            t.line()
                .accent_bold("  Agent: ")
                .text(self.config.agent_id.as_str())
                .build(),
            t.line()
                .accent_bold("  Locale: ")
                .text(self.config.locale.as_str())
                .build(),
            t.line()
                .accent_bold("  Theme: ")
                .text(self.theme.id())
                .build(),
            Line::raw(""),
            t.line().bright("  Keybindings:").build(),
        ];

        for binding in &self.plugin_keybindings {
            lines.push(
                t.line()
                    .accent(format!("    [{:>4}] ", binding.key))
                    .dim(binding.description.as_str())
                    .build(),
            );
        }

        frame.render_widget(Paragraph::new(lines), area);
    }
}
