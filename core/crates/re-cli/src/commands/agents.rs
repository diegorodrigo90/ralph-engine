//! Agent runtime command handlers.

use re_core::RuntimeAgentRegistration;

use crate::{CliError, catalog};

/// Executes the agents command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_agent_listing(&catalog::official_runtime_agents())),
        Some("show") => show_agent(args.get(1).map(String::as_str)),
        Some(other) => Err(CliError::new(format!("unknown agents command: {other}"))),
    }
}

fn show_agent(plugin_id: Option<&str>) -> Result<String, CliError> {
    let plugin_id = plugin_id.ok_or_else(|| CliError::new("agents show requires a plugin id"))?;
    let agents = catalog::official_runtime_agents()
        .into_iter()
        .filter(|registration| registration.plugin_id == plugin_id)
        .collect::<Vec<_>>();

    if agents.is_empty() {
        return Err(CliError::new(format!("unknown agent runtime: {plugin_id}")));
    }

    Ok(render_agent_detail(plugin_id, &agents))
}

fn render_agent_listing(registrations: &[RuntimeAgentRegistration]) -> String {
    if registrations.is_empty() {
        return "Agent runtimes (0)".to_owned();
    }

    let lines = registrations
        .iter()
        .map(|registration| {
            format!(
                "- {} | activation={} | boundary={} | bootstrap_hook={}",
                registration.plugin_id,
                registration.activation.as_str(),
                registration.load_boundary.as_str(),
                registration.bootstrap_hook_registered
            )
        })
        .collect::<Vec<_>>();

    format!("Agent runtimes ({})\n{}", lines.len(), lines.join("\n"))
}

fn render_agent_detail(plugin_id: &str, agents: &[RuntimeAgentRegistration]) -> String {
    let mut lines = vec![
        format!("Agent runtime: {plugin_id}"),
        format!("Providers ({})", agents.len()),
    ];

    for agent in agents {
        lines.push(format!(
            "- {} | activation={} | boundary={} | bootstrap_hook={}",
            agent.plugin_id,
            agent.activation.as_str(),
            agent.load_boundary.as_str(),
            agent.bootstrap_hook_registered
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::RuntimeAgentRegistration;
    use re_plugin::PluginLoadBoundary;

    use super::{render_agent_detail, render_agent_listing};

    #[test]
    fn render_agent_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_agent_listing(&registrations);

        // Assert
        assert_eq!(rendered, "Agent runtimes (0)");
    }

    #[test]
    fn render_agent_detail_is_human_readable() {
        // Arrange
        let agents = [RuntimeAgentRegistration::new(
            "official.codex",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        // Act
        let rendered = render_agent_detail("official.codex", &agents);

        // Assert
        assert!(rendered.contains("Agent runtime: official.codex"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- official.codex | activation=enabled | boundary=in_process | bootstrap_hook=true"
        ));
    }
}
