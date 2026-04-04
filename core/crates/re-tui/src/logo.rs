//! Logo rendering for the TUI startup banner.
//!
//! Embeds the Ralph Engine logo as a PNG and renders it using
//! `ratatui-image` with automatic protocol detection:
//! Kitty > Sixel > iTerm2 > Unicode halfblocks.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;

/// Embedded logo PNG (dark theme — light text on transparent background).
static LOGO_PNG: &[u8] = include_bytes!("../assets/logo-dark.png");

/// Pre-built logo state for rendering across frames.
pub struct LogoState {
    /// The protocol-specific image data.
    pub protocol: StatefulProtocol,
}

/// Attempts to create a renderable logo image.
///
/// Returns `None` if the terminal does not support image rendering
/// or the PNG cannot be decoded. This is best-effort — the TUI
/// works fine without the logo.
#[cfg_attr(coverage_nightly, coverage(off))]
#[must_use]
pub fn create_logo() -> Option<LogoState> {
    let dyn_image = image::load_from_memory(LOGO_PNG).ok()?;
    let picker = Picker::from_query_stdio().ok()?;
    let protocol = picker.new_resize_protocol(dyn_image);
    Some(LogoState { protocol })
}

/// Renders the logo image into the given area.
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn render_logo(frame: &mut Frame<'_>, area: Rect, state: &mut LogoState) {
    let image_widget =
        ratatui_image::StatefulImage::default().resize(ratatui_image::Resize::Fit(None));
    frame.render_stateful_widget(image_widget, area, &mut state.protocol);
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    #[test]
    fn logo_png_is_valid() {
        let img = image::load_from_memory(super::LOGO_PNG);
        assert!(img.is_ok(), "embedded logo PNG should be decodable");
        let img = img.unwrap();
        assert!(img.width() > 0);
        assert!(img.height() > 0);
    }
}
