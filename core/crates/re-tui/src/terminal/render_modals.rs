//! Modal popups, toasts, idle dashboard, and agent switcher rendering.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Borders, Clear, List, ListItem, ListState, Paragraph};

use crate::theme::ThemeExt;

use super::shell::TuiShell;
use super::types::{ToastLevel, TuiState};

impl TuiShell {
    /// Renders the help bar at the bottom.
    pub(super) fn render_help(&self, frame: &mut Frame<'_>, zones: &crate::layout::LayoutZones) {
        let t = self.theme();

        if self.quit_pending || self.help_modal_visible {
            let hint = format!(" {} ", self.labels.modal_open_hint);
            frame.render_widget(Paragraph::new(t.line().dim(hint).build()), zones.help);
            return;
        }

        // When typing, show input-specific help
        if self.input_enabled && !self.text_input_buffer.is_empty() {
            let line = t
                .line()
                .dim(" [Enter]")
                .dim(" send ")
                .dim(" [Alt+Enter]")
                .dim(" newline ")
                .dim(" [Esc]")
                .dim(" cancel ")
                .build();
            frame.render_widget(Paragraph::new(line), zones.help);
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

            let status = t
                .line()
                .warning(format!(" {spinner} "))
                .bright(format!("{message} "))
                .build();
            frame.render_widget(Paragraph::new(status), zones.help);
            return;
        }

        // Contextual help bar — changes based on focused panel
        use super::types::FocusTarget;

        let is_compact = zones.tier == crate::layout::LayoutTier::Compact;
        let line = match self.focus {
            FocusTarget::Activity if is_compact => t
                .line()
                .dim(" [j/k]")
                .dim(" scroll ")
                .dim(" [p]")
                .dim(format!(" {} ", self.labels.pause_label))
                .dim(" [?]")
                .dim(format!(" {} ", self.labels.help_label))
                .dim(" [q]")
                .dim(format!(" {} ", self.labels.quit_label))
                .build(),
            FocusTarget::Activity => t
                .line()
                .dim(" [j/k]")
                .dim(" scroll ")
                .dim(" [Enter]")
                .dim(" expand ")
                .dim(" [Tab]")
                .dim(" focus ")
                .dim(" [/]")
                .dim(" cmds ")
                .dim(" [p]")
                .dim(format!(" {} ", self.labels.pause_label))
                .dim(" [q]")
                .dim(format!(" {} ", self.labels.quit_label))
                .build(),
            FocusTarget::Sidebar => t
                .line()
                .dim(" [j/k]")
                .dim(" scroll ")
                .dim(" [Tab]")
                .dim(" back ")
                .dim(" [F2]")
                .dim(" toggle ")
                .dim(" [q]")
                .dim(format!(" {} ", self.labels.quit_label))
                .build(),
            FocusTarget::Input => t
                .line()
                .dim(" [Enter]")
                .dim(" send ")
                .dim(" [Alt+Enter]")
                .dim(" newline ")
                .dim(" [Esc]")
                .dim(" cancel ")
                .dim(" [Tab]")
                .dim(" back ")
                .build(),
        };

        // Append plugin keybindings + tier label
        let mut spans = line.spans;
        let state_label = format!("{:?}", self.state);
        for binding in &self.plugin_keybindings {
            if binding.active_states.is_empty()
                || binding.active_states.iter().any(|s| s == &state_label)
            {
                spans.push(t.fg_dim(format!(" [{}]", binding.key)).build());
                spans.push(t.fg_dim(format!(" {} ", binding.description)).build());
            }
        }

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
            .map(|agent| ListItem::new(t.line().text(format!("  {agent}")).build()))
            .collect();

        let ls = t.list_styles();
        let list = List::new(items)
            .block(t.block(" Switch Agent (Ctrl+A) ").focused(true).build())
            .highlight_style(ls.highlight)
            .highlight_symbol(ls.symbol);

        frame.render_widget(Clear, popup_area);
        frame.render_stateful_widget(
            list,
            popup_area,
            &mut ListState::default().with_selected(Some(self.agent_switcher_selected)),
        );
    }

