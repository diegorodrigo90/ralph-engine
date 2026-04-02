use super::{PluginLocaleCatalog, TemplateLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "TDD Strict",
    plugin_summary: "Strict TDD policy and template guardrails.",
};

pub const TEMPLATE_LOCALE: TemplateLocaleCatalog = TemplateLocaleCatalog {
    name: "TDD strict starter",
    summary: "Starter template with strict TDD guardrails enabled.",
};
