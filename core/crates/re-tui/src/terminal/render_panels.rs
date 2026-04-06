//! Sidebar and control panel rendering.
//!
//! The sidebar groups plugin panels into semantic domains (Agents,
//! Sprint, Tools, Findings) instead of showing one panel per plugin.
//! This keeps the sidebar scannable even with many active plugins.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::widgets::{Borders, Paragraph};

use crate::theme::ThemeExt;

use super::shell::TuiShell;
use super::sidebar_groups::group_panels;
use super::style::render_panel_item;

impl TuiShell {
    /// Renders the sidebar zone with grouped plugin panels.
    ///
    /// Panels are grouped by domain: Agents, Sprint, Tools, Findings.
    /// Each group gets a heading and condensed item rendering.
    pub(super) fn render_sidebar(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();

        let outer = t.block_plain().borders(Borders::LEFT).build();
        let inner = outer.inner(area);
        frame.render_widget(outer, area);

        if self.sidebar_panels.is_empty() {
            frame.render_widget(Paragraph::new(t.line().dim(" (no panels)").build()), inner);
            return;
        }

        let groups = group_panels(&self.sidebar_panels);
        if groups.is_empty() {
            return;
        }

        let group_colors = [t.accent(), t.info(), t.success(), t.warning()];

        let group_count = groups.len();
        let constraints: Vec<Constraint> = (0..group_count)
            .map(|i| {
                if i < group_count - 1 {
                    Constraint::Ratio(1, group_count as u32)
                } else {
                    Constraint::Fill(1)
                }
            })
            .collect();

        let group_areas = Layout::vertical(constraints).split(inner);

        for (i, group) in groups.iter().enumerate() {
            let color = group_colors[i % group_colors.len()];
            let heading = render_group_heading(group.title, group_areas[i].width, color, t);

            let mut lines: Vec<Line<'_>> = vec![heading];

            for panel in &group.panels {
                if !panel.items.is_empty() {
                    for item in &panel.items {
                        render_panel_item(item, color, t, &mut lines);
                    }
                } else {
                    for s in &panel.lines {
                        lines.push(super::style::style_sidebar_line(s, color, t));
                    }
                }
            }

            frame.render_widget(Paragraph::new(lines), group_areas[i]);
        }
    }

    /// Renders the control panel zone (wide tier only).
    pub(super) fn render_control_panel(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();
        let state_color = self.state.color(t);

        let block = t.block_plain().borders(Borders::RIGHT).build();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let state_label = self.localized_state_label();

        let lines = vec![
            Line::raw(""),
            t.line()
                .dim(format!("  {}: ", self.labels.control_state))
                .build(),
            Line::from(vec![
                t.fg_dim("  ").build(),
                t.badge(format!(" {state_label} "), state_color).build(),
            ]),
            Line::raw(""),
            t.line()
                .dim(format!("  {}: ", self.labels.control_work))
                .build(),
            Line::from(vec![
                t.fg_dim("  ").build(),
                t.fg_bright(self.config.title.as_str()).bold().build(),
            ]),
        ];
        frame.render_widget(Paragraph::new(lines), inner);
    }
}

/// Renders a group heading with a colored title and border line.
fn render_group_heading<'a>(
    title: &str,
    width: u16,
    color: ratatui::style::Color,
    theme: &dyn crate::theme::Theme,
) -> Line<'a> {
    let border_len = width.saturating_sub(title.len() as u16 + 3) as usize;
    Line::from(vec![
        ratatui_themekit::builders::ThemedSpan::with_color(format!(" {title} "), color)
            .bold()
            .build(),
        theme.fg_border("\u{2500}".repeat(border_len)).build(),
    ])
}