    /// Renders the idle dashboard — logo, inline agent status, command hints.
    ///
    /// This is the "welcome screen" — clean, focused, immediately useful.
    /// No sidebar, no tabs, no metrics. Just what the user needs to know.
    pub(super) fn render_idle_dashboard(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();
        let version = env!("CARGO_PKG_VERSION");

        let logo_color = Some(match self.state {
            TuiState::Error => t.error(),
            _ => t.accent(),
        });
        let logo_lines =
            crate::logo::build_logo_lines(area.width, t, logo_color, &self.labels.logo_tagline);

        let mut lines: Vec<Line<'_>> = Vec::new();

        // Estimate content height for vertical centering
        let content_height = logo_lines.len() + 16;
        let pad_top = area.height.saturating_sub(content_height as u16) / 2;
        for _ in 0..pad_top {
            lines.push(Line::raw(""));
        }

        lines.extend(logo_lines);
        lines.push(Line::raw(""));
        lines.push(
            t.line()
                .dim(format!(
                    "  v{version} — {}",
                    self.labels.orchestration_runtime
                ))
                .build(),
        );
        lines.push(Line::raw(""));

        // Inline agent status row — all agents on one line (Model B: uses is_agent flag)
        if let Some(agent_line) = self.build_idle_agent_line(t) {
            lines.push(agent_line);
            lines.push(Line::raw(""));
        }

        // Command hints — plugin-contributed (Model B) + core builtins
        if self.idle_hints.is_empty() {
            // No plugin hints → show no-project message + init
            lines.push(
                t.line()
                    .warning("  ○ ")
                    .text(self.labels.no_project_found.as_str())
                    .build(),
            );
            lines.push(
                t.line()
                    .accent_bold("  ralph-engine init")
                    .dim("  set up project")
                    .build(),
            );
        } else {
            // Render plugin-contributed idle hints
            for hint in &self.idle_hints {
                let pad = 14usize.saturating_sub(hint.command.len());
                lines.push(
                    t.line()
                        .accent_bold(format!("  {}", hint.command))
                        .dim(format!("{:pad$}{}", "", hint.description))
                        .build(),
                );
            }
        }
        // Core builtins (always shown)
        lines.push(
            t.line()
                .accent_bold("  /theme")
                .dim("         change theme")
                .build(),
        );
        lines.push(
            t.line()
                .accent_bold("  /help")
                .dim("          show all commands")
                .build(),
        );

        frame.render_widget(Paragraph::new(lines), area);
    }

    /// Builds the inline agent status line for the idle dashboard.
    /// Returns `None` if no agent panels are present.
    fn build_idle_agent_line<'a>(&'a self, t: &'a dyn crate::theme::Theme) -> Option<Line<'a>> {
        let mut spans: Vec<ratatui::text::Span<'_>> = vec![t.fg_dim("  ").build()];
        let mut found = false;

        for panel in &self.sidebar_panels {
            if !panel.is_agent {
                continue;
            }
            found = true;
            let (icon, icon_color) = self.plugin_status_icon(panel);
            let name = panel.title.to_lowercase();
            let status = panel
                .items
                .iter()
                .find(|i| i.hint == super::types::PanelHint::Indicator)
                .and_then(|i| i.value.as_deref())
                .unwrap_or("\u{2014}")
                .to_lowercase();

            if spans.len() > 1 {
                spans.push(t.fg_dim("    ").build());
            }
            spans.push(
                ratatui_themekit::builders::ThemedSpan::with_color(format!("{icon} "), icon_color)
                    .bold()
                    .build(),
            );
            spans.push(t.fg_text(format!("{name} {status}")).build());
        }

        if found { Some(Line::from(spans)) } else { None }
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

            let ns = t.notification_styles();
            let border_style = match toast.level {
                ToastLevel::Info => ns.info,
                ToastLevel::Success => ns.success,
                ToastLevel::Warning => ns.warning,
                ToastLevel::Error => ns.error,
            };

            frame.render_widget(Clear, popup);
            let styled_block = ratatui::widgets::Block::new()
                .borders(Borders::ALL)
                .border_style(border_style)
                .style(ns.background);
            let inner = styled_block.inner(popup);
            frame.render_widget(styled_block, popup);

            frame.render_widget(Paragraph::new(toast.message.as_str()).style(ns.body), inner);
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
            t.line()
                .warning(format!("  {} ", self.labels.quit_question))
                .accent_bold("y")
                .dim(" yes  ")
                .accent_bold("n")
                .dim(" cancel")
                .build(),
        ];

