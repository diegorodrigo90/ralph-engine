//! Shared CLI locale resolution and message helpers.

use std::env;

use crate::CliError;
use re_config::SupportedLocale;

pub(super) struct CliLocaleCatalog {
    // ── Existing labels ──────────────────────────────────────────
    pub root_bootstrapped: &'static str,
    pub providers_label: &'static str,
    pub name_label: &'static str,
    pub summary_label: &'static str,
    pub kind_label: &'static str,
    pub resolved_activation_label: &'static str,
    pub resolved_from_label: &'static str,
    pub activation_label: &'static str,
    pub load_boundary_label: &'static str,
    pub policy_label: &'static str,
    pub policies_label: &'static str,
    pub policy_enforcement_hook_label: &'static str,
    pub capability_label: &'static str,
    pub capabilities_label: &'static str,
    pub check_label: &'static str,
    pub checks_label: &'static str,
    pub hook_label: &'static str,
    pub hooks_label: &'static str,
    pub registration_hook_label: &'static str,
    pub assets_label: &'static str,
    pub provider_label: &'static str,
    pub locale_id_entity_label: &'static str,
    pub mcp_server_id_entity_label: &'static str,
    pub plugin_config_entity_label: &'static str,
    pub plugin_id_entity_label: &'static str,
    pub agent_id_entity_label: &'static str,
    pub template_id_entity_label: &'static str,
    pub prompt_id_entity_label: &'static str,
    pub policy_id_entity_label: &'static str,
    pub capability_id_entity_label: &'static str,
    pub check_id_entity_label: &'static str,
    pub hook_id_entity_label: &'static str,
    pub provider_id_entity_label: &'static str,
    pub plugin_entity_label: &'static str,
    pub agent_runtime_entity_label: &'static str,
    pub template_entity_label: &'static str,
    pub prompt_entity_label: &'static str,
    pub policy_entity_label: &'static str,
    pub capability_entity_label: &'static str,
    pub check_entity_label: &'static str,
    pub hook_entity_label: &'static str,
    pub provider_entity_label: &'static str,
    pub locale_entity_label: &'static str,
    pub mcp_server_entity_label: &'static str,
    #[allow(dead_code)]
    pub materialized_assets_heading: &'static str,

    // ── Help / usage ─────────────────────────────────────────────
    pub usage_help: &'static str,
    pub commands_heading: &'static str,
    pub set_locale_help: &'static str,
    pub show_version_help: &'static str,
    pub show_help_help: &'static str,
    pub flags_heading: &'static str,
    pub subcommand_label: &'static str,
    pub subcommands_heading: &'static str,
    pub usage_label: &'static str,

    // ── Command descriptions ────────────────────────────────────
    pub cmd_run: &'static str,
    pub cmd_tui: &'static str,
    pub cmd_init: &'static str,
    pub cmd_doctor: &'static str,
    pub cmd_plugins: &'static str,
    pub cmd_install: &'static str,
    pub cmd_uninstall: &'static str,
    pub cmd_agents: &'static str,
    pub cmd_mcp: &'static str,
    pub cmd_checks: &'static str,
    pub cmd_templates: &'static str,
    pub cmd_prompts: &'static str,
    pub cmd_policies: &'static str,
    pub cmd_hooks: &'static str,
    pub cmd_config: &'static str,
    pub cmd_runtime: &'static str,
    pub cmd_capabilities: &'static str,
    pub cmd_providers: &'static str,
    pub cmd_locales: &'static str,

    // ── Run command ──────────────────────────────────────────────
    #[allow(dead_code)]
    pub run_id_required: &'static str,
    pub run_no_items: &'static str,
    pub run_available_items: &'static str,
    pub run_work_item_id_label: &'static str,
    pub run_workflow_label: &'static str,
    pub run_agent_label: &'static str,
    pub run_work_item_label: &'static str,
    pub run_prompt_size_label: &'static str,
    pub run_agent_ready_label: &'static str,
    pub run_execution_plan: &'static str,
    pub run_source_label: &'static str,
    pub run_context_files_label: &'static str,
    pub run_hint_label: &'static str,
    pub run_agent_not_ready: &'static str,
    pub run_work_item_not_found: &'static str,
    pub run_use_list_hint: &'static str,
    #[allow(dead_code)]
    pub run_launching_agent: &'static str,
    pub run_agent_completed: &'static str,
    pub run_agent_failed: &'static str,
    pub run_missing_agent_id: &'static str,
    pub run_config_not_found: &'static str,
    pub run_missing_workflow_plugin: &'static str,
    pub run_missing_agent_plugin: &'static str,
    pub run_workflow_no_runtime: &'static str,
    pub run_agent_no_runtime: &'static str,
    pub run_autonomous_rejected: &'static str,
    pub run_cwd_error: &'static str,
    pub run_autonomous_warning: &'static str,

