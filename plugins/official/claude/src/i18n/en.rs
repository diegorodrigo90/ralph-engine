use super::{AgentLocaleCatalog, PluginLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "Claude",
    plugin_summary: "Claude agent runtime and MCP session integration.",
};

pub const AGENT_LOCALE: AgentLocaleCatalog = AgentLocaleCatalog {
    name: "Claude session",
    summary: "Claude runtime session for Ralph Engine.",
};
