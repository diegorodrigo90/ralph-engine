//! Agent runtime command handlers.

use crate::{CliError, catalog, i18n};

use catalog::OfficialAgentContribution;

/// Executes the agents command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_agent_listing(
            &catalog::official_agent_contributions(),
            locale,
        )),
        Some("show") => show_agent(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "agents", other,
        ))),
    }
}

fn show_agent(agent_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let agent_id = agent_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "agents",
            i18n::agent_id_entity_label(locale),
        ))
    })?;
    let agent = catalog::find_official_agent_contribution(agent_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::agent_runtime_entity_label(locale),
            agent_id,
        ))
    })?;

    Ok(render_agent_detail(agent, locale))
}

fn render_agent_listing(registrations: &[OfficialAgentContribution], locale: &str) -> String {
    let mut lines = Vec::with_capacity(registrations.len() + 1);
    lines.push(i18n::list_heading(
        locale,
        "Agent runtimes",
        "Runtimes de agente",
        registrations.len(),
    ));

    for registration in registrations {
        lines.push(format!(
            "- {} | {} | plugin={} | activation={}",
            registration.descriptor.id,
            registration.descriptor.display_name_for_locale(locale),
            registration.descriptor.plugin_id,
            registration.activation.as_str(),
        ));
    }

    lines.join("\n")
}

fn render_agent_detail(agent: OfficialAgentContribution, locale: &str) -> String {
    format!(
        "Agent runtime: {}\n{name_label}: {}\n{summary_label}: {}\nPlugin: {}\n{activation_label}: {activation}\n{load_boundary_label}: {load_boundary}\n{hook_label}: {runtime_hook}",
        agent.descriptor.id,
        agent.descriptor.display_name_for_locale(locale),
        agent.descriptor.summary_for_locale(locale),
        agent.descriptor.plugin_id,
        name_label = i18n::name_label(locale),
        summary_label = i18n::summary_label(locale),
        activation_label = i18n::activation_label(locale),
        activation = agent.activation.as_str(),
        load_boundary_label = i18n::load_boundary_label(locale),
        load_boundary = agent.load_boundary.as_str(),
        hook_label = i18n::hook_label(locale),
        runtime_hook = if agent.bootstrap_hook_registered {
            "agent_bootstrap"
        } else {
            "missing"
        },
    )
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_plugin::{PluginAgentDescriptor, PluginLoadBoundary, PluginLocalizedText};

    use super::{OfficialAgentContribution, render_agent_detail, render_agent_listing};

    const LOCALIZED_NAMES: &[PluginLocalizedText] =
        &[PluginLocalizedText::new("pt-br", "Sessão Codex")];
    const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
        "pt-br",
        "Sessão de runtime do Codex para o Ralph Engine.",
    )];
    const AGENT_ID: &str = "fixture.codex.session";
    const PLUGIN_ID: &str = "fixture.codex";

    fn agent_descriptor() -> PluginAgentDescriptor {
        PluginAgentDescriptor::new(
            AGENT_ID,
            PLUGIN_ID,
            "Codex session",
            LOCALIZED_NAMES,
            "Codex runtime session for Ralph Engine.",
            LOCALIZED_SUMMARIES,
        )
    }

    #[test]
    fn render_agent_listing_handles_empty_sets() {
        let registrations = [];

        let rendered = render_agent_listing(&registrations, "en");

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
        let rendered = render_agent_detail(
            OfficialAgentContribution {
                descriptor: agent_descriptor(),
                activation: PluginActivation::Enabled,
                load_boundary: PluginLoadBoundary::InProcess,
                bootstrap_hook_registered: true,
            },
            "en",
        );

        assert!(rendered.contains("Agent runtime: fixture.codex.session"));
        assert!(rendered.contains("Name: Codex session"));
        assert!(rendered.contains("Plugin: fixture.codex"));
        assert!(rendered.contains("Activation: enabled"));
        assert!(rendered.contains("Runtime hook: agent_bootstrap"));
    }

    #[test]
    fn render_agent_detail_supports_pt_br() {
        let rendered = render_agent_detail(
            OfficialAgentContribution {
                descriptor: agent_descriptor(),
                activation: PluginActivation::Enabled,
                load_boundary: PluginLoadBoundary::InProcess,
                bootstrap_hook_registered: true,
            },
            "pt-br",
        );

        assert!(rendered.contains("Agent runtime: fixture.codex.session"));
        assert!(rendered.contains("Nome: Sessão Codex"));
        assert!(rendered.contains("Resumo: Sessão de runtime do Codex para o Ralph Engine."));
    }
}