    // ── Init command ─────────────────────────────────────────────
    pub init_help: &'static str,
    pub init_exists_warning: &'static str,
    pub init_overwrite_prompt: &'static str,
    pub init_cancelled: &'static str,
    pub init_no_templates: &'static str,
    pub init_select_template: &'static str,
    pub init_enable_additional: &'static str,
    pub init_enabled_label: &'static str,
    pub init_additional_plugins: &'static str,
    pub init_enable_label: &'static str,
    pub init_created_label: &'static str,
    pub init_done_prefix: &'static str,
    pub init_done_suffix: &'static str,

    // ── Install command ──────────────────────────────────────────
    pub install_help: &'static str,
    pub install_ref_required: &'static str,
    pub uninstall_ref_required: &'static str,
    pub install_success: &'static str,
    pub install_id_label: &'static str,
    pub install_location_label: &'static str,
    pub install_no_manifest: &'static str,

    // ── Agents command ───────────────────────────────────────────
    pub agents_bootstrap_probe: &'static str,
    pub agents_bootstrap_registered: &'static str,
    pub agents_bootstrap_not_registered: &'static str,
    pub agents_no_runtime: &'static str,

    // ── MCP command ──────────────────────────────────────────────
    pub mcp_launch_probe: &'static str,
    pub mcp_no_runtime: &'static str,
    pub mcp_binary_found: &'static str,
    pub mcp_spawning_label: &'static str,
    pub mcp_process_exited: &'static str,
    pub mcp_binary_not_found: &'static str,

    // ── Checks / Doctor command ──────────────────────────────────
    pub checks_plugin_execution: &'static str,
    pub checks_file_validation: &'static str,
    pub checks_missing_label: &'static str,
    pub checks_scaffold_hint: &'static str,

    // ── Policies command ─────────────────────────────────────────
    pub policies_file_validation: &'static str,
    pub policies_missing_label: &'static str,

    // ── Hooks command ────────────────────────────────────────────
    pub hooks_plan_heading: &'static str,

    // ── Parameterized functions (fn_fields in build.rs) ──────────
    pub unknown_command: fn(&str) -> String,
    pub unknown_subcommand: fn(&str, &str) -> String,
    pub missing_id: fn(&str, &str) -> String,
    pub missing_argument: fn(&str, &str) -> String,
    pub unknown_entity: fn(&str, &str) -> String,
    pub missing_asset_path: fn(&str) -> String,
    #[allow(dead_code)]
    pub missing_output_directory: fn(&str) -> String,
    #[allow(dead_code)]
    pub missing_output_path: fn(&str) -> String,
    #[allow(dead_code)]
    pub invalid_embedded_asset_path: fn(&str) -> String,
    #[allow(dead_code)]
    pub failed_to_write_output: fn(&str, &str) -> String,
    #[allow(dead_code)]
    pub wrote_output: fn(&str) -> String,
    pub unknown_template_asset: fn(&str) -> String,
    pub unknown_prompt_asset: fn(&str) -> String,
    pub unknown_check_asset: fn(&str) -> String,
    pub unknown_policy_asset: fn(&str) -> String,
    pub install_already_installed: fn(&str, &str) -> String,
    pub install_create_dir_failed: fn(&str) -> String,
    pub install_clone_exec_failed: fn(&str) -> String,
    pub install_clone_repo_failed: fn(&str) -> String,
    pub install_not_installed: fn(&str) -> String,
    pub install_remove_dir_failed: fn(&str) -> String,
    pub install_uninstalled: fn(&str) -> String,
    pub init_remove_failed: fn(&str) -> String,
    pub mcp_install_hint: fn(&str) -> String,
    pub policies_materialize_hint: fn(&str) -> String,
}

// Hand-coded fn implementations for each locale
mod fn_en;
mod fn_pt_br;

