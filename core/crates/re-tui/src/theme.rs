//! Theme system — re-exports from `ratatui-themekit`.
//!
//! Ralph Engine uses `ratatui-themekit` for all color management.
//! This module re-exports the theme trait, built-in themes, and
//! resolution helpers, plus adds RE-specific spacing constants.

// Re-export everything from ratatui-themekit
pub use ratatui_themekit::{
    BUILTIN_THEMES,
    // Themes (PascalCase aliases)
    CatppuccinMocha,
    CustomTheme,
    Dracula,
    // Widget style bundles
    GaugeStyles,
    GruvboxDark,
    InputStyles,
    ListStyles,
    NoColor,
    Nord,
    NotificationStyles,
    OneDark,
    RosePine,
    ScrollbarStyles,
    SolarizedDark,
    StateStyles,
    TabStyles,
    TableStyles,
    TailwindDark,
    TerminalNative,
    // Core traits
    Theme,
    ThemeData,
    ThemeExt,
    // Builders
    ThemedBar,
    ThemedBlock,
    ThemedLine,
    ThemedSpan,
    ThemedStatusLine,
    TokyoNight,
    // Resolution
    available_theme_ids,
    builtin_themes,
    default_theme,
    no_color_active,
    resolve_theme,
    // Utilities
    zebra_rows,
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
    fn resolve_tokyo_night() {
        let theme = resolve_theme("tokyo-night");
        assert_eq!(theme.id(), "tokyo-night");
    }

    #[test]
    fn resolve_rose_pine() {
        let theme = resolve_theme("rose-pine");
        assert_eq!(theme.id(), "rose-pine");
    }

    #[test]
    fn tokyo_night_colors_are_distinct() {
        let t = TokyoNight;
        assert_ne!(t.success(), t.error());
        assert_ne!(t.diff_added(), t.diff_removed());
    }

    #[test]
    fn rose_pine_colors_are_distinct() {
        let t = RosePine;
        assert_ne!(t.success(), t.error());
        assert_ne!(t.diff_added(), t.diff_removed());
    }

    #[test]
    fn builtin_themes_has_all_entries() {
        assert!(BUILTIN_THEMES.len() >= 10);
    }

    #[test]
    fn default_theme_is_catppuccin() {
        assert_eq!(default_theme().id(), "catppuccin");
    }

    #[test]
    fn spacing_constants_are_reasonable() {
        assert_eq!(BLOCK_PADDING, 1);
        assert_eq!(INDENT_WIDTH, 2);
        assert_eq!(MAX_COLLAPSED_LINES, 20);
    }
}
