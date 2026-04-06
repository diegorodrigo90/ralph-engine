//! CLI output styling — ANSI colors for terminal output.
//!
//! Uses `owo-colors` with `supports-colors` feature for automatic
//! `NO_COLOR` detection. All CLI text surfaces (help, tables, status
//! markers, errors) use this module for consistent coloring.
//!
//! This is the CLI counterpart to `ratatui-themekit` (which owns TUI
//! rendering). CLI commands return styled `String`s; the TUI uses
//! ratatui spans with theme colors.

use owo_colors::OwoColorize;
use owo_colors::Stream::Stdout;

/// Product name, bold and cyan.
pub fn brand(text: &str) -> String {
    text.if_supports_color(Stdout, |t| t.cyan()).to_string()
}

/// Section heading (e.g., "Commands:", "Flags:").
pub fn heading(text: &str) -> String {
    text.if_supports_color(Stdout, |t| t.bold()).to_string()
}

/// Command name in the help listing.
pub fn command(text: &str) -> String {
    text.if_supports_color(Stdout, |t| t.green()).to_string()
}

/// Subcommand or flag argument placeholder.
pub fn arg(text: &str) -> String {
    text.if_supports_color(Stdout, |t| t.yellow()).to_string()
}

/// Dimmed description text.
pub fn dim(text: &str) -> String {
    text.if_supports_color(Stdout, |t| t.bright_black())
        .to_string()
}

/// Success status (e.g., [OK]).
pub fn success(text: &str) -> String {
    text.if_supports_color(Stdout, |t| t.green()).to_string()
}

/// Warning status (e.g., [NOT READY]).
pub fn warning(text: &str) -> String {
    text.if_supports_color(Stdout, |t| t.yellow()).to_string()
}

/// Error status (e.g., [MISSING]).
pub fn error(text: &str) -> String {
    text.if_supports_color(Stdout, |t| t.red()).to_string()
}

/// Table header cell (bold, uppercase is handled by caller).
pub fn table_header(text: &str) -> String {
    text.if_supports_color(Stdout, |t| t.bold()).to_string()
}

/// Key in a key-value detail line.
pub fn detail_key(text: &str) -> String {
    text.if_supports_color(Stdout, |t| t.cyan()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brand_contains_text() {
        assert!(brand("RE").contains("RE"));
    }

    #[test]
    fn heading_contains_text() {
        assert!(heading("Commands:").contains("Commands:"));
    }

    #[test]
    fn command_contains_text() {
        assert!(command("run").contains("run"));
    }

    #[test]
    fn arg_contains_text() {
        assert!(arg("--help").contains("--help"));
    }

    #[test]
    fn dim_contains_text() {
        assert!(dim("description").contains("description"));
    }

    #[test]
    fn success_contains_text() {
        assert!(success("[OK]").contains("[OK]"));
    }

    #[test]
    fn warning_contains_text() {
        assert!(warning("[NOT READY]").contains("[NOT READY]"));
    }

    #[test]
    fn error_contains_text() {
        assert!(error("[MISSING]").contains("[MISSING]"));
    }

    #[test]
    fn table_header_contains_text() {
        assert!(table_header("NAME").contains("NAME"));
    }

    #[test]
    fn detail_key_contains_text() {
        assert!(detail_key("Plugin:").contains("Plugin:"));
    }
}
