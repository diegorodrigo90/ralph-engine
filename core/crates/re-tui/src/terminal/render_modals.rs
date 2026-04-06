//! Modal popups, toasts, idle dashboard, and agent switcher rendering.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

use crate::theme::ThemeExt;

use super::shell::TuiShell;
use super::types::{ToastLevel, TuiState};

impl TuiShell {
    /// Renders the help bar at the bottom.
    pub(super) fn render_help(&self, frame: &mut Frame<'_>, zones: &crate::layout::LayoutZones) {
        let t = self.theme();

        if self.quit_pending || self.help_modal_visible {
            let hint = format!(" {} ", self.labels.modal_open_hint);
            frame.render_widget(
                Paragraph::new(Line::from(vec![t.fg_dim(hint).build()])),
                zones.help,
            );
            return;
        }

        // When typing, show input-specific help
        if self.input_enabled && !self.text_input_buffer.is_empty() {
            let spans = vec![
                t.fg_dim(" [Enter]").build(),
                t.fg_dim(" send ").build(),
                t.fg_dim(" [Alt+Enter]").build(),
                t.fg_dim(" newline ").build(),
                t.fg_dim(" [Esc]").build(),
                t.fg_dim(" cancel ").build(),
            ];
            frame.render_widget(Paragraph::new(Line::from(spans)), zones.help);
            return;
        }

        // Show thinking message from agent plugin
        if self.feed.blocks().iter().any(|b| b.active) {
            const SPINNER: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let idx = self.tick / 2 % SPINNER.len();
            let spinner = SPINNER[idx];

            let message = self
                .thinking_message
                .as_deref()
                .or_else(|| {
                    self.feed
                        .blocks()
                        .iter()
                        .rev()
                        .find(|b| b.active)
                        .map(|b| b.title.as_str())
                })
                .unwrap_or("Processing...");

            let status_spans = vec![
                t.fg_warning(format!(" {spinner} ")).build(),
                t.fg_bright(format!("{message} ")).italic().build(),
            ];
            frame.render_widget(Paragraph::new(Line::from(status_spans)), zones.help);
            return;
        }

        let state_label = format!("{:?}", self.state);
        let mut spans: Vec<Span<'_>> = Vec::new();

        for binding in &self.plugin_keybindings {
            if binding.active_states.is_empty()
                || binding.active_states.iter().any(|s| s == &state_label)
            {
                spans.push(t.fg_dim(format!(" [{}]", binding.key)).build());
                spans.push(t.fg_dim(format!(" {} ", binding.description)).build());
            }
        }

        spans.push(t.fg_dim(" [p]").build());
        spans.push(t.fg_dim(format!(" {} ", self.labels.pause_label)).build());
        spans.push(t.fg_dim(" [?]").build());
        spans.push(t.fg_dim(format!(" {} ", self.labels.help_label)).build());
        spans.push(t.fg_dim(" [q]").build());
        spans.push(t.fg_dim(format!(" {} ", self.labels.quit_label)).build());

        let tier_label = match zones.tier {
            crate::layout::LayoutTier::Compact => "compact",
            crate::layout::LayoutTier::Standard => "standard",
            crate::layout::LayoutTier::Wide => "wide",
        };
        spans.push(t.fg_dim(format!(" │ {tier_label}")).build());

        frame.render_widget(Paragraph::new(Line::from(spans)), zones.help);
    }

