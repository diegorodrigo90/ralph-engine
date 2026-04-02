//! Template command handlers.

use re_core::RuntimeTemplateRegistration;

use crate::{CliError, catalog, i18n};

/// Executes the templates command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_template_listing(
            &catalog::official_runtime_templates(),
            locale,
        )),
        Some("show") => show_template(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale,
            "templates",
            other,
        ))),
    }
}

fn show_template(plugin_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let plugin_id = plugin_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "templates",
            i18n::plugin_id_entity_label(locale),
        ))
    })?;
    let templates = catalog::find_official_runtime_templates(plugin_id);

    if templates.is_empty() {
        return Err(CliError::new(i18n::unknown_entity(
            locale,
            i18n::template_provider_entity_label(locale),
            plugin_id,
        )));
    }

    Ok(render_template_detail(plugin_id, &templates, locale))
}

fn render_template_listing(registrations: &[RuntimeTemplateRegistration], locale: &str) -> String {
    if registrations.is_empty() {
        return i18n::list_heading(
            locale,
            i18n::templates_label(locale),
            i18n::templates_label(locale),
            0,
        );
    }

    let lines = registrations
        .iter()
        .map(|registration| {
            format!(
                "- {} | activation={} | boundary={} | scaffold_hook={}",
                registration.plugin_id,
                registration.activation.as_str(),
                registration.load_boundary.as_str(),
                registration.scaffold_hook_registered
            )
        })
        .collect::<Vec<_>>();

    format!(
        "{}\n{}",
        i18n::list_heading(
            locale,
            i18n::templates_label(locale),
            i18n::templates_label(locale),
            lines.len()
        ),
        lines.join("\n")
    )
}

fn render_template_detail(
    plugin_id: &str,
    templates: &[RuntimeTemplateRegistration],
    locale: &str,
) -> String {
    let mut lines = vec![
        i18n::detail_heading(
            locale,
            i18n::template_provider_label(locale),
            i18n::template_provider_label(locale),
            plugin_id,
        ),
        i18n::providers_heading(locale, templates.len()),
    ];

    for template in templates {
        lines.push(format!(
            "- {} | activation={} | boundary={} | scaffold_hook={}",
            template.plugin_id,
            template.activation.as_str(),
            template.load_boundary.as_str(),
            template.scaffold_hook_registered
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::RuntimeTemplateRegistration;
    use re_plugin::PluginLoadBoundary;

    use super::{render_template_detail, render_template_listing};

    #[test]
    fn render_template_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_template_listing(&registrations, "en");

        // Assert
        assert_eq!(rendered, "Templates (0)");
    }

    #[test]
    fn render_template_listing_handles_empty_sets_in_pt_br() {
        let registrations = [];

        let rendered = render_template_listing(&registrations, "pt-br");

        assert_eq!(rendered, "Templates (0)");
    }

    #[test]
    fn render_template_detail_is_human_readable() {
        // Arrange
        let templates = [RuntimeTemplateRegistration::new(
            "official.basic",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        // Act
        let rendered = render_template_detail("official.basic", &templates, "en");

        // Assert
        assert!(rendered.contains("Template provider: official.basic"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- official.basic | activation=enabled | boundary=in_process | scaffold_hook=true"
        ));
    }

    #[test]
    fn render_template_detail_supports_pt_br() {
        let templates = [RuntimeTemplateRegistration::new(
            "official.basic",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        let rendered = render_template_detail("official.basic", &templates, "pt-br");

        assert!(rendered.contains("Provedor de template: official.basic"));
        assert!(rendered.contains("Provedores (1)"));
    }
}
