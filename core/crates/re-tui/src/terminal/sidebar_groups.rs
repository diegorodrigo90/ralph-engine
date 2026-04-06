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
/// - `official.claude`, `official.claudebox`, `official.codex` → "Agents"
/// - `official.bmad` → "Sprint"
/// - `official.findings` → "Findings"
/// - Everything else → "Tools"
///
/// Empty groups are omitted.
pub(super) fn group_panels(panels: &[SidebarPanel]) -> Vec<SidebarGroup<'_>> {
    let mut agents: Vec<&SidebarPanel> = Vec::new();
    let mut sprint: Vec<&SidebarPanel> = Vec::new();
    let mut findings: Vec<&SidebarPanel> = Vec::new();
    let mut tools: Vec<&SidebarPanel> = Vec::new();

    for panel in panels {
        match classify_plugin(&panel.plugin_id) {
            Domain::Agent => agents.push(panel),
            Domain::Sprint => sprint.push(panel),
            Domain::Findings => findings.push(panel),
            Domain::Tool => tools.push(panel),
        }
    }

    let mut groups = Vec::with_capacity(4);

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
    if !tools.is_empty() {
        groups.push(SidebarGroup {
            title: "Tools",
            panels: tools,
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

/// Semantic domain for sidebar grouping.
enum Domain {
    Agent,
    Sprint,
    Findings,
    Tool,
}

/// Classifies a plugin into a semantic domain by its ID.
///
/// This uses pattern matching on well-known plugin IDs. Community plugins
/// fall into "Tool" by default — a reasonable grouping for unknown plugins.
fn classify_plugin(plugin_id: &str) -> Domain {
    match plugin_id {
        "official.claude" | "official.claudebox" | "official.codex" => Domain::Agent,
        "official.bmad" => Domain::Sprint,
        "official.findings" => Domain::Findings,
        _ => Domain::Tool,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn panel(plugin_id: &str, title: &str) -> SidebarPanel {
        SidebarPanel {
            title: title.to_owned(),
            lines: Vec::new(),
            items: Vec::new(),
            plugin_id: plugin_id.to_owned(),
        }
    }

    #[test]
    fn groups_agents_together() {
        let panels = vec![
            panel("official.claude", "Claude"),
            panel("official.claudebox", "ClaudeBox"),
            panel("official.codex", "Codex"),
        ];
        let groups = group_panels(&panels);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].title, "Agents");
        assert_eq!(groups[0].panels.len(), 3);
    }

    #[test]
    fn groups_sprint_separately() {
        let panels = vec![
            panel("official.bmad", "Sprint"),
            panel("official.claude", "Claude"),
        ];
        let groups = group_panels(&panels);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].title, "Agents");
        assert_eq!(groups[1].title, "Sprint");
    }

    #[test]
    fn groups_tools_as_default() {
        let panels = vec![
            panel("official.github", "GitHub"),
            panel("official.tdd-strict", "TDD"),
            panel("official.router", "Router"),
            panel("official.context", "Context"),
        ];
        let groups = group_panels(&panels);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].title, "Tools");
        assert_eq!(groups[0].panels.len(), 4);
    }

    #[test]
    fn groups_findings_separately() {
        let panels = vec![
            panel("official.findings", "Findings"),
            panel("official.github", "GitHub"),
        ];
        let groups = group_panels(&panels);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].title, "Tools");
        assert_eq!(groups[1].title, "Findings");
    }

    #[test]
    fn empty_groups_omitted() {
        let panels = vec![panel("official.claude", "Claude")];
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
    fn all_four_groups_present() {
        let panels = vec![
            panel("official.claude", "Claude"),
            panel("official.bmad", "Sprint"),
            panel("official.github", "GitHub"),
            panel("official.findings", "Findings"),
        ];
        let groups = group_panels(&panels);
        assert_eq!(groups.len(), 4);
        let titles: Vec<&str> = groups.iter().map(|g| g.title).collect();
        assert_eq!(titles, &["Agents", "Sprint", "Tools", "Findings"]);
    }

    #[test]
    fn community_plugins_go_to_tools() {
        let panels = vec![
            panel("community.acme.jira", "Jira"),
            panel("community.foo.bar", "Custom"),
        ];
        let groups = group_panels(&panels);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].title, "Tools");
        assert_eq!(groups[0].panels.len(), 2);
    }

    #[test]
    fn group_order_is_agents_sprint_tools_findings() {
        let panels = vec![
            panel("official.findings", "Findings"),
            panel("official.github", "GitHub"),
            panel("official.bmad", "Sprint"),
            panel("official.claude", "Claude"),
        ];
        let groups = group_panels(&panels);
        let titles: Vec<&str> = groups.iter().map(|g| g.title).collect();
        assert_eq!(titles, &["Agents", "Sprint", "Tools", "Findings"]);
    }
}
