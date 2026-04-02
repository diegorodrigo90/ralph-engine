//! Plugin command handlers.

use re_config::{
    ConfigScope, PluginActivation, ResolvedPluginConfig, default_project_config_layer,
    resolve_plugin_config,
};
use re_plugin::{render_plugin_detail_for_locale, render_plugin_listing_for_locale};

use crate::{CliError, catalog, i18n};

/// Executes the plugins command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_plugin_listing_for_locale(
            &catalog::official_plugins(),
            locale,
        )),
        Some("show") => show_plugin(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "plugins", other,
        ))),
    }
}

fn show_plugin(plugin_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let plugin_id = plugin_id
        .ok_or_else(|| CliError::new(i18n::missing_id(locale, "plugins", "a plugin id")))?;
    let plugin = catalog::find_official_plugin(plugin_id)
        .ok_or_else(|| CliError::new(i18n::unknown_entity(locale, "plugin", plugin_id)))?;
    let layers = [default_project_config_layer()];
    let resolved = resolve_plugin_config(&layers, plugin.id).unwrap_or(ResolvedPluginConfig::new(
        plugin.id,
        PluginActivation::Disabled,
        ConfigScope::BuiltInDefaults,
    ));
    let mut detail = render_plugin_detail_for_locale(&plugin, locale);
    detail.push_str(&format!(
        "\n{}: {}",
        if i18n::is_pt_br(locale) {
            "Ativação resolvida"
        } else {
            "Resolved activation"
        },
        resolved.activation.as_str()
    ));
    detail.push_str(&format!(
        "\n{}: {}",
        if i18n::is_pt_br(locale) {
            "Resolvido de"
        } else {
            "Resolved from"
        },
        resolved.resolved_from.as_str()
    ));

    Ok(detail)
}
