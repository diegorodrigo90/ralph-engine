pub(super) const ROOT_BOOTSTRAPPED: &str = "Fundação Rust inicializada.";
pub(super) const PROVIDERS_LABEL: &str = "Provedores";
pub(super) const RESOLVED_ACTIVATION_LABEL: &str = "Ativação resolvida";
pub(super) const RESOLVED_FROM_LABEL: &str = "Resolvido de";
pub(super) const ACTIVATION_LABEL: &str = "Ativação";
pub(super) const LOAD_BOUNDARY_LABEL: &str = "Fronteira de carregamento";
pub(super) const AGENT_RUNTIME_LABEL: &str = "Runtime de agente";
pub(super) const AGENT_RUNTIMES_LABEL: &str = "Runtimes de agente";
pub(super) const TEMPLATE_PROVIDER_LABEL: &str = "Provedor de template";
pub(super) const TEMPLATES_LABEL: &str = "Templates";
pub(super) const PROMPT_PROVIDER_LABEL: &str = "Provedor de prompt";
pub(super) const PROMPTS_LABEL: &str = "Prompts";
pub(super) const POLICY_LABEL: &str = "Política";
pub(super) const POLICIES_LABEL: &str = "Políticas";
pub(super) const POLICY_ENFORCEMENT_HOOK_LABEL: &str = "Hook de aplicação de política";
pub(super) const CAPABILITY_LABEL: &str = "Capacidade";
pub(super) const CAPABILITIES_LABEL: &str = "Capacidades";
pub(super) const CHECK_LABEL: &str = "Verificação";
pub(super) const CHECKS_LABEL: &str = "Verificações";
pub(super) const HOOK_LABEL: &str = "Hook de runtime";
pub(super) const HOOKS_LABEL: &str = "Hooks de runtime";
pub(super) const PROVIDER_LABEL: &str = "Provedor";
pub(super) const LOCALE_ID_ENTITY_LABEL: &str = "um id de idioma";
pub(super) const MCP_SERVER_ID_ENTITY_LABEL: &str = "um id de servidor";
pub(super) const PLUGIN_CONFIG_ENTITY_LABEL: &str = "configuração de plugin";
pub(super) const PLUGIN_ID_ENTITY_LABEL: &str = "um id de plugin";
pub(super) const POLICY_ID_ENTITY_LABEL: &str = "um id de política";
pub(super) const CAPABILITY_ID_ENTITY_LABEL: &str = "um id de capacidade";
pub(super) const CHECK_ID_ENTITY_LABEL: &str = "um id de verificação";
pub(super) const HOOK_ID_ENTITY_LABEL: &str = "um id de hook";
pub(super) const PROVIDER_ID_ENTITY_LABEL: &str = "um id de provedor";
pub(super) const PLUGIN_ENTITY_LABEL: &str = "plugin";
pub(super) const AGENT_RUNTIME_ENTITY_LABEL: &str = "runtime de agente";
pub(super) const TEMPLATE_PROVIDER_ENTITY_LABEL: &str = "provedor de template";
pub(super) const PROMPT_PROVIDER_ENTITY_LABEL: &str = "provedor de prompt";
pub(super) const POLICY_ENTITY_LABEL: &str = "política";
pub(super) const CAPABILITY_ENTITY_LABEL: &str = "capacidade";
pub(super) const CHECK_ENTITY_LABEL: &str = "verificação";
pub(super) const HOOK_ENTITY_LABEL: &str = "hook";
pub(super) const PROVIDER_ENTITY_LABEL: &str = "provedor";
pub(super) const LOCALE_ENTITY_LABEL: &str = "idioma";
pub(super) const MCP_SERVER_ENTITY_LABEL: &str = "servidor MCP";

pub(super) fn unknown_command(command_name: &str) -> String {
    format!("comando desconhecido: {command_name}")
}

pub(super) fn unknown_subcommand(command_group: &str, command_name: &str) -> String {
    format!("subcomando desconhecido em {command_group}: {command_name}")
}

pub(super) fn missing_id(command_group: &str, entity_label: &str) -> String {
    format!("{command_group} show exige {entity_label}")
}

pub(super) fn unknown_entity(entity_label: &str, value: &str) -> String {
    let adjective = match entity_label {
        "política" | "capacidade" | "verificação" | "configuração de plugin" => "desconhecida",
        _ => "desconhecido",
    };

    format!("{entity_label} {adjective}: {value}")
}
