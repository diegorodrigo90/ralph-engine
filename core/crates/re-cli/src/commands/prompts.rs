//! Prompt provider command handlers.

use re_core::RuntimePromptRegistration;

use crate::{CliError, catalog};

/// Executes the prompts command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_prompt_listing(&catalog::official_runtime_prompts())),
        Some("show") => show_prompt(args.get(1).map(String::as_str)),
        Some(other) => Err(CliError::new(format!("unknown prompts command: {other}"))),
    }
}

fn show_prompt(plugin_id: Option<&str>) -> Result<String, CliError> {
    let plugin_id = plugin_id.ok_or_else(|| CliError::new("prompts show requires a plugin id"))?;
    let prompts = catalog::official_runtime_prompts()
        .into_iter()
        .filter(|registration| registration.plugin_id == plugin_id)
        .collect::<Vec<_>>();

    if prompts.is_empty() {
        return Err(CliError::new(format!(
            "unknown prompt provider: {plugin_id}"
        )));
    }

    Ok(render_prompt_detail(plugin_id, &prompts))
}

fn render_prompt_listing(registrations: &[RuntimePromptRegistration]) -> String {
    if registrations.is_empty() {
        return "Prompts (0)".to_owned();
    }

    let lines = registrations
        .iter()
        .map(|registration| {
            format!(
                "- {} | activation={} | boundary={} | prompt_hook={}",
                registration.plugin_id,
                registration.activation.as_str(),
                registration.load_boundary.as_str(),
                registration.prompt_hook_registered
            )
        })
        .collect::<Vec<_>>();

    format!("Prompts ({})\n{}", lines.len(), lines.join("\n"))
}

fn render_prompt_detail(plugin_id: &str, prompts: &[RuntimePromptRegistration]) -> String {
    let mut lines = vec![
        format!("Prompt provider: {plugin_id}"),
        format!("Providers ({})", prompts.len()),
    ];

    for prompt in prompts {
        lines.push(format!(
            "- {} | activation={} | boundary={} | prompt_hook={}",
            prompt.plugin_id,
            prompt.activation.as_str(),
            prompt.load_boundary.as_str(),
            prompt.prompt_hook_registered
        ));
    }

    lines.join("\n")
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
        let rendered = render_prompt_listing(&registrations);

        // Assert
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
        let rendered = render_prompt_detail("official.bmad", &prompts);

        // Assert
        assert!(rendered.contains("Prompt provider: official.bmad"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains(
            "- official.bmad | activation=disabled | boundary=in_process | prompt_hook=true"
        ));
    }
}