// Locale modules and dispatch function generated from locales/*.toml
include!(concat!(env!("OUT_DIR"), "/i18n_generated.rs"));

const LOCALE_ENV_KEY: &str = "RALPH_ENGINE_LOCALE";
const LOCALE_FLAG: &str = "--locale";
const LOCALE_SHORT_FLAG: &str = "-L";

#[derive(Debug)]
pub struct ResolvedCliInvocation {
    pub locale: &'static str,
    pub command_index: usize,
}

#[must_use]
pub fn is_pt_br(locale: &str) -> bool {
    matches!(
        re_config::resolve_supported_locale_or_default(locale),
        SupportedLocale::PtBr
    )
}

pub fn resolve_cli_invocation(args: &[String]) -> Result<ResolvedCliInvocation, CliError> {
    resolve_cli_invocation_from_env_result(args, env::var(LOCALE_ENV_KEY))
}

fn resolve_cli_invocation_from_env_result(
    args: &[String],
    env_result: Result<String, env::VarError>,
) -> Result<ResolvedCliInvocation, CliError> {
    match args.get(1).map(String::as_str) {
        Some(LOCALE_FLAG | LOCALE_SHORT_FLAG) => {
            let locale_value = args
                .get(2)
                .ok_or_else(|| CliError::new(format!("{LOCALE_FLAG} requires a locale id")))?;

            Ok(ResolvedCliInvocation {
                locale: normalize_cli_locale(locale_value)?,
                command_index: 3,
            })
        }
        _ => Ok(ResolvedCliInvocation {
            locale: resolve_cli_locale_from_env_result(env_result)?,
            command_index: 1,
        }),
    }
}

fn resolve_cli_locale_from_env_result(
    env_result: Result<String, env::VarError>,
) -> Result<&'static str, CliError> {
    let os_values: Vec<Option<String>> = ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .map(|key| env::var(key).ok())
        .collect();

    resolve_cli_locale_from_env_and_os(env_result, &os_values)
}

fn resolve_cli_locale_from_env_and_os(
    env_result: Result<String, env::VarError>,
    os_values: &[Option<String>],
) -> Result<&'static str, CliError> {
    match env_result {
        Ok(value) => normalize_cli_locale(&value),
        Err(env::VarError::NotPresent) => resolve_locale_from_os_values(os_values),
        Err(error) => Err(CliError::new(format!(
            "failed to read {LOCALE_ENV_KEY}: {error}"
        ))),
    }
}

/// Pure function that resolves a locale from a list of OS locale values.
/// Checks each value in priority order and returns the first match.
fn resolve_locale_from_os_values(os_values: &[Option<String>]) -> Result<&'static str, CliError> {
    for value in os_values.iter().flatten() {
        if let Some(locale) = re_config::parse_os_locale(value) {
            return Ok(locale.as_str());
        }
    }

    // No OS locale matched — use the project config default (English)
    normalize_cli_locale(re_config::default_project_config().default_locale)
}

fn normalize_cli_locale(value: &str) -> Result<&'static str, CliError> {
    let normalized = value.trim();
    match re_config::parse_supported_locale(normalized) {
        Some(locale) => Ok(locale.as_str()),
        None => {
            let other = normalized.to_ascii_lowercase();
            Err(CliError::new(format!(
                "unsupported locale: {other}. supported locales: {}",
                supported_locale_ids(),
            )))
        }
    }
}

fn supported_locale_ids() -> String {
    re_config::supported_locales()
        .iter()
        .map(|locale| locale.id)
        .collect::<Vec<_>>()
        .join(", ")
}

// ── Catalog accessor helpers ─────────────────────────────────────

/// Generates a public accessor function that reads a `&'static str`
/// field from the locale catalog.
macro_rules! catalog_str {
    ($name:ident, $field:ident) => {
        #[must_use]
        pub fn $name(locale: &str) -> &'static str {
            locale_catalog(locale).$field
        }
    };
}

/// Generates a public accessor function for a single-argument
/// parameterized locale function.
macro_rules! catalog_fn1 {
    ($name:ident, $field:ident) => {
        #[must_use]
        pub fn $name(locale: &str, arg: &str) -> String {
            (locale_catalog(locale).$field)(arg)
        }
    };
}

