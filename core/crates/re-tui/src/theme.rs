//! Design system for the orchestration TUI.
//!
//! Defines a [`Theme`] trait with semantic color slots and spacing
//! constants. ALL render functions use theme slots — zero hardcoded
//! `Color::*` values in the codebase.
//!
//! Ships 6 built-in themes: Catppuccin Mocha, Dracula, Nord, Gruvbox
//! Dark, One Dark, and Solarized Dark. Users select via
//! `tui.theme: "catppuccin"` in config.yaml.

use ratatui::style::Color;

// ── Spacing constants ──────────────────────────────────────────────

/// Padding lines between blocks in the feed.
pub const BLOCK_PADDING: u16 = 1;

/// Indent width for nested content (spaces).
pub const INDENT_WIDTH: u16 = 2;

/// Maximum content lines shown before truncation with `[expand]`.
pub const MAX_COLLAPSED_LINES: usize = 20;

// ── Theme trait ────────────────────────────────────────────────────

/// Semantic color contract for the TUI.
///
/// Every render function receives a `&dyn Theme` and maps visual
/// elements to these slots. Themes provide the concrete colors.
///
/// **Model B:** the theme is core infrastructure. Plugins render
/// through semantic slots, never hardcode colors.
pub trait Theme: Send + Sync {
    /// Human-readable theme name (e.g. `"Catppuccin Mocha"`).
    fn name(&self) -> &str;

    /// Short identifier for config files (e.g. `"catppuccin"`).
    fn id(&self) -> &str;

    // ── Brand ──────────────────────────────────────────────────

    /// Primary brand/accent color (header, logo, prompts).
    fn accent(&self) -> Color;

    /// Secondary accent (less prominent highlights).
    fn accent_dim(&self) -> Color;

    // ── Text ───────────────────────────────────────────────────

    /// Default text color.
    fn text(&self) -> Color;

    /// Dimmed/muted text (timestamps, hints, inactive elements).
    fn text_dim(&self) -> Color;

    /// Bright text for emphasis (bold titles, active elements).
    fn text_bright(&self) -> Color;

    // ── Status ─────────────────────────────────────────────────

    /// Success / passed / running indicator.
    fn success(&self) -> Color;

    /// Error / failed indicator.
    fn error(&self) -> Color;

    /// Warning / pending / in-progress indicator.
    fn warning(&self) -> Color;

    /// Informational / neutral status.
    fn info(&self) -> Color;

    // ── Diff ───────────────────────────────────────────────────

    /// Lines added (entering the codebase).
    fn diff_added(&self) -> Color;

    /// Lines removed (leaving the codebase).
    fn diff_removed(&self) -> Color;

    /// Context/unchanged diff lines.
    fn diff_context(&self) -> Color;

    // ── Structure ──────────────────────────────────────────────

    /// Border/separator color.
    fn border(&self) -> Color;

    /// Background highlight for focused/selected blocks.
    fn surface(&self) -> Color;

    // ── Block kinds ────────────────────────────────────────────

    /// Color for file-read block icons.
    fn block_file_read(&self) -> Color {
        self.text_dim()
    }

    /// Color for file-edit block icons.
    fn block_file_edit(&self) -> Color {
        self.diff_added()
    }

    /// Color for command/shell block icons.
    fn block_command(&self) -> Color {
        self.text_bright()
    }

    /// Color for thinking/reasoning block icons.
    fn block_thinking(&self) -> Color {
        self.text_dim()
    }

    /// Color for passed-indicator block icons.
    fn block_pass(&self) -> Color {
        self.success()
    }

    /// Color for failed-indicator block icons.
    fn block_fail(&self) -> Color {
        self.error()
    }

    /// Color for system message block icons.
    fn block_system(&self) -> Color {
        self.text_dim()
    }

    // ── Indicator states ───────────────────────────────────────

