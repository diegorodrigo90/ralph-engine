use super::{McpServerLocaleCatalog, PluginLocaleCatalog, ProviderLocaleCatalog};

pub const LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    name: "GitHub",
    summary: "GitHub data, context, forge, and MCP integration.",
};

pub const MCP_SERVER_LOCALE: McpServerLocaleCatalog = McpServerLocaleCatalog {
    name: "GitHub Repository",
};

pub const DATA_SOURCE_LOCALE: ProviderLocaleCatalog = ProviderLocaleCatalog {
    name: "GitHub data source",
    summary: "Exposes typed repository data to Ralph Engine workflows.",
};

pub const CONTEXT_PROVIDER_LOCALE: ProviderLocaleCatalog = ProviderLocaleCatalog {
    name: "GitHub context provider",
    summary: "Exposes typed GitHub context for Ralph Engine workflows.",
};

pub const FORGE_PROVIDER_LOCALE: ProviderLocaleCatalog = ProviderLocaleCatalog {
    name: "GitHub forge provider",
    summary: "Exposes typed forge automation for GitHub-backed workflows.",
};