/// Generates a public accessor function for a two-argument
/// parameterized locale function.
macro_rules! catalog_fn2 {
    ($name:ident, $field:ident) => {
        #[must_use]
        pub fn $name(locale: &str, a: &str, b: &str) -> String {
            (locale_catalog(locale).$field)(a, b)
        }
    };
}

// ── Simple catalog accessors ─────────────────────────────────────

#[must_use]
pub fn root_bootstrapped(locale: &str) -> &'static str {
    locale_catalog(locale).root_bootstrapped
}

#[must_use]
pub fn unknown_command(locale: &str, command_name: &str) -> String {
    (locale_catalog(locale).unknown_command)(command_name)
}

#[must_use]
pub fn unknown_subcommand(locale: &str, command_group: &str, command_name: &str) -> String {
    (locale_catalog(locale).unknown_subcommand)(command_group, command_name)
}

#[must_use]
pub fn missing_id(locale: &str, command_group: &str, entity_label: &str) -> String {
    (locale_catalog(locale).missing_id)(command_group, entity_label)
}

#[must_use]
pub fn missing_argument(locale: &str, command_path: &str, entity_label: &str) -> String {
    (locale_catalog(locale).missing_argument)(command_path, entity_label)
}

#[must_use]
pub fn unknown_entity(locale: &str, entity_label: &str, value: &str) -> String {
    (locale_catalog(locale).unknown_entity)(entity_label, value)
}

#[must_use]
pub fn missing_asset_path(locale: &str, command_group: &str) -> String {
    (locale_catalog(locale).missing_asset_path)(command_group)
}

#[must_use]
#[allow(dead_code)]
pub fn missing_output_directory(locale: &str, command_group: &str) -> String {
    (locale_catalog(locale).missing_output_directory)(command_group)
}

#[must_use]
#[allow(dead_code)]
pub fn missing_output_path(locale: &str, command_group: &str) -> String {
    (locale_catalog(locale).missing_output_path)(command_group)
}

#[must_use]
#[allow(dead_code)]
pub fn invalid_embedded_asset_path(locale: &str, value: &str) -> String {
    (locale_catalog(locale).invalid_embedded_asset_path)(value)
}

#[must_use]
#[allow(dead_code)]
pub fn failed_to_write_output(locale: &str, path: &str, error: &str) -> String {
    (locale_catalog(locale).failed_to_write_output)(path, error)
}

#[must_use]
#[allow(dead_code)]
pub fn wrote_output(locale: &str, path: &str) -> String {
    (locale_catalog(locale).wrote_output)(path)
}

#[must_use]
pub fn unknown_template_asset(locale: &str, value: &str) -> String {
    (locale_catalog(locale).unknown_template_asset)(value)
}

#[must_use]
pub fn unknown_prompt_asset(locale: &str, value: &str) -> String {
    (locale_catalog(locale).unknown_prompt_asset)(value)
}

#[must_use]
pub fn unknown_check_asset(locale: &str, value: &str) -> String {
    (locale_catalog(locale).unknown_check_asset)(value)
}

#[must_use]
pub fn unknown_policy_asset(locale: &str, value: &str) -> String {
    (locale_catalog(locale).unknown_policy_asset)(value)
}

// ── Heading helpers ──────────────────────────────────────────────

#[must_use]
pub fn list_heading(locale: &str, singular_en: &str, singular_pt: &str, count: usize) -> String {
    if is_pt_br(locale) {
        format!("{singular_pt} ({count})")
    } else {
        format!("{singular_en} ({count})")
    }
}

#[must_use]
pub fn detail_heading(locale: &str, label_en: &str, label_pt: &str, value: &str) -> String {
    if is_pt_br(locale) {
        format!("{label_pt}: {value}")
    } else {
        format!("{label_en}: {value}")
    }
}

#[must_use]
pub fn providers_heading(locale: &str, count: usize) -> String {
    format!("{} ({count})", locale_catalog(locale).providers_label)
}

#[must_use]
#[allow(dead_code)]
pub fn materialized_assets_heading(locale: &str, count: usize) -> String {
    format!(
        "{} ({count})",
        locale_catalog(locale).materialized_assets_heading
    )
}

// ── Entity/surface label accessors (existing) ────────────────────