    /// Pending indicator color.
    fn indicator_pending(&self) -> Color {
        self.text_dim()
    }

    /// Running indicator color.
    fn indicator_running(&self) -> Color {
        self.warning()
    }

    /// Passed indicator color.
    fn indicator_passed(&self) -> Color {
        self.success()
    }

    /// Failed indicator color.
    fn indicator_failed(&self) -> Color {
        self.error()
    }

    /// Skipped indicator color.
    fn indicator_skipped(&self) -> Color {
        self.text_dim()
    }
}

// ── Built-in themes ────────────────────────────────────────────────

/// Catppuccin Mocha — warm dark theme with pastel colors.
pub struct CatppuccinMocha;

impl Theme for CatppuccinMocha {
    fn name(&self) -> &str {
        "Catppuccin Mocha"
    }
    fn id(&self) -> &str {
        "catppuccin"
    }
    fn accent(&self) -> Color {
        Color::Rgb(137, 180, 250) // Blue
    }
    fn accent_dim(&self) -> Color {
        Color::Rgb(108, 112, 134) // Overlay0
    }
    fn text(&self) -> Color {
        Color::Rgb(205, 214, 244) // Text
    }
    fn text_dim(&self) -> Color {
        Color::Rgb(127, 132, 156) // Overlay1 — dimmed but readable
    }
    fn text_bright(&self) -> Color {
        Color::Rgb(245, 224, 220) // Rosewater
    }
    fn success(&self) -> Color {
        Color::Rgb(166, 227, 161) // Green
    }
    fn error(&self) -> Color {
        Color::Rgb(243, 139, 168) // Red
    }
    fn warning(&self) -> Color {
        Color::Rgb(249, 226, 175) // Yellow
    }
    fn info(&self) -> Color {
        Color::Rgb(137, 220, 235) // Teal
    }
    fn diff_added(&self) -> Color {
        Color::Rgb(166, 227, 161) // Green
    }
    fn diff_removed(&self) -> Color {
        Color::Rgb(243, 139, 168) // Red
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(127, 132, 156) // Overlay1
    }
    fn border(&self) -> Color {
        Color::Rgb(69, 71, 90) // Surface1
    }
    fn surface(&self) -> Color {
        Color::Rgb(49, 50, 68) // Surface0
    }
}

/// Dracula — dark theme with vivid colors.
pub struct Dracula;

impl Theme for Dracula {
    fn name(&self) -> &str {
        "Dracula"
    }
    fn id(&self) -> &str {
        "dracula"
    }
    fn accent(&self) -> Color {
        Color::Rgb(189, 147, 249) // Purple
    }
    fn accent_dim(&self) -> Color {
        Color::Rgb(98, 114, 164) // Comment
    }
    fn text(&self) -> Color {
        Color::Rgb(248, 248, 242) // Foreground
    }
    fn text_dim(&self) -> Color {
        Color::Rgb(98, 114, 164) // Comment
    }
    fn text_bright(&self) -> Color {
        Color::Rgb(255, 255, 255) // White
    }
    fn success(&self) -> Color {
        Color::Rgb(80, 250, 123) // Green
    }
    fn error(&self) -> Color {
        Color::Rgb(255, 85, 85) // Red
    }
    fn warning(&self) -> Color {
        Color::Rgb(241, 250, 140) // Yellow
    }
    fn info(&self) -> Color {
        Color::Rgb(139, 233, 253) // Cyan
    }
    fn diff_added(&self) -> Color {
        Color::Rgb(80, 250, 123) // Green
    }
    fn diff_removed(&self) -> Color {
        Color::Rgb(255, 85, 85) // Red
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(98, 114, 164) // Comment
    }
    fn border(&self) -> Color {
        Color::Rgb(68, 71, 90) // Current Line
    }
    fn surface(&self) -> Color {
        Color::Rgb(50, 52, 68) // Slightly lighter bg
    }
}

/// Nord — arctic blue-gray color palette.
pub struct Nord;

