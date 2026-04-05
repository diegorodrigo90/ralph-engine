//! Plugin command handlers.

use re_config::{
    ConfigScope, PluginActivation, ResolvedPluginConfig, default_project_config_layer,
    resolve_plugin_config,
};
use re_plugin::PluginDescriptor;

use super::format;
use crate::{CliError, catalog, i18n};

/// Executes the plugins command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_plugin_list(locale)),
        Some("show") => show_plugin(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::usage(i18n::unknown_subcommand(
            locale, "plugins", other,
        ))),
    }
}

/// Renders the plugin list as a clean table.
fn render_plugin_list(locale: &str) -> String {
    let plugins = catalog::official_plugins();
    let heading = format::render_count_heading("Plugins", plugins.len());

    let headers = &["PLUGIN", "KIND", "STATUS", "DESCRIPTION"];
    let layers = [default_project_config_layer()];

    let rows: Vec<Vec<String>> = plugins
        .iter()
        .map(|p| {
            let resolved =
                resolve_plugin_config(&layers, p.id).unwrap_or(ResolvedPluginConfig::new(
                    p.id,
                    PluginActivation::Disabled,
                    ConfigScope::BuiltInDefaults,
                ));
            vec![
                p.id.to_owned(),
                p.kind.to_string(),
                resolved.activation.as_str().to_owned(),
                p.summary_for_locale(locale).to_owned(),
            ]
        })
        .collect();

    format!("{heading}\n\n{}", format::render_table(headers, &rows))
}

/// Shows detailed info for one plugin.
fn show_plugin(plugin_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let plugin_id = plugin_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "plugins",
            i18n::plugin_id_entity_label(locale),
        ))
    })?;
    let plugin = catalog::find_official_plugin(plugin_id).ok_or_else(|| {
        CliError::usage(i18n::unknown_entity(
            locale,
            i18n::plugin_entity_label(locale),
            plugin_id,
        ))
    })?;
    let layers = [default_project_config_layer()];
    let resolved = resolve_plugin_config(&layers, plugin.id).unwrap_or(ResolvedPluginConfig::new(
        plugin.id,
        PluginActivation::Disabled,
        ConfigScope::BuiltInDefaults,
    ));

    Ok(render_plugin_detail(&plugin, &resolved, locale))
}

/// Renders the detail view for one plugin (kubectl describe pattern).
fn render_plugin_detail(
    plugin: &PluginDescriptor,
    resolved: &ResolvedPluginConfig,
    locale: &str,
) -> String {
    let capabilities = plugin
        .capabilities
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");

    let lifecycle = plugin
        .lifecycle
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");

    let hooks = plugin
        .runtime_hooks
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");

    let pairs = vec![
        ("Plugin:", plugin.id.to_owned()),
        (
            i18n::name_label(locale),
            plugin.display_name_for_locale(locale).to_owned(),
        ),
        (i18n::kind_label(locale), plugin.kind.to_string()),
        ("Trust:", plugin.trust_level.to_string()),
        ("Version:", plugin.version.to_owned()),
        (
            i18n::summary_label(locale),
            plugin.summary_for_locale(locale).to_owned(),
        ),
        ("", String::new()),
        (
            i18n::activation_label(locale),
            resolved.activation.as_str().to_owned(),
        ),
        (
            i18n::resolved_from_label(locale),
            resolved.resolved_from.as_str().to_owned(),
        ),
        (
            i18n::load_boundary_label(locale),
            plugin.load_boundary.as_str().to_owned(),
        ),
        ("", String::new()),
        (i18n::capabilities_label(locale), capabilities),
        ("Lifecycle:", lifecycle),
        (i18n::hooks_label(locale), hooks),
    ];

    format::render_detail(&pairs)
}
