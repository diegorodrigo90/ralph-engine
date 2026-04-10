//! Sidebar grouping — organizes plugin panels into semantic domains.
//!
//! Instead of rendering N individual panels (one per plugin), the sidebar
//! groups panels into 4 domains: Agents, Sprint, Tools, Findings.
//! The grouping is based on `plugin_id` patterns — plugins declare data,
//! core decides how to present it (Model B).

use super::types::SidebarPanel;

/// A semantic group for sidebar rendering.
#[derive(Debug)]
pub(super) struct SidebarGroup<'a> {
    /// Group heading (e.g., "Agents", "Sprint").
    pub title: &'static str,
    /// Panels belonging to this group.
    pub panels: Vec<&'a SidebarPanel>,
}

/// Groups sidebar panels by semantic domain.
///
/// Rules:
/// - `panel.is_agent == true` → "Agents" (Model B — no hardcoded plugin IDs)
/// - `official.bmad` → "Sprint" (presentation hint — not business logic)
/// - `official.findings` → "Findings" (presentation hint)
/// - Everything else → hidden (Config tab only)
///
/// Empty groups are omitted.
pub(super) fn group_panels(panels: &[SidebarPanel]) -> Vec<SidebarGroup<'_>> {
    let mut agents: Vec<&SidebarPanel> = Vec::new();
    let mut sprint: Vec<&SidebarPanel> = Vec::new();
    let mut findings: Vec<&SidebarPanel> = Vec::new();

    for panel in panels {
        if panel.is_agent {
            agents.push(panel);
        } else {
            match classify_non_agent(&panel.plugin_id) {
                Domain::Sprint => sprint.push(panel),
                Domain::Findings => findings.push(panel),
                Domain::Hidden => {} // Goes to Config tab only
            }
        }
    }

    let mut groups = Vec::with_capacity(3);

    if !agents.is_empty() {
        groups.push(SidebarGroup {
            title: "Agents",
            panels: agents,
        });
    }
    if !sprint.is_empty() {
        groups.push(SidebarGroup {
            title: "Sprint",
            panels: sprint,
        });
    }
    if !findings.is_empty() {
        groups.push(SidebarGroup {
            title: "Findings",
            panels: findings,
        });
    }

    groups
}

/// Semantic domain for non-agent sidebar grouping.
enum Domain {
    Sprint,
    Findings,
    /// Hidden from sidebar — details go to Config tab only.
    Hidden,
}

/// Classifies a non-agent plugin into a presentation domain by its ID.
///
/// Agent plugins are already classified via `panel.is_agent` (Model B).
/// This classifies the remaining plugins into sidebar sections.
/// Tool/infra plugins (github, tdd-strict, router, context, guided, ssh)
/// are hidden — their details live in the Config tab (Level 2 disclosure).
fn classify_non_agent(plugin_id: &str) -> Domain {
    match plugin_id {
        "official.bmad" => Domain::Sprint,
        "official.findings" => Domain::Findings,
        _ => Domain::Hidden,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn agent(plugin_id: &str, title: &str) -> SidebarPanel {
        SidebarPanel {
            title: title.to_owned(),
            items: Vec::new(),
            plugin_id: plugin_id.to_owned(),
            is_agent: true,
        }
    }

    fn non_agent(plugin_id: &str, title: &str) -> SidebarPanel {
        SidebarPanel {
            title: title.to_owned(),
            items: Vec::new(),
            plugin_id: plugin_id.to_owned(),
            is_agent: false,
        }
    }

    #[test]
    fn groups_agents_together() {
        let panels = vec![
            agent("official.claude", "Claude"),
            agent("official.claudebox", "ClaudeBox"),
            agent("official.codex", "Codex"),
        ];
        let groups = group_panels(&panels);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].title, "Agents");
        assert_eq!(groups[0].panels.len(), 3);
    }

    #[test]
    fn groups_sprint_separately() {
        let panels = vec![
            non_agent("official.bmad", "Sprint"),
            agent("official.claude", "Claude"),
        ];
        let groups = group_panels(&panels);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].title, "Agents");
        assert_eq!(groups[1].title, "Sprint");
    }

    #[test]
    fn tool_plugins_hidden_from_sidebar() {
        let panels = vec![
            non_agent("official.github", "GitHub"),
            non_agent("official.tdd-strict", "TDD"),
            non_agent("official.router", "Router"),
            non_agent("official.context", "Context"),
        ];
        let groups = group_panels(&panels);
        assert!(
            groups.is_empty(),
            "tool plugins should be hidden from sidebar"
        );
    }

    #[test]
    fn findings_shown_without_tools() {
        let panels = vec![
            non_agent("official.findings", "Findings"),
            non_agent("official.github", "GitHub"),
        ];
        let groups = group_panels(&panels);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].title, "Findings");
    }

    #[test]
    fn empty_groups_omitted() {
        let panels = vec![agent("official.claude", "Claude")];
        let groups = group_panels(&panels);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].title, "Agents");
    }

    #[test]
    fn empty_input_returns_empty() {
        let groups = group_panels(&[]);
        assert!(groups.is_empty());
    }

    #[test]
    fn all_three_sidebar_groups() {
        let panels = vec![
            agent("official.claude", "Claude"),
            non_agent("official.bmad", "Sprint"),
            non_agent("official.github", "GitHub"), // hidden
            non_agent("official.findings", "Findings"),
        ];
        let groups = group_panels(&panels);
        assert_eq!(groups.len(), 3);
        let titles: Vec<&str> = groups.iter().map(|g| g.title).collect();
        assert_eq!(titles, &["Agents", "Sprint", "Findings"]);
    }

    #[test]
    fn community_plugins_hidden() {
        let panels = vec![
            non_agent("community.acme.jira", "Jira"),
            non_agent("community.foo.bar", "Custom"),
        ];
        let groups = group_panels(&panels);
        assert!(groups.is_empty(), "community plugins hidden from sidebar");
    }

    #[test]
    fn group_order_is_agents_sprint_findings() {
        let panels = vec![
            non_agent("official.findings", "Findings"),
            non_agent("official.github", "GitHub"),
            non_agent("official.bmad", "Sprint"),
            agent("official.claude", "Claude"),
        ];
        let groups = group_panels(&panels);
        let titles: Vec<&str> = groups.iter().map(|g| g.title).collect();
        assert_eq!(titles, &["Agents", "Sprint", "Findings"]);
    }
}