impl Theme for Nord {
    fn name(&self) -> &str {
        "Nord"
    }
    fn id(&self) -> &str {
        "nord"
    }
    fn accent(&self) -> Color {
        Color::Rgb(136, 192, 208) // Nord8 (frost)
    }
    fn accent_dim(&self) -> Color {
        Color::Rgb(76, 86, 106) // Nord3
    }
    fn text(&self) -> Color {
        Color::Rgb(216, 222, 233) // Nord4
    }
    fn text_dim(&self) -> Color {
        Color::Rgb(76, 86, 106) // Nord3
    }
    fn text_bright(&self) -> Color {
        Color::Rgb(236, 239, 244) // Nord6
    }
    fn success(&self) -> Color {
        Color::Rgb(163, 190, 140) // Nord14 (green)
    }
    fn error(&self) -> Color {
        Color::Rgb(191, 97, 106) // Nord11 (red)
    }
    fn warning(&self) -> Color {
        Color::Rgb(235, 203, 139) // Nord13 (yellow)
    }
    fn info(&self) -> Color {
        Color::Rgb(136, 192, 208) // Nord8
    }
    fn diff_added(&self) -> Color {
        Color::Rgb(163, 190, 140) // Nord14
    }
    fn diff_removed(&self) -> Color {
        Color::Rgb(191, 97, 106) // Nord11
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(76, 86, 106) // Nord3
    }
    fn border(&self) -> Color {
        Color::Rgb(59, 66, 82) // Nord1
    }
    fn surface(&self) -> Color {
        Color::Rgb(46, 52, 64) // Nord0
    }
}

/// Gruvbox Dark — retro warm dark theme.
pub struct GruvboxDark;

impl Theme for GruvboxDark {
    fn name(&self) -> &str {
        "Gruvbox Dark"
    }
    fn id(&self) -> &str {
        "gruvbox"
    }
    fn accent(&self) -> Color {
        Color::Rgb(215, 153, 33) // Bright yellow
    }
    fn accent_dim(&self) -> Color {
        Color::Rgb(124, 111, 100) // Gray
    }
    fn text(&self) -> Color {
        Color::Rgb(235, 219, 178) // Foreground
    }
    fn text_dim(&self) -> Color {
        Color::Rgb(146, 131, 116) // Gray
    }
    fn text_bright(&self) -> Color {
        Color::Rgb(251, 241, 199) // Foreground0
    }
    fn success(&self) -> Color {
        Color::Rgb(184, 187, 38) // Green
    }
    fn error(&self) -> Color {
        Color::Rgb(251, 73, 52) // Red
    }
    fn warning(&self) -> Color {
        Color::Rgb(250, 189, 47) // Yellow
    }
    fn info(&self) -> Color {
        Color::Rgb(131, 165, 152) // Aqua
    }
    fn diff_added(&self) -> Color {
        Color::Rgb(184, 187, 38) // Green
    }
    fn diff_removed(&self) -> Color {
        Color::Rgb(251, 73, 52) // Red
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(146, 131, 116) // Gray
    }
    fn border(&self) -> Color {
        Color::Rgb(80, 73, 69) // Bg2
    }
    fn surface(&self) -> Color {
        Color::Rgb(60, 56, 54) // Bg1
    }
}

/// One Dark — Atom's iconic dark theme.
pub struct OneDark;

