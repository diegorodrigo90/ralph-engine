//! Prompt provider command handlers.

use re_core::RuntimePromptRegistration;

use crate::{
    CliError, catalog,
    commands::plugin_surfaces::{
        render_plugin_owned_surface_detail, render_plugin_owned_surface_listing,
    },
    i18n,
};

/// Executes the prompts command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_prompt_listing(
            &catalog::official_runtime_prompts(),
            locale,
        )),
        Some("show") => show_prompt(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "prompts", other,
        ))),
    }
}

fn show_prompt(plugin_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let plugin_id = plugin_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "prompts",
            i18n::plugin_id_entity_label(locale),
        ))
    })?;
    let prompts = catalog::find_official_runtime_prompts(plugin_id);

    if prompts.is_empty() {
        return Err(CliError::new(i18n::unknown_entity(
            locale,
            i18n::prompt_provider_entity_label(locale),
            plugin_id,
        )));
    }

    Ok(render_prompt_detail(plugin_id, &prompts, locale))
}

fn render_prompt_listing(registrations: &[RuntimePromptRegistration], locale: &str) -> String {
    render_plugin_owned_surface_listing(
        registrations,
        locale,
        i18n::prompts_label,
        render_prompt_registration,
    )
}

fn render_prompt_detail(
    plugin_id: &str,
    prompts: &[RuntimePromptRegistration],
    locale: &str,
) -> String {
    render_plugin_owned_surface_detail(
        plugin_id,
        prompts,
        locale,
        i18n::prompt_provider_label,
        render_prompt_registration,
    )
}

fn render_prompt_registration(registration: &RuntimePromptRegistration) -> String {
    format!(
        "- {} | activation={} | boundary={} | prompt_hook={}",
        registration.plugin_id,
        registration.activation.as_str(),
        registration.load_boundary.as_str(),
        registration.prompt_hook_registered
    )
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::RuntimePromptRegistration;
    use re_plugin::PluginLoadBoundary;

    use super::{render_prompt_detail, render_prompt_listing};

    #[test]
    fn render_prompt_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_prompt_listing(&registrations, "en");

        // Assert
        assert_eq!(rendered, "Prompts (0)");
    }

    #[test]
    fn render_prompt_listing_handles_empty_sets_in_pt_br() {
        let registrations = [];

        let rendered = render_prompt_listing(&registrations, "pt-br");

        assert_eq!(rendered, "Prompts (0)");
    }

    #[test]
    fn render_prompt_detail_is_human_readable() {
        // Arrange
        let prompts = [RuntimePromptRegistration::new(
            "official.bmad",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        // Act
        let rendered = render_prompt_detail("official.bmad", &prompts, "en");

        // Assert
        assert!(rendered.contains("Prompt provider: official.bmad"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- official.bmad | activation=disabled | boundary=in_process | prompt_hook=true"
        ));
    }

    #[test]
    fn render_prompt_detail_supports_pt_br() {
        let prompts = [RuntimePromptRegistration::new(
            "official.bmad",
            PluginActivation::Disabled,
            PluginLoadBoundary::InProcess,
            true,
        )];

        let rendered = render_prompt_detail("official.bmad", &prompts, "pt-br");

        assert!(rendered.contains("Provedor de prompt: official.bmad"));
        assert!(rendered.contains("Provedores (1)"));
    }
}
