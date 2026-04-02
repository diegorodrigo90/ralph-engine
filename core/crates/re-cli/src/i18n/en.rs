pub(super) const ROOT_BOOTSTRAPPED: &str = "Rust foundation bootstrapped.";
pub(super) const PROVIDERS_LABEL: &str = "Providers";
pub(super) const RESOLVED_ACTIVATION_LABEL: &str = "Resolved activation";
pub(super) const RESOLVED_FROM_LABEL: &str = "Resolved from";
pub(super) const AGENT_RUNTIME_LABEL: &str = "Agent runtime";
pub(super) const AGENT_RUNTIMES_LABEL: &str = "Agent runtimes";
pub(super) const TEMPLATE_PROVIDER_LABEL: &str = "Template provider";
pub(super) const TEMPLATES_LABEL: &str = "Templates";
pub(super) const PROMPT_PROVIDER_LABEL: &str = "Prompt provider";
pub(super) const PROMPTS_LABEL: &str = "Prompts";
pub(super) const POLICY_LABEL: &str = "Policy";
pub(super) const POLICIES_LABEL: &str = "Policies";
pub(super) const POLICY_ENFORCEMENT_HOOK_LABEL: &str = "Policy enforcement hook";
pub(super) const PLUGIN_ID_ENTITY_LABEL: &str = "a plugin id";
pub(super) const POLICY_ID_ENTITY_LABEL: &str = "a policy id";
pub(super) const PLUGIN_ENTITY_LABEL: &str = "plugin";
pub(super) const AGENT_RUNTIME_ENTITY_LABEL: &str = "agent runtime";
pub(super) const TEMPLATE_PROVIDER_ENTITY_LABEL: &str = "template provider";
pub(super) const PROMPT_PROVIDER_ENTITY_LABEL: &str = "prompt provider";
pub(super) const POLICY_ENTITY_LABEL: &str = "policy";

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
