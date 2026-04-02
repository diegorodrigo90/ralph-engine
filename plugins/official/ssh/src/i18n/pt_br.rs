use super::{PluginLocaleCatalog, ProviderLocaleCatalog};

pub const LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    name: "SSH",
    summary: "Integração de controle remoto por SSH.",
};

pub const REMOTE_CONTROL_LOCALE: ProviderLocaleCatalog = ProviderLocaleCatalog {
    name: "Controle remoto SSH",
    summary: "Expõe controle remoto tipado por SSH para workflows Ralph Engine.",
};