impl Theme for OneDark {
    fn name(&self) -> &str {
        "One Dark"
    }
    fn id(&self) -> &str {
        "one-dark"
    }
    fn accent(&self) -> Color {
        Color::Rgb(97, 175, 239) // Blue
    }
    fn accent_dim(&self) -> Color {
        Color::Rgb(92, 99, 112) // Comment
    }
    fn text(&self) -> Color {
        Color::Rgb(171, 178, 191) // Foreground
    }
    fn text_dim(&self) -> Color {
        Color::Rgb(92, 99, 112) // Comment
    }
    fn text_bright(&self) -> Color {
        Color::Rgb(220, 223, 228) // White
    }
    fn success(&self) -> Color {
        Color::Rgb(152, 195, 121) // Green
    }
    fn error(&self) -> Color {
        Color::Rgb(224, 108, 117) // Red
    }
    fn warning(&self) -> Color {
        Color::Rgb(229, 192, 123) // Yellow
    }
    fn info(&self) -> Color {
        Color::Rgb(86, 182, 194) // Cyan
    }
    fn diff_added(&self) -> Color {
        Color::Rgb(152, 195, 121) // Green
    }
    fn diff_removed(&self) -> Color {
        Color::Rgb(224, 108, 117) // Red
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(92, 99, 112) // Comment
    }
    fn border(&self) -> Color {
        Color::Rgb(62, 68, 81) // Gutter
    }
    fn surface(&self) -> Color {
        Color::Rgb(44, 49, 58) // Selection
    }
}

/// Solarized Dark — Ethan Schoonover's precision colors.
pub struct SolarizedDark;

impl Theme for SolarizedDark {
    fn name(&self) -> &str {
        "Solarized Dark"
    }
    fn id(&self) -> &str {
        "solarized"
    }
    fn accent(&self) -> Color {
        Color::Rgb(38, 139, 210) // Blue
    }
    fn accent_dim(&self) -> Color {
        Color::Rgb(88, 110, 117) // Base01
    }
    fn text(&self) -> Color {
        Color::Rgb(147, 161, 161) // Base1
    }
    fn text_dim(&self) -> Color {
        Color::Rgb(88, 110, 117) // Base01
    }
    fn text_bright(&self) -> Color {
        Color::Rgb(238, 232, 213) // Base2
    }
    fn success(&self) -> Color {
        Color::Rgb(133, 153, 0) // Green
    }
    fn error(&self) -> Color {
        Color::Rgb(220, 50, 47) // Red
    }
    fn warning(&self) -> Color {
        Color::Rgb(181, 137, 0) // Yellow
    }
    fn info(&self) -> Color {
        Color::Rgb(42, 161, 152) // Cyan
    }
    fn diff_added(&self) -> Color {
        Color::Rgb(133, 153, 0) // Green
    }
    fn diff_removed(&self) -> Color {
        Color::Rgb(220, 50, 47) // Red
    }
    fn diff_context(&self) -> Color {
        Color::Rgb(88, 110, 117) // Base01
    }
    fn border(&self) -> Color {
        Color::Rgb(7, 54, 66) // Base02
    }
    fn surface(&self) -> Color {
        Color::Rgb(0, 43, 54) // Base03
    }
}

// ── Theme registry ─────────────────────────────────────────────────

/// Returns all built-in themes.
#[must_use]
pub fn builtin_themes() -> Vec<Box<dyn Theme>> {
    vec![
        Box::new(CatppuccinMocha),
        Box::new(Dracula),
        Box::new(Nord),
        Box::new(GruvboxDark),
        Box::new(OneDark),
        Box::new(SolarizedDark),
    ]
}

/// Resolves a theme by its config ID, defaulting to Catppuccin Mocha.
#[must_use]
pub fn resolve_theme(id: &str) -> Box<dyn Theme> {
    match id {
        "catppuccin" => Box::new(CatppuccinMocha),
        "dracula" => Box::new(Dracula),
        "nord" => Box::new(Nord),
        "gruvbox" => Box::new(GruvboxDark),
        "one-dark" => Box::new(OneDark),
        "solarized" => Box::new(SolarizedDark),
        _ => Box::new(CatppuccinMocha), // safe default
    }
}

/// Returns a list of available theme IDs for config validation.
#[must_use]
pub fn available_theme_ids() -> Vec<&'static str> {
    vec![
        "catppuccin",
        "dracula",
        "nord",
        "gruvbox",
        "one-dark",
        "solarized",
    ]
}

// ── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_builtin_themes_have_unique_ids() {
        let themes = builtin_themes();
        let mut ids: Vec<&str> = themes.iter().map(|t| t.id()).collect();
        let original_len = ids.len();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), original_len, "theme IDs must be unique");
    }

    #[test]
    fn all_builtin_themes_have_names() {
        for theme in builtin_themes() {
            assert!(!theme.name().is_empty(), "theme must have a name");
            assert!(!theme.id().is_empty(), "theme must have an ID");
        }
    }

    #[test]
    fn resolve_theme_returns_default_for_unknown() {
        let theme = resolve_theme("nonexistent");
        assert_eq!(theme.id(), "catppuccin");
    }

    #[test]
    fn resolve_theme_finds_each_builtin() {
        for id in available_theme_ids() {
            let theme = resolve_theme(id);
            assert_eq!(theme.id(), id);
        }
    }

    #[test]
    fn catppuccin_colors_are_distinct() {
        let t = CatppuccinMocha;
        // Core semantic colors should differ from each other
        assert_ne!(t.success(), t.error(), "success and error must differ");
        assert_ne!(t.diff_added(), t.diff_removed(), "add/remove must differ");
        assert_ne!(t.text(), t.text_dim(), "text and dim must differ");
        assert_ne!(t.accent(), t.border(), "accent and border must differ");
    }

    #[test]
    fn dracula_colors_are_distinct() {
        let t = Dracula;
        assert_ne!(t.success(), t.error());
        assert_ne!(t.diff_added(), t.diff_removed());
    }

    #[test]
    fn nord_colors_are_distinct() {
        let t = Nord;
        assert_ne!(t.success(), t.error());
        assert_ne!(t.diff_added(), t.diff_removed());
    }

    #[test]
    fn gruvbox_colors_are_distinct() {
        let t = GruvboxDark;
        assert_ne!(t.success(), t.error());
        assert_ne!(t.diff_added(), t.diff_removed());
    }

    #[test]
    fn one_dark_colors_are_distinct() {
        let t = OneDark;
        assert_ne!(t.success(), t.error());
        assert_ne!(t.diff_added(), t.diff_removed());
    }

    #[test]
    fn solarized_colors_are_distinct() {
        let t = SolarizedDark;
        assert_ne!(t.success(), t.error());
        assert_ne!(t.diff_added(), t.diff_removed());
    }

    #[test]
    fn default_block_kind_colors_delegate_to_semantics() {
        let t = CatppuccinMocha;
        assert_eq!(t.block_file_read(), t.text_dim());
        assert_eq!(t.block_file_edit(), t.diff_added());
        assert_eq!(t.block_command(), t.text_bright());
        assert_eq!(t.block_thinking(), t.text_dim());
        assert_eq!(t.block_pass(), t.success());
        assert_eq!(t.block_fail(), t.error());
        assert_eq!(t.block_system(), t.text_dim());
    }

    #[test]
    fn default_indicator_colors_delegate_to_semantics() {
        let t = CatppuccinMocha;
        assert_eq!(t.indicator_pending(), t.text_dim());
        assert_eq!(t.indicator_running(), t.warning());
        assert_eq!(t.indicator_passed(), t.success());
        assert_eq!(t.indicator_failed(), t.error());
        assert_eq!(t.indicator_skipped(), t.text_dim());
    }

    #[test]
    fn available_theme_ids_matches_builtin_count() {
        assert_eq!(available_theme_ids().len(), builtin_themes().len());
    }

    #[test]
    fn spacing_constants_are_reasonable() {
        // Verify constants have expected values (not just existence).
        assert_eq!(BLOCK_PADDING, 1);
        assert_eq!(INDENT_WIDTH, 2);
        assert_eq!(MAX_COLLAPSED_LINES, 20);
    }
}
