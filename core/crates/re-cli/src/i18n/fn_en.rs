//! English format functions for CLI error messages.

pub(super) fn unknown_command(command_name: &str) -> String {
    format!("unknown command: {command_name}")
}

pub(super) fn unknown_subcommand(command_group: &str, command_name: &str) -> String {
    format!("unknown {command_group} command: {command_name}")
}

pub(super) fn missing_id(command_group: &str, entity_label: &str) -> String {
    format!("{command_group} show requires {entity_label}")
}

pub(super) fn missing_argument(command_path: &str, entity_label: &str) -> String {
    format!("{command_path} requires {entity_label}")
}

pub(super) fn unknown_entity(entity_label: &str, value: &str) -> String {
    format!("unknown {entity_label}: {value}")
}

pub(super) fn missing_asset_path(command_group: &str) -> String {
    format!("subcommand `{command_group}` requires an asset path")
}

pub(super) fn missing_output_directory(command_group: &str) -> String {
    format!("subcommand `{command_group}` requires an output directory")
}

pub(super) fn missing_output_path(command_group: &str) -> String {
    format!("subcommand `{command_group}` requires an output path")
}

pub(super) fn invalid_embedded_asset_path(value: &str) -> String {
    format!("invalid embedded asset path: {value}")
}

pub(super) fn failed_to_write_output(path: &str, error: &str) -> String {
    format!("failed to write output `{path}`: {error}")
}

pub(super) fn wrote_output(path: &str) -> String {
    format!("Wrote output: {path}")
}

pub(super) fn unknown_template_asset(value: &str) -> String {
    format!("unknown template asset: {value}")
}

pub(super) fn unknown_prompt_asset(value: &str) -> String {
    format!("unknown prompt asset: {value}")
}

pub(super) fn unknown_check_asset(value: &str) -> String {
    format!("unknown check asset: {value}")
}

pub(super) fn unknown_policy_asset(value: &str) -> String {
    format!("unknown policy asset: {value}")
}