catalog_str!(resolved_activation_label, resolved_activation_label);
catalog_str!(resolved_from_label, resolved_from_label);
catalog_str!(name_label, name_label);
catalog_str!(summary_label, summary_label);
catalog_str!(kind_label, kind_label);
catalog_str!(activation_label, activation_label);
catalog_str!(load_boundary_label, load_boundary_label);
catalog_str!(policy_label, policy_label);
catalog_str!(policies_label, policies_label);
catalog_str!(policy_enforcement_hook_label, policy_enforcement_hook_label);
catalog_str!(capability_label, capability_label);
catalog_str!(capabilities_label, capabilities_label);
catalog_str!(check_label, check_label);
catalog_str!(checks_label, checks_label);
catalog_str!(hook_label, hook_label);
catalog_str!(hooks_label, hooks_label);
catalog_str!(registration_hook_label, registration_hook_label);
catalog_str!(assets_label, assets_label);
catalog_str!(providers_label, providers_label);
catalog_str!(provider_label, provider_label);
catalog_str!(plugin_id_entity_label, plugin_id_entity_label);
catalog_str!(policy_id_entity_label, policy_id_entity_label);
catalog_str!(capability_id_entity_label, capability_id_entity_label);
catalog_str!(check_id_entity_label, check_id_entity_label);
catalog_str!(hook_id_entity_label, hook_id_entity_label);
catalog_str!(provider_id_entity_label, provider_id_entity_label);
catalog_str!(locale_id_entity_label, locale_id_entity_label);
catalog_str!(mcp_server_id_entity_label, mcp_server_id_entity_label);
catalog_str!(plugin_config_entity_label, plugin_config_entity_label);
catalog_str!(agent_id_entity_label, agent_id_entity_label);
catalog_str!(template_id_entity_label, template_id_entity_label);
catalog_str!(prompt_id_entity_label, prompt_id_entity_label);
catalog_str!(plugin_entity_label, plugin_entity_label);
catalog_str!(mcp_server_entity_label, mcp_server_entity_label);
catalog_str!(capability_entity_label, capability_entity_label);
catalog_str!(check_entity_label, check_entity_label);
catalog_str!(hook_entity_label, hook_entity_label);
catalog_str!(provider_entity_label, provider_entity_label);
catalog_str!(locale_entity_label, locale_entity_label);
catalog_str!(agent_runtime_entity_label, agent_runtime_entity_label);
catalog_str!(template_entity_label, template_entity_label);
catalog_str!(prompt_entity_label, prompt_entity_label);
catalog_str!(policy_entity_label, policy_entity_label);

// ── Help / usage accessors ───────────────────────────────────────

catalog_str!(usage_help, usage_help);
catalog_str!(commands_heading, commands_heading);
catalog_str!(set_locale_help, set_locale_help);
catalog_str!(show_version_help, show_version_help);
catalog_str!(show_help_help, show_help_help);
catalog_str!(subcommand_label, subcommand_label);
catalog_str!(flags_heading, flags_heading);
catalog_str!(subcommands_heading, subcommands_heading);
catalog_str!(usage_label, usage_label);

// ── Command description accessors ───────────────────────────────

catalog_str!(cmd_run, cmd_run);
catalog_str!(cmd_tui, cmd_tui);
catalog_str!(cmd_init, cmd_init);
catalog_str!(cmd_doctor, cmd_doctor);
catalog_str!(cmd_plugins, cmd_plugins);
catalog_str!(cmd_install, cmd_install);
catalog_str!(cmd_uninstall, cmd_uninstall);
catalog_str!(cmd_agents, cmd_agents);
catalog_str!(cmd_mcp, cmd_mcp);
catalog_str!(cmd_checks, cmd_checks);
catalog_str!(cmd_templates, cmd_templates);
catalog_str!(cmd_prompts, cmd_prompts);
catalog_str!(cmd_policies, cmd_policies);
catalog_str!(cmd_hooks, cmd_hooks);
catalog_str!(cmd_config, cmd_config);
catalog_str!(cmd_runtime, cmd_runtime);
catalog_str!(cmd_capabilities, cmd_capabilities);
catalog_str!(cmd_providers, cmd_providers);
catalog_str!(cmd_locales, cmd_locales);

// ── Run command accessors ────────────────────────────────────────

