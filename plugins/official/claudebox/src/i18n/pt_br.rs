use super::{AgentLocaleCatalog, McpServerLocaleCatalog, PluginLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "Claude Box",
    plugin_summary: "Integração do runtime Claude Box com sessão MCP.",
};

pub const AGENT_LOCALE: AgentLocaleCatalog = AgentLocaleCatalog {
    name: "Sessão Claude Box",
    summary: "Sessão de runtime do Claude Box para o Ralph Engine.",
};

pub const MCP_SERVER_LOCALE: McpServerLocaleCatalog = McpServerLocaleCatalog {
    name: "Sessão Claude Box",
};
