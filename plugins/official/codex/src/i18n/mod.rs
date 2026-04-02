pub mod en;
pub mod pt_br;

use re_mcp::McpLocalizedText;
use re_plugin::PluginLocalizedText;

pub struct PluginLocaleCatalog {
    pub plugin_name: &'static str,
    pub plugin_summary: &'static str,
}

pub struct AgentLocaleCatalog {
    pub name: &'static str,
    pub summary: &'static str,
}

pub struct McpServerLocaleCatalog {
    pub name: &'static str,
}

const LOCALIZED_NAMES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::PLUGIN_LOCALE.plugin_name,
)];
const LOCALIZED_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::PLUGIN_LOCALE.plugin_summary,
)];
const LOCALIZED_AGENT_NAMES: &[PluginLocalizedText] =
    &[PluginLocalizedText::new("pt-br", pt_br::AGENT_LOCALE.name)];
const LOCALIZED_AGENT_SUMMARIES: &[PluginLocalizedText] = &[PluginLocalizedText::new(
    "pt-br",
    pt_br::AGENT_LOCALE.summary,
)];
const LOCALIZED_MCP_SERVER_NAMES: &[McpLocalizedText] = &[McpLocalizedText::new(
    "pt-br",
    pt_br::MCP_SERVER_LOCALE.name,
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
pub const fn default_agent_name() -> &'static str {
    en::AGENT_LOCALE.name
}

#[must_use]
pub const fn default_agent_summary() -> &'static str {
    en::AGENT_LOCALE.summary
}

#[must_use]
pub const fn localized_agent_names() -> &'static [PluginLocalizedText] {
    LOCALIZED_AGENT_NAMES
}

#[must_use]
pub const fn localized_agent_summaries() -> &'static [PluginLocalizedText] {
    LOCALIZED_AGENT_SUMMARIES
}

#[must_use]
pub const fn default_mcp_server_name() -> &'static str {
    en::MCP_SERVER_LOCALE.name
}

#[must_use]
pub const fn localized_mcp_server_names() -> &'static [McpLocalizedText] {
    LOCALIZED_MCP_SERVER_NAMES
}
