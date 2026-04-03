//! Portuguese (Brazil) format functions for CLI error messages.

pub(super) fn unknown_command(command_name: &str) -> String {
    format!("comando desconhecido: {command_name}")
}

pub(super) fn unknown_subcommand(command_group: &str, command_name: &str) -> String {
    format!("subcomando desconhecido em {command_group}: {command_name}")
}

pub(super) fn missing_id(command_group: &str, entity_label: &str) -> String {
    format!("{command_group} show exige {entity_label}")
}

pub(super) fn missing_argument(command_path: &str, entity_label: &str) -> String {
    format!("{command_path} exige {entity_label}")
}

pub(super) fn unknown_entity(entity_label: &str, value: &str) -> String {
    let adjective = match entity_label {
        "política" | "capacidade" | "verificação" | "configuração de plugin" => "desconhecida",
        _ => "desconhecido",
    };

    format!("{entity_label} {adjective}: {value}")
}

pub(super) fn missing_asset_path(command_group: &str) -> String {
    format!("subcomando `{command_group}` exige um caminho de asset")
}

pub(super) fn missing_output_directory(command_group: &str) -> String {
    format!("subcomando `{command_group}` exige um diretório de saída")
}

pub(super) fn missing_output_path(command_group: &str) -> String {
    format!("subcomando `{command_group}` exige um caminho de saída")
}

pub(super) fn invalid_embedded_asset_path(value: &str) -> String {
    format!("caminho de asset embutido inválido: {value}")
}

pub(super) fn failed_to_write_output(path: &str, error: &str) -> String {
    format!("falha ao gravar a saída `{path}`: {error}")
}

pub(super) fn wrote_output(path: &str) -> String {
    format!("Saída gravada: {path}")
}

pub(super) fn unknown_template_asset(value: &str) -> String {
    format!("asset de template desconhecido: {value}")
}

pub(super) fn unknown_prompt_asset(value: &str) -> String {
    format!("asset de prompt desconhecido: {value}")
}

pub(super) fn unknown_check_asset(value: &str) -> String {
    format!("asset de verificação desconhecido: {value}")
}

pub(super) fn unknown_policy_asset(value: &str) -> String {
    format!("asset de política desconhecido: {value}")
}
