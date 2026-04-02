use super::{AgentLocaleCatalog, PluginLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "Codex",
    plugin_summary: "Integração do runtime Codex com sessão MCP.",
};

pub const AGENT_LOCALE: AgentLocaleCatalog = AgentLocaleCatalog {
    name: "Sessão Codex",
    summary: "Sessão de runtime do Codex para o Ralph Engine.",
};
