pub mod en;
pub mod pt_br;

use re_plugin::PluginLocalizedText;

const LOCALIZED_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new("pt-br", pt_br::NAME)];
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", pt_br::SUMMARY)];

#[must_use]
pub const fn default_name() -> &'static str {
    en::NAME
}

#[must_use]
pub const fn default_summary() -> &'static str {
    en::SUMMARY
}

#[must_use]
pub const fn localized_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_NAMES
}

#[must_use]
pub const fn localized_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_SUMMARIES
}
