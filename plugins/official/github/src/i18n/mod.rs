pub mod en;
pub mod pt_br;

use re_mcp::McpLocalizedText;
use re_plugin::PluginLocalizedText;

pub struct PluginLocaleCatalog {
    pub name: &'static str,
    pub summary: &'static str,
}

pub struct McpServerLocaleCatalog {
    pub name: &'static str,
}

pub struct ProviderLocaleCatalog {
    pub name: &'static str,
    pub summary: &'static str,
}

const LOCALIZED_NAMES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", pt_br::LOCALE.name)];
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", pt_br::LOCALE.summary)];
const LOCALIZED_MCP_SERVER_NAMES: &[McpLocalizedText] = &[McpLocalizedText::new(
    "pt-br",
    pt_br::MCP_SERVER_LOCALE.name,
)];
const LOCALIZED_DATA_SOURCE_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::DATA_SOURCE_LOCALE.name,
)];
const LOCALIZED_DATA_SOURCE_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::DATA_SOURCE_LOCALE.summary,
)];
const LOCALIZED_CONTEXT_PROVIDER_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::CONTEXT_PROVIDER_LOCALE.name,
)];
const LOCALIZED_CONTEXT_PROVIDER_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::CONTEXT_PROVIDER_LOCALE.summary,
)];
const LOCALIZED_FORGE_PROVIDER_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::FORGE_PROVIDER_LOCALE.name,
)];
const LOCALIZED_FORGE_PROVIDER_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::FORGE_PROVIDER_LOCALE.summary,
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
pub const fn default_mcp_server_name() -> &'static str {
    en::MCP_SERVER_LOCALE.name
}

#[must_use]
pub const fn localized_mcp_server_names() -> &'static [McpLocalizedText] {
    LOCALIZED_MCP_SERVER_NAMES
}

#[must_use]
pub const fn default_data_source_name() -> &'static str {
    en::DATA_SOURCE_LOCALE.name
}

#[must_use]
pub const fn default_data_source_summary() -> &'static str {
    en::DATA_SOURCE_LOCALE.summary
}

#[must_use]
pub const fn localized_data_source_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_DATA_SOURCE_NAMES
}

#[must_use]
pub const fn localized_data_source_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_DATA_SOURCE_SUMMARIES
}

#[must_use]
pub const fn default_context_provider_name() -> &'static str {
    en::CONTEXT_PROVIDER_LOCALE.name
}

#[must_use]
pub const fn default_context_provider_summary() -> &'static str {
    en::CONTEXT_PROVIDER_LOCALE.summary
}

#[must_use]
pub const fn localized_context_provider_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_CONTEXT_PROVIDER_NAMES
}

#[must_use]
pub const fn localized_context_provider_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_CONTEXT_PROVIDER_SUMMARIES
}

#[must_use]
pub const fn default_forge_provider_name() -> &'static str {
    en::FORGE_PROVIDER_LOCALE.name
}

#[must_use]
pub const fn default_forge_provider_summary() -> &'static str {
    en::FORGE_PROVIDER_LOCALE.summary
}

#[must_use]
pub const fn localized_forge_provider_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_FORGE_PROVIDER_NAMES
}

#[must_use]
pub const fn localized_forge_provider_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_FORGE_PROVIDER_SUMMARIES
}
