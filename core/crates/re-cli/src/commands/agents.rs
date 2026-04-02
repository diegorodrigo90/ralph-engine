//! Agent runtime command handlers.

use re_core::RuntimeAgentRegistration;

use crate::{CliError, catalog, i18n};

/// Executes the agents command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_agent_listing(
            &catalog::official_runtime_agents(),
            locale,
        )),
        Some("show") => show_agent(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "agents", other,
        ))),
    }
}

fn show_agent(plugin_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let plugin_id = plugin_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "agents",
            i18n::plugin_id_entity_label(locale),
        ))
    })?;
    let agents = catalog::official_runtime_agents()
        .into_iter()
        .filter(|registration| registration.plugin_id == plugin_id)
        .collect::<Vec<_>>();

    if agents.is_empty() {
        return Err(CliError::new(i18n::unknown_entity(
            locale,
            i18n::agent_runtime_entity_label(locale),
            plugin_id,
        )));
    }

    Ok(render_agent_detail(plugin_id, &agents, locale))
}

fn render_agent_listing(registrations: &[RuntimeAgentRegistration], locale: &str) -> String {
    if registrations.is_empty() {
        return i18n::list_heading(
            locale,
            i18n::agent_runtimes_label(locale),
            i18n::agent_runtimes_label(locale),
            0,
        );
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

    format!(
        "{}\n{}",
        i18n::list_heading(
            locale,
            i18n::agent_runtimes_label(locale),
            i18n::agent_runtimes_label(locale),
            lines.len(),
        ),
        lines.join("\n")
    )
}

fn render_agent_detail(
    plugin_id: &str,
    agents: &[RuntimeAgentRegistration],
    locale: &str,
) -> String {
    let mut lines = vec![
        i18n::detail_heading(
            locale,
            i18n::agent_runtime_label(locale),
            i18n::agent_runtime_label(locale),
            plugin_id,
        ),
        i18n::providers_heading(locale, agents.len()),
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
        let rendered = render_agent_listing(&registrations, "en");

        // Assert
        assert_eq!(rendered, "Agent runtimes (0)");
    }

    #[test]
    fn render_agent_listing_handles_empty_sets_in_pt_br() {
        let registrations = [];

        let rendered = render_agent_listing(&registrations, "pt-br");

        assert_eq!(rendered, "Runtimes de agente (0)");
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
        let rendered = render_agent_detail("official.codex", &agents, "en");

        // Assert
        assert!(rendered.contains("Agent runtime: official.codex"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- official.codex | activation=enabled | boundary=in_process | bootstrap_hook=true"
        ));
    }

    #[test]
    fn render_agent_detail_supports_pt_br() {
        let agents = [RuntimeAgentRegistration::new(
            "official.codex",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        let rendered = render_agent_detail("official.codex", &agents, "pt-br");

        assert!(rendered.contains("Runtime de agente: official.codex"));
        assert!(rendered.contains("Provedores (1)"));
    }
}