// run_id_required: available in TOML catalog, not used — run without args enters loop mode.
catalog_str!(run_no_items, run_no_items);
catalog_str!(run_available_items, run_available_items);
catalog_str!(run_work_item_id_label, run_work_item_id_label);
catalog_str!(run_workflow_label, run_workflow_label);
catalog_str!(run_agent_label, run_agent_label);
catalog_str!(run_work_item_label, run_work_item_label);
catalog_str!(run_prompt_size_label, run_prompt_size_label);
catalog_str!(run_agent_ready_label, run_agent_ready_label);
catalog_str!(run_execution_plan, run_execution_plan);
catalog_str!(run_source_label, run_source_label);
catalog_str!(run_context_files_label, run_context_files_label);
catalog_str!(run_hint_label, run_hint_label);
catalog_str!(run_agent_not_ready, run_agent_not_ready);
catalog_str!(run_work_item_not_found, run_work_item_not_found);
catalog_str!(run_use_list_hint, run_use_list_hint);
// run_launching_agent: available in TOML catalog, not currently used in code.
catalog_str!(run_agent_completed, run_agent_completed);
catalog_str!(run_agent_failed, run_agent_failed);
catalog_str!(run_missing_agent_id, run_missing_agent_id);
catalog_str!(run_config_not_found, run_config_not_found);
catalog_str!(run_missing_workflow_plugin, run_missing_workflow_plugin);
catalog_str!(run_missing_agent_plugin, run_missing_agent_plugin);
catalog_str!(run_workflow_no_runtime, run_workflow_no_runtime);
catalog_str!(run_agent_no_runtime, run_agent_no_runtime);
catalog_str!(run_autonomous_rejected, run_autonomous_rejected);
catalog_str!(run_cwd_error, run_cwd_error);
catalog_str!(run_autonomous_warning, run_autonomous_warning);

// ── Init command accessors ───────────────────────────────────────

catalog_str!(init_help, init_help);
catalog_str!(init_exists_warning, init_exists_warning);
catalog_str!(init_overwrite_prompt, init_overwrite_prompt);
catalog_str!(init_cancelled, init_cancelled);
catalog_str!(init_no_templates, init_no_templates);
catalog_str!(init_select_template, init_select_template);
catalog_str!(init_enable_additional, init_enable_additional);
catalog_str!(init_enabled_label, init_enabled_label);
catalog_str!(init_additional_plugins, init_additional_plugins);
catalog_str!(init_enable_label, init_enable_label);
catalog_str!(init_created_label, init_created_label);
catalog_str!(init_done_prefix, init_done_prefix);
catalog_str!(init_done_suffix, init_done_suffix);

// ── Install command accessors ────────────────────────────────────

catalog_str!(install_help, install_help);
catalog_str!(install_ref_required, install_ref_required);
catalog_str!(uninstall_ref_required, uninstall_ref_required);
catalog_str!(install_success, install_success);
catalog_str!(install_id_label, install_id_label);
catalog_str!(install_location_label, install_location_label);
catalog_str!(install_no_manifest, install_no_manifest);
catalog_fn2!(install_already_installed, install_already_installed);
catalog_fn1!(install_create_dir_failed, install_create_dir_failed);
catalog_fn1!(install_clone_exec_failed, install_clone_exec_failed);
catalog_fn1!(install_clone_repo_failed, install_clone_repo_failed);
catalog_fn1!(install_not_installed, install_not_installed);
catalog_fn1!(install_remove_dir_failed, install_remove_dir_failed);
catalog_fn1!(install_uninstalled, install_uninstalled);

// ── Agents command accessors ─────────────────────────────────────

catalog_str!(agents_bootstrap_probe, agents_bootstrap_probe);
catalog_str!(agents_bootstrap_registered, agents_bootstrap_registered);
catalog_str!(
    agents_bootstrap_not_registered,
    agents_bootstrap_not_registered
);
catalog_str!(agents_no_runtime, agents_no_runtime);

// ── MCP command accessors ────────────────────────────────────────

catalog_str!(mcp_launch_probe, mcp_launch_probe);
catalog_str!(mcp_no_runtime, mcp_no_runtime);
catalog_str!(mcp_binary_found, mcp_binary_found);
catalog_str!(mcp_spawning_label, mcp_spawning_label);
catalog_str!(mcp_process_exited, mcp_process_exited);
catalog_str!(mcp_binary_not_found, mcp_binary_not_found);
catalog_fn1!(mcp_install_hint, mcp_install_hint);

// ── Checks / Doctor command accessors ────────────────────────────

