//! Capability command handlers.

use re_core::RuntimeCapabilityRegistration;
use re_plugin::parse_reviewed_plugin_capability;

use super::format;
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
    // Group by capability kind (deduplicate)
    let mut seen = Vec::new();
    let mut grouped_rows: Vec<Vec<String>> = Vec::new();

    for reg in registrations {
        let key = reg.capability.as_str();
        if seen.contains(&key) {
            continue;
        }
        seen.push(key);

        let all = registrations
            .iter()
            .filter(|r| r.capability.as_str() == key)
            .collect::<Vec<_>>();
        let enabled = all.iter().filter(|r| r.is_enabled()).count();

        grouped_rows.push(vec![
            key.to_owned(),
            all.len().to_string(),
            enabled.to_string(),
        ]);
    }

    let label = i18n::capabilities_label(locale);
    let heading = i18n::list_heading(locale, label, label, grouped_rows.len());

    if grouped_rows.is_empty() {
        return heading;
    }

    let headers = &["CAPABILITY", "PROVIDERS", "ENABLED"];
    format!(
        "{heading}\n\n{}",
        format::render_table(headers, &grouped_rows)
    )
}

fn render_capability_detail(
    capability_id: &str,
    providers: &[RuntimeCapabilityRegistration],
    locale: &str,
) -> String {
    let label = i18n::capability_label(locale);
    let heading = i18n::detail_heading(locale, label, label, capability_id);
    let providers_heading = i18n::providers_heading(locale, providers.len());

    let headers = &["PLUGIN", "STATUS", "BOUNDARY"];
    let rows: Vec<Vec<String>> = providers
        .iter()
        .map(|p| {
            vec![
                p.plugin_id.to_owned(),
                p.activation.as_str().to_owned(),
                p.load_boundary.as_str().to_owned(),
            ]
        })
        .collect();

    format!(
        "{heading}\n{providers_heading}\n\n{}",
        format::render_table(headers, &rows)
    )
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::RuntimeCapabilityRegistration;
    use re_plugin::{PluginCapability, PluginLoadBoundary};

    use super::{render_capability_detail, render_capability_listing};

    const PLUGIN_ID: &str = "fixture.templates";

    #[test]
    fn render_capability_listing_handles_empty_sets() {
        let registrations = [];

        let rendered = render_capability_listing(&registrations, "en");

        assert!(rendered.contains("Capabilities (0)"));
    }

    #[test]
    fn render_capability_detail_is_human_readable() {
        let providers = [RuntimeCapabilityRegistration::new(
            PluginCapability::new("template"),
            PLUGIN_ID,
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
        )];

        let rendered = render_capability_detail("template", &providers, "en");

        assert!(rendered.contains("Capability: template"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains("fixture.templates"));
        assert!(rendered.contains("enabled"));
        assert!(rendered.contains("in_process"));
    }

    #[test]
    fn render_capability_detail_supports_pt_br() {
        let providers = [RuntimeCapabilityRegistration::new(
            PluginCapability::new("template"),
            PLUGIN_ID,
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
        )];

        let rendered = render_capability_detail("template", &providers, "pt-br");

        assert!(rendered.contains("Capacidade: template"));
        assert!(rendered.contains("Provedores (1)"));
    }
}
