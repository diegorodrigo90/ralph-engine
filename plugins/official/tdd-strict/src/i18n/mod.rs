pub mod en;
pub mod pt_br;

use re_plugin::PluginLocalizedText;

pub struct PluginLocaleCatalog {
    pub plugin_name: &'static str,
    pub plugin_summary: &'static str,
}

pub struct TemplateLocaleCatalog {
    pub name: &'static str,
    pub summary: &'static str,
}

pub struct PolicyLocaleCatalog {
    pub name: &'static str,
    pub summary: &'static str,
}

const LOCALIZED_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::PLUGIN_LOCALE.plugin_name,
)];
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::PLUGIN_LOCALE.plugin_summary,
)];
const LOCALIZED_TEMPLATE_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::TEMPLATE_LOCALE.name,
)];
const LOCALIZED_TEMPLATE_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::TEMPLATE_LOCALE.summary,
)];
const LOCALIZED_POLICY_NAMES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", pt_br::POLICY_LOCALE.name)];
const LOCALIZED_POLICY_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::POLICY_LOCALE.summary,
)];

#[must_use]
pub const fn default_name() -> &'static str {
    en::PLUGIN_LOCALE.plugin_name
}

#[must_use]
pub const fn default_summary() -> &'static str {
    en::PLUGIN_LOCALE.plugin_summary
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
pub const fn default_template_name() -> &'static str {
    en::TEMPLATE_LOCALE.name
}

#[must_use]
pub const fn default_template_summary() -> &'static str {
    en::TEMPLATE_LOCALE.summary
}

#[must_use]
pub const fn localized_template_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_TEMPLATE_NAMES
}

#[must_use]
pub const fn localized_template_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_TEMPLATE_SUMMARIES
}

#[must_use]
pub const fn default_policy_name() -> &'static str {
    en::POLICY_LOCALE.name
}

#[must_use]
pub const fn default_policy_summary() -> &'static str {
    en::POLICY_LOCALE.summary
}

#[must_use]
pub const fn localized_policy_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_POLICY_NAMES
}

#[must_use]
pub const fn localized_policy_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_POLICY_SUMMARIES
}
