use super::{AgentLocaleCatalog, McpServerLocaleCatalog, PluginLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "Codex",
    plugin_summary: "Codex runtime and MCP session integration.",
};

pub const AGENT_LOCALE: AgentLocaleCatalog = AgentLocaleCatalog {
    name: "Codex session",
    summary: "Codex runtime session for Ralph Engine.",
};

pub const MCP_SERVER_LOCALE: McpServerLocaleCatalog = McpServerLocaleCatalog {
    name: "Codex Session",
};
