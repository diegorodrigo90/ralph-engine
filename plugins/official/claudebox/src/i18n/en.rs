use super::{AgentLocaleCatalog, PluginLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "Claude Box",
    plugin_summary: "Claude Box runtime and MCP session integration.",
};

pub const AGENT_LOCALE: AgentLocaleCatalog = AgentLocaleCatalog {
    name: "Claude Box session",
    summary: "Claude Box runtime session for Ralph Engine.",
};
