use super::{PluginLocaleCatalog, PromptLocaleCatalog, TemplateLocaleCatalog};

pub const PLUGIN_LOCALE: PluginLocaleCatalog = PluginLocaleCatalog {
    plugin_name: "BMAD",
    plugin_summary: "Plugin de workflow para scaffolding e prompts do BMAD.",
};

pub const TEMPLATE_LOCALE: TemplateLocaleCatalog = TemplateLocaleCatalog {
    name: "Starter BMAD",
    summary: "Template inicial para projetos Ralph Engine guiados por BMAD.",
};

pub const PROMPT_LOCALE: PromptLocaleCatalog = PromptLocaleCatalog {
    name: "Prompt de workflow BMAD",
    summary: "Pacote de prompts para montar workflows BMAD.",
};
