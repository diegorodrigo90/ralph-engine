//! Shared rendering helpers for plugin-owned runtime surfaces.

use crate::i18n;

pub(crate) fn render_plugin_owned_surface_listing<T>(
    registrations: &[T],
    locale: &str,
    plural_label: fn(&str) -> &'static str,
    line_of: fn(&T) -> String,
) -> String {
    let surface_label = plural_label(locale);

    if registrations.is_empty() {
        return i18n::list_heading(locale, surface_label, surface_label, 0);
    }

    let lines = registrations.iter().map(line_of).collect::<Vec<_>>();

    format!(
        "{}\n{}",
        i18n::list_heading(locale, surface_label, surface_label, lines.len()),
        lines.join("\n")
    )
}

pub(crate) fn render_plugin_owned_surface_detail<T>(
    plugin_id: &str,
    registrations: &[T],
    locale: &str,
    singular_label: fn(&str) -> &'static str,
    line_of: fn(&T) -> String,
) -> String {
    let surface_label = singular_label(locale);
    let mut lines = vec![
        i18n::detail_heading(locale, surface_label, surface_label, plugin_id),
        i18n::providers_heading(locale, registrations.len()),
    ];

    lines.extend(registrations.iter().map(line_of));
    lines.join("\n")
}
