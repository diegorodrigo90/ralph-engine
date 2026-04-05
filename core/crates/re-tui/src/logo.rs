//! Logo rendering for the TUI startup banner.
//!
//! Hand-crafted Unicode representation of the Ralph Engine orbit logo.
//! Uses ratatui Spans with theme colors for consistent rendering
//! across all terminals — no image protocol dependencies.

use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::theme::Theme;

/// Builds the startup logo as styled ratatui Lines.
///
/// Responsive: returns a compact version if `width` < 60.
/// The `logo_color` overrides the accent for the orbit graphic,
/// enabling the k9s pattern where the logo reflects session health:
/// idle = `text_dim`, running = `accent`, error = `error`.
#[must_use]
pub fn build_logo_lines(
    width: u16,
    theme: &dyn Theme,
    logo_color: Option<ratatui::style::Color>,
    tagline: &str,
) -> Vec<Line<'static>> {
    let color = logo_color.unwrap_or_else(|| theme.accent());
    if width < 60 {
        build_compact_logo(theme, color)
    } else {
        build_full_logo(theme, color, tagline)
    }
}

/// Full logo with orbit icon + text (for terminals >= 60 cols).
fn build_full_logo(
    theme: &dyn Theme,
    color: ratatui::style::Color,
    tagline: &str,
) -> Vec<Line<'static>> {
    let b = Style::default().fg(color);
    let bb = Style::default().fg(color).add_modifier(Modifier::BOLD);
    let w = Style::default()
        .fg(theme.text_bright())
        .add_modifier(Modifier::BOLD);
    let d = Style::default().fg(theme.text_dim());

    vec![
        Line::from(""),
        Line::from(vec![Span::styled("      ╭───╮ ", b), Span::styled("●", bb)]),
        Line::from(vec![
            Span::styled("    ╭╯", b),
            Span::styled("     ╰╮", b),
            Span::styled("    Ralph ", w),
            Span::styled("Engine", bb),
        ]),
        Line::from(vec![
            Span::styled("    │", b),
            Span::styled("  ◉  ", bb),
            Span::styled(" │", b),
        ]),
        Line::from(vec![
            Span::styled("    ╰╮", b),
            Span::styled("     ╭╯", b),
            Span::styled(format!("    {tagline}"), d),
        ]),
        Line::from(vec![Span::styled("  ●", bb), Span::styled("  ╰───╯", b)]),
        Line::from(""),
    ]
}

/// Compact logo for narrow terminals (< 60 cols).
fn build_compact_logo(theme: &dyn Theme, color: ratatui::style::Color) -> Vec<Line<'static>> {
    let bb = Style::default().fg(color).add_modifier(Modifier::BOLD);
    let w = Style::default()
        .fg(theme.text_bright())
        .add_modifier(Modifier::BOLD);

    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ◎ ", bb),
            Span::styled("Ralph ", w),
            Span::styled("Engine", bb),
        ]),
        Line::from(""),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::CatppuccinMocha;

    #[test]
    fn full_logo_has_lines() {
        let lines = build_logo_lines(80, &CatppuccinMocha, None, "Test");
        assert!(lines.len() >= 5, "full logo should have 5+ lines");
    }

    #[test]
    fn compact_logo_has_lines() {
        let lines = build_logo_lines(50, &CatppuccinMocha, None, "Test");
        assert!(lines.len() >= 2, "compact logo should have 2+ lines");
    }

    #[test]
    fn full_logo_contains_brand_name() {
        let lines = build_logo_lines(80, &CatppuccinMocha, None, "Test");
        let text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect();
        assert!(text.contains("Ralph"), "logo should contain Ralph");
        assert!(text.contains("Engine"), "logo should contain Engine");
    }

    #[test]
    fn compact_logo_contains_brand_name() {
        let lines = build_logo_lines(50, &CatppuccinMocha, None, "Test");
        let text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect();
        assert!(text.contains("Ralph"));
    }

    #[test]
    fn logo_responsive_threshold() {
        let t = CatppuccinMocha;
        let full = build_logo_lines(60, &t, None, "Test");
        let compact = build_logo_lines(59, &t, None, "Test");
        assert!(full.len() > compact.len());
    }
}
