//! Agent runtime command handlers.

use re_core::RuntimeAgentBootstrapPlan;

use super::format;
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
        Some("plan") => show_agent_plan(args.get(1).map(String::as_str), locale),
        Some("launch") => probe_agent_launch(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::usage(i18n::unknown_subcommand(
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
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::agent_runtime_entity_label(locale),
            agent_id,
        ))
    })?;

    Ok(render_agent_detail(agent, locale))
}

fn show_agent_plan(agent_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let agent_id = agent_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "agents",
            i18n::agent_id_entity_label(locale),
        ))
    })?;
    let agent = catalog::find_official_agent_contribution(agent_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::agent_runtime_entity_label(locale),
            agent_id,
        ))
    })?;

    let plan = RuntimeAgentBootstrapPlan::new(
        agent.descriptor.id,
        agent.descriptor.plugin_id,
        agent.load_boundary,
        agent.bootstrap_hook_registered,
    );

    Ok(render_agent_plan(agent, plan, locale))
}

fn probe_agent_launch(agent_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let agent_id = agent_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "agents launch",
            i18n::agent_id_entity_label(locale),
        ))
    })?;
    let agent = catalog::find_official_agent_contribution(agent_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::agent_runtime_entity_label(locale),
            agent_id,
        ))
    })?;

    let plan = RuntimeAgentBootstrapPlan::new(
        agent.descriptor.id,
        agent.descriptor.plugin_id,
        agent.load_boundary,
        agent.bootstrap_hook_registered,
    );

    let mut lines = Vec::new();

    let heading = i18n::agents_bootstrap_probe(locale);
    lines.push(format!("--- {heading}: {} ---", agent.descriptor.id));
    lines.push(format!("plugin: {}", agent.descriptor.plugin_id));
    lines.push(format!("load_boundary: {}", agent.load_boundary));

    if agent.bootstrap_hook_registered {
        lines.push(format!(
            "{} {}",
            super::status_ok(),
            i18n::agents_bootstrap_registered(locale)
        ));
    } else {
        lines.push(format!(
            "{} {}",
            super::status_missing(),
            i18n::agents_bootstrap_not_registered(locale)
        ));
    }

    match catalog::official_plugin_runtime(agent.descriptor.plugin_id) {
        Some(runtime) => match runtime.bootstrap_agent(agent.descriptor.id) {
            Ok(result) => {
                let status = if result.ready {
                    super::status_ok()
                } else {
                    super::status_not_ready()
                };
                lines.push(format!("{status} {}", result.message));
            }
            Err(err) => {
                lines.push(format!("{} {err}", super::status_unsupported()));
            }
        },
        None => {
            lines.push(i18n::agents_no_runtime(locale).to_owned());
        }
    }

    lines.push(String::new());
    lines.push(render_agent_plan(agent, plan, locale));

    Ok(lines.join("\n"))
}

fn render_agent_listing(registrations: &[OfficialAgentContribution], locale: &str) -> String {
    let heading = i18n::list_heading(
        locale,
        "Agent runtimes",
        "Runtimes de agente",
        registrations.len(),
    );

    let headers = &["ID", "NAME", "PLUGIN", "STATUS"];
    let rows: Vec<Vec<String>> = registrations
        .iter()
        .map(|r| {
            vec![
                r.descriptor.id.to_owned(),
                r.descriptor.display_name_for_locale(locale).to_owned(),
                r.descriptor.plugin_id.to_owned(),
                r.activation.as_str().to_owned(),
            ]
        })
        .collect();

    if rows.is_empty() {
        return heading;
    }

    format!("{heading}\n\n{}", format::render_table(headers, &rows))
}

fn render_agent_detail(agent: OfficialAgentContribution, locale: &str) -> String {
    let pairs = vec![
        ("Agent runtime:", agent.descriptor.id.to_owned()),
        (
            i18n::name_label(locale),
            agent.descriptor.display_name_for_locale(locale).to_owned(),
        ),
        (
            i18n::summary_label(locale),
            agent.descriptor.summary_for_locale(locale).to_owned(),
        ),
        ("Plugin:", agent.descriptor.plugin_id.to_owned()),
        ("", String::new()),
        (
            i18n::activation_label(locale),
            agent.activation.as_str().to_owned(),
        ),
        (
            i18n::load_boundary_label(locale),
            agent.load_boundary.as_str().to_owned(),
        ),
        (
            i18n::hook_label(locale),
            if agent.bootstrap_hook_registered {
                "agent_bootstrap"
            } else {
                "missing"
            }
            .to_owned(),
        ),
    ];

    format::render_detail(&pairs)
}

