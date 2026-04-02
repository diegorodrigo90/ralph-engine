//! Shared rendering helpers for grouped runtime surfaces.

use crate::i18n;

pub(crate) fn render_grouped_surface_listing<T>(
    registrations: &[T],
    locale: &str,
    plural_label: fn(&str) -> &'static str,
    group_key_of: fn(&T) -> &'static str,
    is_enabled: fn(&T) -> bool,
) -> String {
    let mut seen = Vec::new();
    let mut lines = Vec::new();

    for registration in registrations {
        let key = group_key_of(registration);

        if seen.contains(&key) {
            continue;
        }

        seen.push(key);

        let grouped = registrations
            .iter()
            .filter(|candidate| group_key_of(candidate) == key)
            .collect::<Vec<_>>();
        let enabled = grouped
            .iter()
            .filter(|candidate| is_enabled(candidate))
            .count();

        lines.push(format!(
            "- {} | providers={} | enabled={}",
            key,
            grouped.len(),
            enabled
        ));
    }

    let surface_label = plural_label(locale);

    if lines.is_empty() {
        i18n::list_heading(locale, surface_label, surface_label, 0)
    } else {
        format!(
            "{}\n{}",
            i18n::list_heading(locale, surface_label, surface_label, lines.len()),
            lines.join("\n")
        )
    }
}

pub(crate) fn render_grouped_surface_detail<T>(
    group_id: &str,
    registrations: &[T],
    locale: &str,
    singular_label: fn(&str) -> &'static str,
    line_of: fn(&T) -> String,
) -> String {
    let surface_label = singular_label(locale);
    let mut lines = vec![
        i18n::detail_heading(locale, surface_label, surface_label, group_id),
        i18n::providers_heading(locale, registrations.len()),
    ];

    lines.extend(registrations.iter().map(line_of));
    lines.join("\n")
}
