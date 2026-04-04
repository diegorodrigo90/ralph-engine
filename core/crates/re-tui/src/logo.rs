//! Logo rendering for the TUI startup banner.
//!
//! Hand-crafted Unicode representation of the Ralph Engine orbit logo.
//! Uses ratatui Spans with brand colors for consistent rendering
//! across all terminals — no image protocol dependencies.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

/// Brand purple color (`#5B6AD0` → ANSI 256 index 105).
const BRAND: Color = Color::Indexed(105);

/// Light text color for dark terminals.
const TEXT_LIGHT: Color = Color::Indexed(252);

/// Dim text color.
const TEXT_DIM: Color = Color::Indexed(245);

/// Builds the startup logo as styled ratatui Lines.
///
/// Responsive: returns a compact version if `width` < 60.
#[must_use]
pub fn build_logo_lines(width: u16) -> Vec<Line<'static>> {
    if width < 60 {
        build_compact_logo()
    } else {
        build_full_logo()
    }
}

/// Full logo with orbit icon + text (for terminals >= 60 cols).
fn build_full_logo() -> Vec<Line<'static>> {
    let b = Style::default().fg(BRAND);
    let bb = Style::default().fg(BRAND).add_modifier(Modifier::BOLD);
    let w = Style::default().fg(TEXT_LIGHT).add_modifier(Modifier::BOLD);
    let d = Style::default().fg(TEXT_DIM);

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
            Span::styled("    Autonomous AI Dev Loop", d),
        ]),
        Line::from(vec![Span::styled("  ●", bb), Span::styled("  ╰───╯", b)]),
        Line::from(""),
    ]
}

/// Compact logo for narrow terminals (< 60 cols).
fn build_compact_logo() -> Vec<Line<'static>> {
    let bb = Style::default().fg(BRAND).add_modifier(Modifier::BOLD);
    let w = Style::default().fg(TEXT_LIGHT).add_modifier(Modifier::BOLD);

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

    #[test]
    fn full_logo_has_lines() {
        let lines = build_logo_lines(80);
        assert!(lines.len() >= 5, "full logo should have 5+ lines");
    }

    #[test]
    fn compact_logo_has_lines() {
        let lines = build_logo_lines(50);
        assert!(lines.len() >= 2, "compact logo should have 2+ lines");
    }

    #[test]
    fn full_logo_contains_brand_name() {
        let lines = build_logo_lines(80);
        let text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect();
        assert!(text.contains("Ralph"), "logo should contain Ralph");
        assert!(text.contains("Engine"), "logo should contain Engine");
    }

    #[test]
    fn compact_logo_contains_brand_name() {
        let lines = build_logo_lines(50);
        let text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect();
        assert!(text.contains("Ralph"));
    }

    #[test]
    fn logo_responsive_threshold() {
        let full = build_logo_lines(60);
        let compact = build_logo_lines(59);
        assert!(full.len() > compact.len());
    }
}