catalog_str!(checks_plugin_execution, checks_plugin_execution);
catalog_str!(checks_file_validation, checks_file_validation);
catalog_str!(checks_missing_label, checks_missing_label);
catalog_str!(checks_scaffold_hint, checks_scaffold_hint);

// ── Policies command accessors ───────────────────────────────────

catalog_str!(policies_file_validation, policies_file_validation);
catalog_str!(policies_missing_label, policies_missing_label);
catalog_fn1!(policies_materialize_hint, policies_materialize_hint);

// ── Hooks command accessors ──────────────────────────────────────

catalog_str!(hooks_plan_heading, hooks_plan_heading);

// ── Init parameterized ───────────────────────────────────────────

catalog_fn1!(init_remove_failed, init_remove_failed);

#[cfg(test)]
mod tests {
    use std::env;

    use crate::CliError;

    use super::{
        LOCALE_FLAG, activation_label, agents_bootstrap_probe, checks_file_validation,
        hooks_plan_heading, install_already_installed, install_help, install_uninstalled,
        load_boundary_label, mcp_install_hint, mcp_launch_probe, missing_id, normalize_cli_locale,
        policies_file_validation, providers_heading, resolve_cli_invocation,
        resolve_cli_invocation_from_env_result, resolve_cli_locale_from_env_and_os,
        resolve_cli_locale_from_env_result, resolve_locale_from_os_values, root_bootstrapped,
        run_available_items, unknown_command, unknown_entity, unknown_subcommand,
    };

    #[test]
    fn normalize_cli_locale_accepts_supported_values() {
        assert!(matches!(normalize_cli_locale("en"), Ok("en")));
        assert!(matches!(normalize_cli_locale("pt-BR"), Ok("pt-br")));
    }

