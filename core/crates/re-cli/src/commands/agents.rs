//! Agent runtime command handlers.

use re_core::RuntimeAgentBootstrapPlan;

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

fn show_agent_plan(agent_id: Option<&str>, locale: &str) -> Result<String, CliError> {
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
        CliError::new(i18n::unknown_entity(
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

    let heading = if locale == "pt-br" {
        "Verificação de bootstrap de agente"
    } else {
        "Agent bootstrap probe"
    };
    lines.push(format!("--- {heading}: {} ---", agent.descriptor.id));
    lines.push(format!("plugin: {}", agent.descriptor.plugin_id));
    lines.push(format!("load_boundary: {}", agent.load_boundary));

    if agent.bootstrap_hook_registered {
        let label = if locale == "pt-br" {
            "Hook de bootstrap registrado"
        } else {
            "Bootstrap hook registered"
        };
        lines.push(format!("[OK] {label}"));
    } else {
        let label = if locale == "pt-br" {
            "Hook de bootstrap NÃO registrado"
        } else {
            "Bootstrap hook NOT registered"
        };
        lines.push(format!("[MISSING] {label}"));
    }

    match catalog::official_plugin_runtime(agent.descriptor.plugin_id) {
        Some(runtime) => match runtime.bootstrap_agent(agent.descriptor.id) {
            Ok(result) => {
                let status = if result.ready { "[OK]" } else { "[NOT READY]" };
                lines.push(format!("{status} {}", result.message));
            }
            Err(err) => {
                lines.push(format!("[UNSUPPORTED] {err}"));
            }
        },
        None => {
            let msg = if locale == "pt-br" {
                "Plugin não fornece implementação de runtime."
            } else {
                "Plugin does not provide a runtime implementation."
            };
            lines.push(msg.to_owned());
        }
    }

    lines.push(String::new());
    lines.push(render_agent_plan(agent, plan, locale));

    Ok(lines.join("\n"))
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

fn render_agent_plan(
    agent: OfficialAgentContribution,
    plan: RuntimeAgentBootstrapPlan,
    locale: &str,
) -> String {
    format!(
        "Agent bootstrap plan: {}\n{name_label}: {}\nPlugin: {}\n{activation_label}: {activation}\n{load_boundary_label}: {load_boundary}\n{hook_label}: agent_bootstrap\n{registered_label}: {registered}",
        agent.descriptor.id,
        agent.descriptor.display_name_for_locale(locale),
        agent.descriptor.plugin_id,
        name_label = i18n::name_label(locale),
        activation_label = i18n::activation_label(locale),
        activation = agent.activation.as_str(),
        load_boundary_label = i18n::load_boundary_label(locale),
        load_boundary = plan.load_boundary.as_str(),
        hook_label = i18n::hook_label(locale),
        registered_label = i18n::registration_hook_label(locale),
        registered = plan.bootstrap_hook_registered,
    )
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

        assert!(rendered.contains("Agent bootstrap plan: fixture.codex.session"));
        assert!(rendered.contains("Plugin: fixture.codex"));
        assert!(rendered.contains("Registration hook: true"));
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
