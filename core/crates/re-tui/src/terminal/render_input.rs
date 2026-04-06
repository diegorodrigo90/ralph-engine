//! Input bar and autocomplete popup rendering.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

use crate::theme::ThemeExt;

use super::shell::TuiShell;

impl TuiShell {
    /// Renders the chat input bar — separator, `>` prompt, multi-line, native cursor.
    pub(super) fn render_input_bar(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();
        let prompt = " > ";
        let prompt_width = prompt.len() as u16;

        let rows = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).split(area);

        frame.render_widget(
            Paragraph::new(Line::from(vec![
                t.fg_border("\u{2500}".repeat(area.width as usize)).build(),
            ])),
            rows[0],
        );

        let text_area = rows[1];

        if self.text_input_buffer.is_empty() {
            let line = Line::from(vec![t.fg_accent(prompt).bold().build()]);
            frame.render_widget(Paragraph::new(line), text_area);
            frame.set_cursor_position((text_area.x + prompt_width, text_area.y));
        } else {
            let content_width = text_area.width.saturating_sub(prompt_width).max(1) as usize;
            let mut display_lines: Vec<Line<'_>> = Vec::new();
            let mut first = true;

            for text_line in self.text_input_buffer.split('\n') {
                if text_line.is_empty() {
                    let pfx = if first { prompt } else { "   " };
                    first = false;
                    display_lines.push(Line::from(t.fg_accent(pfx.to_owned()).bold().build()));
                    continue;
                }
                let mut pos = 0;
                while pos < text_line.len() {
                    let end = (pos + content_width).min(text_line.len());
                    let chunk = &text_line[pos..end];
                    let pfx = if first { prompt } else { "   " };
                    first = false;
                    display_lines.push(Line::from(vec![
                        t.fg_accent(pfx.to_owned()).bold().build(),
                        t.fg_text(chunk.to_owned()).build(),
                    ]));
                    pos = end;
                }
            }

            let visible = text_area.height as usize;
            let start = display_lines.len().saturating_sub(visible);
            let shown: Vec<Line<'_>> = display_lines.into_iter().skip(start).collect();
            let line_count = shown.len();

            frame.render_widget(Paragraph::new(shown), text_area);

            let last_text_line = self.text_input_buffer.rsplit('\n').next().unwrap_or("");
            let cursor_col = (last_text_line.chars().count() % content_width) as u16;
            let cursor_row = (line_count.saturating_sub(1)) as u16;
            frame.set_cursor_position((
                text_area.x + prompt_width + cursor_col,
                text_area.y + cursor_row.min(text_area.height.saturating_sub(1)),
            ));
        }
    }

    /// Renders the autocomplete popup above the input bar.
    pub(super) fn render_autocomplete(&self, frame: &mut Frame<'_>, input_area: Rect) {
        if !self.autocomplete.visible || self.autocomplete.filtered.is_empty() {
            return;
        }

        let max_visible = 8u16;
        let item_count = (self.autocomplete.filtered.len() as u16).min(max_visible);
        let popup_height = item_count + 2;
        let popup_width = input_area.width.min(60);

        let popup_area = Rect {
            x: input_area.x,
            y: input_area.y.saturating_sub(popup_height),
            width: popup_width,
            height: popup_height,
        };

        let t = self.theme();
        let items: Vec<ListItem<'_>> = self
            .autocomplete
            .filtered
            .iter()
            .map(|&idx| {
                let cmd = &self.autocomplete.commands[idx];
                ListItem::new(Line::from(vec![
                    t.fg_info(format!("{}{}", self.autocomplete.prefix, cmd.name))
                        .bold()
                        .build(),
                    Span::raw("  "),
                    t.fg_dim(&cmd.description).build(),
                    Span::raw("  "),
                    t.fg_dim(cmd.source.label()).build(),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(t.border()))
                    .title(" Commands ")
                    .title_style(Style::default().fg(t.accent())),
            )
            .highlight_style(
                Style::default()
                    .bg(t.surface())
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_widget(Clear, popup_area);
        frame.render_stateful_widget(
            list,
            popup_area,
            &mut ListState::default().with_selected(Some(self.autocomplete.selected)),
        );
    }
}
