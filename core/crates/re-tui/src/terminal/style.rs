//! Styling functions for feed content and sidebar panels.
//!
//! Free functions that convert raw text + context into styled
//! ratatui `Line` / `Span` sequences. Used by render modules.

use ratatui::style::Color;
use ratatui::text::{Line, Span};
use ratatui_themekit::ThemeExt;
use ratatui_themekit::builders::ThemedSpan;

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
        PanelHint::Inline => render_inline(item, accent, theme, lines),
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
        ThemedSpan::with_color(format!("  {icon} "), sev_color)
            .bold()
            .build(),
        theme.fg_dim(format!("{label} ")).build(),
        ThemedSpan::with_color(value, sev_color).bold().build(),
    ]));
}

fn render_inline<'a>(
    item: &'a PanelItem,
    accent: Color,
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
            theme.fg_dim(format!("  {label} ")).build(),
            ThemedSpan::with_color(num_str, accent).bold().build(),
        ];
        if let Some(t) = item.total {
            let pct = if t > 0 { num * 100 / t } else { 0 };
            spans.push(theme.fg_dim(format!("/{t}")).build());
            let mini_fill = (pct as usize * 3 / 100).min(3);
            let mini_bar = format!(" {}{}", "▰".repeat(mini_fill), "▱".repeat(3 - mini_fill));
            spans.push(ThemedSpan::with_color(mini_bar, accent).build());
        }
        lines.push(Line::from(spans));
    } else {
        let value = item.value.as_deref().unwrap_or("");
        lines.push(Line::from(vec![
            theme.fg_dim(format!("  {label}: ")).build(),
            theme.fg_bright(value).build(),
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
        theme.fg_dim(format!("  {label} ")).build(),
        ThemedSpan::with_color(bar, bar_color).build(),
        ThemedSpan::with_color(format!(" {pct}%"), bar_color)
            .bold()
            .build(),
    ]));
}

fn render_pairs<'a>(
    item: &'a PanelItem,
    theme: &dyn crate::theme::Theme,
    lines: &mut Vec<Line<'a>>,
) {
    for (key, val) in &item.pairs {
        lines.push(Line::from(vec![
            theme.fg_dim(format!("  {key}")).build(),
            theme.fg_border(" → ").build(),
            theme.fg_bright(val.as_str()).build(),
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
            ThemedSpan::with_color(bullet, accent).build(),
            theme.fg_text(item_text.as_str()).build(),
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
        Span::raw("  "),
        theme.fg_dim(text).italic().build(),
    ]));
}

fn render_separator(theme: &dyn crate::theme::Theme, lines: &mut Vec<Line<'_>>) {
    lines.push(Line::from(vec![
        Span::raw("  "),
        theme.fg_border("· · · · · · · ·").build(),
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
        return Line::from(vec![theme.fg_success(format!("  {line}")).build()]);
    }
    if line.starts_with('✗')
        || line.contains("Error")
        || line.contains("Failed")
        || line.contains("Not found")
    {
        return Line::from(vec![theme.fg_error(format!("  {line}")).build()]);
    }

    // Key: Value pattern → dim key, bright value
    if let Some(colon_pos) = line.find(": ") {
        let (key, val) = line.split_at(colon_pos + 2);
        let val_trimmed = val.trim();
        let val_span = if val_trimmed.parse::<f64>().is_ok()
            || val_trimmed.starts_with('$')
            || val_trimmed.ends_with('%')
        {
            ThemedSpan::with_color(val, panel_color).bold().build()
        } else if val_trimmed == "true" || val_trimmed == "enabled" || val_trimmed == "yes" {
            theme.fg_success(val).build()
        } else if val_trimmed == "false" || val_trimmed == "disabled" || val_trimmed == "no" {
            theme.fg_dim(val).build()
        } else {
            theme.fg_bright(val).build()
        };
        return Line::from(vec![theme.fg_dim(format!("  {key}")).build(), val_span]);
    }

    // Pure number lines → bold accent
    let trimmed = line.trim();
    if !trimmed.is_empty() && trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Line::from(vec![
            ThemedSpan::with_color(format!("  {line}"), panel_color)
                .bold()
                .build(),
        ]);
    }

    // Default: dim text
    Line::from(vec![theme.fg_dim(format!("  {line}")).build()])
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
                vec![theme.fg_added(line).build()]
            } else if line.starts_with('-') {
                vec![theme.fg_removed(line).build()]
            } else if line.starts_with("@@") {
                vec![theme.fg_info(line).bold().build()]
            } else {
                vec![ThemedSpan::with_color(line, theme.diff_context()).build()]
            }
        }
        BlockKind::Command => {
            if line.contains("FAIL") || line.contains("error") || line.contains("Error") {
                vec![theme.fg_error(line).build()]
            } else if line.contains("PASS") || line.contains("✓") || line.starts_with("ok") {
                vec![theme.fg_success(line).build()]
            } else {
                vec![theme.fg_dim(line).build()]
            }
        }
        BlockKind::Thinking => vec![theme.fg_dim(line).italic().build()],
        BlockKind::GateFail => vec![theme.fg_error(line).build()],
        BlockKind::GatePass => vec![theme.fg_success(line).build()],
        _ => vec![Span::raw(line)],
    }
}
