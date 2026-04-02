pub(super) const ROOT_BOOTSTRAPPED: &str = "Rust foundation bootstrapped.";
pub(super) const PROVIDERS_LABEL: &str = "Providers";

pub(super) fn unknown_command(command_name: &str) -> String {
    format!("unknown command: {command_name}")
}

pub(super) fn unknown_subcommand(command_group: &str, command_name: &str) -> String {
    format!("unknown {command_group} command: {command_name}")
}

pub(super) fn missing_id(command_group: &str, entity_label: &str) -> String {
    format!("{command_group} show requires {entity_label}")
}

pub(super) fn unknown_entity(entity_label: &str, value: &str) -> String {
    format!("unknown {entity_label}: {value}")
}
