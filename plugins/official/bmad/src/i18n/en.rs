use super::{PluginLocaleCatalog, PromptLocaleCatalog, TemplateLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "BMAD",
    plugin_summary: "Workflow plugin for BMAD scaffolding and prompts.",
};

pub const TEMPLATE_LOCALE: TemplateLocaleCatalog = TemplateLocaleCatalog {
    name: "BMAD starter",
    summary: "Starter template for BMAD-guided Ralph Engine projects.",
};

pub const PROMPT_LOCALE: PromptLocaleCatalog = PromptLocaleCatalog {
    name: "BMAD workflow prompt",
    summary: "Prompt bundle for BMAD workflow assembly.",
};
