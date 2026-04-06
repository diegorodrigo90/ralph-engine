//! Status indicator panel for the orchestration TUI.
//!
//! Indicators are generic status items with a lifecycle:
//! pending → running → passed/failed/skipped.
//!
//! **Model B:** Core renders indicators generically. Plugins declare
//! which indicators exist and update their state. Core never knows
//! what any indicator represents — it just renders the icon and label.

use ratatui::text::{Line, Span};
use ratatui_themekit::builders::ThemedSpan;

use crate::theme::Theme;

/// The current state of a status indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicatorState {
    /// Indicator has not started yet.
    Pending,
    /// Indicator is currently active.
    Running,
    /// Indicator completed successfully.
    Passed,
    /// Indicator failed.
    Failed,
    /// Indicator was skipped by user.
    Skipped,
}

impl IndicatorState {
    /// The icon for this indicator state.
    #[must_use]
    pub fn icon(self) -> &'static str {
        match self {
            Self::Pending => "○",
            Self::Running => "⏳",
            Self::Passed => "✓",
            Self::Failed => "✗",
            Self::Skipped => "⊘",
        }
    }

    /// Returns the color for this state from the active theme.
    #[must_use]
    pub fn color(self, theme: &dyn Theme) -> ratatui::style::Color {
        match self {
            Self::Pending => theme.indicator_pending(),
            Self::Running => theme.indicator_running(),
            Self::Passed => theme.indicator_passed(),
            Self::Failed => theme.indicator_failed(),
            Self::Skipped => theme.indicator_skipped(),
        }
    }
}

/// A single status indicator in the panel.
///
/// Plugins create indicators and update their state. Core renders
/// them without knowing what they represent.
#[derive(Debug, Clone)]
pub struct StatusIndicator {
    /// Short identifier declared by the plugin.
    pub id: String,
    /// Display label (may be localized by the plugin).
    pub label: String,
    /// Current state.
    pub state: IndicatorState,
    /// Optional detail message (shown when failed).
    pub detail: Option<String>,
}

impl StatusIndicator {
    /// Creates a new pending indicator.
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            state: IndicatorState::Pending,
            detail: None,
        }
    }

    /// Transitions to running.
    pub fn start(&mut self) {
        self.state = IndicatorState::Running;
        self.detail = None;
    }

    /// Transitions to passed.
    pub fn pass(&mut self) {
        self.state = IndicatorState::Passed;
        self.detail = None;
    }

    /// Transitions to failed with a detail message.
    pub fn fail(&mut self, detail: impl Into<String>) {
        self.state = IndicatorState::Failed;
        self.detail = Some(detail.into());
    }

    /// Transitions to skipped.
    pub fn skip(&mut self) {
        self.state = IndicatorState::Skipped;
        self.detail = None;
    }

    /// Whether this indicator is in a terminal state.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(
            self.state,
            IndicatorState::Passed | IndicatorState::Failed | IndicatorState::Skipped
        )
    }
}

/// An ordered panel of status indicators.
///
/// Plugins add indicators at setup time. The panel tracks their
/// state and provides rendering helpers for the status bar.
#[derive(Debug, Clone)]
pub struct IndicatorPanel {
    indicators: Vec<StatusIndicator>,
}

impl IndicatorPanel {
    /// Creates an empty panel.
    #[must_use]
    pub fn new() -> Self {
        Self {
            indicators: Vec::new(),
        }
    }

    /// Adds an indicator to the panel.
    ///
    /// Indicators are declared by plugins — core never decides
    /// which indicators exist.
    pub fn add(&mut self, indicator: StatusIndicator) {
        self.indicators.push(indicator);
    }

    /// Returns all indicators.
    #[must_use]
    pub fn indicators(&self) -> &[StatusIndicator] {
        &self.indicators
    }

