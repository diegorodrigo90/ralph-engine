//! Locale catalog command handlers.

use re_config::{
    find_locale_descriptor, render_locale_descriptor_yaml, render_supported_locales_yaml,
    supported_locales,
};

use crate::{CliError, i18n};

/// Executes the locales command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_supported_locales_yaml(supported_locales())),
        Some("show") => show_locale(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "locales", other,
        ))),
    }
}

fn show_locale(locale_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let locale_id = locale_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "locales",
            i18n::locale_id_entity_label(locale),
        ))
    })?;
    let supported_locale = find_locale_descriptor(locale_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::locale_entity_label(locale),
            locale_id,
        ))
    })?;

    Ok(render_locale_descriptor_yaml(&supported_locale))
}
