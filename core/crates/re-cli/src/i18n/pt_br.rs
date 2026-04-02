pub(super) const ROOT_BOOTSTRAPPED: &str = "Fundação Rust inicializada.";
pub(super) const PROVIDERS_LABEL: &str = "Provedores";
pub(super) const RESOLVED_ACTIVATION_LABEL: &str = "Ativação resolvida";
pub(super) const RESOLVED_FROM_LABEL: &str = "Resolvido de";
pub(super) const AGENT_RUNTIME_LABEL: &str = "Runtime de agente";
pub(super) const AGENT_RUNTIMES_LABEL: &str = "Runtimes de agente";
pub(super) const TEMPLATE_PROVIDER_LABEL: &str = "Provedor de template";
pub(super) const TEMPLATES_LABEL: &str = "Templates";
pub(super) const PROMPT_PROVIDER_LABEL: &str = "Provedor de prompt";
pub(super) const PROMPTS_LABEL: &str = "Prompts";
pub(super) const POLICY_LABEL: &str = "Policy";
pub(super) const POLICIES_LABEL: &str = "Policies";
pub(super) const POLICY_ENFORCEMENT_HOOK_LABEL: &str = "Hook de enforcement de policy";
pub(super) const PLUGIN_ID_ENTITY_LABEL: &str = "um id de plugin";
pub(super) const POLICY_ID_ENTITY_LABEL: &str = "um id de policy";
pub(super) const PLUGIN_ENTITY_LABEL: &str = "plugin";
pub(super) const AGENT_RUNTIME_ENTITY_LABEL: &str = "runtime de agente";
pub(super) const TEMPLATE_PROVIDER_ENTITY_LABEL: &str = "provedor de template";
pub(super) const PROMPT_PROVIDER_ENTITY_LABEL: &str = "provedor de prompt";
pub(super) const POLICY_ENTITY_LABEL: &str = "policy";

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
    format!("{entity_label} desconhecido: {value}")
}
