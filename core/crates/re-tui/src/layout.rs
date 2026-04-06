//! Responsive zone-based layout engine.
//!
//! Computes layout zones based on terminal dimensions. Three tiers:
//!
//! - **Compact** (< 120 cols): single column — header, activity, metrics, help.
//! - **Standard** (120..160 cols): two columns — activity + sidebar.
//! - **Wide** (>= 160 cols): three columns — control + activity + sidebar (guided mode).
//!
//! The layout tier is recomputed on every frame, so terminal resizes
//! are handled automatically.

use ratatui::layout::{Constraint, Layout, Rect};

use crate::{MIN_TERMINAL_HEIGHT, MIN_TERMINAL_WIDTH, THREE_COLUMN_WIDTH, TWO_COLUMN_WIDTH};

/// Layout tier based on terminal width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutTier {
    /// Single column (< 120 cols). Activity only, no sidebar.
    Compact,
    /// Two columns (120..160 cols). Activity + sidebar.
    Standard,
    /// Three columns (>= 160 cols). Control + activity + sidebar.
    Wide,
}

impl LayoutTier {
    /// Determines the layout tier from terminal dimensions.
    #[must_use]
    pub fn from_size(width: u16, _height: u16) -> Self {
        if width >= THREE_COLUMN_WIDTH {
            Self::Wide
        } else if width >= TWO_COLUMN_WIDTH {
            Self::Standard
        } else {
            Self::Compact
        }
    }

    /// Whether the sidebar zone is visible in this tier.
    #[must_use]
    pub fn has_sidebar(self) -> bool {
        matches!(self, Self::Standard | Self::Wide)
    }

    /// Whether the control panel zone is visible (guided mode, wide only).
    #[must_use]
    pub fn has_control_panel(self) -> bool {
        matches!(self, Self::Wide)
    }
}

/// Computed layout zones for one frame.
///
/// All fields are `Option<Rect>` because zones may not be visible
/// depending on the layout tier.
#[derive(Debug, Clone)]
pub struct LayoutZones {
    /// Header bar (always visible, 1 line).
    pub header: Rect,
    /// Tab bar (Standard + Wide tiers, 1 line). Hidden in Compact.
    pub tab_bar: Option<Rect>,
    /// Main activity stream (always visible, fills remaining space).
    pub activity: Rect,
    /// Metrics bar (always visible, 1 line).
    pub metrics: Rect,
    /// Chat input bar (visible when interactive plugin is enabled, 1 line).
    pub input: Option<Rect>,
    /// Help/keybinding bar (always visible, 1 line).
    pub help: Rect,
    /// Sidebar for plugin panels (Standard + Wide tiers).
    pub sidebar: Option<Rect>,
    /// Control panel for guided mode (Wide tier only).
    pub control: Option<Rect>,
    /// The computed layout tier.
    pub tier: LayoutTier,
}

/// Computes layout zones for the given terminal area.
///
/// The `has_input_bar` flag is set when an interactive plugin (e.g.
/// `official.guided`) registers keybindings — it enables the chat
/// input row. Without it, the TUI is a read-only dashboard.
///
/// Pure function — no side effects, easy to test.
#[must_use]
pub fn compute_zones(area: Rect, has_input_bar: bool) -> LayoutZones {
    let tier = LayoutTier::from_size(area.width, area.height);

    let has_tabs = tier.has_sidebar(); // Standard + Wide get tab bar

    let (header, tab_bar, body, metrics, input, help) = match (has_tabs, has_input_bar) {
        (true, true) => {
            let rows = Layout::vertical([
                Constraint::Length(1), // header
                Constraint::Length(1), // tab bar
                Constraint::Fill(1),   // body
                Constraint::Length(1), // metrics
                Constraint::Length(4), // input
                Constraint::Length(1), // help
            ])
            .split(area);
            (
                rows[0],
                Some(rows[1]),
                rows[2],
                rows[3],
                Some(rows[4]),
                rows[5],
            )
        }
        (true, false) => {
            let rows = Layout::vertical([
                Constraint::Length(1), // header
                Constraint::Length(1), // tab bar
                Constraint::Fill(1),   // body
                Constraint::Length(1), // metrics
                Constraint::Length(1), // help
            ])
            .split(area);
            (rows[0], Some(rows[1]), rows[2], rows[3], None, rows[4])
        }
        (false, true) => {
            let rows = Layout::vertical([
                Constraint::Length(1), // header
                Constraint::Fill(1),   // body
                Constraint::Length(1), // metrics
                Constraint::Length(4), // input
                Constraint::Length(1), // help
            ])
            .split(area);
            (rows[0], None, rows[1], rows[2], Some(rows[3]), rows[4])
        }
        (false, false) => {
            let rows = Layout::vertical([
                Constraint::Length(1), // header
                Constraint::Fill(1),   // body
                Constraint::Length(1), // metrics
                Constraint::Length(1), // help
            ])
            .split(area);
            (rows[0], None, rows[1], rows[2], None, rows[3])
        }
    };

    // Horizontal split of the body depends on tier
    let (control, activity, sidebar) = split_body(body, tier);

    LayoutZones {
        header,
        tab_bar,
        activity,
        metrics,
        input,
        help,
        sidebar,
        control,
        tier,
    }
}

