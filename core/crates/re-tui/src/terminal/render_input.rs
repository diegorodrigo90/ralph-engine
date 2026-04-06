//! Input bar rendering with scroll support and visual focus indicator.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::{
    Clear, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};

use crate::theme::ThemeExt;

use super::shell::TuiShell;

/// Maximum characters stored in input buffer (safety limit).
const MAX_INPUT_CHARS: usize = 50_000;

/// When the input buffer exceeds this many lines, the render shows a
/// compact summary instead of the full text. The full content is still
/// sent to the agent on submit.
const COLLAPSE_DISPLAY_LINES: usize = 5;

impl TuiShell {
    /// Renders the chat input bar with scroll support and visual focus.
    ///
    /// When the buffer has more than `COLLAPSE_DISPLAY_LINES` lines, shows a
    /// compact `[Pasted text #N +M lines]` indicator instead of the full text.
    /// The full content is still submitted on Enter.
    pub(super) fn render_input_bar(&self, frame: &mut Frame<'_>, area: Rect) {
        use super::types::FocusTarget;

        let t = self.theme();
        let is = t.input_styles();
        let is_focused = self.focus == FocusTarget::Input;
        let prompt = if is_focused { " > " } else { " · " };
        let prompt_width = prompt.len() as u16;

        // Split: separator (1 row) + text area (rest)
        let rows = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).split(area);

        // Separator line — accent when focused, dim when unfocused
        let sep_style = if is_focused {
            is.border_focused
        } else {
            is.border
        };
        let sep_line = "─".repeat(area.width as usize);
        frame.render_widget(
            Paragraph::new(Line::from(ratatui::text::Span::styled(sep_line, sep_style))),
            rows[0],
        );

        let text_area = rows[1];

        if self.text_input_buffer.is_empty() {
            let prompt_span = ratatui::text::Span::styled(prompt, is.prompt);
            frame.render_widget(Paragraph::new(Line::from(prompt_span)), text_area);
            frame.set_cursor_position((text_area.x + prompt_width, text_area.y));
            return;
        }

        // Check if content should be collapsed (too many lines for the input area)
        let buffer_lines = self.text_input_buffer.lines().count();
        if buffer_lines > COLLAPSE_DISPLAY_LINES {
            self.render_collapsed_indicator(frame, text_area, buffer_lines, &is);
            return;
        }

        // Build display lines with wrapping
        let content_width = text_area.width.saturating_sub(prompt_width + 1).max(1) as usize;
        let display_lines = self.build_input_lines(prompt, content_width, &is);
        let total_lines = display_lines.len();
        let visible = text_area.height as usize;

        // Scroll: always show the last lines (cursor at bottom)
        let start = total_lines.saturating_sub(visible);
        let shown: Vec<Line<'_>> = display_lines.into_iter().skip(start).collect();
        let line_count = shown.len();

        // Render text content (leave 1 col for scrollbar when needed)
        let needs_scroll = total_lines > visible;
        let (text_rect, scroll_rect) = if needs_scroll {
            let cols =
                Layout::horizontal([Constraint::Fill(1), Constraint::Length(1)]).split(text_area);
            (cols[0], Some(cols[1]))
        } else {
            (text_area, None)
        };

        frame.render_widget(Paragraph::new(shown), text_rect);

