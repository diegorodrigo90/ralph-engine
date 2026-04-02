pub mod en;
pub mod pt_br;

use re_plugin::PluginLocalizedText;

pub struct PluginLocaleCatalog {
    pub name: &'static str,
    pub summary: &'static str,
}

pub struct ProviderLocaleCatalog {
    pub name: &'static str,
    pub summary: &'static str,
}

const LOCALIZED_NAMES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", pt_br::LOCALE.name)];
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", pt_br::LOCALE.summary)];
const LOCALIZED_REMOTE_CONTROL_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::REMOTE_CONTROL_LOCALE.name,
)];
const LOCALIZED_REMOTE_CONTROL_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::REMOTE_CONTROL_LOCALE.summary,
)];

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

#[must_use]
pub const fn default_remote_control_name() -> &'static str {
    en::REMOTE_CONTROL_LOCALE.name
}

#[must_use]
pub const fn default_remote_control_summary() -> &'static str {
    en::REMOTE_CONTROL_LOCALE.summary
}

#[must_use]
pub const fn localized_remote_control_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_REMOTE_CONTROL_NAMES
}

#[must_use]
pub const fn localized_remote_control_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_REMOTE_CONTROL_SUMMARIES
}