fn render_agent_plan(
    agent: OfficialAgentContribution,
    plan: RuntimeAgentBootstrapPlan,
    locale: &str,
) -> String {
    let pairs = vec![
        ("Agent bootstrap plan:", agent.descriptor.id.to_owned()),
        (
            i18n::name_label(locale),
            agent.descriptor.display_name_for_locale(locale).to_owned(),
        ),
        ("Plugin:", agent.descriptor.plugin_id.to_owned()),
        (
            i18n::activation_label(locale),
            agent.activation.as_str().to_owned(),
        ),
        (
            i18n::load_boundary_label(locale),
            plan.load_boundary.as_str().to_owned(),
        ),
        (i18n::hook_label(locale), "agent_bootstrap".to_owned()),
        (
            i18n::registration_hook_label(locale),
            plan.bootstrap_hook_registered.to_string(),
        ),
    ];

    format::render_detail(&pairs)
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_plugin::{PluginAgentDescriptor, PluginLoadBoundary, PluginLocalizedText};

    use re_core::RuntimeAgentBootstrapPlan;

    use super::{
        OfficialAgentContribution, render_agent_detail, render_agent_listing, render_agent_plan,
    };

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

        assert!(rendered.contains("Agent runtimes (0)"));
    }

    #[test]
    fn render_agent_listing_handles_empty_sets_in_pt_br() {
        let registrations = [];

        let rendered = render_agent_listing(&registrations, "pt-br");

        assert!(rendered.contains("Runtimes de agente (0)"));
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

        assert!(rendered.contains("fixture.codex.session"));
        assert!(rendered.contains("Codex session"));
        assert!(rendered.contains("fixture.codex"));
        assert!(rendered.contains("enabled"));
        assert!(rendered.contains("agent_bootstrap"));
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

        assert!(rendered.contains("fixture.codex.session"));
        assert!(rendered.contains("Sessão Codex"));
        assert!(rendered.contains("Sessão de runtime do Codex para o Ralph Engine."));
    }

    #[test]
    fn render_agent_plan_is_human_readable() {
        let rendered = render_agent_plan(
            OfficialAgentContribution {
                descriptor: agent_descriptor(),
                activation: PluginActivation::Enabled,
                load_boundary: PluginLoadBoundary::InProcess,
                bootstrap_hook_registered: true,
            },
            RuntimeAgentBootstrapPlan::new(
                AGENT_ID,
                PLUGIN_ID,
                PluginLoadBoundary::InProcess,
                true,
            ),
            "en",
        );

        assert!(rendered.contains("Agent bootstrap plan:"));
        assert!(rendered.contains("fixture.codex.session"));
        assert!(rendered.contains("fixture.codex"));
        assert!(rendered.contains("true"));
    }

    #[test]
    fn probe_agent_launch_requires_agent_id() {
        let args = vec!["launch".to_owned()];
        let result = super::execute(&args, "en");
        assert!(result.is_err());
    }

    #[test]
    fn probe_agent_launch_rejects_unknown_agent_id() {
        let args = vec!["launch".to_owned(), "unknown.agent".to_owned()];
        let result = super::execute(&args, "en");
        assert!(result.is_err());
    }

    #[test]
    fn probe_agent_launch_reports_bootstrap_status() {
        let args = vec!["launch".to_owned(), "official.claude.session".to_owned()];
        let result = super::execute(&args, "en");
        assert!(result.is_ok());
        let output = result.ok().unwrap_or_default();
        assert!(output.contains("Agent bootstrap probe"));
        assert!(output.contains("official.claude"));
        // Claude plugin has runtime — should report binary status
        assert!(output.contains("[OK]") || output.contains("[NOT READY]"));
    }

    #[test]
    fn probe_agent_launch_supports_pt_br() {
        let args = vec!["launch".to_owned(), "official.claude.session".to_owned()];
        let result = super::execute(&args, "pt-br");
        assert!(result.is_ok());
        let output = result.ok().unwrap_or_default();
        assert!(output.contains("Verificação de bootstrap de agente"));
    }
}
