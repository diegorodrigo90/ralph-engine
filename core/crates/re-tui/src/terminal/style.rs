//! Styling functions for feed content and sidebar panels.
//!
//! Free functions that convert raw text + context into styled
//! ratatui `Line` / `Span` sequences. Used by render modules.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use super::types::{PanelHint, PanelItem, PanelSeverity};

/// Renders a typed panel item into lines with theme-driven styling.
///
/// This is the core of the design system: each `PanelItem` variant
/// has a consistent, beautiful rendering regardless of which plugin
/// created it.
pub(super) fn render_panel_item<'a>(
    item: &'a PanelItem,
    accent: Color,
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    let sev_color = match item.severity {
        PanelSeverity::Success => theme.success(),
        PanelSeverity::Warning => theme.warning(),
        PanelSeverity::Error => theme.error(),
        PanelSeverity::Neutral => accent,
    };

    match item.hint {
        PanelHint::Indicator => render_indicator(item, sev_color, theme, lines),
        PanelHint::Inline => render_inline(item, accent, sev_color, theme, lines),
        PanelHint::Bar => render_bar(item, accent, theme, lines),
        PanelHint::Pairs => render_pairs(item, theme, lines),
        PanelHint::List => render_list(item, accent, theme, lines),
        PanelHint::Text => render_text(item, theme, lines),
        PanelHint::Separator => render_separator(theme, lines),
    }
}

fn render_indicator<'a>(
    item: &'a PanelItem,
    sev_color: Color,
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    let icon = match item.severity {
        PanelSeverity::Success => "●",
        PanelSeverity::Warning => "◆",
        PanelSeverity::Error => "⬤",
        PanelSeverity::Neutral => "○",
    };
    let label = item.label.as_deref().unwrap_or("");
    let value = item.value.as_deref().unwrap_or("");
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {icon} "),
            Style::default().fg(sev_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(format!("{label} "), Style::default().fg(theme.text_dim())),
        Span::styled(
            value,
            Style::default().fg(sev_color).add_modifier(Modifier::BOLD),
        ),
    ]));
}

fn render_inline<'a>(
    item: &'a PanelItem,
    accent: Color,
    _sev_color: Color,
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    let label = item.label.as_deref().unwrap_or("");
    if let Some(num) = item.numeric {
        let num_str = if num >= 1000 {
            format!("{}.{}k", num / 1000, (num % 1000) / 100)
        } else {
            format!("{num}")
        };
        let mut spans = vec![
            Span::styled(format!("  {label} "), Style::default().fg(theme.text_dim())),
            Span::styled(
                num_str,
                Style::default().fg(accent).add_modifier(Modifier::BOLD),
            ),
        ];
        if let Some(t) = item.total {
            let pct = if t > 0 { num * 100 / t } else { 0 };
            spans.push(Span::styled(
                format!("/{t}"),
                Style::default().fg(theme.text_dim()),
            ));
            let mini_fill = (pct as usize * 3 / 100).min(3);
            let mini_bar = format!(" {}{}", "▰".repeat(mini_fill), "▱".repeat(3 - mini_fill));
            spans.push(Span::styled(mini_bar, Style::default().fg(accent)));
        }
        lines.push(Line::from(spans));
    } else {
        let value = item.value.as_deref().unwrap_or("");
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {label}: "),
                Style::default().fg(theme.text_dim()),
            ),
            Span::styled(value, Style::default().fg(theme.text_bright())),
        ]));
    }
}

fn render_bar<'a>(
    item: &'a PanelItem,
    accent: Color,
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    let pct = item.numeric.unwrap_or(0).min(100) as usize;
    let bar_width = 12;
    let filled = bar_width * pct / 100;
    let empty = bar_width - filled;
    let bar = format!("{}{}", "▰".repeat(filled), "▱".repeat(empty));
    let label = item.label.as_deref().unwrap_or("");
    let bar_color = if pct >= 80 {
        theme.success()
    } else if pct >= 40 {
        accent
    } else {
        theme.warning()
    };
    lines.push(Line::from(vec![
        Span::styled(format!("  {label} "), Style::default().fg(theme.text_dim())),
        Span::styled(bar, Style::default().fg(bar_color)),
        Span::styled(
            format!(" {pct}%"),
            Style::default().fg(bar_color).add_modifier(Modifier::BOLD),
        ),
    ]));
}

fn render_pairs<'a>(
    item: &'a PanelItem,
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    for (key, val) in &item.pairs {
        lines.push(Line::from(vec![
            Span::styled(format!("  {key}"), Style::default().fg(theme.text_dim())),
            Span::styled(" → ", Style::default().fg(theme.border())),
            Span::styled(val.as_str(), Style::default().fg(theme.text_bright())),
        ]));
    }
}

