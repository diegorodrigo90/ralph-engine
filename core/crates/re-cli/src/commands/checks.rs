//! Runtime check command handlers.

use re_core::{RuntimeCheckKind, RuntimeCheckRegistration, parse_runtime_check_kind};

use crate::{
    CliError, catalog,
    commands::grouped_surfaces::{render_grouped_surface_detail, render_grouped_surface_listing},
    i18n,
};

/// Executes the checks command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_check_listing(
            &catalog::official_runtime_checks(),
            locale,
        )),
        Some("show") => show_check(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "checks", other,
        ))),
    }
}

fn show_check(check_kind: Option<&str>, locale: &str) -> Result<String, CliError> {
    let check_id = check_kind.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "checks",
            i18n::check_id_entity_label(locale),
        ))
    })?;

    if let Some(contribution) = catalog::find_official_check_contribution(check_id) {
        let registration = catalog::find_official_runtime_checks(kind_for_check(contribution))
            .into_iter()
            .find(|candidate| candidate.plugin_id == contribution.descriptor.plugin_id)
            .ok_or_else(|| {
                CliError::new(i18n::unknown_entity(
                    locale,
                    i18n::check_entity_label(locale),
                    check_id,
                ))
            })?;
        return Ok(render_check_contribution_detail(
            contribution,
            registration,
            locale,
        ));
    }

    let kind = parse_runtime_check_kind(check_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::check_entity_label(locale),
            check_id,
        ))
    })?;
    let checks = catalog::find_official_runtime_checks(kind);
    let contributions = catalog::find_official_check_contributions(kind);

    Ok(render_check_detail(kind, &checks, &contributions, locale))
}

fn kind_for_check(contribution: catalog::OfficialCheckContribution) -> RuntimeCheckKind {
    match contribution.descriptor.kind {
        re_plugin::PluginCheckKind::Prepare => RuntimeCheckKind::Prepare,
        re_plugin::PluginCheckKind::Doctor => RuntimeCheckKind::Doctor,
    }
}

fn render_check_listing(registrations: &[RuntimeCheckRegistration], locale: &str) -> String {
    render_grouped_surface_listing(
        registrations,
        locale,
        i18n::checks_label,
        |registration| registration.kind.as_str(),
        |registration| registration.is_enabled(),
    )
}

fn render_check_detail(
    kind: RuntimeCheckKind,
    checks: &[RuntimeCheckRegistration],
    contributions: &[catalog::OfficialCheckContribution],
    locale: &str,
) -> String {
    render_grouped_surface_detail(kind.as_str(), checks, locale, i18n::check_label, |check| {
        let contribution = contributions
            .iter()
            .find(|candidate| candidate.descriptor.plugin_id == check.plugin_id);

        format!(
            "- {} | plugin={} | name={} | summary={} | activation={} | boundary={} | runtime_hook={}",
            contribution.map_or(check.plugin_id, |entry| entry.descriptor.id),
            check.plugin_id,
            contribution.map_or(check.plugin_id, |entry| entry
                .descriptor
                .display_name_for_locale(locale)),
            contribution.map_or("-", |entry| entry.descriptor.summary_for_locale(locale)),
            check.activation.as_str(),
            check.load_boundary.as_str(),
            check.runtime_hook_registered
        )
    })
}

fn render_check_contribution_detail(
    contribution: catalog::OfficialCheckContribution,
    registration: RuntimeCheckRegistration,
    locale: &str,
) -> String {
    format!(
        "{}: {}\n{name_label}: {}\n{summary_label}: {}\nPlugin: {}\n{kind_label}: {kind}\n{activation_label}: {activation}\n{load_boundary_label}: {load_boundary}\n{hook_label}: {runtime_hook}",
        i18n::check_label(locale),
        contribution.descriptor.id,
        contribution.descriptor.display_name_for_locale(locale),
        contribution.descriptor.summary_for_locale(locale),
        contribution.descriptor.plugin_id,
        name_label = i18n::name_label(locale),
        summary_label = i18n::summary_label(locale),
        kind_label = i18n::kind_label(locale),
        kind = contribution.descriptor.kind.as_str(),
        activation_label = i18n::activation_label(locale),
        activation = registration.activation.as_str(),
        load_boundary_label = i18n::load_boundary_label(locale),
        load_boundary = registration.load_boundary.as_str(),
        hook_label = i18n::hook_label(locale),
        runtime_hook = registration.runtime_hook_registered,
    )
}

#[cfg(test)]
mod tests {
    use crate::catalog::OfficialCheckContribution;
    use re_config::PluginActivation;
    use re_core::{RuntimeCheckKind, RuntimeCheckRegistration};
    use re_plugin::{
        PluginCheckDescriptor, PluginCheckKind, PluginLoadBoundary, PluginLocalizedText,
    };

