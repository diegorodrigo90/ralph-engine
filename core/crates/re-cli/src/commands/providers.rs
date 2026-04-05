//! Provider command handlers.

use re_core::{RuntimeProviderKind, RuntimeProviderRegistration, parse_runtime_provider_kind};

use super::format;
use crate::{CliError, catalog, i18n};

/// Executes the providers command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_provider_listing(
            &catalog::official_runtime_providers(),
            locale,
        )),
        Some("show") => show_provider(args.get(1).map(String::as_str), locale),
        Some("plan") => show_provider_plan(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::usage(i18n::unknown_subcommand(
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

    if let Some(surface) = catalog::find_official_provider_surface(provider_id) {
        return Ok(render_provider_contribution_detail(
            surface.contribution,
            surface.registration,
            locale,
        ));
    }

    let kind = parse_runtime_provider_kind(provider_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
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

fn show_provider_plan(provider_kind: Option<&str>, locale: &str) -> Result<String, CliError> {
    let provider_id = provider_kind.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "providers",
            i18n::provider_id_entity_label(locale),
        ))
    })?;

    let surface = catalog::find_official_provider_surface(provider_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::provider_entity_label(locale),
            provider_id,
        ))
    })?;

    let plan = re_core::RuntimeProviderRegistrationPlan::new(
        surface.registration.kind,
        surface.registration.plugin_id,
        surface.registration.load_boundary,
        match surface.registration.kind {
            RuntimeProviderKind::DataSource => re_plugin::PluginRuntimeHook::DataSourceRegistration,
            RuntimeProviderKind::ContextProvider => {
                re_plugin::PluginRuntimeHook::ContextProviderRegistration
            }
            RuntimeProviderKind::ForgeProvider => {
                re_plugin::PluginRuntimeHook::ForgeProviderRegistration
            }
            RuntimeProviderKind::RemoteControl => {
                re_plugin::PluginRuntimeHook::RemoteControlBootstrap
            }
        },
        surface.registration.registration_hook_registered,
    );

    Ok(render_provider_plan(surface.contribution, plan, locale))
}

fn render_provider_listing(registrations: &[RuntimeProviderRegistration], locale: &str) -> String {
    let mut seen = Vec::new();
    let mut grouped_rows: Vec<Vec<String>> = Vec::new();

    for reg in registrations {
        let key = reg.kind.as_str();
        if seen.contains(&key) {
            continue;
        }
        seen.push(key);

        let all = registrations
            .iter()
            .filter(|r| r.kind.as_str() == key)
            .collect::<Vec<_>>();
        let enabled = all.iter().filter(|r| r.is_enabled()).count();

        grouped_rows.push(vec![
            key.to_owned(),
            all.len().to_string(),
            enabled.to_string(),
        ]);
    }

    let label = i18n::providers_label(locale);
    let heading = i18n::list_heading(locale, label, label, grouped_rows.len());

    if grouped_rows.is_empty() {
        return heading;
    }

    let headers = &["PROVIDER", "COUNT", "ENABLED"];
    format!(
        "{heading}\n\n{}",
        format::render_table(headers, &grouped_rows)
    )
}

fn render_provider_detail(
    provider_kind: RuntimeProviderKind,
    providers: &[RuntimeProviderRegistration],
    contributions: &[catalog::OfficialProviderContribution],
    locale: &str,
) -> String {
    let label = i18n::provider_label(locale);
    let heading = i18n::detail_heading(locale, label, label, provider_kind.as_str());
    let providers_heading = i18n::providers_heading(locale, providers.len());

    let headers = &["ID", "PLUGIN", "NAME", "STATUS"];
    let rows: Vec<Vec<String>> = providers
        .iter()
        .map(|provider| {
            let contribution = contributions
                .iter()
                .find(|c| c.descriptor.plugin_id == provider.plugin_id);

            vec![
                contribution
                    .map_or(provider.plugin_id, |e| e.descriptor.id)
                    .to_owned(),
                provider.plugin_id.to_owned(),
                contribution
                    .map_or(provider.plugin_id, |e| {
                        e.descriptor.display_name_for_locale(locale)
                    })
                    .to_owned(),
                provider.activation.as_str().to_owned(),
            ]
        })
        .collect();

    format!(
        "{heading}\n{providers_heading}\n\n{}",
        format::render_table(headers, &rows)
    )
}

fn render_provider_contribution_detail(
    contribution: catalog::OfficialProviderContribution,
    registration: RuntimeProviderRegistration,
    locale: &str,
) -> String {
    let heading = format!("{}:", i18n::provider_label(locale));
    let pairs = vec![
        (heading.as_str(), contribution.descriptor.id.to_owned()),
        (
            i18n::name_label(locale),
            contribution
                .descriptor
                .display_name_for_locale(locale)
                .to_owned(),
        ),
        (
            i18n::summary_label(locale),
            contribution
                .descriptor
                .summary_for_locale(locale)
                .to_owned(),
        ),
        ("Plugin:", contribution.descriptor.plugin_id.to_owned()),
        ("", String::new()),
        (
            i18n::kind_label(locale),
            contribution.descriptor.kind.as_str().to_owned(),
        ),
        (
            i18n::activation_label(locale),
            registration.activation.as_str().to_owned(),
        ),
        (
            i18n::load_boundary_label(locale),
            registration.load_boundary.as_str().to_owned(),
        ),
        (
            i18n::registration_hook_label(locale),
            registration.registration_hook_registered.to_string(),
        ),
    ];

    format::render_detail(&pairs)
}

