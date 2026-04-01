//! Runtime-hook command handlers.

use re_core::RuntimeHookRegistration;

use crate::{CliError, catalog};

/// Executes the hooks command tree.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_hook_listing(&catalog::official_runtime_hooks())),
        Some("show") => show_hook(args.get(1).map(String::as_str)),
        Some(other) => Err(CliError::new(format!("unknown hooks command: {other}"))),
    }
}

fn show_hook(hook_id: Option<&str>) -> Result<String, CliError> {
    let hook_id = hook_id.ok_or_else(|| CliError::new("hooks show requires a hook id"))?;
    let providers = catalog::official_runtime_hooks()
        .into_iter()
        .filter(|registration| registration.hook.as_str() == hook_id)
        .collect::<Vec<_>>();

    if providers.is_empty() {
        return Err(CliError::new(format!("unknown hook: {hook_id}")));
    }

    Ok(render_hook_detail(hook_id, &providers))
}

fn render_hook_listing(registrations: &[RuntimeHookRegistration]) -> String {
    let mut seen = Vec::new();
    let mut lines = Vec::new();

    for registration in registrations {
        let hook = registration.hook.as_str();

        if seen.contains(&hook) {
            continue;
        }

        seen.push(hook);

        let providers = registrations
            .iter()
            .filter(|candidate| candidate.hook == registration.hook)
            .collect::<Vec<_>>();
        let enabled_providers = providers
            .iter()
            .filter(|provider| provider.is_enabled())
            .count();

        lines.push(format!(
            "- {} | providers={} | enabled={}",
            hook,
            providers.len(),
            enabled_providers
        ));
    }

    if lines.is_empty() {
        "Runtime hooks (0)".to_owned()
    } else {
        format!("Runtime hooks ({})\n{}", lines.len(), lines.join("\n"))
    }
}

fn render_hook_detail(hook_id: &str, providers: &[RuntimeHookRegistration]) -> String {
    let mut lines = vec![
        format!("Runtime hook: {hook_id}"),
        format!("Providers ({})", providers.len()),
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
    use re_core::RuntimeHookRegistration;
    use re_plugin::{PluginLoadBoundary, PluginRuntimeHook};

    use super::{render_hook_detail, render_hook_listing};

    #[test]
    fn render_hook_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_hook_listing(&registrations);

        // Assert
        assert_eq!(rendered, "Runtime hooks (0)");
    }

    #[test]
    fn render_hook_detail_is_human_readable() {
        // Arrange
        let providers = [RuntimeHookRegistration::new(
            PluginRuntimeHook::Scaffold,
            "official.basic",
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
        )];

        // Act
        let rendered = render_hook_detail("scaffold", &providers);

        // Assert
        assert!(rendered.contains("Runtime hook: scaffold"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains("- official.basic | activation=enabled | boundary=in_process"));
    }
}