    /// Finds an indicator by ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&StatusIndicator> {
        self.indicators.iter().find(|i| i.id == id)
    }

    /// Finds a mutable indicator by ID.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut StatusIndicator> {
        self.indicators.iter_mut().find(|i| i.id == id)
    }

    /// Starts an indicator by ID. Returns false if not found.
    pub fn start(&mut self, id: &str) -> bool {
        if let Some(ind) = self.get_mut(id) {
            ind.start();
            true
        } else {
            false
        }
    }

    /// Marks an indicator as passed. Returns false if not found.
    pub fn pass(&mut self, id: &str) -> bool {
        if let Some(ind) = self.get_mut(id) {
            ind.pass();
            true
        } else {
            false
        }
    }

    /// Marks an indicator as failed. Returns false if not found.
    pub fn fail(&mut self, id: &str, detail: impl Into<String>) -> bool {
        if let Some(ind) = self.get_mut(id) {
            ind.fail(detail);
            true
        } else {
            false
        }
    }

    /// Skips an indicator. Returns false if not found.
    pub fn skip(&mut self, id: &str) -> bool {
        if let Some(ind) = self.get_mut(id) {
            ind.skip();
            true
        } else {
            false
        }
    }

    /// Whether all indicators have completed.
    #[must_use]
    pub fn all_complete(&self) -> bool {
        !self.indicators.is_empty() && self.indicators.iter().all(StatusIndicator::is_complete)
    }

    /// Whether all indicators passed (or were skipped).
    #[must_use]
    pub fn all_passed(&self) -> bool {
        !self.indicators.is_empty()
            && self
                .indicators
                .iter()
                .all(|i| matches!(i.state, IndicatorState::Passed | IndicatorState::Skipped))
    }

    /// Whether any indicator has failed.
    #[must_use]
    pub fn has_failures(&self) -> bool {
        self.indicators
            .iter()
            .any(|i| i.state == IndicatorState::Failed)
    }

    /// Number of indicators.
    #[must_use]
    pub fn len(&self) -> usize {
        self.indicators.len()
    }

    /// Whether the panel is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.indicators.is_empty()
    }

    /// Renders the indicator bar as a styled ratatui `Line`.
    #[must_use]
    pub fn render_bar(&self, theme: &dyn Theme) -> Line<'static> {
        if self.indicators.is_empty() {
            return Line::raw("");
        }

        let mut spans: Vec<Span<'static>> = Vec::new();

        for (i, ind) in self.indicators.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw("  "));
            }
            let color = ind.state.color(theme);
            spans.push(
                ThemedSpan::with_color(format!("{}{}", ind.state.icon(), ind.label), color).build(),
            );
        }

        Line::from(spans)
    }
}

