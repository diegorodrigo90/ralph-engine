//! Provider command handlers.

use re_core::{RuntimeProviderKind, RuntimeProviderRegistration, parse_runtime_provider_kind};

use crate::{
    CliError, catalog,
    commands::grouped_surfaces::{render_grouped_surface_detail, render_grouped_surface_listing},
    i18n,
};

/// Executes the providers command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_provider_listing(
            &catalog::official_runtime_providers(),
            locale,
        )),
        Some("show") => show_provider(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale,
            "providers",
            other,
        ))),
    }
}

fn show_provider(provider_kind: Option<&str>, locale: &str) -> Result<String, CliError> {
    let provider_id = provider_kind.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "providers",
            i18n::provider_id_entity_label(locale),
        ))
    })?;

    if let Some(contribution) = catalog::find_official_provider_contribution(provider_id) {
        let registration =
            catalog::find_official_runtime_providers(kind_for_provider(contribution))
                .into_iter()
                .find(|candidate| candidate.plugin_id == contribution.descriptor.plugin_id)
                .ok_or_else(|| {
                    CliError::new(i18n::unknown_entity(
                        locale,
                        i18n::provider_entity_label(locale),
                        provider_id,
                    ))
                })?;
        return Ok(render_provider_contribution_detail(
            contribution,
            registration,
            locale,
        ));
    }

    let kind = parse_runtime_provider_kind(provider_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::provider_entity_label(locale),
            provider_id,
        ))
    })?;
    let providers = catalog::find_official_runtime_providers(kind);
    let contributions = catalog::find_official_provider_contributions(kind);

    Ok(render_provider_detail(
        kind,
        &providers,
        &contributions,
        locale,
    ))
}

fn kind_for_provider(contribution: catalog::OfficialProviderContribution) -> RuntimeProviderKind {
    match contribution.descriptor.kind {
        re_plugin::PluginProviderKind::DataSource => RuntimeProviderKind::DataSource,
        re_plugin::PluginProviderKind::ContextProvider => RuntimeProviderKind::ContextProvider,
        re_plugin::PluginProviderKind::ForgeProvider => RuntimeProviderKind::ForgeProvider,
        re_plugin::PluginProviderKind::RemoteControl => RuntimeProviderKind::RemoteControl,
    }
}

fn render_provider_listing(registrations: &[RuntimeProviderRegistration], locale: &str) -> String {
    render_grouped_surface_listing(
        registrations,
        locale,
        i18n::providers_label,
        |registration| registration.kind.as_str(),
        |registration| registration.is_enabled(),
    )
}

fn render_provider_detail(
    provider_kind: RuntimeProviderKind,
    providers: &[RuntimeProviderRegistration],
    contributions: &[catalog::OfficialProviderContribution],
    locale: &str,
) -> String {
    render_grouped_surface_detail(
        provider_kind.as_str(),
        providers,
        locale,
        i18n::provider_label,
        |provider| {
            let contribution = contributions
                .iter()
                .find(|candidate| candidate.descriptor.plugin_id == provider.plugin_id);

            format!(
                "- {} | plugin={} | name={} | summary={} | activation={} | boundary={} | registration_hook={}",
                contribution.map_or(provider.plugin_id, |entry| entry.descriptor.id),
                provider.plugin_id,
                contribution.map_or(provider.plugin_id, |entry| entry
                    .descriptor
                    .display_name_for_locale(locale)),
                contribution.map_or("-", |entry| entry.descriptor.summary_for_locale(locale)),
                provider.activation.as_str(),
                provider.load_boundary.as_str(),
                provider.registration_hook_registered
            )
        },
    )
}

fn render_provider_contribution_detail(
    contribution: catalog::OfficialProviderContribution,
    registration: RuntimeProviderRegistration,
    locale: &str,
) -> String {
    let name_label = if i18n::is_pt_br(locale) {
        "Nome"
    } else {
        "Name"
    };
    let summary_label = if i18n::is_pt_br(locale) {
        "Resumo"
    } else {
        "Summary"
    };
    let kind_label = if i18n::is_pt_br(locale) {
        "Tipo"
    } else {
        "Kind"
    };
    let hook_label = "Registration hook";

    format!(
        "{}: {}\n{name_label}: {}\n{summary_label}: {}\nPlugin: {}\n{kind_label}: {}\n{}: {}\n{}: {}\n{hook_label}: {}",
        i18n::provider_label(locale),
        contribution.descriptor.id,
        contribution.descriptor.display_name_for_locale(locale),
        contribution.descriptor.summary_for_locale(locale),
        contribution.descriptor.plugin_id,
        contribution.descriptor.kind.as_str(),
        i18n::activation_label(locale),
        registration.activation.as_str(),
        i18n::load_boundary_label(locale),
        registration.load_boundary.as_str(),
        registration.registration_hook_registered,
    )
}

