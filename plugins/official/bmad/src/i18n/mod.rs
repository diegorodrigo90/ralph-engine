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

pub struct PromptLocaleCatalog {
    pub name: &'static str,
    pub summary: &'static str,
}

pub struct CheckLocaleCatalog {
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
const LOCALIZED_PROMPT_NAMES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", pt_br::PROMPT_LOCALE.name)];
const LOCALIZED_PROMPT_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::PROMPT_LOCALE.summary,
)];
const LOCALIZED_PREPARE_CHECK_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::PREPARE_CHECK_LOCALE.name,
)];
const LOCALIZED_PREPARE_CHECK_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::PREPARE_CHECK_LOCALE.summary,
)];
const LOCALIZED_DOCTOR_CHECK_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::DOCTOR_CHECK_LOCALE.name,
)];
const LOCALIZED_DOCTOR_CHECK_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::DOCTOR_CHECK_LOCALE.summary,
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
pub const fn default_prompt_name() -> &'static str {
    en::PROMPT_LOCALE.name
}

#[must_use]
pub const fn default_prompt_summary() -> &'static str {
    en::PROMPT_LOCALE.summary
}

#[must_use]
pub const fn localized_prompt_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_PROMPT_NAMES
}

#[must_use]
pub const fn localized_prompt_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_PROMPT_SUMMARIES
}

#[must_use]
pub const fn default_prepare_check_name() -> &'static str {
    en::PREPARE_CHECK_LOCALE.name
}

#[must_use]
pub const fn default_prepare_check_summary() -> &'static str {
    en::PREPARE_CHECK_LOCALE.summary
}

#[must_use]
pub const fn localized_prepare_check_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_PREPARE_CHECK_NAMES
}

#[must_use]
pub const fn localized_prepare_check_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_PREPARE_CHECK_SUMMARIES
}

#[must_use]
pub const fn default_doctor_check_name() -> &'static str {
    en::DOCTOR_CHECK_LOCALE.name
}

#[must_use]
pub const fn default_doctor_check_summary() -> &'static str {
    en::DOCTOR_CHECK_LOCALE.summary
}

#[must_use]
pub const fn localized_doctor_check_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_DOCTOR_CHECK_NAMES
}

#[must_use]
pub const fn localized_doctor_check_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_DOCTOR_CHECK_SUMMARIES
}
