//! Template command handlers.

use re_core::RuntimeTemplateRegistration;

use crate::{CliError, catalog};

/// Executes the templates command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_template_listing(
            &catalog::official_runtime_templates(),
        )),
        Some("show") => show_template(args.get(1).map(String::as_str)),
        Some(other) => Err(CliError::new(format!("unknown templates command: {other}"))),
    }
}

fn show_template(plugin_id: Option<&str>) -> Result<String, CliError> {
    let plugin_id =
        plugin_id.ok_or_else(|| CliError::new("templates show requires a plugin id"))?;
    let templates = catalog::official_runtime_templates()
        .into_iter()
        .filter(|registration| registration.plugin_id == plugin_id)
        .collect::<Vec<_>>();

    if templates.is_empty() {
        return Err(CliError::new(format!(
            "unknown template provider: {plugin_id}"
        )));
    }

    Ok(render_template_detail(plugin_id, &templates))
}

fn render_template_listing(registrations: &[RuntimeTemplateRegistration]) -> String {
    if registrations.is_empty() {
        return "Templates (0)".to_owned();
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

    format!("Templates ({})\n{}", lines.len(), lines.join("\n"))
}

fn render_template_detail(plugin_id: &str, templates: &[RuntimeTemplateRegistration]) -> String {
    let mut lines = vec![
        format!("Template provider: {plugin_id}"),
        format!("Providers ({})", templates.len()),
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
        let rendered = render_template_listing(&registrations);

        // Assert
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
        let rendered = render_template_detail("official.basic", &templates);

        // Assert
        assert!(rendered.contains("Template provider: official.basic"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- official.basic | activation=enabled | boundary=in_process | scaffold_hook=true"
        ));
    }
}
