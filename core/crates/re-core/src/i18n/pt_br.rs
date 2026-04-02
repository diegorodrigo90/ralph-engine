use super::RuntimeLocaleCatalog;

pub(super) const LOCALE: RuntimeLocaleCatalog = RuntimeLocaleCatalog {
    runtime_phase: "Fase do runtime",
    runtime_health: "Saúde do runtime",
    locale: "Idioma",
    plugins: "Plugins",
    capabilities: "Capacidades",
    templates: "Templates",
    prompts: "Prompts",
    agent_runtimes: "Runtimes de agente",
    checks: "Verificações",
    providers: "Provedores",
    policies: "Políticas",
    runtime_hooks: "Hooks de runtime",
    mcp_servers: "Servidores MCP",
    runtime_mcp_launch_plans: "Planos de lançamento MCP do runtime",
    runtime_issues: "Problemas do runtime",
    runtime_action_plan: "Plano de ação do runtime",
    runtime_doctor: "Diagnóstico do runtime",
    translate_runtime_reason,
};

fn translate_runtime_reason(reason: &str) -> String {
    if let Some(capability) = reason.strip_prefix("the provider still disables capability ") {
        return format!("o provedor ainda desabilita a capacidade {capability}");
    }

    if let Some(check_kind) = reason.strip_prefix("the provider still disables runtime check ") {
        return format!("o provedor ainda desabilita a verificação de runtime {check_kind}");
    }

    if let Some(provider_kind) = reason.strip_prefix("the provider still disables contribution ") {
        return format!("o provedor ainda desabilita a contribuição {provider_kind}");
    }

    if let Some(policy_id) = reason.strip_prefix("the provider still disables policy ") {
        return format!("o provedor ainda desabilita a política {policy_id}");
    }

    if let Some(hook_id) = reason.strip_prefix("the provider still disables runtime hook ") {
        return format!("o provedor ainda desabilita o hook de runtime {hook_id}");
    }

    match reason {
        "enable the plugin in typed project configuration" => {
            "ative o plugin na configuração tipada do projeto".to_owned()
        }
        "enable the provider plugin that owns this capability" => {
            "ative o plugin provedor responsável por esta capacidade".to_owned()
        }
        "enable the provider plugin that owns this template surface" => {
            "ative o plugin provedor responsável por esta superfície de template".to_owned()
        }
        "enable the provider plugin that owns this prompt surface" => {
            "ative o plugin provedor responsável por esta superfície de prompt".to_owned()
        }
        "enable the provider plugin that owns this agent runtime" => {
            "ative o plugin provedor responsável por este runtime de agente".to_owned()
        }
        "enable the provider plugin that owns this runtime check" => {
            "ative o plugin provedor responsável por esta verificação de runtime".to_owned()
        }
        "enable the provider plugin that owns this contribution" => {
            "ative o plugin provedor responsável por esta contribuição".to_owned()
        }
        "enable the provider plugin that owns this policy" => {
            "ative o plugin provedor responsável por esta política".to_owned()
        }
        "enable the provider plugin that owns this runtime hook" => {
            "ative o plugin provedor responsável por este hook de runtime".to_owned()
        }
        "enable the owning plugin or opt in to the MCP server" => {
            "ative o plugin responsável ou faça opt-in no servidor MCP".to_owned()
        }
        "the plugin is registered but disabled" => {
            "o plugin está registrado, mas desabilitado".to_owned()
        }
        "the provider still disables the template surface" => {
            "o provedor ainda desabilita a superfície de template".to_owned()
        }
        "the provider still disables the prompt surface" => {
            "o provedor ainda desabilita a superfície de prompt".to_owned()
        }
        "the provider still disables the agent runtime" => {
            "o provedor ainda desabilita o runtime de agente".to_owned()
        }
        "the MCP contribution is registered but disabled" => {
            "a contribuição MCP está registrada, mas desabilitada".to_owned()
        }
        _ => reason.to_owned(),
    }
}
