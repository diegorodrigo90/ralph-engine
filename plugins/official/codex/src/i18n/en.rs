use super::{AgentLocaleCatalog, PluginLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "Codex",
    plugin_summary: "Codex runtime and MCP session integration.",
};

pub const AGENT_LOCALE: AgentLocaleCatalog = AgentLocaleCatalog {
    name: "Codex session",
    summary: "Codex runtime session for Ralph Engine.",
};
