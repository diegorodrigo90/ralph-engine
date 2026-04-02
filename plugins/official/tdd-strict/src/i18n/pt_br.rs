use super::{PluginLocaleCatalog, PolicyLocaleCatalog, TemplateLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "TDD Estrito",
    plugin_summary: "Política estrita de TDD com guardrails de template.",
};

pub const TEMPLATE_LOCALE: TemplateLocaleCatalog = TemplateLocaleCatalog {
    name: "Starter TDD estrito",
    summary: "Template inicial com guardrails estritos de TDD ativados.",
};

pub const POLICY_LOCALE: PolicyLocaleCatalog = PolicyLocaleCatalog {
    name: "Guardrails TDD estrito",
    summary: "Política oficial com guardrails estritos de TDD.",
};
