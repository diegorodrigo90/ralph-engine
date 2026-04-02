use super::CliLocaleCatalog;

pub(super) const LOCALE: CliLocaleCatalog = CliLocaleCatalog {
    root_bootstrapped: "Fundação Rust inicializada.",
    providers_label: "Provedores",
    resolved_activation_label: "Ativação resolvida",
    resolved_from_label: "Resolvido de",
    activation_label: "Ativação",
    load_boundary_label: "Fronteira de carregamento",
    agent_runtime_label: "Runtime de agente",
    agent_runtimes_label: "Runtimes de agente",
    template_provider_label: "Provedor de template",
    templates_label: "Templates",
    prompt_provider_label: "Provedor de prompt",
    prompts_label: "Prompts",
    policy_label: "Política",
    policies_label: "Políticas",
    policy_enforcement_hook_label: "Hook de aplicação de política",
    capability_label: "Capacidade",
    capabilities_label: "Capacidades",
    check_label: "Verificação",
    checks_label: "Verificações",
    hook_label: "Hook de runtime",
    hooks_label: "Hooks de runtime",
    provider_label: "Provedor",
    locale_id_entity_label: "um id de idioma",
    mcp_server_id_entity_label: "um id de servidor",
    plugin_config_entity_label: "configuração de plugin",
    plugin_id_entity_label: "um id de plugin",
    policy_id_entity_label: "um id de política",
    capability_id_entity_label: "um id de capacidade",
    check_id_entity_label: "um id de verificação",
    hook_id_entity_label: "um id de hook",
    provider_id_entity_label: "um id de provedor",
    plugin_entity_label: "plugin",
    agent_runtime_entity_label: "runtime de agente",
    template_provider_entity_label: "provedor de template",
    prompt_provider_entity_label: "provedor de prompt",
    policy_entity_label: "política",
    capability_entity_label: "capacidade",
    check_entity_label: "verificação",
    hook_entity_label: "hook",
    provider_entity_label: "provedor",
    locale_entity_label: "idioma",
    mcp_server_entity_label: "servidor MCP",
    unknown_command,
    unknown_subcommand,
    missing_id,
    unknown_entity,
};

fn unknown_command(command_name: &str) -> String {
    format!("comando desconhecido: {command_name}")
}

fn unknown_subcommand(command_group: &str, command_name: &str) -> String {
    format!("subcomando desconhecido em {command_group}: {command_name}")
}

fn missing_id(command_group: &str, entity_label: &str) -> String {
    format!("{command_group} show exige {entity_label}")
}

fn unknown_entity(entity_label: &str, value: &str) -> String {
    let adjective = match entity_label {
        "política" | "capacidade" | "verificação" | "configuração de plugin" => "desconhecida",
        _ => "desconhecido",
    };

    format!("{entity_label} {adjective}: {value}")
}
