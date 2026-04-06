//! Sidebar and control panel rendering.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_themekit::builders::ThemedSpan;

use crate::theme::ThemeExt;

use super::shell::TuiShell;
use super::style::{render_panel_item, style_sidebar_line};

impl TuiShell {
    /// Renders the sidebar zone with auto-discovered plugin panels.
    pub(super) fn render_sidebar(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();

        let outer = Block::default()
            .borders(Borders::LEFT)
            .border_style(t.style_border());
        let inner = outer.inner(area);
        frame.render_widget(outer, area);

        if self.sidebar_panels.is_empty() {
            frame.render_widget(
                Paragraph::new(Line::from(vec![t.fg_dim(" (no panels)").build()])),
                inner,
            );
            return;
        }

        let panel_colors = [t.info(), t.accent(), t.success(), t.warning()];

        let panel_count = self.sidebar_panels.len();
        let constraints: Vec<Constraint> = (0..panel_count)
            .map(|i| {
                if i < panel_count - 1 {
                    Constraint::Ratio(1, panel_count as u32)
                } else {
                    Constraint::Fill(1)
                }
            })
            .collect();

        let panel_areas = Layout::vertical(constraints).split(inner);

        for (i, panel) in self.sidebar_panels.iter().enumerate() {
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

    /// Renders the control panel zone (wide tier only).
    pub(super) fn render_control_panel(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();
        let state_color = self.state.color(t);

        let block = Block::default()
            .borders(Borders::RIGHT)
            .border_style(t.style_border());
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let state_label = self.localized_state_label();

        let lines = vec![
            Line::raw(""),
            Line::from(vec![
                t.fg_dim(format!("  {}: ", self.labels.control_state))
                    .build(),
                t.badge(format!(" {state_label} "), state_color).build(),
            ]),
            Line::raw(""),
            Line::from(vec![
                t.fg_dim(format!("  {}: ", self.labels.control_work))
                    .build(),
                t.fg_bright(self.config.title.as_str()).bold().build(),
            ]),
        ];
        frame.render_widget(Paragraph::new(lines), inner);
    }
}
