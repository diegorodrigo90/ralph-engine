pub mod en;
pub mod pt_br;

use re_plugin::PluginLocalizedText;

pub struct PluginLocaleCatalog {
    pub name: &'static str,
    pub summary: &'static str,
}

const LOCALIZED_NAMES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", pt_br::LOCALE.name)];
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", pt_br::LOCALE.summary)];

#[must_use]
pub const fn default_name() -> &'static str {
    en::LOCALE.name
}

#[must_use]
pub const fn default_summary() -> &'static str {
    en::LOCALE.summary
}

#[must_use]
pub const fn localized_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_NAMES
}

#[must_use]
pub const fn localized_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_SUMMARIES
}
