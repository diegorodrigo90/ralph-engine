//! Theme system — re-exports from `ratatui-themekit`.
//!
//! Ralph Engine uses `ratatui-themekit` for all color management.
//! This module re-exports the theme trait, built-in themes, and
//! resolution helpers, plus adds RE-specific spacing constants.

// Re-export everything from ratatui-themekit
pub use ratatui_themekit::{
    CatppuccinMocha, CustomTheme, Dracula, GruvboxDark, NoColor, Nord, OneDark, SolarizedDark,
    TailwindDark, TerminalNative, Theme, ThemeExt, available_theme_ids, builtin_themes,
    no_color_active, resolve_theme,
};

// ── RE-specific spacing constants ─────────────────────────────────

/// Padding lines between blocks in the feed.
pub const BLOCK_PADDING: u16 = 1;

/// Indent width for nested content (spaces).
pub const INDENT_WIDTH: u16 = 2;

/// Maximum content lines shown before truncation with expand hint.
pub const MAX_COLLAPSED_LINES: usize = 20;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_known_theme() {
        let theme = resolve_theme("dracula");
        assert_eq!(theme.id(), "dracula");
    }

    #[test]
    fn resolve_unknown_falls_back() {
        let theme = resolve_theme("nonexistent");
        assert_eq!(theme.id(), "catppuccin");
    }

    #[test]
    fn catppuccin_colors_are_distinct() {
        let t = CatppuccinMocha;
        assert_ne!(t.success(), t.error());
        assert_ne!(t.diff_added(), t.diff_removed());
    }

    #[test]
    fn no_color_is_all_reset() {
        let t = NoColor;
        assert_eq!(t.accent(), ratatui::style::Color::Reset);
        assert_eq!(t.error(), ratatui::style::Color::Reset);
    }

    #[test]
    fn derived_methods_delegate() {
        let t = CatppuccinMocha;
        assert_eq!(t.block_pass(), t.success());
        assert_eq!(t.block_fail(), t.error());
        assert_eq!(t.indicator_passed(), t.success());
    }

    #[test]
    fn available_ids_match_builtins() {
        assert_eq!(available_theme_ids().len(), builtin_themes().len());
    }

    #[test]
    fn spacing_constants_are_reasonable() {
        assert_eq!(BLOCK_PADDING, 1);
        assert_eq!(INDENT_WIDTH, 2);
        assert_eq!(MAX_COLLAPSED_LINES, 20);
    }
}
