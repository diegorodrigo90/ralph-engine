//! Clipboard support for copying block content.
//!
//! Uses `arboard` for native OS clipboard access. When arboard fails
//! (headless, SSH), falls back to OSC 52 escape sequence which works
//! in most modern terminals including over SSH.

use base64::Engine;

/// Copies text to the system clipboard.
///
/// Strategy: try arboard (native OS clipboard) first. If that fails
/// (SSH, headless, Wayland without wl-copy), fall back to OSC 52
/// terminal escape sequence.
///
/// Returns `"clipboard"` if native worked, `"terminal"` if OSC 52
/// was used, or empty string on total failure.
pub fn copy_to_clipboard(text: &str) -> bool {
    // Try native clipboard first
    if let Ok(mut clipboard) = arboard::Clipboard::new()
        && clipboard.set_text(text).is_ok()
    {
        tracing::debug!("copied {} bytes via native clipboard", text.len());
        return true;
    }

    // Fallback: OSC 52 (terminal clipboard escape sequence)
    if copy_via_osc52(text) {
        tracing::debug!("copied {} bytes via OSC 52", text.len());
        return true;
    }

    tracing::warn!("clipboard unavailable (native and OSC 52 both failed)");
    false
}

/// Copies text via OSC 52 escape sequence.
///
/// OSC 52 sets the terminal's clipboard directly via an escape sequence:
/// `\x1b]52;c;<base64-encoded-text>\x07`
///
/// Works in: kitty, alacritty, iTerm2, `WezTerm`, tmux (`set-clipboard` on),
/// Windows Terminal, most modern terminals. Works over SSH.
fn copy_via_osc52(text: &str) -> bool {
    let encoded = base64::engine::general_purpose::STANDARD.encode(text.as_bytes());
    let sequence = format!("\x1b]52;c;{encoded}\x07");

    // Write directly to stdout (terminal receives the escape sequence)
    use std::io::Write;
    std::io::stdout().write_all(sequence.as_bytes()).is_ok() && std::io::stdout().flush().is_ok()
}

/// Extracts clean copyable text from a feed block.
///
/// Joins the block title and content lines with newlines,
/// stripping any decorative prefixes (focus indicators, icons).
pub fn block_to_copyable_text(title: &str, content: &[String]) -> String {
    let mut result = String::new();
    if !title.is_empty() {
        result.push_str(title);
        result.push('\n');
    }
    for line in content {
        result.push_str(line);
        result.push('\n');
    }
    // Trim trailing newline
    if result.ends_with('\n') {
        result.pop();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_to_text_title_and_content() {
        let text = block_to_copyable_text("file.rs", &["+ added".into(), "- removed".into()]);
        assert_eq!(text, "file.rs\n+ added\n- removed");
    }

    #[test]
    fn block_to_text_empty_title() {
        let text = block_to_copyable_text("", &["line 1".into(), "line 2".into()]);
        assert_eq!(text, "line 1\nline 2");
    }

    #[test]
    fn block_to_text_no_content() {
        let text = block_to_copyable_text("just a title", &[]);
        assert_eq!(text, "just a title");
    }

    #[test]
    fn block_to_text_empty() {
        let text = block_to_copyable_text("", &[]);
        assert_eq!(text, "");
    }
}