#[cfg(test)]
mod tests {
    use crate::catalog::OfficialProviderContribution;
    use re_config::PluginActivation;
    use re_core::{RuntimeProviderKind, RuntimeProviderRegistration};
    use re_plugin::{
        PluginLoadBoundary, PluginLocalizedText, PluginProviderDescriptor, PluginProviderKind,
    };

    use super::{render_provider_detail, render_provider_listing};
    use re_core::parse_runtime_provider_kind;

    const PROVIDER_LOCALIZED_NAMES: &[PluginLocalizedText] =
        &[PluginLocalizedText::new("pt-br", "Fonte de dados GitHub")];
    const PROVIDER_LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
        "pt-br",
        "Expõe dados tipados de repositório para workflows Ralph Engine.",
    )];

    #[test]
    fn parse_provider_kind_supports_stable_identifiers() {
        // Arrange
        let values = [
            "data_source",
            "context_provider",
            "forge_provider",
            "remote_control",
        ];

        // Act
        let parsed = values
            .into_iter()
            .map(parse_runtime_provider_kind)
            .collect::<Vec<_>>();

        // Assert
        assert_eq!(
            parsed,
            vec![
                Some(RuntimeProviderKind::DataSource),
                Some(RuntimeProviderKind::ContextProvider),
                Some(RuntimeProviderKind::ForgeProvider),
                Some(RuntimeProviderKind::RemoteControl),
            ]
        );
    }

    #[test]
    fn parse_provider_kind_rejects_unknown_identifier() {
        // Arrange
        let value = "unknown";

        // Act
        let parsed = parse_runtime_provider_kind(value);

        // Assert
        assert_eq!(parsed, None);
    }

    #[test]
    fn render_provider_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_provider_listing(&registrations, "en");

        // Assert
        assert_eq!(rendered, "Providers (0)");
    }

    #[test]
    fn render_provider_listing_deduplicates_provider_kinds() {
        // Arrange
        let registrations = [
            RuntimeProviderRegistration::new(
                RuntimeProviderKind::DataSource,
                "official.github",
                PluginActivation::Disabled,
                PluginLoadBoundary::InProcess,
                true,
            ),
            RuntimeProviderRegistration::new(
                RuntimeProviderKind::DataSource,
                "official.archive",
                PluginActivation::Enabled,
                PluginLoadBoundary::InProcess,
                false,
            ),
        ];

        // Act
        let rendered = render_provider_listing(&registrations, "en");

        // Assert
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains("- data_source | providers=2 | enabled=1"));
    }

    #[test]
    fn render_provider_detail_is_human_readable() {
        // Arrange
        let providers = [RuntimeProviderRegistration::new(
            RuntimeProviderKind::DataSource,
            "official.github",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        // Act
        let contributions = [OfficialProviderContribution {
            descriptor: PluginProviderDescriptor::new(
                "official.github.data",
                "official.github",
                PluginProviderKind::DataSource,
                "GitHub data source",
                PROVIDER_LOCALIZED_NAMES,
                "Exposes typed repository data to Ralph Engine workflows.",
                PROVIDER_LOCALIZED_SUMMARIES,
            ),
            activation: PluginActivation::Enabled,
            load_boundary: PluginLoadBoundary::InProcess,
            registration_hook_registered: true,
        }];

        let rendered = render_provider_detail(
            RuntimeProviderKind::DataSource,
            &providers,
            &contributions,
            "en",
        );

        // Assert
        assert!(rendered.contains("Provider: data_source"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- official.github.data | plugin=official.github | name=GitHub data source | summary=Exposes typed repository data to Ralph Engine workflows. | activation=enabled | boundary=in_process | registration_hook=true"
        ));
    }
}