fn render_list<'a>(
    item: &'a PanelItem,
    accent: Color,
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    for (i, item_text) in item.items.iter().enumerate() {
        let bullet = if item.items.len() > 5 {
            format!("  {:>2}. ", i + 1)
        } else {
            "  ▸ ".to_owned()
        };
        lines.push(Line::from(vec![
            Span::styled(bullet, Style::default().fg(accent)),
            Span::styled(item_text.as_str(), Style::default().fg(theme.text())),
        ]));
    }
}

fn render_text<'a>(
    item: &'a PanelItem,
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    let text = item.label.as_deref().unwrap_or("");
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(
            text,
            Style::default()
                .fg(theme.text_dim())
                .add_modifier(Modifier::ITALIC),
        ),
    ]));
}

fn render_separator(theme: &dyn crate::theme::Theme, lines: &mut Vec<Line<'_>>) {
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled("· · · · · · · ·", Style::default().fg(theme.border())),
    ]));
}

/// Styles a sidebar panel line with visual emphasis based on content patterns.
pub(super) fn style_sidebar_line<'a>(
    line: &'a str,
    panel_color: Color,
    theme: &dyn crate::theme::Theme,
) -> Line<'a> {
    // Status indicators
    if line.starts_with('✓') || line.contains("Available") || line.contains("Ready") {
        return Line::from(vec![Span::styled(
            format!("  {line}"),
            Style::default().fg(theme.success()),
        )]);
    }
    if line.starts_with('✗')
        || line.contains("Error")
        || line.contains("Failed")
        || line.contains("Not found")
    {
        return Line::from(vec![Span::styled(
            format!("  {line}"),
            Style::default().fg(theme.error()),
        )]);
    }

    // Key: Value pattern → dim key, bright value
    if let Some(colon_pos) = line.find(": ") {
        let (key, val) = line.split_at(colon_pos + 2);
        let val_trimmed = val.trim();
        let val_style = if val_trimmed.parse::<f64>().is_ok()
            || val_trimmed.starts_with('$')
            || val_trimmed.ends_with('%')
        {
            Style::default()
                .fg(panel_color)
                .add_modifier(Modifier::BOLD)
        } else if val_trimmed == "true" || val_trimmed == "enabled" || val_trimmed == "yes" {
            Style::default().fg(theme.success())
        } else if val_trimmed == "false" || val_trimmed == "disabled" || val_trimmed == "no" {
            Style::default().fg(theme.text_dim())
        } else {
            Style::default().fg(theme.text_bright())
        };
        return Line::from(vec![
            Span::styled(format!("  {key}"), Style::default().fg(theme.text_dim())),
            Span::styled(val, val_style),
        ]);
    }

    // Pure number lines → bold accent
    let trimmed = line.trim();
    if !trimmed.is_empty() && trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Line::from(vec![Span::styled(
            format!("  {line}"),
            Style::default()
                .fg(panel_color)
                .add_modifier(Modifier::BOLD),
        )]);
    }

    // Default: dim text
    Line::from(vec![Span::styled(
        format!("  {line}"),
        Style::default().fg(theme.text_dim()),
    )])
}

/// Styles a content line as a vec of spans (preserving per-character coloring).
///
/// Returns spans (not a Line) so the caller can prepend border spans
/// without losing the style information.
pub(super) fn style_content_line<'a>(
    line: &'a str,
    kind: crate::feed::BlockKind,
    theme: &dyn crate::theme::Theme,
) -> Vec<Span<'a>> {
    use crate::feed::BlockKind;

    match kind {
        BlockKind::FileEdit => {
            if line.starts_with('+') {
                vec![Span::styled(line, Style::default().fg(theme.diff_added()))]
            } else if line.starts_with('-') {
                vec![Span::styled(
                    line,
                    Style::default().fg(theme.diff_removed()),
                )]
            } else if line.starts_with("@@") {
                vec![Span::styled(
                    line,
                    Style::default()
                        .fg(theme.info())
                        .add_modifier(Modifier::BOLD),
                )]
            } else {
                vec![Span::styled(
                    line,
                    Style::default().fg(theme.diff_context()),
                )]
            }
        }
        BlockKind::Command => {
            if line.contains("FAIL") || line.contains("error") || line.contains("Error") {
                vec![Span::styled(line, Style::default().fg(theme.error()))]
            } else if line.contains("PASS") || line.contains("✓") || line.starts_with("ok") {
                vec![Span::styled(line, Style::default().fg(theme.success()))]
            } else {
                vec![Span::styled(line, Style::default().fg(theme.text_dim()))]
            }
        }
        BlockKind::Thinking => vec![Span::styled(
            line,
            Style::default()
                .fg(theme.text_dim())
                .add_modifier(Modifier::ITALIC),
        )],
        BlockKind::GateFail => vec![Span::styled(line, Style::default().fg(theme.error()))],
        BlockKind::GatePass => vec![Span::styled(line, Style::default().fg(theme.success()))],
        _ => vec![Span::raw(line)],
    }
}
