use super::{AgentLocaleCatalog, McpServerLocaleCatalog, PluginLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "Claude",
    plugin_summary: "Claude agent runtime and MCP session integration.",
};

pub const AGENT_LOCALE: AgentLocaleCatalog = AgentLocaleCatalog {
    name: "Claude session",
    summary: "Claude runtime session for Ralph Engine.",
};

pub const MCP_SERVER_LOCALE: McpServerLocaleCatalog = McpServerLocaleCatalog {
    name: "Claude Session",
};
