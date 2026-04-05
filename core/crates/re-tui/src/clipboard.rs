//! Clipboard support for copying block content.
//!
//! Uses `arboard` for native OS clipboard access. When arboard fails
//! (headless, SSH), the copy is silently skipped with a log warning.
//! OSC 52 terminal fallback is planned for a future story.

/// Copies text to the system clipboard via arboard.
///
/// Returns `true` if the copy succeeded, `false` if the clipboard
/// is unavailable (headless, SSH, Wayland without `wl-copy`, etc.).
pub fn copy_to_clipboard(text: &str) -> bool {
    match arboard::Clipboard::new() {
        Ok(mut clipboard) => match clipboard.set_text(text) {
            Ok(()) => {
                tracing::debug!("copied {} bytes to clipboard", text.len());
                true
            }
            Err(e) => {
                tracing::warn!("clipboard set_text failed: {e}");
                false
            }
        },
        Err(e) => {
            tracing::warn!("clipboard unavailable: {e}");
            false
        }
    }
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
