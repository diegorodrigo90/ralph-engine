//! Template command handlers.

use crate::{CliError, catalog, i18n};

use catalog::OfficialTemplateContribution;

/// Executes the templates command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_template_listing(
            &catalog::official_template_contributions(),
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

fn show_template(template_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let template_id = template_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "templates",
            i18n::template_id_entity_label(locale),
        ))
    })?;
    let template = catalog::find_official_template_contribution(template_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::template_entity_label(locale),
            template_id,
        ))
    })?;

    Ok(render_template_detail(template, locale))
}

fn render_template_listing(registrations: &[OfficialTemplateContribution], locale: &str) -> String {
    let mut lines = Vec::with_capacity(registrations.len() + 1);
    lines.push(i18n::list_heading(
        locale,
        "Templates",
        "Templates",
        registrations.len(),
    ));

    for registration in registrations {
        lines.push(format!(
            "- {} | {} | plugin={} | activation={}",
            registration.descriptor.id,
            registration.descriptor.display_name_for_locale(locale),
            registration.descriptor.plugin_id,
            registration.activation.as_str(),
        ));
    }

    lines.join("\n")
}

fn render_template_detail(template: OfficialTemplateContribution, locale: &str) -> String {
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
    let hook_label = if i18n::is_pt_br(locale) {
        "Hook de runtime"
    } else {
        "Runtime hook"
    };

    format!(
        "Template: {}\n{name_label}: {}\n{summary_label}: {}\nPlugin: {}\n{}: {}\n{}: {}\n{hook_label}: {}",
        template.descriptor.id,
        template.descriptor.display_name_for_locale(locale),
        template.descriptor.summary_for_locale(locale),
        template.descriptor.plugin_id,
        i18n::activation_label(locale),
        template.activation.as_str(),
        i18n::load_boundary_label(locale),
        template.load_boundary.as_str(),
        if template.scaffold_hook_registered {
            "scaffold"
        } else {
            "missing"
        },
    )
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_plugin::{PluginLoadBoundary, PluginLocalizedText, PluginTemplateDescriptor};

    use super::{OfficialTemplateContribution, render_template_detail, render_template_listing};

    const LOCALIZED_NAMES: &[PluginLocalizedText] =
        &[PluginLocalizedText::new("pt-br", "Starter básico")];
    const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
        "pt-br",
        "Template inicial para novos projetos Ralph Engine.",
    )];

    fn template_descriptor() -> PluginTemplateDescriptor {
        PluginTemplateDescriptor::new(
            "official.basic.starter",
            "official.basic",
            "Basic starter",
            LOCALIZED_NAMES,
            "Starter template for new Ralph Engine projects.",
            LOCALIZED_SUMMARIES,
        )
    }

    #[test]
    fn render_template_listing_handles_empty_sets() {
        let registrations = [];

        let rendered = render_template_listing(&registrations, "en");

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
        let rendered = render_template_detail(
            OfficialTemplateContribution {
                descriptor: template_descriptor(),
                activation: PluginActivation::Enabled,
                load_boundary: PluginLoadBoundary::InProcess,
                scaffold_hook_registered: true,
            },
            "en",
        );

        assert!(rendered.contains("Template: official.basic.starter"));
        assert!(rendered.contains("Name: Basic starter"));
        assert!(rendered.contains("Plugin: official.basic"));
        assert!(rendered.contains("Activation: enabled"));
        assert!(rendered.contains("Runtime hook: scaffold"));
    }

    #[test]
    fn render_template_detail_supports_pt_br() {
        let rendered = render_template_detail(
            OfficialTemplateContribution {
                descriptor: template_descriptor(),
                activation: PluginActivation::Enabled,
                load_boundary: PluginLoadBoundary::InProcess,
                scaffold_hook_registered: true,
            },
            "pt-br",
        );

        assert!(rendered.contains("Template: official.basic.starter"));
        assert!(rendered.contains("Nome: Starter básico"));
        assert!(rendered.contains("Resumo: Template inicial para novos projetos Ralph Engine."));
        assert!(rendered.contains("Plugin: official.basic"));
    }
}