    #[test]
    fn normalize_cli_locale_rejects_unknown_values() {
        let result = normalize_cli_locale("es");

        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "unsupported locale: es. supported locales: en, pt-br"
            );
        }
    }

    #[test]
    fn helper_messages_render_pt_br() {
        assert_eq!(root_bootstrapped("pt-br"), "Fundação Rust inicializada.");
        assert_eq!(
            unknown_command("pt-br", "oops"),
            "comando desconhecido: oops"
        );
        assert_eq!(
            unknown_subcommand("pt-br", "plugins", "oops"),
            "subcomando desconhecido em plugins: oops"
        );
        assert_eq!(
            missing_id("pt-br", "plugins", "um id de plugin"),
            "plugins show exige um id de plugin"
        );
        assert_eq!(
            unknown_entity("pt-br", "plugin", "fixture.missing"),
            "plugin desconhecido: fixture.missing"
        );
        assert_eq!(
            unknown_entity("pt-br", "capacidade", "template"),
            "capacidade desconhecida: template"
        );
        assert_eq!(providers_heading("pt-br", 2), "Provedores (2)");
        assert_eq!(activation_label("pt-br"), "Ativação");
        assert_eq!(load_boundary_label("pt-br"), "Fronteira de carregamento");
    }

    #[test]
    fn new_catalog_accessors_work_en() {
        assert_eq!(run_available_items("en"), "Available work items");
        assert!(install_help("en").contains("ralph-engine install"));
        assert_eq!(agents_bootstrap_probe("en"), "Agent bootstrap probe");
        assert_eq!(mcp_launch_probe("en"), "MCP launch probe");
        assert_eq!(checks_file_validation("en"), "Project file validation");
        assert_eq!(policies_file_validation("en"), "Policy file validation");
        assert_eq!(hooks_plan_heading("en"), "Runtime hook plan");
    }

    #[test]
    fn new_catalog_accessors_work_pt_br() {
        assert_eq!(run_available_items("pt-br"), "Work items disponíveis");
        assert!(install_help("pt-br").contains("ralph-engine install"));
        assert_eq!(
            agents_bootstrap_probe("pt-br"),
            "Verificação de bootstrap de agente"
        );
        assert_eq!(mcp_launch_probe("pt-br"), "Verificação de lançamento MCP");
        assert_eq!(
            checks_file_validation("pt-br"),
            "Validação de arquivos do projeto"
        );
        assert_eq!(
            policies_file_validation("pt-br"),
            "Validação de arquivos da política"
        );
        assert_eq!(hooks_plan_heading("pt-br"), "Plano de hook de runtime");
    }

    #[test]
    fn parameterized_accessors_work() {
        assert!(
            install_already_installed("en", "test.plugin", "/path").contains("already installed")
        );
        assert!(
            install_already_installed("pt-br", "test.plugin", "/path")
                .contains("já está instalado")
        );
        assert!(install_uninstalled("en", "test.plugin").contains("uninstalled"));
        assert!(install_uninstalled("pt-br", "test.plugin").contains("desinstalado"));
        assert!(mcp_install_hint("en", "node").contains("install 'node'"));
        assert!(mcp_install_hint("pt-br", "node").contains("instale 'node'"));
    }

    #[test]
    fn resolve_cli_invocation_reads_locale_flag() {
        let args = vec![
            "ralph-engine".to_owned(),
            "--locale".to_owned(),
            "pt-br".to_owned(),
            "plugins".to_owned(),
        ];

        let invocation = resolve_cli_invocation(&args);

        assert!(invocation.is_ok());

        if let Ok(resolved) = invocation {
            assert_eq!(resolved.locale, "pt-br");
            assert_eq!(resolved.command_index, 3);
        }
    }

    #[test]
    fn resolve_cli_invocation_falls_back_to_default_when_env_not_set() {
        let invocation =
            resolve_cli_locale_from_env_and_os(Err(env::VarError::NotPresent), &[None, None, None]);

        // With no env and no OS locale, falls back to English
        assert_eq!(invocation, Ok("en"));
    }

    #[test]
    fn resolve_cli_locale_from_env_result_accepts_supported_value() {
        let result = resolve_cli_locale_from_env_result(Ok("pt-br".to_owned()));

        assert_eq!(result, Ok("pt-br"));
    }

    #[test]
    fn resolve_cli_locale_from_env_result_falls_back_to_en_when_not_set() {
        let result: Result<&'static str, CliError> =
            resolve_cli_locale_from_env_result(Err(env::VarError::NotPresent));

        assert!(result.is_ok());
    }

    #[test]
    fn resolve_cli_locale_from_env_and_os_picks_explicit_over_os() {
        let result = resolve_cli_locale_from_env_and_os(
            Ok("pt-br".to_owned()),
            &[Some("en_US.UTF-8".to_owned())],
        );
        assert_eq!(result, Ok("pt-br"));
    }

    #[test]
    fn resolve_cli_locale_from_env_and_os_reads_os_when_env_not_set() {
        let result = resolve_cli_locale_from_env_and_os(
            Err(env::VarError::NotPresent),
            &[None, None, Some("pt_BR.UTF-8".to_owned())],
        );
        assert_eq!(result, Ok("pt-br"));
    }

    #[test]
    fn resolve_locale_from_os_values_returns_first_match() {
        let result = resolve_locale_from_os_values(&[
            None,
            Some("pt_BR.UTF-8".to_owned()),
            Some("en_US.UTF-8".to_owned()),
        ]);
        assert_eq!(result, Ok("pt-br"));
    }

    #[test]
    fn resolve_locale_from_os_values_falls_back_to_english() {
        let result = resolve_locale_from_os_values(&[None, None, None]);
        assert_eq!(result, Ok("en"));
    }

    #[test]
    fn resolve_locale_from_os_values_skips_unsupported() {
        let result = resolve_locale_from_os_values(&[
            Some("ja_JP.UTF-8".to_owned()),
            Some("en_US.UTF-8".to_owned()),
        ]);
        assert_eq!(result, Ok("en"));
    }

    #[test]
    fn resolve_cli_invocation_reads_short_locale_flag() {
        let args = vec![
            "ralph-engine".to_owned(),
            "-L".to_owned(),
            "pt-br".to_owned(),
            "plugins".to_owned(),
        ];

        let result = resolve_cli_invocation_from_env_result(&args, Err(env::VarError::NotPresent));

        assert!(result.is_ok());
        if let Ok(resolved) = result {
            assert_eq!(resolved.locale, "pt-br");
            assert_eq!(resolved.command_index, 3);
        }
    }

    #[test]
    fn resolve_cli_invocation_locale_flag_without_value_errors() {
        let args = vec!["ralph-engine".to_owned(), LOCALE_FLAG.to_owned()];

        let result = resolve_cli_invocation_from_env_result(&args, Err(env::VarError::NotPresent));

        assert!(result.is_err());
    }
}
