//! Runtime-hook command handlers.

use re_core::RuntimeHookRegistration;
use re_plugin::parse_plugin_runtime_hook;

use crate::{
    CliError, catalog,
    commands::grouped_surfaces::{render_grouped_surface_detail, render_grouped_surface_listing},
    i18n,
};

/// Executes the hooks command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_hook_listing(
            &catalog::official_runtime_hooks(),
            locale,
        )),
        Some("show") => show_hook(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "hooks", other,
        ))),
    }
}

fn show_hook(hook_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let hook_id = hook_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "hooks",
            i18n::hook_id_entity_label(locale),
        ))
    })?;
    let hook = parse_plugin_runtime_hook(hook_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::hook_entity_label(locale),
            hook_id,
        ))
    })?;
    let providers = catalog::find_official_runtime_hooks(hook);

    Ok(render_hook_detail(hook_id, &providers, locale))
}

fn render_hook_listing(registrations: &[RuntimeHookRegistration], locale: &str) -> String {
    render_grouped_surface_listing(
        registrations,
        locale,
        i18n::hooks_label,
        |registration| registration.hook.as_str(),
        |registration| registration.is_enabled(),
    )
}

fn render_hook_detail(
    hook_id: &str,
    providers: &[RuntimeHookRegistration],
    locale: &str,
) -> String {
    render_grouped_surface_detail(hook_id, providers, locale, i18n::hook_label, |provider| {
        format!(
            "- {} | activation={} | boundary={}",
            provider.plugin_id,
            provider.activation.as_str(),
            provider.load_boundary.as_str()
        )
    })
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
        let rendered = render_hook_listing(&registrations, "en");

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
        let rendered = render_hook_detail("scaffold", &providers, "en");

        // Assert
        assert!(rendered.contains("Runtime hook: scaffold"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains("- official.basic | activation=enabled | boundary=in_process"));
    }
}
