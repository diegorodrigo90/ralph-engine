use super::CliLocaleCatalog;

pub(super) const LOCALE: CliLocaleCatalog = CliLocaleCatalog {
    root_bootstrapped: "Fundação Rust inicializada.",
    providers_label: "Provedores",
    name_label: "Nome",
    summary_label: "Resumo",
    kind_label: "Tipo",
    resolved_activation_label: "Ativação resolvida",
    resolved_from_label: "Resolvido de",
    activation_label: "Ativação",
    load_boundary_label: "Fronteira de carregamento",
    policy_label: "Política",
    policies_label: "Políticas",
    policy_enforcement_hook_label: "Hook de aplicação de política",
    capability_label: "Capacidade",
    capabilities_label: "Capacidades",
    check_label: "Verificação",
    checks_label: "Verificações",
    hook_label: "Hook de runtime",
    hooks_label: "Hooks de runtime",
    registration_hook_label: "Registration hook",
    assets_label: "Assets",
    provider_label: "Provedor",
    locale_id_entity_label: "um id de idioma",
    mcp_server_id_entity_label: "um id de servidor",
    plugin_config_entity_label: "configuração de plugin",
    plugin_id_entity_label: "um id de plugin",
    agent_id_entity_label: "um id de agente",
    template_id_entity_label: "um id de template",
    prompt_id_entity_label: "um id de prompt",
    policy_id_entity_label: "um id de política",
    capability_id_entity_label: "um id de capacidade",
    check_id_entity_label: "um id de verificação",
    hook_id_entity_label: "um id de hook",
    provider_id_entity_label: "um id de provedor",
    plugin_entity_label: "plugin",
    agent_runtime_entity_label: "runtime de agente",
    template_entity_label: "template",
    prompt_entity_label: "prompt",
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
    missing_argument,
    unknown_entity,
    missing_asset_path,
    missing_output_directory,
    missing_output_path,
    invalid_embedded_asset_path,
    failed_to_write_output,
    wrote_output,
    unknown_template_asset,
    unknown_prompt_asset,
    unknown_check_asset,
    unknown_policy_asset,
    materialized_assets_heading: "Assets materializados",
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

fn missing_argument(command_path: &str, entity_label: &str) -> String {
    format!("{command_path} exige {entity_label}")
}

fn unknown_entity(entity_label: &str, value: &str) -> String {
    let adjective = match entity_label {
        "política" | "capacidade" | "verificação" | "configuração de plugin" => "desconhecida",
        _ => "desconhecido",
    };

    format!("{entity_label} {adjective}: {value}")
}

fn missing_asset_path(command_group: &str) -> String {
    format!("subcomando `{command_group}` exige um caminho de asset")
}

fn missing_output_directory(command_group: &str) -> String {
    format!("subcomando `{command_group}` exige um diretório de saída")
}

fn missing_output_path(command_group: &str) -> String {
    format!("subcomando `{command_group}` exige um caminho de saída")
}

fn invalid_embedded_asset_path(value: &str) -> String {
    format!("caminho de asset embutido inválido: {value}")
}

fn failed_to_write_output(path: &str, error: &str) -> String {
    format!("falha ao gravar a saída `{path}`: {error}")
}

fn wrote_output(path: &str) -> String {
    format!("Saída gravada: {path}")
}

fn unknown_template_asset(value: &str) -> String {
    format!("asset de template desconhecido: {value}")
}

fn unknown_prompt_asset(value: &str) -> String {
    format!("asset de prompt desconhecido: {value}")
}

fn unknown_check_asset(value: &str) -> String {
    format!("asset de verificação desconhecido: {value}")
}

fn unknown_policy_asset(value: &str) -> String {
    format!("asset de política desconhecido: {value}")
}