        let title = format!(" {} ", self.labels.quit_title);
        let block = t.block(&title).focused(true).build();

        frame.render_widget(Clear, popup);
        frame.render_widget(Paragraph::new(lines).block(block), popup);
    }

    /// Renders the help modal popup (centered overlay with grouped keys).
    pub(super) fn render_help_modal(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();
        let version = env!("CARGO_PKG_VERSION");

        let mut lines: Vec<Line<'_>> = Vec::new();

        lines.push(
            t.line()
                .accent_bold(format!("  Ralph Engine v{version}"))
                .build(),
        );
        lines.push(Line::raw(""));

        // Navigation keys
        lines.push(
            t.line()
                .bright(format!("  {}", self.labels.nav_heading))
                .build(),
        );
        for (key, desc) in &self.labels.nav_keys {
            lines.push(
                t.line()
                    .accent(format!("  {key:<12}"))
                    .dim(desc.as_str())
                    .build(),
            );
        }

        lines.push(Line::raw(""));

        // Action keys
        lines.push(
            t.line()
                .bright(format!("  {}", self.labels.actions_heading))
                .build(),
        );
        for (key, desc) in &self.labels.action_keys {
            lines.push(
                t.line()
                    .accent(format!("  {key:<12}"))
                    .dim(desc.as_str())
                    .build(),
            );
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
            lines.push(
                t.line()
                    .bright(format!("  {}", self.labels.plugins_heading))
                    .build(),
            );
            for binding in plugin_keys {
                lines.push(
                    t.line()
                        .accent(format!("  {:<12}", binding.key))
                        .dim(binding.description.as_str())
                        .build(),
                );
            }
        }

        if self.input_enabled {
            lines.push(Line::raw(""));
            lines.push(
                t.line()
                    .dim(format!("  {}", self.labels.slash_hint))
                    .build(),
            );
        }

        lines.push(Line::raw(""));
        lines.push(
            t.line()
                .border(format!("  {}", self.labels.press_any_key))
                .build(),
        );

        let popup_h = (lines.len() as u16 + 2).min(area.height.saturating_sub(2));
        let popup_w = 44u16.min(area.width.saturating_sub(4));
        let popup = Rect {
            x: area.x + (area.width.saturating_sub(popup_w)) / 2,
            y: area.y + (area.height.saturating_sub(popup_h)) / 2,
            width: popup_w,
            height: popup_h,
        };

        let title = format!(" {} ", self.labels.help_title);
        let block = t.block(&title).focused(true).build();

        frame.render_widget(Clear, popup);
        frame.render_widget(Paragraph::new(lines).block(block), popup);
    }

    /// Renders the theme selector modal with live preview.
    pub(super) fn render_theme_selector(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();
        let ids = crate::theme::available_theme_ids();

        let themes = crate::theme::builtin_themes();
        let items: Vec<ListItem<'_>> = ids
            .iter()
            .enumerate()
            .map(|(i, id)| {
                let name = themes
                    .iter()
                    .find(|td| td.id() == *id)
                    .map_or(*id, |td| td.name());
                let marker = if Some(id.to_string()) == self.theme_selector_previous
                    || (self.theme_selector_previous.is_none() && i == 0)
                {
                    " ●"
                } else {
                    "  "
                };
                ListItem::new(
                    t.line()
                        .dim(marker.to_owned())
                        .text(format!(" {name}"))
                        .dim(format!("  ({id})"))
                        .build(),
                )
            })
            .collect();

        let popup_h = (items.len() as u16 + 2).min(area.height.saturating_sub(4));
        let popup_w = 44u16.min(area.width.saturating_sub(4));
        let popup = Rect {
            x: area.x + (area.width.saturating_sub(popup_w)) / 2,
            y: area.y + (area.height.saturating_sub(popup_h)) / 2,
            width: popup_w,
            height: popup_h,
        };

        let ls = t.list_styles();
        let list = List::new(items)
            .block(t.block(" Theme ").focused(true).build())
            .highlight_style(ls.highlight)
            .highlight_symbol(ls.symbol);

        frame.render_widget(Clear, popup);
        frame.render_stateful_widget(
            list,
            popup,
            &mut ListState::default().with_selected(Some(self.theme_selector_selected)),
        );
    }
}