impl Default for IndicatorPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::theme::CatppuccinMocha;

    // ── IndicatorState ──

    #[test]
    fn indicator_state_icons() {
        assert_eq!(IndicatorState::Pending.icon(), "○");
        assert_eq!(IndicatorState::Running.icon(), "⏳");
        assert_eq!(IndicatorState::Passed.icon(), "✓");
        assert_eq!(IndicatorState::Failed.icon(), "✗");
        assert_eq!(IndicatorState::Skipped.icon(), "⊘");
    }

    #[test]
    fn indicator_state_colors_from_theme() {
        let t = CatppuccinMocha;
        assert_eq!(IndicatorState::Pending.color(&t), t.indicator_pending());
        assert_eq!(IndicatorState::Running.color(&t), t.indicator_running());
        assert_eq!(IndicatorState::Passed.color(&t), t.indicator_passed());
        assert_eq!(IndicatorState::Failed.color(&t), t.indicator_failed());
        assert_eq!(IndicatorState::Skipped.color(&t), t.indicator_skipped());
    }

    // ── StatusIndicator ──

    #[test]
    fn new_indicator_is_pending() {
        let ind = StatusIndicator::new("ind-a", "Indicator A");
        assert_eq!(ind.state, IndicatorState::Pending);
        assert!(ind.detail.is_none());
        assert!(!ind.is_complete());
    }

    #[test]
    fn indicator_lifecycle() {
        let mut ind = StatusIndicator::new("ind-b", "Indicator B");

        ind.start();
        assert_eq!(ind.state, IndicatorState::Running);
        assert!(!ind.is_complete());

        ind.pass();
        assert_eq!(ind.state, IndicatorState::Passed);
        assert!(ind.is_complete());
    }

    #[test]
    fn indicator_failure_stores_detail() {
        let mut ind = StatusIndicator::new("ind-a", "Indicator A");
        ind.fail("3 checks failed");
        assert_eq!(ind.state, IndicatorState::Failed);
        assert_eq!(ind.detail.as_deref(), Some("3 checks failed"));
        assert!(ind.is_complete());
    }

    #[test]
    fn indicator_skip_clears_detail() {
        let mut ind = StatusIndicator::new("ind-d", "Indicator D");
        ind.fail("timeout");
        ind.skip();
        assert_eq!(ind.state, IndicatorState::Skipped);
        assert!(ind.detail.is_none());
    }

    // ── IndicatorPanel ──

    fn test_panel() -> IndicatorPanel {
        let mut p = IndicatorPanel::new();
        p.add(StatusIndicator::new("ind-a", "Indicator A"));
        p.add(StatusIndicator::new("ind-b", "Indicator B"));
        p.add(StatusIndicator::new("ind-c", "Indicator C"));
        p.add(StatusIndicator::new("ind-d", "Indicator D"));
        p
    }

    #[test]
    fn empty_panel() {
        let panel = IndicatorPanel::new();
        assert!(panel.is_empty());
        assert_eq!(panel.len(), 0);
        assert!(!panel.all_complete());
        assert!(!panel.all_passed());
    }

    #[test]
    fn panel_maintains_insertion_order() {
        let panel = test_panel();
        assert_eq!(panel.len(), 4);
        assert_eq!(panel.indicators()[0].id, "ind-a");
        assert_eq!(panel.indicators()[1].id, "ind-b");
        assert_eq!(panel.indicators()[2].id, "ind-c");
        assert_eq!(panel.indicators()[3].id, "ind-d");
    }

    #[test]
    fn panel_start_pass_fail() {
        let mut panel = test_panel();

        assert!(panel.start("ind-a"));
        assert_eq!(panel.get("ind-a").unwrap().state, IndicatorState::Running);

        assert!(panel.pass("ind-a"));
        assert_eq!(panel.get("ind-a").unwrap().state, IndicatorState::Passed);

        assert!(panel.fail("ind-b", "exit code 1"));
        assert_eq!(panel.get("ind-b").unwrap().state, IndicatorState::Failed);
        assert_eq!(
            panel.get("ind-b").unwrap().detail.as_deref(),
            Some("exit code 1")
        );
    }

    #[test]
    fn panel_skip() {
        let mut panel = test_panel();
        assert!(panel.skip("ind-d"));
        assert_eq!(panel.get("ind-d").unwrap().state, IndicatorState::Skipped);
    }

    #[test]
    fn panel_unknown_returns_false() {
        let mut panel = test_panel();
        assert!(!panel.start("unknown"));
        assert!(!panel.pass("unknown"));
        assert!(!panel.fail("unknown", "err"));
        assert!(!panel.skip("unknown"));
    }

    #[test]
    fn all_complete_when_all_terminal() {
        let mut panel = test_panel();
        panel.pass("ind-a");
        panel.pass("ind-b");
        panel.pass("ind-c");
        panel.pass("ind-d");
        assert!(panel.all_complete());
        assert!(panel.all_passed());
        assert!(!panel.has_failures());
    }

    #[test]
    fn not_all_complete_when_pending() {
        let mut panel = test_panel();
        panel.pass("ind-a");
        panel.pass("ind-b");
        assert!(!panel.all_complete());
    }

    #[test]
    fn has_failures_detects_failed() {
        let mut panel = test_panel();
        panel.pass("ind-a");
        panel.fail("ind-b", "error");
        assert!(panel.has_failures());
        assert!(!panel.all_passed());
    }

    #[test]
    fn all_passed_includes_skipped() {
        let mut panel = test_panel();
        panel.pass("ind-a");
        panel.pass("ind-b");
        panel.pass("ind-c");
        panel.skip("ind-d");
        assert!(panel.all_passed());
    }

    #[test]
    fn add_extends_panel() {
        let mut panel = test_panel();
        panel.add(StatusIndicator::new("ind-e", "Indicator E"));
        assert_eq!(panel.len(), 5);
        assert!(panel.get("ind-e").is_some());
    }

    #[test]
    fn render_bar_empty() {
        let t = CatppuccinMocha;
        let panel = IndicatorPanel::new();
        let line = panel.render_bar(&t);
        assert!(line.spans.is_empty());
    }

    #[test]
    fn render_bar_with_indicators() {
        let t = CatppuccinMocha;
        let mut panel = test_panel();
        panel.pass("ind-a");
        panel.start("ind-b");
        let line = panel.render_bar(&t);
        let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(text.contains("✓Indicator A"));
        assert!(text.contains("⏳Indicator B"));
        assert!(text.contains("○Indicator C"));
        assert!(text.contains("○Indicator D"));
    }
}
