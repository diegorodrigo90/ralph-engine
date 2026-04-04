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

// ── Install/uninstall ────────────────────────────────────────────

pub(super) fn install_already_installed(plugin_id: &str, path: &str) -> String {
    format!("Plugin '{plugin_id}' is already installed at {path}")
}

pub(super) fn install_create_dir_failed(error: &str) -> String {
    format!("Failed to create plugins directory: {error}")
}

pub(super) fn install_clone_exec_failed(error: &str) -> String {
    format!("Failed to run git clone: {error}")
}

pub(super) fn install_clone_repo_failed(url: &str) -> String {
    format!("Failed to clone {url}. Check that the repository exists and is public.")
}

pub(super) fn install_not_installed(plugin_id: &str) -> String {
    format!("Plugin '{plugin_id}' is not installed.")
}

pub(super) fn install_remove_dir_failed(error: &str) -> String {
    format!("Failed to remove plugin directory: {error}")
}

pub(super) fn install_uninstalled(plugin_id: &str) -> String {
    format!("Plugin '{plugin_id}' uninstalled.")
}

// ── Init ─────────────────────────────────────────────────────────

pub(super) fn init_remove_failed(error: &str) -> String {
    format!("Failed to remove .ralph-engine/: {error}")
}

// ── MCP ──────────────────────────────────────────────────────────

pub(super) fn mcp_install_hint(program: &str) -> String {
    format!("Hint: install '{program}' or add it to PATH to enable this MCP server")
}

// ── Policies ─────────────────────────────────────────────────────

pub(super) fn policies_materialize_hint(policy_id: &str) -> String {
    format!("Hint: run 'ralph-engine policies materialize {policy_id} <dir>' to generate the files")
}