    use super::{render_check_detail, render_check_listing};
    use re_core::parse_runtime_check_kind;

    const CHECK_LOCALIZED_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
        "pt-br",
        "Verificação de preparo BMAD",
    )];
    const CHECK_LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
        "pt-br",
        "Executa validação tipada de preparo para workflows BMAD.",
    )];
    const CHECK_ID: &str = "fixture.bmad.prepare";
    const PRIMARY_PLUGIN_ID: &str = "fixture.bmad";
    const SECONDARY_PLUGIN_ID: &str = "fixture.other";

    #[test]
    fn parse_check_kind_supports_stable_identifiers() {
        // Arrange
        let values = ["prepare", "doctor"];

        // Act
        let parsed = values
            .into_iter()
            .map(parse_runtime_check_kind)
            .collect::<Vec<_>>();

        // Assert
        assert_eq!(
            parsed,
            vec![
                Some(RuntimeCheckKind::Prepare),
                Some(RuntimeCheckKind::Doctor)
            ]
        );
    }

    #[test]
    fn parse_check_kind_rejects_unknown_identifier() {
        // Arrange
        let value = "unknown";

        // Act
        let parsed = parse_runtime_check_kind(value);

        // Assert
        assert_eq!(parsed, None);
    }

    #[test]
    fn render_check_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_check_listing(&registrations, "en");

        // Assert
        assert_eq!(rendered, "Checks (0)");
    }

    #[test]
    fn render_check_listing_deduplicates_check_kinds() {
        // Arrange
        let registrations = [
            RuntimeCheckRegistration::new(
                RuntimeCheckKind::Prepare,
                PRIMARY_PLUGIN_ID,
                PluginActivation::Enabled,
                PluginLoadBoundary::InProcess,
                true,
            ),
            RuntimeCheckRegistration::new(
                RuntimeCheckKind::Prepare,
                SECONDARY_PLUGIN_ID,
                PluginActivation::Disabled,
                PluginLoadBoundary::InProcess,
                false,
            ),
        ];

        // Act
        let rendered = render_check_listing(&registrations, "en");

        // Assert
        assert_eq!(rendered, "Checks (1)\n- prepare | providers=2 | enabled=1");
    }

    #[test]
    fn render_check_detail_is_human_readable() {
        // Arrange
        let checks = [RuntimeCheckRegistration::new(
            RuntimeCheckKind::Prepare,
            PRIMARY_PLUGIN_ID,
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        // Act
        let contributions = [OfficialCheckContribution {
            descriptor: PluginCheckDescriptor::new(
                CHECK_ID,
                PRIMARY_PLUGIN_ID,
                PluginCheckKind::Prepare,
                "BMAD prepare check",
                CHECK_LOCALIZED_NAMES,
                "Runs typed prepare-time validation for BMAD workflows.",
                CHECK_LOCALIZED_SUMMARIES,
            ),
            activation: PluginActivation::Enabled,
            load_boundary: PluginLoadBoundary::InProcess,
            runtime_hook_registered: true,
        }];

        let rendered =
            render_check_detail(RuntimeCheckKind::Prepare, &checks, &contributions, "en");

        // Assert
        assert!(rendered.contains("Check: prepare"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- fixture.bmad.prepare | plugin=fixture.bmad | name=BMAD prepare check | summary=Runs typed prepare-time validation for BMAD workflows. | activation=enabled | boundary=in_process | runtime_hook=true"
        ));
    }

    #[test]
    fn render_check_detail_supports_pt_br() {
        let checks = [RuntimeCheckRegistration::new(
            RuntimeCheckKind::Prepare,
            PRIMARY_PLUGIN_ID,
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        let contributions = [OfficialCheckContribution {
            descriptor: PluginCheckDescriptor::new(
                CHECK_ID,
                PRIMARY_PLUGIN_ID,
                PluginCheckKind::Prepare,
                "BMAD prepare check",
                CHECK_LOCALIZED_NAMES,
                "Runs typed prepare-time validation for BMAD workflows.",
                CHECK_LOCALIZED_SUMMARIES,
            ),
            activation: PluginActivation::Enabled,
            load_boundary: PluginLoadBoundary::InProcess,
            runtime_hook_registered: true,
        }];

        let rendered =
            render_check_detail(RuntimeCheckKind::Prepare, &checks, &contributions, "pt-br");

        assert!(rendered.contains("Verificação: prepare"));
        assert!(rendered.contains("Provedores (1)"));
        assert!(rendered.contains("name=Verificação de preparo BMAD"));
    }
}