/// Splits the body region into columns based on the layout tier.
fn split_body(body: Rect, tier: LayoutTier) -> (Option<Rect>, Rect, Option<Rect>) {
    match tier {
        LayoutTier::Compact => (None, body, None),
        LayoutTier::Standard => {
            let cols = Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(body);
            (None, cols[0], Some(cols[1]))
        }
        LayoutTier::Wide => {
            let cols = Layout::horizontal([
                Constraint::Percentage(18),
                Constraint::Percentage(52),
                Constraint::Percentage(30),
            ])
            .split(body);
            (Some(cols[0]), cols[1], Some(cols[2]))
        }
    }
}

/// Returns `true` if the terminal is too small for the TUI.
#[must_use]
pub fn is_terminal_too_small(area: Rect) -> bool {
    area.width < MIN_TERMINAL_WIDTH || area.height < MIN_TERMINAL_HEIGHT
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn rect(width: u16, height: u16) -> Rect {
        Rect::new(0, 0, width, height)
    }

    #[test]
    fn tier_compact_below_120() {
        assert_eq!(LayoutTier::from_size(80, 24), LayoutTier::Compact);
        assert_eq!(LayoutTier::from_size(119, 40), LayoutTier::Compact);
    }

    #[test]
    fn tier_standard_120_to_159() {
        assert_eq!(LayoutTier::from_size(120, 40), LayoutTier::Standard);
        assert_eq!(LayoutTier::from_size(159, 50), LayoutTier::Standard);
    }

    #[test]
    fn tier_wide_160_plus() {
        assert_eq!(LayoutTier::from_size(160, 50), LayoutTier::Wide);
        assert_eq!(LayoutTier::from_size(200, 60), LayoutTier::Wide);
    }

    #[test]
    fn tier_sidebar_visibility() {
        assert!(!LayoutTier::Compact.has_sidebar());
        assert!(LayoutTier::Standard.has_sidebar());
        assert!(LayoutTier::Wide.has_sidebar());
    }

    #[test]
    fn tier_control_panel_visibility() {
        assert!(!LayoutTier::Compact.has_control_panel());
        assert!(!LayoutTier::Standard.has_control_panel());
        assert!(LayoutTier::Wide.has_control_panel());
    }

    #[test]
    fn compact_zones_no_sidebar_no_control() {
        let zones = compute_zones(rect(80, 24), false);
        assert_eq!(zones.tier, LayoutTier::Compact);
        assert!(zones.sidebar.is_none());
        assert!(zones.control.is_none());
        assert!(zones.input.is_none());
        // Header + metrics + help = 3 lines, activity gets the rest
        assert_eq!(zones.header.height, 1);
        assert_eq!(zones.metrics.height, 1);
        assert_eq!(zones.help.height, 1);
        assert_eq!(zones.activity.height, 21); // 24 - 3
    }

    #[test]
    fn compact_zones_with_input_bar() {
        let zones = compute_zones(rect(80, 24), true);
        assert!(zones.input.is_some());
        assert_eq!(zones.input.unwrap().height, 4); // separator + 3 input lines
        assert_eq!(zones.activity.height, 17); // 24 - 7
    }

    #[test]
    fn standard_zones_has_sidebar() {
        let zones = compute_zones(rect(140, 40), false);
        assert_eq!(zones.tier, LayoutTier::Standard);
        assert!(zones.sidebar.is_some());
        assert!(zones.control.is_none());
        // Sidebar should be ~30% of width
        let sidebar = zones.sidebar.as_ref().unwrap();
        assert!(sidebar.width > 30, "sidebar too narrow: {}", sidebar.width);
        assert!(sidebar.width < 60, "sidebar too wide: {}", sidebar.width);
    }

    #[test]
    fn wide_zones_has_control_and_sidebar() {
        let zones = compute_zones(rect(200, 60), false);
        assert_eq!(zones.tier, LayoutTier::Wide);
        assert!(zones.sidebar.is_some());
        assert!(zones.control.is_some());
        let control = zones.control.as_ref().unwrap();
        let sidebar = zones.sidebar.as_ref().unwrap();
        // Control ~18%, sidebar ~30%
        assert!(control.width > 20, "control too narrow: {}", control.width);
        assert!(sidebar.width > 40, "sidebar too narrow: {}", sidebar.width);
    }

    #[test]
    fn zones_fixed_rows_always_one_line() {
        for (w, h) in [(80, 24), (140, 40), (200, 60)] {
            let zones = compute_zones(rect(w, h), false);
            assert_eq!(zones.header.height, 1, "header height for {w}x{h}");
            assert_eq!(zones.metrics.height, 1, "metrics height for {w}x{h}");
            assert_eq!(zones.help.height, 1, "help height for {w}x{h}");
        }
    }

    #[test]
    fn activity_fills_remaining_vertical_space() {
        let zones = compute_zones(rect(80, 30), false);
        // 30 - header(1) - metrics(1) - help(1) = 27
        assert_eq!(zones.activity.height, 27);
    }

    #[test]
    fn is_terminal_too_small_detects_narrow() {
        assert!(is_terminal_too_small(rect(79, 24)));
        assert!(!is_terminal_too_small(rect(80, 24)));
    }

    #[test]
    fn is_terminal_too_small_detects_short() {
        assert!(is_terminal_too_small(rect(80, 23)));
        assert!(!is_terminal_too_small(rect(80, 24)));
    }
}
