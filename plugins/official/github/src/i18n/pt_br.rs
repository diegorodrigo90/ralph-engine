use super::{McpServerLocaleCatalog, PluginLocaleCatalog, ProviderLocaleCatalog};

pub const LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    name: "GitHub",
    summary: "Integração de dados, contexto, forge e MCP do GitHub.",
};

pub const MCP_SERVER_LOCALE: McpServerLocaleCatalog = McpServerLocaleCatalog {
    name: "Repositório GitHub",
};

pub const DATA_SOURCE_LOCALE: ProviderLocaleCatalog = ProviderLocaleCatalog {
    name: "Fonte de dados GitHub",
    summary: "Expõe dados tipados de repositório para workflows Ralph Engine.",
};

pub const CONTEXT_PROVIDER_LOCALE: ProviderLocaleCatalog = ProviderLocaleCatalog {
    name: "Provedor de contexto GitHub",
    summary: "Expõe contexto tipado do GitHub para workflows Ralph Engine.",
};

pub const FORGE_PROVIDER_LOCALE: ProviderLocaleCatalog = ProviderLocaleCatalog {
    name: "Provedor forge GitHub",
    summary: "Expõe automação forge tipada para workflows baseados em GitHub.",
};
