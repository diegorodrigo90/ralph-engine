//! Provider command handlers.

use re_core::{RuntimeProviderKind, RuntimeProviderRegistration};

use crate::{CliError, catalog};

/// Executes the providers command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_provider_listing(
            &catalog::official_runtime_providers(),
        )),
        Some("show") => show_provider(args.get(1).map(String::as_str)),
        Some(other) => Err(CliError::new(format!("unknown providers command: {other}"))),
    }
}

fn show_provider(provider_kind: Option<&str>) -> Result<String, CliError> {
    let provider_kind =
        provider_kind.ok_or_else(|| CliError::new("providers show requires a provider id"))?;
    let kind = parse_provider_kind(provider_kind)
        .ok_or_else(|| CliError::new(format!("unknown provider: {provider_kind}")))?;
    let providers = catalog::official_runtime_providers()
        .into_iter()
        .filter(|registration| registration.kind == kind)
        .collect::<Vec<_>>();

    Ok(render_provider_detail(kind, &providers))
}

fn parse_provider_kind(value: &str) -> Option<RuntimeProviderKind> {
    match value {
        "data_source" => Some(RuntimeProviderKind::DataSource),
        "context_provider" => Some(RuntimeProviderKind::ContextProvider),
        "forge_provider" => Some(RuntimeProviderKind::ForgeProvider),
        "remote_control" => Some(RuntimeProviderKind::RemoteControl),
        _ => None,
    }
}

fn render_provider_listing(registrations: &[RuntimeProviderRegistration]) -> String {
    let mut seen = Vec::new();
    let mut lines = Vec::new();

    for registration in registrations {
        let provider = registration.kind.as_str();

        if seen.contains(&provider) {
            continue;
        }

        seen.push(provider);

        let providers = registrations
            .iter()
            .filter(|candidate| candidate.kind == registration.kind)
            .collect::<Vec<_>>();
        let enabled_providers = providers
            .iter()
            .filter(|provider| provider.is_enabled())
            .count();

        lines.push(format!(
            "- {} | providers={} | enabled={}",
            provider,
            providers.len(),
            enabled_providers
        ));
    }

    if lines.is_empty() {
        "Providers (0)".to_owned()
    } else {
        format!("Providers ({})\n{}", lines.len(), lines.join("\n"))
    }
}

fn render_provider_detail(
    provider_kind: RuntimeProviderKind,
    providers: &[RuntimeProviderRegistration],
) -> String {
    let mut lines = vec![
        format!("Provider: {}", provider_kind.as_str()),
        format!("Providers ({})", providers.len()),
    ];

    for provider in providers {
        lines.push(format!(
            "- {} | activation={} | boundary={} | registration_hook={}",
            provider.plugin_id,
            provider.activation.as_str(),
            provider.load_boundary.as_str(),
            provider.registration_hook_registered
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::{RuntimeProviderKind, RuntimeProviderRegistration};
    use re_plugin::PluginLoadBoundary;

    use super::{parse_provider_kind, render_provider_detail, render_provider_listing};

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
            .map(parse_provider_kind)
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
        let parsed = parse_provider_kind(value);

        // Assert
        assert_eq!(parsed, None);
    }

    #[test]
    fn render_provider_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_provider_listing(&registrations);

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
        let rendered = render_provider_listing(&registrations);

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
        let rendered = render_provider_detail(RuntimeProviderKind::DataSource, &providers);

        // Assert
        assert!(rendered.contains("Provider: data_source"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- official.github | activation=enabled | boundary=in_process | registration_hook=true"
        ));
    }
}
