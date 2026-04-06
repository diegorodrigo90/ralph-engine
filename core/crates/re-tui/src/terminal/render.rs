//! Main render orchestration — header, activity, feed, metrics.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect, Size};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
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
        self.render_activity(frame, zones.activity);
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
        right_spans.push(Span::raw(" "));

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
            self.render_idle_dashboard(frame, area);
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
                    Line::raw(s.as_str())
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
                header = header.style(Style::default().bg(t.surface()));
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
                        bordered = bordered.style(Style::default().bg(t.surface()));
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

            let mut scroll_view = ScrollView::new(Size::new(content_width, content_height));
            scroll_view.render_widget(
                Paragraph::new(all_lines),
                Rect::new(0, 0, content_width, content_height),
            );
            frame.render_stateful_widget(scroll_view, feed_area, &mut self.feed_scroll);
        } else {
            let mut scroll_view = ScrollView::new(Size::new(content_width, content_height));
            scroll_view.render_widget(
                Paragraph::new(all_lines),
                Rect::new(0, 0, content_width, content_height),
            );
            frame.render_stateful_widget(scroll_view, area, &mut self.feed_scroll);
        }
    }

    /// Renders the metrics bar.
    fn render_metrics(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme.as_ref();

        if !self.indicator_panel.is_empty() {
            let indicator_bar = self.indicator_panel.render_bar(t);
            frame.render_widget(Paragraph::new(indicator_bar), area);
        } else {
            let metrics = Line::from(vec![
                t.fg_info(format!(
                    " {}: {} ",
                    self.labels.tools_label, self.tool_count
                ))
                .build(),
                Span::raw("│ "),
                Span::raw(format!(
                    "{}: {} ",
                    self.labels.lines_label,
                    self.activity_lines.len()
                )),
                Span::raw("│ "),
                Span::raw(format!(
                    "{}: {}% ",
                    self.labels.progress_label, self.progress
                )),
            ]);
            frame.render_widget(Paragraph::new(metrics), area);
        }
    }
}
