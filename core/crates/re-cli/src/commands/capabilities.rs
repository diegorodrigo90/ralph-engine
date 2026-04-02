//! Capability command handlers.

use re_core::RuntimeCapabilityRegistration;

use crate::{CliError, catalog, i18n};

/// Executes the capabilities command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_capability_listing(
            &catalog::official_runtime_capabilities(),
            locale,
        )),
        Some("show") => show_capability(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale,
            "capabilities",
            other,
        ))),
    }
}

fn show_capability(capability_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let capability_id = capability_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(locale, "capabilities", "a capability id"))
    })?;
    let providers = catalog::official_runtime_capabilities()
        .into_iter()
        .filter(|registration| registration.capability.as_str() == capability_id)
        .collect::<Vec<_>>();

    if providers.is_empty() {
        return Err(CliError::new(i18n::unknown_entity(
            locale,
            "capability",
            capability_id,
        )));
    }

    Ok(render_capability_detail(capability_id, &providers, locale))
}

fn render_capability_listing(
    registrations: &[RuntimeCapabilityRegistration],
    locale: &str,
) -> String {
    let mut seen = Vec::new();
    let mut lines = Vec::new();

    for registration in registrations {
        let capability = registration.capability.as_str();

        if seen.contains(&capability) {
            continue;
        }

        seen.push(capability);

        let providers = registrations
            .iter()
            .filter(|candidate| candidate.capability == registration.capability)
            .collect::<Vec<_>>();
        let enabled_providers = providers
            .iter()
            .filter(|provider| provider.is_enabled())
            .count();

        lines.push(format!(
            "- {} | providers={} | enabled={}",
            capability,
            providers.len(),
            enabled_providers
        ));
    }

    if lines.is_empty() {
        i18n::list_heading(locale, "Capabilities", "Capabilities", 0)
    } else {
        format!(
            "{}\n{}",
            i18n::list_heading(locale, "Capabilities", "Capabilities", lines.len()),
            lines.join("\n")
        )
    }
}

fn render_capability_detail(
    capability_id: &str,
    providers: &[RuntimeCapabilityRegistration],
    locale: &str,
) -> String {
    let mut lines = vec![
        i18n::detail_heading(locale, "Capability", "Capability", capability_id),
        i18n::providers_heading(locale, providers.len()),
    ];

    for provider in providers {
        lines.push(format!(
            "- {} | activation={} | boundary={}",
            provider.plugin_id,
            provider.activation.as_str(),
            provider.load_boundary.as_str()
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::RuntimeCapabilityRegistration;
    use re_plugin::{PluginCapability, PluginLoadBoundary};

    use super::{render_capability_detail, render_capability_listing};

    #[test]
    fn render_capability_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_capability_listing(&registrations, "en");

        // Assert
        assert_eq!(rendered, "Capabilities (0)");
    }

    #[test]
    fn render_capability_detail_is_human_readable() {
        // Arrange
        let providers = [RuntimeCapabilityRegistration::new(
            PluginCapability::new("template"),
            "official.basic",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
        )];

        // Act
        let rendered = render_capability_detail("template", &providers, "en");

        // Assert
        assert!(rendered.contains("Capability: template"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains("- official.basic | activation=enabled | boundary=in_process"));
    }

    #[test]
    fn render_capability_detail_supports_pt_br() {
        let providers = [RuntimeCapabilityRegistration::new(
            PluginCapability::new("template"),
            "official.basic",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
        )];

        let rendered = render_capability_detail("template", &providers, "pt-br");

        assert!(rendered.contains("Capability: template"));
        assert!(rendered.contains("Provedores (1)"));
    }
}
