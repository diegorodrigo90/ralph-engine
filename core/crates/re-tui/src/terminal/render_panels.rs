//! Sidebar and control panel rendering.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use super::shell::TuiShell;
use super::style::{render_panel_item, style_sidebar_line};

impl TuiShell {
    /// Renders the sidebar zone with auto-discovered plugin panels.
    pub(super) fn render_sidebar(&self, frame: &mut Frame<'_>, area: Rect) {
        let theme = self.theme();

        let outer = Block::default()
            .borders(Borders::LEFT)
            .border_style(Style::default().fg(theme.border()));
        let inner = outer.inner(area);
        frame.render_widget(outer, area);

        if self.sidebar_panels.is_empty() {
            frame.render_widget(
                Paragraph::new(Line::styled(
                    " (no panels)",
                    Style::default().fg(theme.text_dim()),
                )),
                inner,
            );
            return;
        }

        let panel_colors = [
            theme.info(),
            theme.accent(),
            theme.success(),
            theme.warning(),
        ];

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
                Span::styled(
                    format!(" {} ", panel.title),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "─".repeat(
                        panel_areas[i]
                            .width
                            .saturating_sub(panel.title.len() as u16 + 3)
                            as usize,
                    ),
                    Style::default().fg(theme.border()),
                ),
            ]);

            let mut lines: Vec<Line<'_>> = vec![separator];

            if !panel.items.is_empty() {
                for item in &panel.items {
                    render_panel_item(item, color, theme, &mut lines);
                }
            } else {
                for s in &panel.lines {
                    lines.push(style_sidebar_line(s, color, theme));
                }
            }
            frame.render_widget(Paragraph::new(lines), panel_areas[i]);
        }
    }

    /// Renders the control panel zone (wide tier only).
    pub(super) fn render_control_panel(&self, frame: &mut Frame<'_>, area: Rect) {
        let theme = self.theme();
        let state_color = self.state.color(theme);

        let block = Block::default()
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(theme.border()));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let state_label = self.localized_state_label();

        let lines = vec![
            Line::raw(""),
            Line::from(vec![
                Span::styled(
                    format!("  {}: ", self.labels.control_state),
                    Style::default().fg(theme.text_dim()),
                ),
                Span::styled(
                    format!(" {state_label} "),
                    Style::default()
                        .fg(theme.surface())
                        .bg(state_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::raw(""),
            Line::from(vec![
                Span::styled(
                    format!("  {}: ", self.labels.control_work),
                    Style::default().fg(theme.text_dim()),
                ),
                Span::styled(
                    self.config.title.as_str(),
                    Style::default()
                        .fg(theme.text_bright())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ];
        frame.render_widget(Paragraph::new(lines), inner);
    }
}