        // Scrollbar when content overflows
        if let Some(sb_area) = scroll_rect {
            let ss = t.scrollbar_styles();
            let mut sb_state = ScrollbarState::new(total_lines)
                .position(start + visible)
                .viewport_content_length(visible);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .track_style(ss.track)
                    .thumb_style(ss.thumb),
                sb_area,
                &mut sb_state,
            );
        }

        // Cursor position
        let last_text_line = self.text_input_buffer.rsplit('\n').next().unwrap_or("");
        let cursor_col = (last_text_line.chars().count() % content_width) as u16;
        let cursor_row = (line_count.saturating_sub(1)) as u16;
        frame.set_cursor_position((
            text_rect.x + prompt_width + cursor_col,
            text_rect.y + cursor_row.min(text_rect.height.saturating_sub(1)),
        ));
    }

    /// Renders a collapsed indicator for large input content.
    ///
    /// Shows: `> [Pasted text +N lines]` with accent styling.
    fn render_collapsed_indicator(
        &self,
        frame: &mut Frame<'_>,
        area: Rect,
        line_count: usize,
        is: &crate::theme::InputStyles,
    ) {
        let label = &self.labels.pasted_text_label;
        let suffix = &self.labels.paste_lines_suffix;
        let char_count = self.text_input_buffer.len();

        // First line of content (truncated) for preview
        let first_line = self.text_input_buffer.lines().next().unwrap_or("");
        let max_preview = (area.width as usize).saturating_sub(10);
        let preview = if first_line.len() > max_preview {
            format!("{}…", &first_line[..max_preview])
        } else {
            first_line.to_owned()
        };

        let indicator = format!("[{label} +{line_count} {suffix}, {char_count} chars]");

        let lines = vec![
            Line::from(vec![
                ratatui::text::Span::styled(" > ", is.prompt),
                ratatui::text::Span::styled(preview, is.text),
            ]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("   {indicator}"),
                is.placeholder,
            )]),
        ];

        frame.render_widget(Paragraph::new(lines), area);
        // Cursor at end of indicator
        frame.set_cursor_position((area.x + 3 + indicator.len() as u16, area.y + 1));
    }

    /// Builds wrapped display lines from the input buffer.
    fn build_input_lines<'a>(
        &self,
        prompt: &'a str,
        content_width: usize,
        is: &crate::theme::InputStyles,
    ) -> Vec<Line<'a>> {
        let mut lines: Vec<Line<'a>> = Vec::new();
        let mut first = true;

        for text_line in self.text_input_buffer.split('\n') {
            if text_line.is_empty() {
                let pfx = if first { prompt } else { "   " };
                first = false;
                lines.push(Line::from(ratatui::text::Span::styled(
                    pfx.to_owned(),
                    is.prompt,
                )));
                continue;
            }
            let mut pos = 0;
            while pos < text_line.len() {
                let end = (pos + content_width).min(text_line.len());
                let chunk = &text_line[pos..end];
                let pfx = if first { prompt } else { "   " };
                first = false;
                lines.push(Line::from(vec![
                    ratatui::text::Span::styled(pfx.to_owned(), is.prompt),
                    ratatui::text::Span::styled(chunk.to_owned(), is.text),
                ]));
                pos = end;
            }
        }
        lines
    }

    /// Handles paste — appends text to buffer with size limit.
    ///
    /// Large text is stored in full — the render layer handles visual collapse.
    /// This works regardless of whether the paste arrives as `Event::Paste` or
    /// as rapid `Event::Key` events (terminal-dependent).
    pub fn handle_paste_with_limit(&mut self, text: &str) {
        if !self.input_enabled || text.is_empty() {
            return;
        }
        self.save_undo_snapshot();
        let remaining = MAX_INPUT_CHARS.saturating_sub(self.text_input_buffer.len());
        if remaining == 0 {
            return;
        }
        let truncated = &text[..text.len().min(remaining)];
        self.text_input_buffer.push_str(truncated);
        self.autocomplete.update_filter(&self.text_input_buffer);
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
        let ls = t.list_styles();

        let items: Vec<ListItem<'_>> = self
            .autocomplete
            .filtered
            .iter()
            .map(|&idx| {
                let cmd = &self.autocomplete.commands[idx];
                ListItem::new(
                    t.line()
                        .info(format!("{}{}", self.autocomplete.prefix, cmd.name))
                        .dim("  ")
                        .dim(&cmd.description)
                        .dim("  ")
                        .dim(cmd.source.label())
                        .build(),
                )
            })
            .collect();

        let list = List::new(items)
            .block(t.block(" Commands ").focused(true).build())
            .highlight_style(ls.highlight)
            .highlight_symbol(ls.symbol);

        frame.render_widget(Clear, popup_area);
        frame.render_stateful_widget(
            list,
            popup_area,
            &mut ListState::default().with_selected(Some(self.autocomplete.selected)),
        );
    }
}
