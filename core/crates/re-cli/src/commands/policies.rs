//! Policy command handlers.

use crate::{CliError, catalog, i18n};

use catalog::OfficialPolicyContribution;

/// Executes the policies command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_policy_listing(
            &catalog::official_policy_contributions(),
            locale,
        )),
        Some("show") => show_policy(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "policies", other,
        ))),
    }
}

fn show_policy(policy_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let policy_id = policy_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "policies",
            i18n::policy_id_entity_label(locale),
        ))
    })?;
    let policy = catalog::find_official_policy_contribution(policy_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::policy_entity_label(locale),
            policy_id,
        ))
    })?;

    Ok(render_policy_detail(policy, locale))
}

fn render_policy_listing(registrations: &[OfficialPolicyContribution], locale: &str) -> String {
    if registrations.is_empty() {
        return i18n::list_heading(
            locale,
            i18n::policies_label(locale),
            i18n::policies_label(locale),
            0,
        );
    }

    let lines = registrations
        .iter()
        .map(|registration| {
            format!(
                "- {} | {} | plugin={} | activation={}",
                registration.descriptor.id,
                registration.descriptor.display_name_for_locale(locale),
                registration.descriptor.plugin_id,
                registration.activation.as_str()
            )
        })
        .collect::<Vec<_>>();

    format!(
        "{}\n{}",
        i18n::list_heading(
            locale,
            i18n::policies_label(locale),
            i18n::policies_label(locale),
            registrations.len(),
        ),
        lines.join("\n")
    )
}

fn render_policy_detail(policy: OfficialPolicyContribution, locale: &str) -> String {
    format!(
        "{}: {}\n{name_label}: {}\n{summary_label}: {}\n{provider_label}: {provider}\n{activation_label}: {activation}\n{load_boundary_label}: {load_boundary}\n{policy_hook_label}: {policy_hook}",
        i18n::policy_label(locale),
        policy.descriptor.id,
        policy.descriptor.display_name_for_locale(locale),
        policy.descriptor.summary_for_locale(locale),
        name_label = i18n::name_label(locale),
        summary_label = i18n::summary_label(locale),
        provider_label = i18n::provider_label(locale),
        provider = policy.descriptor.plugin_id,
        activation_label = i18n::activation_label(locale),
        activation = policy.activation.as_str(),
        load_boundary_label = i18n::load_boundary_label(locale),
        load_boundary = policy.load_boundary.as_str(),
        policy_hook_label = i18n::policy_enforcement_hook_label(locale),
        policy_hook = if policy.enforcement_hook_registered {
            "policy_enforcement"
        } else {
            "missing"
        },
    )
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_plugin::{PluginLoadBoundary, PluginLocalizedText, PluginPolicyDescriptor};

    use super::{OfficialPolicyContribution, render_policy_detail, render_policy_listing};

    const LOCALIZED_NAMES: &[PluginLocalizedText] =
        &[PluginLocalizedText::new("pt-br", "Guardrails TDD estrito")];
    const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
        "pt-br",
        "Política oficial com guardrails estritos de TDD.",
    )];
    const POLICY_ID: &str = "fixture.strict.guardrails";
    const PLUGIN_ID: &str = "fixture.strict";

    fn policy_descriptor() -> PluginPolicyDescriptor {
        PluginPolicyDescriptor::new(
            POLICY_ID,
            PLUGIN_ID,
            "TDD strict guardrails",
            LOCALIZED_NAMES,
            "Official policy with strict TDD guardrails.",
            LOCALIZED_SUMMARIES,
        )
    }

    #[test]
    fn render_policy_listing_handles_empty_sets() {
        let registrations = [];

        let rendered = render_policy_listing(&registrations, "en");

        assert_eq!(rendered, "Policies (0)");
    }

    #[test]
    fn render_policy_detail_is_human_readable() {
        let rendered = render_policy_detail(
            OfficialPolicyContribution {
                descriptor: policy_descriptor(),
                activation: PluginActivation::Enabled,
                load_boundary: PluginLoadBoundary::InProcess,
                enforcement_hook_registered: true,
            },
            "en",
        );

        assert!(rendered.contains("Policy: fixture.strict.guardrails"));
        assert!(rendered.contains("Name: TDD strict guardrails"));
        assert!(rendered.contains("Provider: fixture.strict"));
        assert!(rendered.contains("Activation: enabled"));
        assert!(rendered.contains("Policy enforcement hook: policy_enforcement"));
    }

    #[test]
    fn render_policy_detail_supports_pt_br() {
        let rendered = render_policy_detail(
            OfficialPolicyContribution {
                descriptor: policy_descriptor(),
                activation: PluginActivation::Enabled,
                load_boundary: PluginLoadBoundary::InProcess,
                enforcement_hook_registered: true,
            },
            "pt-br",
        );

        assert!(rendered.contains("Política: fixture.strict.guardrails"));
        assert!(rendered.contains("Nome: Guardrails TDD estrito"));
        assert!(rendered.contains("Provedor: fixture.strict"));
        assert!(rendered.contains("Hook de aplicação de política: policy_enforcement"));
    }
}
