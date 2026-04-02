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
    let provider_kind = provider_kind.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "providers",
            i18n::provider_id_entity_label(locale),
        ))
    })?;
    let kind = parse_runtime_provider_kind(provider_kind).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::provider_entity_label(locale),
            provider_kind,
        ))
    })?;
    let providers = catalog::find_official_runtime_providers(kind);

    Ok(render_provider_detail(kind, &providers, locale))
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
    locale: &str,
) -> String {
    render_grouped_surface_detail(
        provider_kind.as_str(),
        providers,
        locale,
        i18n::provider_label,
        |provider| {
            format!(
                "- {} | activation={} | boundary={} | registration_hook={}",
                provider.plugin_id,
                provider.activation.as_str(),
                provider.load_boundary.as_str(),
                provider.registration_hook_registered
            )
        },
    )
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::{RuntimeProviderKind, RuntimeProviderRegistration};
    use re_plugin::PluginLoadBoundary;

    use super::{render_provider_detail, render_provider_listing};
    use re_core::parse_runtime_provider_kind;

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
        let rendered = render_provider_detail(RuntimeProviderKind::DataSource, &providers, "en");

        // Assert
        assert!(rendered.contains("Provider: data_source"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- official.github | activation=enabled | boundary=in_process | registration_hook=true"
        ));
    }
}
