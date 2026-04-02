pub(super) const ROOT_BOOTSTRAPPED: &str = "Fundação Rust inicializada.";
pub(super) const PROVIDERS_LABEL: &str = "Provedores";

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
