use super::CliLocaleCatalog;

pub(super) const LOCALE: CliLocaleCatalog = CliLocaleCatalog {
    root_bootstrapped: "Rust foundation bootstrapped.",
    providers_label: "Providers",
    name_label: "Name",
    summary_label: "Summary",
    kind_label: "Kind",
    resolved_activation_label: "Resolved activation",
    resolved_from_label: "Resolved from",
    activation_label: "Activation",
    load_boundary_label: "Load boundary",
    policy_label: "Policy",
    policies_label: "Policies",
    policy_enforcement_hook_label: "Policy enforcement hook",
    capability_label: "Capability",
    capabilities_label: "Capabilities",
    check_label: "Check",
    checks_label: "Checks",
    hook_label: "Runtime hook",
    hooks_label: "Runtime hooks",
    registration_hook_label: "Registration hook",
    assets_label: "Assets",
    provider_label: "Provider",
    locale_id_entity_label: "a locale id",
    mcp_server_id_entity_label: "a server id",
    plugin_config_entity_label: "plugin config",
    plugin_id_entity_label: "a plugin id",
    agent_id_entity_label: "an agent id",
    template_id_entity_label: "a template id",
    prompt_id_entity_label: "a prompt id",
    policy_id_entity_label: "a policy id",
    capability_id_entity_label: "a capability id",
    check_id_entity_label: "a check id",
    hook_id_entity_label: "a hook id",
    provider_id_entity_label: "a provider id",
    plugin_entity_label: "plugin",
    agent_runtime_entity_label: "agent runtime",
    template_entity_label: "template",
    prompt_entity_label: "prompt",
    policy_entity_label: "policy",
    capability_entity_label: "capability",
    check_entity_label: "check",
    hook_entity_label: "hook",
    provider_entity_label: "provider",
    locale_entity_label: "locale",
    mcp_server_entity_label: "mcp server",
    unknown_command,
    unknown_subcommand,
    missing_id,
    unknown_entity,
    missing_asset_path,
    missing_output_directory,
    missing_output_path,
    invalid_embedded_asset_path,
    failed_to_write_output,
    wrote_output,
    unknown_template_asset,
    unknown_prompt_asset,
    materialized_assets_heading: "Materialized assets",
};

fn unknown_command(command_name: &str) -> String {
    format!("unknown command: {command_name}")
}

fn unknown_subcommand(command_group: &str, command_name: &str) -> String {
    format!("unknown {command_group} command: {command_name}")
}

fn missing_id(command_group: &str, entity_label: &str) -> String {
    format!("{command_group} show requires {entity_label}")
}

fn unknown_entity(entity_label: &str, value: &str) -> String {
    format!("unknown {entity_label}: {value}")
}

fn missing_asset_path(command_group: &str) -> String {
    format!("subcommand `{command_group}` requires an asset path")
}

fn missing_output_directory(command_group: &str) -> String {
    format!("subcommand `{command_group}` requires an output directory")
}

fn missing_output_path(command_group: &str) -> String {
    format!("subcommand `{command_group}` requires an output path")
}

fn invalid_embedded_asset_path(value: &str) -> String {
    format!("invalid embedded asset path: {value}")
}

fn failed_to_write_output(path: &str, error: &str) -> String {
    format!("failed to write output `{path}`: {error}")
}

fn wrote_output(path: &str) -> String {
    format!("Wrote output: {path}")
}

fn unknown_template_asset(value: &str) -> String {
    format!("unknown template asset: {value}")
}

fn unknown_prompt_asset(value: &str) -> String {
    format!("unknown prompt asset: {value}")
}
