//! Policy command handlers.

use re_core::RuntimePolicyRegistration;

use crate::{CliError, catalog, i18n};

/// Executes the policies command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_policy_listing(
            &catalog::official_runtime_policies(),
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
    let registration = catalog::official_runtime_policies()
        .into_iter()
        .find(|registration| registration.policy_id == policy_id)
        .ok_or_else(|| {
            CliError::new(i18n::unknown_entity(
                locale,
                i18n::policy_entity_label(locale),
                policy_id,
            ))
        })?;

    Ok(render_policy_detail(&registration, locale))
}

fn render_policy_listing(registrations: &[RuntimePolicyRegistration], locale: &str) -> String {
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
                "- {} | plugin={} | activation={}",
                registration.policy_id,
                registration.plugin_id,
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

fn render_policy_detail(registration: &RuntimePolicyRegistration, locale: &str) -> String {
    [
        i18n::detail_heading(
            locale,
            i18n::policy_label(locale),
            i18n::policy_label(locale),
            registration.policy_id,
        ),
        i18n::detail_heading(
            locale,
            i18n::provider_label(locale),
            i18n::provider_label(locale),
            registration.plugin_id,
        ),
        i18n::detail_heading(
            locale,
            "Activation",
            "Ativação",
            registration.activation.as_str(),
        ),
        i18n::detail_heading(
            locale,
            "Load boundary",
            "Fronteira de carregamento",
            registration.load_boundary.as_str(),
        ),
        format!(
            "{}: {}",
            i18n::policy_enforcement_hook_label(locale),
            registration.enforcement_hook_registered
        ),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::RuntimePolicyRegistration;
    use re_plugin::PluginLoadBoundary;

    use super::{render_policy_detail, render_policy_listing};

    #[test]
    fn render_policy_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_policy_listing(&registrations, "en");

        // Assert
        assert_eq!(rendered, "Policies (0)");
    }

    #[test]
    fn render_policy_detail_is_human_readable() {
        // Arrange
        let registration = RuntimePolicyRegistration::new(
            "official.tdd-strict",
            "official.tdd-strict",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        );

        // Act
        let rendered = render_policy_detail(&registration, "en");

        // Assert
        assert!(rendered.contains("Policy: official.tdd-strict"));
        assert!(rendered.contains("Provider: official.tdd-strict"));
        assert!(rendered.contains("Activation: enabled"));
        assert!(rendered.contains("Load boundary: in_process"));
        assert!(rendered.contains("Policy enforcement hook: true"));
    }

    #[test]
    fn render_policy_detail_supports_pt_br() {
        let registration = RuntimePolicyRegistration::new(
            "official.tdd-strict",
            "official.tdd-strict",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        );

        let rendered = render_policy_detail(&registration, "pt-br");

        assert!(rendered.contains("Política: official.tdd-strict"));
        assert!(rendered.contains("Provedor: official.tdd-strict"));
        assert!(rendered.contains("Fronteira de carregamento: in_process"));
        assert!(rendered.contains("Hook de aplicação de política: true"));
    }
}