    /// Renders the agent switcher popup (Ctrl+A).
    pub(super) fn render_agent_switcher(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();
        let popup_height = (self.available_agents.len() as u16 + 2).min(area.height);
        let popup_width = 40u16.min(area.width);
        let popup_area = Rect {
            x: area.x + (area.width.saturating_sub(popup_width)) / 2,
            y: area.y + (area.height.saturating_sub(popup_height)) / 2,
            width: popup_width,
            height: popup_height,
        };

        let items: Vec<ListItem<'_>> = self
            .available_agents
            .iter()
            .map(|agent| ListItem::new(Line::from(vec![t.fg_text(format!("  {agent}")).build()])))
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(t.accent()))
                    .title(" Switch Agent (Ctrl+A) ")
                    .title_style(Style::default().fg(t.accent()).add_modifier(Modifier::BOLD)),
            )
            .highlight_style(
                Style::default()
                    .bg(t.surface())
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▸ ");

        frame.render_widget(Clear, popup_area);
        frame.render_stateful_widget(
            list,
            popup_area,
            &mut ListState::default().with_selected(Some(self.agent_switcher_selected)),
        );
    }

    /// Renders the idle dashboard when no agent is running.
    pub(super) fn render_idle_dashboard(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();
        let version = env!("CARGO_PKG_VERSION");

        let logo_color = Some(match self.state {
            TuiState::Running => t.text_dim(),
            TuiState::Error => t.error(),
            _ => t.accent(),
        });
        let logo_lines =
            crate::logo::build_logo_lines(area.width, t, logo_color, &self.labels.logo_tagline);

        let mut lines: Vec<Line<'_>> = Vec::new();

        let content_height = logo_lines.len() + 14;
        let pad_top = area.height.saturating_sub(content_height as u16) / 2;
        for _ in 0..pad_top {
            lines.push(Line::raw(""));
        }

        lines.extend(logo_lines);
        lines.push(Line::raw(""));
        lines.push(Line::from(vec![
            t.fg_dim(format!(
                "  v{version} — {}",
                self.labels.orchestration_runtime
            ))
            .build(),
        ]));
        lines.push(Line::raw(""));

        let has_config = std::path::Path::new(".ralph-engine/config.yaml").exists();
        if has_config {
            lines.push(Line::from(vec![
                t.fg_success("  ✓ ").build(),
                t.fg_text(self.labels.project_configured.as_str()).build(),
            ]));
            lines.push(Line::from(vec![
                t.fg_accent(format!("  {}", self.labels.type_run))
                    .italic()
                    .build(),
            ]));
        } else {
            lines.push(Line::from(vec![
                t.fg_warning("  ○ ").build(),
                t.fg_text(self.labels.no_project_found.as_str()).build(),
            ]));
            lines.push(Line::from(vec![
                t.fg_dim(format!("  {}", self.labels.type_init))
                    .italic()
                    .build(),
            ]));
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(vec![
            t.fg_accent("  q").bold().build(),
            t.fg_dim(" quit  ").build(),
            t.fg_accent("?").bold().build(),
            t.fg_dim(" help  ").build(),
            t.fg_accent("F2").bold().build(),
            t.fg_dim(" sidebar  ").build(),
            t.fg_accent("j/k").bold().build(),
            t.fg_dim(" navigate").build(),
        ]));

        frame.render_widget(Paragraph::new(lines), area);
    }

    /// Renders and ticks down active toast notifications.
    pub(super) fn render_toasts(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.toasts.retain_mut(|toast| {
            toast.remaining_ticks = toast.remaining_ticks.saturating_sub(1);
            toast.remaining_ticks > 0
        });

        if self.toasts.is_empty() {
            return;
        }

        let max_toasts = 3;
        let toast_w = 40u16.min(area.width.saturating_sub(2));
        let toast_h = 3u16;
        let t = self.theme();

        for (i, toast) in self.toasts.iter().rev().take(max_toasts).enumerate() {
            let y = area.height.saturating_sub((i as u16 + 1) * (toast_h + 1));
            let x = area.width.saturating_sub(toast_w + 1);
            let popup = Rect::new(x, y, toast_w, toast_h);

            let color = match toast.level {
                ToastLevel::Info => t.accent(),
                ToastLevel::Success => t.success(),
                ToastLevel::Warning => t.warning(),
                ToastLevel::Error => t.error(),
            };

            frame.render_widget(Clear, popup);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color))
                .style(Style::default().bg(t.surface()));
            let inner = block.inner(popup);
            frame.render_widget(block, popup);

            frame.render_widget(
                Paragraph::new(toast.message.as_str()).style(Style::default().fg(t.text_bright())),
                inner,
            );
        }
    }

    /// Renders the quit confirmation modal (centered overlay).
    pub(super) fn render_quit_modal(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();
        let popup_w = 36u16.min(area.width);
        let popup_h = 5u16.min(area.height);
        let popup = Rect {
            x: area.x + (area.width.saturating_sub(popup_w)) / 2,
            y: area.y + (area.height.saturating_sub(popup_h)) / 2,
            width: popup_w,
            height: popup_h,
        };

        let lines = vec![
            Line::raw(""),
            Line::from(vec![
                t.fg_warning(format!("  {} ", self.labels.quit_question))
                    .bold()
                    .build(),
                t.fg_accent("y").bold().build(),
                t.fg_dim(" yes  ").build(),
                t.fg_accent("n").bold().build(),
                t.fg_dim(" cancel").build(),
            ]),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(t.warning()))
            .title(format!(" {} ", self.labels.quit_title))
            .title_style(
                Style::default()
                    .fg(t.warning())
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_widget(Clear, popup);
        frame.render_widget(Paragraph::new(lines).block(block), popup);
    }

    /// Renders the help modal popup (centered overlay with grouped keys).
    pub(super) fn render_help_modal(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();
        let version = env!("CARGO_PKG_VERSION");

        let mut lines: Vec<Line<'_>> = Vec::new();

        lines.push(Line::from(vec![
            t.fg_accent(format!("  Ralph Engine v{version}"))
                .bold()
                .build(),
        ]));
        lines.push(Line::raw(""));

        // Navigation keys
        lines.push(Line::from(vec![
            t.fg_bright(format!("  {}", self.labels.nav_heading))
                .bold()
                .build(),
        ]));
        for (key, desc) in &self.labels.nav_keys {
            lines.push(Line::from(vec![
                t.fg_accent(format!("  {key:<12}")).build(),
                t.fg_dim(desc.as_str()).build(),
            ]));
        }

        lines.push(Line::raw(""));

        // Action keys
        lines.push(Line::from(vec![
            t.fg_bright(format!("  {}", self.labels.actions_heading))
                .bold()
                .build(),
        ]));
        for (key, desc) in &self.labels.action_keys {
            lines.push(Line::from(vec![
                t.fg_accent(format!("  {key:<12}")).build(),
                t.fg_dim(desc.as_str()).build(),
            ]));
        }

        // Plugin keybindings
        let state_label = format!("{:?}", self.state);
        let plugin_keys: Vec<_> = self
            .plugin_keybindings
            .iter()
            .filter(|b| {
                b.active_states.is_empty() || b.active_states.iter().any(|s| s == &state_label)
            })
            .collect();

        if !plugin_keys.is_empty() {
            lines.push(Line::raw(""));
            lines.push(Line::from(vec![
                t.fg_bright(format!("  {}", self.labels.plugins_heading))
                    .bold()
                    .build(),
            ]));
            for binding in plugin_keys {
                lines.push(Line::from(vec![
                    t.fg_accent(format!("  {:<12}", binding.key)).build(),
                    t.fg_dim(binding.description.as_str()).build(),
                ]));
            }
        }

        if self.input_enabled {
            lines.push(Line::raw(""));
            lines.push(Line::from(vec![
                t.fg_dim(format!("  {}", self.labels.slash_hint))
                    .italic()
                    .build(),
            ]));
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(vec![
            t.fg_border(format!("  {}", self.labels.press_any_key))
                .build(),
        ]));

        let popup_h = (lines.len() as u16 + 2).min(area.height.saturating_sub(2));
        let popup_w = 44u16.min(area.width.saturating_sub(4));
        let popup = Rect {
            x: area.x + (area.width.saturating_sub(popup_w)) / 2,
            y: area.y + (area.height.saturating_sub(popup_h)) / 2,
            width: popup_w,
            height: popup_h,
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(t.accent()))
            .title(format!(" {} ", self.labels.help_title))
            .title_style(Style::default().fg(t.accent()).add_modifier(Modifier::BOLD));

        frame.render_widget(Clear, popup);
        frame.render_widget(Paragraph::new(lines).block(block), popup);
    }
}
