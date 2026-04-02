use super::{AgentLocaleCatalog, PluginLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "Claude Box",
    plugin_summary: "Integração do runtime Claude Box com sessão MCP.",
};

pub const AGENT_LOCALE: AgentLocaleCatalog = AgentLocaleCatalog {
    name: "Sessão Claude Box",
    summary: "Sessão de runtime do Claude Box para o Ralph Engine.",
};
