use super::{PluginLocaleCatalog, ProviderLocaleCatalog};

pub const LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    name: "SSH",
    summary: "SSH remote control integration.",
};

pub const REMOTE_CONTROL_LOCALE: ProviderLocaleCatalog = ProviderLocaleCatalog {
    name: "SSH remote control",
    summary: "Exposes typed remote control over SSH for Ralph Engine workflows.",
};
