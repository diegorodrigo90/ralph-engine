//! Sidebar rendering — polished groups using `ThemeKit` builders.
//!
//! Each group is a titled block with consistent visual treatment.
//! Agents show badge-style status, Sprint shows progress, Findings show bullets.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::widgets::{Borders, Paragraph};
use ratatui_themekit::builders::ThemedSpan;

use crate::theme::ThemeExt;

use super::shell::TuiShell;
use super::sidebar_groups::group_panels;
use super::types::{PanelHint, PanelSeverity, SidebarPanel};

impl TuiShell {
    /// Renders the sidebar with polished grouped panels.
    pub(super) fn render_sidebar(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();

        // Outer border separating sidebar from main content
        let outer = t.block_plain().borders(Borders::LEFT).build();
        let inner = outer.inner(area);
        frame.render_widget(outer, area);

        if self.sidebar_panels.is_empty() {
            return;
        }

        let groups = group_panels(&self.sidebar_panels);
        if groups.is_empty() {
            return;
        }

        // Split vertical space between groups
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
            // Each group gets a themed block container with title
            let group_block = t.block(format!(" {} ", group.title).leak()).build();
            let group_inner = group_block.inner(group_areas[i]);
            frame.render_widget(group_block, group_areas[i]);

            let mut lines: Vec<Line<'_>> = Vec::new();

            match group.title {
                "Agents" => render_agents(&group.panels, t, &mut lines),
                "Sprint" => render_sprint(&group.panels, t, &mut lines),
                "Findings" => render_findings(&group.panels, t, &mut lines),
                _ => {}
            }

            frame.render_widget(Paragraph::new(lines), group_inner);
        }
    }

    /// Renders the control panel (wide tier, active mode only).
    pub(super) fn render_control_panel(&self, frame: &mut Frame<'_>, area: Rect) {
        let t = self.theme();
        let state_color = self.state.color(t);
        let state_label = self.localized_state_label();

        let block = t.block_plain().borders(Borders::RIGHT).build();
        let inner = block.inner(area);
        frame.render_widget(block, area);

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

/// Agents: name + badge status, one line each.
///
/// ```text
///  claude       [ready]
///  claudebox    [ready]
///  codex        [—]
/// ```
fn render_agents<'a>(
    panels: &'a [&'a SidebarPanel],
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    for panel in panels {
        let (status, severity) = extract_status(panel);
        let badge_color = match severity {
            PanelSeverity::Success => theme.success(),
            PanelSeverity::Error => theme.error(),
            PanelSeverity::Warning => theme.warning(),
            PanelSeverity::Neutral => theme.text_dim(),
        };

        let name = panel.title.to_lowercase();
        let padded = format!(" {name:<13}");

        lines.push(Line::from(vec![
            theme.fg_text(padded).build(),
            theme.badge(format!(" {status} "), badge_color).build(),
        ]));
    }
}

/// Sprint: progress bar + top stories.
fn render_sprint<'a>(
    panels: &'a [&'a SidebarPanel],
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    let accent = theme.info();
    for panel in panels {
        for item in &panel.items {
            if item.hint == PanelHint::Bar {
                crate::terminal::style::render_panel_item(item, accent, theme, lines);
            }
        }
        for item in &panel.items {
            if item.hint == PanelHint::List {
                render_story_list(item, accent, theme, lines);
            }
        }
    }
}

/// Renders a single story list item (max 5 stories + overflow indicator).
fn render_story_list<'a>(
    item: &'a super::types::PanelItem,
    accent: ratatui::style::Color,
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    let sev_color = match item.severity {
        PanelSeverity::Warning => theme.warning(),
        PanelSeverity::Success => theme.success(),
        _ => accent,
    };

    for story in item.items.iter().take(5) {
        lines.push(Line::from(vec![
            ThemedSpan::with_color(" → ", sev_color).build(),
            theme.fg_text(story.as_str()).build(),
        ]));
    }

    let remaining = item.items.len().saturating_sub(5);
    if remaining > 0 {
        lines.push(
            theme
                .fg_dim(format!("   +{remaining} more"))
                .italic()
                .build()
                .into(),
        );
    }
}

/// Findings: bullet list only.
fn render_findings<'a>(
    panels: &'a [&'a SidebarPanel],
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    let accent = theme.warning();
    for panel in panels {
        for item in &panel.items {
            if item.hint == PanelHint::List {
                for heading in &item.items {
                    lines.push(Line::from(vec![
                        ThemedSpan::with_color(" • ", accent).build(),
                        theme.fg_text(heading.as_str()).build(),
                    ]));
                }
            }
        }
    }
}

/// Extracts primary status text and severity from a panel's Indicator item.
fn extract_status(panel: &SidebarPanel) -> (String, PanelSeverity) {
    for item in &panel.items {
        if item.hint == PanelHint::Indicator {
            let status = item.value.as_deref().unwrap_or("\u{2014}").to_lowercase();
            return (status, item.severity);
        }
    }
    ("\u{2014}".to_owned(), PanelSeverity::Neutral)
}
