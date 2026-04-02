//! Capability command handlers.

use re_core::RuntimeCapabilityRegistration;
use re_plugin::parse_reviewed_plugin_capability;

use crate::{
    CliError, catalog,
    commands::grouped_surfaces::{render_grouped_surface_detail, render_grouped_surface_listing},
    i18n,
};

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
        CliError::new(i18n::missing_id(
            locale,
            "capabilities",
            i18n::capability_id_entity_label(locale),
        ))
    })?;
    let capability = parse_reviewed_plugin_capability(capability_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::capability_entity_label(locale),
            capability_id,
        ))
    })?;
    let providers = catalog::find_official_runtime_capabilities(capability);

    Ok(render_capability_detail(capability_id, &providers, locale))
}

fn render_capability_listing(
    registrations: &[RuntimeCapabilityRegistration],
    locale: &str,
) -> String {
    render_grouped_surface_listing(
        registrations,
        locale,
        i18n::capabilities_label,
        |registration| registration.capability.as_str(),
        |registration| registration.is_enabled(),
    )
}

fn render_capability_detail(
    capability_id: &str,
    providers: &[RuntimeCapabilityRegistration],
    locale: &str,
) -> String {
    render_grouped_surface_detail(
        capability_id,
        providers,
        locale,
        i18n::capability_label,
        |provider| {
            format!(
                "- {} | activation={} | boundary={}",
                provider.plugin_id,
                provider.activation.as_str(),
                provider.load_boundary.as_str()
            )
        },
    )
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

        assert!(rendered.contains("Capacidade: template"));
        assert!(rendered.contains("Provedores (1)"));
    }
}