fn render_provider_plan(
    contribution: catalog::OfficialProviderContribution,
    plan: re_core::RuntimeProviderRegistrationPlan,
    locale: &str,
) -> String {
    let pairs = vec![
        (
            "Provider registration plan:",
            contribution.descriptor.id.to_owned(),
        ),
        (
            i18n::name_label(locale),
            contribution
                .descriptor
                .display_name_for_locale(locale)
                .to_owned(),
        ),
        ("Plugin:", contribution.descriptor.plugin_id.to_owned()),
        (
            i18n::kind_label(locale),
            contribution.descriptor.kind.as_str().to_owned(),
        ),
        (
            i18n::activation_label(locale),
            contribution.activation.as_str().to_owned(),
        ),
        (
            i18n::load_boundary_label(locale),
            plan.load_boundary.as_str().to_owned(),
        ),
        (
            i18n::registration_hook_label(locale),
            plan.registration_hook.as_str().to_owned(),
        ),
        ("registered:", plan.registration_hook_registered.to_string()),
    ];

    format::render_detail(&pairs)
}

#[cfg(test)]
mod tests {
    use crate::catalog::OfficialProviderContribution;
    use re_config::PluginActivation;
    use re_core::{
        RuntimeProviderKind, RuntimeProviderRegistration, RuntimeProviderRegistrationPlan,
        parse_runtime_provider_kind,
    };
    use re_plugin::{
        PluginLoadBoundary, PluginLocalizedText, PluginProviderDescriptor, PluginProviderKind,
        PluginRuntimeHook,
    };

    use super::{render_provider_detail, render_provider_listing, render_provider_plan};

    const PROVIDER_LOCALIZED_NAMES: &[PluginLocalizedText] =
        &[PluginLocalizedText::new("pt-br", "Fonte de dados GitHub")];
    const PROVIDER_LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
        "pt-br",
        "Expõe dados tipados de repositório para workflows Ralph Engine.",
    )];
    const PRIMARY_PLUGIN_ID: &str = "fixture.github";
    const SECONDARY_PLUGIN_ID: &str = "fixture.archive";
    const PROVIDER_ID: &str = "fixture.github.data";

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
        let registrations = [];

        let rendered = render_provider_listing(&registrations, "en");

        assert!(rendered.contains("Providers (0)"));
    }

    #[test]
    fn render_provider_listing_deduplicates_provider_kinds() {
        let registrations = [
            RuntimeProviderRegistration::new(
                PROVIDER_ID,
                RuntimeProviderKind::DataSource,
                PRIMARY_PLUGIN_ID,
                PluginActivation::Disabled,
                PluginLoadBoundary::InProcess,
                true,
            ),
            RuntimeProviderRegistration::new(
                "fixture.archive.data",
                RuntimeProviderKind::DataSource,
                SECONDARY_PLUGIN_ID,
                PluginActivation::Enabled,
                PluginLoadBoundary::InProcess,
                false,
            ),
        ];

        let rendered = render_provider_listing(&registrations, "en");

        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains("data_source"));
        assert!(rendered.contains("2"));
        assert!(rendered.contains("1"));
    }

    #[test]
    fn render_provider_detail_is_human_readable() {
        let providers = [RuntimeProviderRegistration::new(
            PROVIDER_ID,
            RuntimeProviderKind::DataSource,
            PRIMARY_PLUGIN_ID,
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        let contributions = [OfficialProviderContribution {
            descriptor: PluginProviderDescriptor::new(
                PROVIDER_ID,
                PRIMARY_PLUGIN_ID,
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

        assert!(rendered.contains("Provider: data_source"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains("fixture.github.data"));
        assert!(rendered.contains("fixture.github"));
        assert!(rendered.contains("GitHub data source"));
        assert!(rendered.contains("enabled"));
    }

    #[test]
    fn render_provider_plan_is_human_readable() {
        let contribution = OfficialProviderContribution {
            descriptor: PluginProviderDescriptor::new(
                PROVIDER_ID,
                PRIMARY_PLUGIN_ID,
                PluginProviderKind::DataSource,
                "GitHub data source",
                PROVIDER_LOCALIZED_NAMES,
                "Exposes typed repository data to Ralph Engine workflows.",
                PROVIDER_LOCALIZED_SUMMARIES,
            ),
            activation: PluginActivation::Enabled,
            load_boundary: PluginLoadBoundary::InProcess,
            registration_hook_registered: true,
        };

        let rendered = render_provider_plan(
            contribution,
            RuntimeProviderRegistrationPlan::new(
                RuntimeProviderKind::DataSource,
                PRIMARY_PLUGIN_ID,
                PluginLoadBoundary::InProcess,
                PluginRuntimeHook::DataSourceRegistration,
                true,
            ),
            "en",
        );

        assert!(rendered.contains("Provider registration plan:"));
        assert!(rendered.contains("fixture.github.data"));
        assert!(rendered.contains("fixture.github"));
        assert!(rendered.contains("data_source_registration"));
    }
}
