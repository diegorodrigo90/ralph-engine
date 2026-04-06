//! Sidebar and control panel rendering.
//!
//! The sidebar groups plugin panels into semantic domains (Agents,
//! Sprint, Tools, Findings) instead of showing one panel per plugin.
//! Each group has a distinct rendering style optimized for its content.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Borders, Paragraph};
use ratatui_themekit::builders::ThemedSpan;

use crate::theme::ThemeExt;

use super::shell::TuiShell;
use super::sidebar_groups::group_panels;
use super::types::{PanelHint, PanelSeverity, SidebarPanel};

impl TuiShell {
    /// Renders the sidebar zone with grouped plugin panels.
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
        let mut all_lines: Vec<Line<'_>> = Vec::new();

        for (i, group) in groups.iter().enumerate() {
            let color = group_colors[i % group_colors.len()];

            if i > 0 {
                all_lines.push(Line::raw(""));
            }
            all_lines.push(render_group_heading(group.title, inner.width, color, t));

            // Each group type has a condensed rendering style
            match group.title {
                "Agents" => render_agent_group(group.panels.as_slice(), t, &mut all_lines),
                "Sprint" => render_sprint_group(group.panels.as_slice(), color, t, &mut all_lines),
                "Findings" => {
                    render_findings_group(group.panels.as_slice(), color, t, &mut all_lines);
                }
                _ => render_tool_group(group.panels.as_slice(), t, &mut all_lines),
            }
        }

        frame.render_widget(Paragraph::new(all_lines), inner);
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

/// Renders a group heading.
fn render_group_heading<'a>(
    title: &str,
    width: u16,
    color: ratatui::style::Color,
    theme: &dyn crate::theme::Theme,
) -> Line<'a> {
    let border_len = width.saturating_sub(title.len() as u16 + 3) as usize;
    Line::from(vec![
        ThemedSpan::with_color(format!(" {title} "), color)
            .bold()
            .build(),
        theme.fg_border("\u{2500}".repeat(border_len)).build(),
    ])
}

/// Agents group: one-line per agent with status dot.
///
/// ```text
///  ● claude         ready
///  ○ claudebox      —
///  ○ codex          —
/// ```
fn render_agent_group<'a>(
    panels: &'a [&'a SidebarPanel],
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    for panel in panels {
        // Extract status from the first indicator block
        let (status, severity) = extract_indicator_status(panel);
        let (icon, icon_color) = match severity {
            PanelSeverity::Success => ("●", theme.success()),
            PanelSeverity::Error => ("✗", theme.error()),
            PanelSeverity::Warning => ("◆", theme.warning()),
            PanelSeverity::Neutral => ("○", theme.text_dim()),
        };

        // Short name from panel title (e.g., "Claude" → "claude")
        let name = panel.title.to_lowercase();
        let padded = format!("{name:<14}");

        lines.push(Line::from(vec![
            ThemedSpan::with_color(format!("  {icon} "), icon_color)
                .bold()
                .build(),
            theme.fg_text(padded).build(),
            ThemedSpan::with_color(status, icon_color).build(),
        ]));
    }
}

/// Sprint group: progress bar + active stories.
///
/// ```text
///  ████████░░ 72%  (36/50)
///  → Story 5.3   doing
///  → Story 5.4   todo
/// ```
fn render_sprint_group<'a>(
    panels: &'a [&'a SidebarPanel],
    accent: ratatui::style::Color,
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    for panel in panels {
        for item in &panel.items {
            match item.hint {
                PanelHint::Bar => {
                    super::style::render_panel_item(item, accent, theme, lines);
                }
                PanelHint::List => {
                    // Stories as arrow-prefixed lines
                    for story in &item.items {
                        let sev_color = match item.severity {
                            PanelSeverity::Warning => theme.warning(),
                            PanelSeverity::Success => theme.success(),
                            _ => accent,
                        };
                        lines.push(Line::from(vec![
                            ThemedSpan::with_color("  → ", sev_color).build(),
                            theme.fg_text(story.as_str()).build(),
                        ]));
                    }
                }
                PanelHint::Inline => {
                    // Metrics: compact inline
                    super::style::render_panel_item(item, accent, theme, lines);
                }
                _ => {
                    super::style::render_panel_item(item, accent, theme, lines);
                }
            }
        }
        // No legacy lines fallback — all plugins use typed items
    }
}

/// Tools group: one-line per tool with status dot.
///
/// ```text
///  ● GitHub MCP     ready
///  ● TDD Strict     active
///  ● Router         3 rules
/// ```
fn render_tool_group<'a>(
    panels: &'a [&'a SidebarPanel],
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    for panel in panels {
        let (status, severity) = extract_indicator_status(panel);
        let (icon, icon_color) = match severity {
            PanelSeverity::Success => ("●", theme.success()),
            PanelSeverity::Error => ("✗", theme.error()),
            PanelSeverity::Warning => ("◆", theme.warning()),
            PanelSeverity::Neutral => ("○", theme.text_dim()),
        };

        let name = &panel.title;
        let padded = format!("{name:<14}");

        lines.push(Line::from(vec![
            ThemedSpan::with_color(format!("  {icon} "), icon_color)
                .bold()
                .build(),
            theme.fg_text(padded).build(),
            ThemedSpan::with_color(status, icon_color).build(),
        ]));
    }
}

/// Findings group: section count + headings list.
///
/// ```text
///  3 sections
///  • Bug in parser
///  • Performance issue
/// ```
fn render_findings_group<'a>(
    panels: &'a [&'a SidebarPanel],
    accent: ratatui::style::Color,
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    for panel in panels {
        // Metrics first (section count)
        for item in &panel.items {
            if item.hint == PanelHint::Inline && item.numeric.is_some() {
                super::style::render_panel_item(item, accent, theme, lines);
            }
        }
        // Section headings as bullets
        for item in &panel.items {
            if item.hint == PanelHint::List {
                for heading in &item.items {
                    lines.push(Line::from(vec![
                        ThemedSpan::with_color("  • ", accent).build(),
                        theme.fg_text(heading.as_str()).build(),
                    ]));
                }
            }
        }
        // No legacy lines — all plugins use typed items
    }
}

/// Extracts the primary status text and severity from a panel.
///
/// Looks for the first Indicator item. Falls back to first line or "—".
fn extract_indicator_status(panel: &SidebarPanel) -> (String, PanelSeverity) {
    for item in &panel.items {
        if item.hint == PanelHint::Indicator {
            let status = item.value.as_deref().unwrap_or("\u{2014}").to_lowercase();
            return (status, item.severity);
        }
    }
    ("\u{2014}".to_owned(), PanelSeverity::Neutral) // em dash
}
