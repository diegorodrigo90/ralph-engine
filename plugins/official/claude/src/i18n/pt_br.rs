use super::{AgentLocaleCatalog, McpServerLocaleCatalog, PluginLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "Claude",
    plugin_summary: "Integração do runtime de agente Claude com sessão MCP.",
};

pub const AGENT_LOCALE: AgentLocaleCatalog = AgentLocaleCatalog {
    name: "Sessão Claude",
    summary: "Sessão de runtime do Claude para o Ralph Engine.",
};

pub const MCP_SERVER_LOCALE: McpServerLocaleCatalog = McpServerLocaleCatalog {
    name: "Sessão Claude",
};
