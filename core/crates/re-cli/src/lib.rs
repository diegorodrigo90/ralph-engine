//! CLI command execution for Ralph Engine.

mod catalog;
mod commands;
mod error;

pub use error::CliError;

/// Executes the current CLI foundation command set.
pub fn execute<I>(args: I) -> Result<String, CliError>
where
    I: IntoIterator<Item = String>,
{
    let collected: Vec<String> = args.into_iter().collect();
    commands::execute(&collected)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::execute;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn execute_without_subcommand_prints_foundation_banner() {
        // Arrange
        let command = args(&["ralph-engine"]);

        // Act
        let output = execute(command).expect("default command should succeed");

        // Assert
        assert!(output.contains("Rust foundation bootstrapped."));
        assert!(output.contains("Ralph Engine"));
    }

    #[test]
    fn execute_version_returns_package_version() {
        // Arrange
        let command = args(&["ralph-engine", "--version"]);

        // Act
        let output = execute(command).expect("version should succeed");

        // Assert
        assert_eq!(output, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn execute_agents_lists_runtime_agents() {
        // Arrange
        let command = args(&["ralph-engine", "agents", "list"]);

        // Act
        let output = execute(command).expect("agents list should succeed");

        // Assert
        assert!(output.contains("Agent runtimes (3)"));
        assert!(output.contains("- official.claude | activation=disabled"));
        assert!(output.contains("- official.codex | activation=disabled"));
    }

    #[test]
    fn execute_agents_show_returns_provider_detail() {
        // Arrange
        let command = args(&["ralph-engine", "agents", "show", "official.codex"]);

        // Act
        let output = execute(command).expect("agents show should succeed");

        // Assert
        assert!(output.contains("Agent runtime: official.codex"));
        assert!(output.contains("Providers (1)"));
        assert!(output.contains(
            "- official.codex | activation=disabled | boundary=in_process | bootstrap_hook=true"
        ));
    }

    #[test]
    fn execute_agents_show_requires_plugin_id() {
        // Arrange
        let command = args(&["ralph-engine", "agents", "show"]);

        // Act
        let error = execute(command).expect_err("missing plugin id should fail");

        // Assert
        assert_eq!(error.to_string(), "agents show requires a plugin id");
    }

    #[test]
    fn execute_agents_show_rejects_unknown_plugin() {
        // Arrange
        let command = args(&["ralph-engine", "agents", "show", "official.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown agent runtime should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown agent runtime: official.unknown");
    }

    #[test]
    fn execute_templates_list_runtime_templates() {
        // Arrange
        let command = args(&["ralph-engine", "templates", "list"]);

        // Act
        let output = execute(command).expect("templates list should succeed");

        // Assert
        assert!(output.contains("Templates (3)"));
        assert!(output.contains("- official.basic | activation=enabled"));
        assert!(output.contains("- official.bmad | activation=disabled"));
    }

    #[test]
    fn execute_templates_show_returns_provider_detail() {
        // Arrange
        let command = args(&["ralph-engine", "templates", "show", "official.basic"]);

        // Act
        let output = execute(command).expect("templates show should succeed");

        // Assert
        assert!(output.contains("Template provider: official.basic"));
        assert!(output.contains("Providers (1)"));
        assert!(output.contains(
            "- official.basic | activation=enabled | boundary=in_process | scaffold_hook=true"
        ));
    }

    #[test]
    fn execute_templates_show_requires_plugin_id() {
        // Arrange
        let command = args(&["ralph-engine", "templates", "show"]);

        // Act
        let error = execute(command).expect_err("missing template plugin id should fail");

        // Assert
        assert_eq!(error.to_string(), "templates show requires a plugin id");
    }

    #[test]
    fn execute_templates_show_rejects_unknown_plugin() {
        // Arrange
        let command = args(&["ralph-engine", "templates", "show", "official.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown template provider should fail");

        // Assert
        assert_eq!(
            error.to_string(),
            "unknown template provider: official.unknown"
        );
    }

    #[test]
    fn execute_prompts_list_runtime_prompts() {
        // Arrange
        let command = args(&["ralph-engine", "prompts", "list"]);

        // Act
        let output = execute(command).expect("prompts list should succeed");

        // Assert
        assert!(output.contains("Prompts (1)"));
        assert!(output.contains("- official.bmad | activation=disabled"));
    }

    #[test]
    fn execute_prompts_show_returns_provider_detail() {
        // Arrange
        let command = args(&["ralph-engine", "prompts", "show", "official.bmad"]);

        // Act
        let output = execute(command).expect("prompts show should succeed");

        // Assert
        assert!(output.contains("Prompt provider: official.bmad"));
        assert!(output.contains("Providers (1)"));
        assert!(output.contains(
            "- official.bmad | activation=disabled | boundary=in_process | prompt_hook=true"
        ));
    }

    #[test]
    fn execute_prompts_show_requires_plugin_id() {
        // Arrange
        let command = args(&["ralph-engine", "prompts", "show"]);

        // Act
        let error = execute(command).expect_err("missing prompt plugin id should fail");

        // Assert
        assert_eq!(error.to_string(), "prompts show requires a plugin id");
    }

    #[test]
    fn execute_prompts_show_rejects_unknown_plugin() {
        // Arrange
        let command = args(&["ralph-engine", "prompts", "show", "official.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown prompt provider should fail");

        // Assert
        assert_eq!(
            error.to_string(),
            "unknown prompt provider: official.unknown"
        );
    }

    #[test]
    fn execute_capabilities_lists_runtime_capabilities() {
        // Arrange
        let command = args(&["ralph-engine", "capabilities", "list"]);

        // Act
        let output = execute(command).expect("capabilities list should succeed");

        // Assert
        assert!(output.contains("Capabilities (11)"));
        assert!(output.contains("- agent_runtime | providers=3 | enabled=0"));
        assert!(output.contains("- template | providers=3 | enabled=1"));
    }

    #[test]
    fn execute_capabilities_show_returns_provider_detail() {
        // Arrange
        let command = args(&["ralph-engine", "capabilities", "show", "mcp_contribution"]);

        // Act
        let output = execute(command).expect("capabilities show should succeed");

        // Assert
        assert!(output.contains("Capability: mcp_contribution"));
        assert!(output.contains("Providers (4)"));
        assert!(output.contains("- official.claude | activation=disabled | boundary=in_process"));
        assert!(output.contains("- official.github | activation=disabled | boundary=in_process"));
    }

    #[test]
    fn execute_capabilities_show_requires_capability_id() {
        // Arrange
        let command = args(&["ralph-engine", "capabilities", "show"]);

        // Act
        let error = execute(command).expect_err("missing capability id should fail");

        // Assert
        assert_eq!(
            error.to_string(),
            "capabilities show requires a capability id"
        );
    }

    #[test]
    fn execute_capabilities_show_rejects_unknown_capability() {
        // Arrange
        let command = args(&["ralph-engine", "capabilities", "show", "unknown"]);

        // Act
        let error = execute(command).expect_err("unknown capability should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown capability: unknown");
    }

    #[test]
    fn execute_checks_lists_runtime_checks() {
        // Arrange
        let command = args(&["ralph-engine", "checks", "list"]);

        // Act
        let output = execute(command).expect("checks list should succeed");

        // Assert
        assert!(output.contains("Checks (2)"));
        assert!(output.contains("- prepare | providers=1 | enabled=0"));
        assert!(output.contains("- doctor | providers=1 | enabled=0"));
    }

    #[test]
    fn execute_checks_show_returns_provider_detail() {
        // Arrange
        let command = args(&["ralph-engine", "checks", "show", "prepare"]);

        // Act
        let output = execute(command).expect("checks show should succeed");

        // Assert
        assert!(output.contains("Check: prepare"));
        assert!(output.contains("Providers (1)"));
        assert!(output.contains(
            "- official.bmad | activation=disabled | boundary=in_process | runtime_hook=true"
        ));
    }

    #[test]
    fn execute_checks_show_requires_check_id() {
        // Arrange
        let command = args(&["ralph-engine", "checks", "show"]);

        // Act
        let error = execute(command).expect_err("missing check id should fail");

        // Assert
        assert_eq!(error.to_string(), "checks show requires a check id");
    }

    #[test]
    fn execute_checks_show_rejects_unknown_check() {
        // Arrange
        let command = args(&["ralph-engine", "checks", "show", "unknown"]);

        // Act
        let error = execute(command).expect_err("unknown check should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown check: unknown");
    }

    #[test]
    fn execute_hooks_lists_runtime_hooks() {
        // Arrange
        let command = args(&["ralph-engine", "hooks", "list"]);

        // Act
        let output = execute(command).expect("hooks list should succeed");

        // Assert
        assert!(output.contains("Runtime hooks (11)"));
        assert!(output.contains("- scaffold | providers=3 | enabled=1"));
        assert!(output.contains("- mcp_registration | providers=4 | enabled=0"));
    }

    #[test]
    fn execute_hooks_show_returns_provider_detail() {
        // Arrange
        let command = args(&["ralph-engine", "hooks", "show", "mcp_registration"]);

        // Act
        let output = execute(command).expect("hooks show should succeed");

        // Assert
        assert!(output.contains("Runtime hook: mcp_registration"));
        assert!(output.contains("Providers (4)"));
        assert!(output.contains("- official.claude | activation=disabled | boundary=in_process"));
        assert!(output.contains("- official.github | activation=disabled | boundary=in_process"));
    }

    #[test]
    fn execute_hooks_show_requires_hook_id() {
        // Arrange
        let command = args(&["ralph-engine", "hooks", "show"]);

        // Act
        let error = execute(command).expect_err("missing hook id should fail");

        // Assert
        assert_eq!(error.to_string(), "hooks show requires a hook id");
    }

    #[test]
    fn execute_hooks_show_rejects_unknown_hook() {
        // Arrange
        let command = args(&["ralph-engine", "hooks", "show", "unknown"]);

        // Act
        let error = execute(command).expect_err("unknown hook should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown hook: unknown");
    }

    #[test]
    fn execute_policies_lists_runtime_policies() {
        // Arrange
        let command = args(&["ralph-engine", "policies", "list"]);

        // Act
        let output = execute(command).expect("policies list should succeed");

        // Assert
        assert!(output.contains("Policies (1)"));
        assert!(output.contains("official.tdd-strict"));
    }

    #[test]
    fn execute_policies_show_returns_provider_detail() {
        // Arrange
        let command = args(&["ralph-engine", "policies", "show", "official.tdd-strict"]);

        // Act
        let output = execute(command).expect("policies show should succeed");

        // Assert
        assert!(output.contains("Policy: official.tdd-strict"));
        assert!(output.contains("Provider: official.tdd-strict"));
        assert!(output.contains("Policy enforcement hook: true"));
    }

    #[test]
    fn execute_policies_show_requires_policy_id() {
        // Arrange
        let command = args(&["ralph-engine", "policies", "show"]);

        // Act
        let error = execute(command).expect_err("missing policy id should fail");

        // Assert
        assert_eq!(error.to_string(), "policies show requires a policy id");
    }

    #[test]
    fn execute_policies_show_rejects_unknown_policy() {
        // Arrange
        let command = args(&["ralph-engine", "policies", "show", "official.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown policy should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown policy: official.unknown");
    }

    #[test]
    fn execute_providers_lists_runtime_providers() {
        // Arrange
        let command = args(&["ralph-engine", "providers", "list"]);

        // Act
        let output = execute(command).expect("providers list should succeed");

        // Assert
        assert!(output.contains("Providers (4)"));
        assert!(output.contains("- data_source | providers=1 | enabled=0"));
        assert!(output.contains("- remote_control | providers=1 | enabled=0"));
    }

    #[test]
    fn execute_providers_show_returns_provider_detail() {
        // Arrange
        let command = args(&["ralph-engine", "providers", "show", "data_source"]);

        // Act
        let output = execute(command).expect("providers show should succeed");

        // Assert
        assert!(output.contains("Provider: data_source"));
        assert!(output.contains("Providers (1)"));
        assert!(output.contains(
            "- official.github | activation=disabled | boundary=in_process | registration_hook=true"
        ));
    }

    #[test]
    fn execute_providers_show_requires_provider_id() {
        // Arrange
        let command = args(&["ralph-engine", "providers", "show"]);

        // Act
        let error = execute(command).expect_err("missing provider id should fail");

        // Assert
        assert_eq!(error.to_string(), "providers show requires a provider id");
    }

    #[test]
    fn execute_providers_show_rejects_unknown_provider() {
        // Arrange
        let command = args(&["ralph-engine", "providers", "show", "unknown"]);

        // Act
        let error = execute(command).expect_err("unknown provider should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown provider: unknown");
    }

    #[test]
    fn execute_plugins_lists_official_plugins() {
        // Arrange
        let command = args(&["ralph-engine", "plugins", "list"]);

        // Act
        let output = execute(command).expect("plugins list should succeed");

        // Assert
        assert!(output.contains("Official plugins (8)"));
        assert!(output.contains("official.basic"));
        assert!(output.contains("official.github"));
    }

    #[test]
    fn execute_plugins_show_returns_plugin_detail() {
        // Arrange
        let command = args(&["ralph-engine", "plugins", "show", "official.github"]);

        // Act
        let output = execute(command).expect("plugins show should succeed");

        // Assert
        assert!(output.contains("Plugin: official.github"));
        assert!(output.contains("Name: GitHub"));
        assert!(output.contains("Lifecycle: discover -> configure -> load"));
        assert!(output.contains("Load boundary: in_process"));
        assert!(output.contains(
            "Runtime hooks: mcp_registration, data_source_registration, context_provider_registration, forge_provider_registration"
        ));
        assert!(output.contains("Resolved activation: disabled"));
        assert!(output.contains("Resolved from: built_in_defaults"));
    }

    #[test]
    fn execute_plugins_show_reports_enabled_default_activation() {
        // Arrange
        let command = args(&["ralph-engine", "plugins", "show", "official.basic"]);

        // Act
        let output = execute(command).expect("plugins show should succeed");

        // Assert
        assert!(output.contains("Plugin: official.basic"));
        assert!(output.contains("Resolved activation: enabled"));
        assert!(output.contains("Resolved from: built_in_defaults"));
    }

    #[test]
    fn execute_plugins_show_requires_plugin_id() {
        // Arrange
        let command = args(&["ralph-engine", "plugins", "show"]);

        // Act
        let error = execute(command).expect_err("missing plugin id should fail");

        // Assert
        assert_eq!(error.to_string(), "plugins show requires a plugin id");
    }

    #[test]
    fn execute_plugins_show_rejects_unknown_plugin() {
        // Arrange
        let command = args(&["ralph-engine", "plugins", "show", "official.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown plugin id should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown plugin: official.unknown");
    }

    #[test]
    fn execute_config_show_defaults_returns_yaml_contract() {
        // Arrange
        let command = args(&["ralph-engine", "config", "show-defaults"]);

        // Act
        let output = execute(command).expect("config show-defaults should succeed");

        // Assert
        assert!(output.contains("schema_version: 1"));
        assert!(output.contains("default_locale: en"));
        assert!(output.contains("official.basic"));
    }

    #[test]
    fn execute_config_layers_returns_typed_resolution_stack() {
        // Arrange
        let command = args(&["ralph-engine", "config", "layers"]);

        // Act
        let output = execute(command).expect("config layers should succeed");

        // Assert
        assert!(output.contains("layers:"));
        assert!(output.contains("scope: built_in_defaults"));
        assert!(output.contains("schema_version: 1"));
        assert!(output.contains("plugin_count: 1"));
        assert!(output.contains("mcp_enabled: true"));
        assert!(output.contains("prompt_tokens: 8192"));
        assert!(output.contains("context_tokens: 32768"));
    }

    #[test]
    fn execute_config_show_layers_alias_returns_typed_resolution_stack() {
        // Arrange
        let command = args(&["ralph-engine", "config", "show-layers"]);

        // Act
        let output = execute(command).expect("config show-layers should succeed");

        // Assert
        assert!(output.contains("layers:"));
        assert!(output.contains("scope: built_in_defaults"));
    }

    #[test]
    fn execute_config_budgets_returns_typed_budget_contract() {
        // Arrange
        let command = args(&["ralph-engine", "config", "budgets"]);

        // Act
        let output = execute(command).expect("config budgets should succeed");

        // Assert
        assert!(output.contains("budgets:"));
        assert!(output.contains("prompt_tokens: 8192"));
        assert!(output.contains("context_tokens: 32768"));
    }

    #[test]
    fn execute_config_show_budgets_alias_returns_typed_budget_contract() {
        // Arrange
        let command = args(&["ralph-engine", "config", "show-budgets"]);

        // Act
        let output = execute(command).expect("config show-budgets should succeed");

        // Assert
        assert!(output.contains("budgets:"));
        assert!(output.contains("prompt_tokens: 8192"));
    }

    #[test]
    fn execute_config_show_plugin_returns_resolved_yaml() {
        // Arrange
        let command = args(&["ralph-engine", "config", "show-plugin", "official.basic"]);

        // Act
        let output = execute(command).expect("config show-plugin should succeed");

        // Assert
        assert!(output.contains("id: official.basic"));
        assert!(output.contains("activation: enabled"));
        assert!(output.contains("resolved_from: built_in_defaults"));
    }

    #[test]
    fn execute_config_show_plugin_returns_disabled_built_in_default_for_known_plugin() {
        // Arrange
        let command = args(&["ralph-engine", "config", "show-plugin", "official.github"]);

        // Act
        let output = execute(command).expect("config show-plugin should succeed");

        // Assert
        assert!(output.contains("id: official.github"));
        assert!(output.contains("activation: disabled"));
        assert!(output.contains("resolved_from: built_in_defaults"));
    }

    #[test]
    fn execute_config_show_plugin_requires_plugin_id() {
        // Arrange
        let command = args(&["ralph-engine", "config", "show-plugin"]);

        // Act
        let error = execute(command).expect_err("missing plugin id should fail");

        // Assert
        assert_eq!(error.to_string(), "config show-plugin requires a plugin id");
    }

    #[test]
    fn execute_config_show_plugin_rejects_unknown_plugin() {
        // Arrange
        let command = args(&["ralph-engine", "config", "show-plugin", "official.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown plugin id should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown plugin config: official.unknown");
    }

    #[test]
    fn execute_config_without_subcommand_returns_yaml_contract() {
        // Arrange
        let command = args(&["ralph-engine", "config"]);

        // Act
        let output = execute(command).expect("config command should succeed");

        // Assert
        assert!(output.contains("mcp:"));
    }

    #[test]
    fn execute_doctor_returns_typed_runtime_report() {
        // Arrange
        let command = args(&["ralph-engine", "doctor"]);

        // Act
        let output = execute(command).expect("doctor should succeed");

        // Assert
        assert!(output.contains("Runtime doctor"));
        assert!(output.contains("Runtime health: degraded"));
        assert!(output.contains("Runtime issues (58)"));
        assert!(output.contains("Runtime action plan (58)"));
    }

    #[test]
    fn execute_doctor_runtime_returns_typed_runtime_report() {
        // Arrange
        let command = args(&["ralph-engine", "doctor", "runtime"]);

        // Act
        let output = execute(command).expect("doctor runtime should succeed");

        // Assert
        assert!(output.contains("Runtime doctor"));
        assert!(output.contains("Runtime issues (58)"));
    }

    #[test]
    fn execute_unknown_doctor_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "doctor", "plugins"]);

        // Act
        let error = execute(command).expect_err("unknown doctor command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown doctor command: plugins");
    }

    #[test]
    fn execute_unknown_config_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "config", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown config command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown config command: doctor");
    }

    #[test]
    fn execute_mcp_lists_official_servers() {
        // Arrange
        let command = args(&["ralph-engine", "mcp", "list"]);

        // Act
        let output = execute(command).expect("mcp list should succeed");

        // Assert
        assert!(output.contains("Official MCP servers (4)"));
        assert!(output.contains("official.codex.session"));
        assert!(output.contains("official.github.repository"));
    }

    #[test]
    fn execute_mcp_show_returns_server_detail() {
        // Arrange
        let command = args(&["ralph-engine", "mcp", "show", "official.github.repository"]);

        // Act
        let output = execute(command).expect("mcp show should succeed");

        // Assert
        assert!(output.contains("MCP server: official.github.repository"));
        assert!(output.contains("Process model: external_binary"));
        assert!(output.contains("Launch policy: spawn_process"));
        assert!(output.contains("Availability: explicit_opt_in"));
        assert!(output.contains("Command: ralph-engine-github-mcp serve"));
        assert!(output.contains("Working directory: project_root"));
        assert!(output.contains("Environment: plugin_scoped"));
    }

    #[test]
    fn execute_mcp_show_requires_server_id() {
        // Arrange
        let command = args(&["ralph-engine", "mcp", "show"]);

        // Act
        let error = execute(command).expect_err("missing server id should fail");

        // Assert
        assert_eq!(error.to_string(), "mcp show requires a server id");
    }

    #[test]
    fn execute_mcp_show_rejects_unknown_server() {
        // Arrange
        let command = args(&["ralph-engine", "mcp", "show", "official.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown server id should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown mcp server: official.unknown");
    }

    #[test]
    fn execute_mcp_without_subcommand_lists_official_servers() {
        // Arrange
        let command = args(&["ralph-engine", "mcp"]);

        // Act
        let output = execute(command).expect("mcp command should succeed");

        // Assert
        assert!(output.contains("Official MCP servers (4)"));
    }

    #[test]
    fn execute_runtime_show_returns_resolved_topology() {
        // Arrange
        let command = args(&["ralph-engine", "runtime", "show"]);

        // Act
        let output = execute(command).expect("runtime show should succeed");

        // Assert
        assert!(output.contains("Runtime phase: ready"));
        assert!(output.contains("Locale: en"));
        assert!(output.contains("Plugins (8)"));
        assert!(output.contains("official.basic | activation=enabled | scope=built_in_defaults"));
        assert!(output.contains("official.github | activation=disabled | scope=built_in_defaults"));
        assert!(output.contains("Capabilities (18)"));
        assert!(output.contains("template | plugin=official.basic | activation=enabled"));
        assert!(output.contains("Templates (3)"));
        assert!(output.contains("official.basic | activation=enabled"));
        assert!(output.contains("official.tdd-strict | activation=disabled"));
        assert!(output.contains("Prompts (1)"));
        assert!(output.contains("official.bmad | activation=disabled"));
        assert!(output.contains("Agent runtimes (3)"));
        assert!(output.contains("official.claude | activation=disabled"));
        assert!(output.contains("official.codex | activation=disabled"));
        assert!(output.contains("Checks (2)"));
        assert!(output.contains("prepare | plugin=official.bmad | activation=disabled"));
        assert!(output.contains("doctor | plugin=official.bmad | activation=disabled"));
        assert!(output.contains("Providers (4)"));
        assert!(output.contains("data_source | plugin=official.github | activation=disabled"));
        assert!(output.contains("remote_control | plugin=official.ssh | activation=disabled"));
        assert!(output.contains("Policies (1)"));
        assert!(
            output
                .contains("official.tdd-strict | plugin=official.tdd-strict | activation=disabled")
        );
        assert!(output.contains("Runtime hooks (18)"));
        assert!(output.contains("scaffold | plugin=official.basic | activation=enabled"));
        assert!(output.contains("MCP servers (4)"));
    }

    #[test]
    fn execute_runtime_status_returns_runtime_health_summary() {
        // Arrange
        let command = args(&["ralph-engine", "runtime", "status"]);

        // Act
        let output = execute(command).expect("runtime status should succeed");

        // Assert
        assert!(output.contains("Runtime phase: ready"));
        assert!(output.contains("Runtime health: degraded"));
        assert!(output.contains("Plugins: enabled=1, disabled=7"));
        assert!(output.contains("Capabilities: enabled=1, disabled=17"));
        assert!(output.contains("Templates: enabled=1, disabled=2"));
        assert!(output.contains("Prompts: enabled=0, disabled=1"));
        assert!(output.contains("Agent runtimes: enabled=0, disabled=3"));
        assert!(output.contains("Checks: enabled=0, disabled=2"));
        assert!(output.contains("Providers: enabled=0, disabled=4"));
        assert!(output.contains("Policies: enabled=0, disabled=1"));
        assert!(output.contains("Runtime hooks: enabled=1, disabled=17"));
        assert!(output.contains("MCP servers: enabled=0, disabled=4"));
    }

    #[test]
    fn execute_runtime_issues_returns_typed_issue_summary() {
        // Arrange
        let command = args(&["ralph-engine", "runtime", "issues"]);

        // Act
        let output = execute(command).expect("runtime issues should succeed");

        // Assert
        assert!(output.contains("Runtime issues (58)"));
        assert!(output.contains(
            "plugin_disabled | subject=official.github | action=enable the plugin in typed project configuration"
        ));
        assert!(output.contains(
            "template_disabled | subject=official.bmad | action=enable the provider plugin that owns this template surface"
        ));
        assert!(output.contains(
            "prompt_provider_disabled | subject=official.bmad | action=enable the provider plugin that owns this prompt surface"
        ));
        assert!(output.contains(
            "agent_runtime_disabled | subject=official.codex | action=enable the provider plugin that owns this agent runtime"
        ));
        assert!(output.contains(
            "check_disabled | subject=prepare | action=enable the provider plugin that owns this runtime check"
        ));
        assert!(output.contains(
            "provider_disabled | subject=data_source | action=enable the provider plugin that owns this contribution"
        ));
        assert!(output.contains(
            "policy_disabled | subject=official.tdd-strict | action=enable the provider plugin that owns this policy"
        ));
        assert!(output.contains(
            "hook_disabled | subject=mcp_registration | action=enable the provider plugin that owns this runtime hook"
        ));
        assert!(output.contains(
            "mcp_server_disabled | subject=official.github.repository | action=enable the owning plugin or opt in to the MCP server"
        ));
    }

    #[test]
    fn execute_runtime_plan_returns_typed_action_plan() {
        // Arrange
        let command = args(&["ralph-engine", "runtime", "plan"]);

        // Act
        let output = execute(command).expect("runtime plan should succeed");

        // Assert
        assert!(output.contains("Runtime action plan (58)"));
        assert!(output.contains(
            "enable_plugin | target=official.github | reason=the plugin is registered but disabled"
        ));
        assert!(output.contains(
            "enable_template_provider | target=official.bmad | reason=the provider still disables the template surface"
        ));
        assert!(output.contains(
            "enable_prompt_provider | target=official.bmad | reason=the provider still disables the prompt surface"
        ));
        assert!(output.contains(
            "enable_agent_runtime_provider | target=official.codex | reason=the provider still disables the agent runtime"
        ));
        assert!(output.contains(
            "enable_check_provider | target=official.bmad | reason=the provider still disables runtime check prepare"
        ));
        assert!(output.contains(
            "enable_provider | target=official.github | reason=the provider still disables contribution data_source"
        ));
        assert!(output.contains(
            "enable_capability_provider | target=official.github | reason=the provider still disables capability forge_provider"
        ));
        assert!(output.contains(
            "enable_policy_provider | target=official.tdd-strict | reason=the provider still disables policy official.tdd-strict"
        ));
        assert!(output.contains(
            "enable_hook_provider | target=official.github | reason=the provider still disables runtime hook forge_provider_registration"
        ));
        assert!(output.contains(
            "enable_mcp_server | target=official.github.repository | reason=the MCP contribution is registered but disabled"
        ));
    }

    #[test]
    fn execute_runtime_without_subcommand_returns_resolved_topology() {
        // Arrange
        let command = args(&["ralph-engine", "runtime"]);

        // Act
        let output = execute(command).expect("runtime command should succeed");

        // Assert
        assert!(output.contains("Runtime phase: ready"));
    }

    #[test]
    fn execute_unknown_runtime_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "runtime", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown runtime command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown runtime command: doctor");
    }

    #[test]
    fn execute_capabilities_without_subcommand_lists_runtime_capabilities() {
        // Arrange
        let command = args(&["ralph-engine", "capabilities"]);

        // Act
        let output = execute(command).expect("capabilities command should succeed");

        // Assert
        assert!(output.contains("Capabilities (11)"));
    }

    #[test]
    fn execute_agents_without_subcommand_lists_runtime_agents() {
        // Arrange
        let command = args(&["ralph-engine", "agents"]);

        // Act
        let output = execute(command).expect("agents command should succeed");

        // Assert
        assert!(output.contains("Agent runtimes (3)"));
    }

    #[test]
    fn execute_templates_without_subcommand_lists_runtime_templates() {
        // Arrange
        let command = args(&["ralph-engine", "templates"]);

        // Act
        let output = execute(command).expect("templates command should succeed");

        // Assert
        assert!(output.contains("Templates (3)"));
    }

    #[test]
    fn execute_prompts_without_subcommand_lists_runtime_prompts() {
        // Arrange
        let command = args(&["ralph-engine", "prompts"]);

        // Act
        let output = execute(command).expect("prompts command should succeed");

        // Assert
        assert!(output.contains("Prompts (1)"));
    }

    #[test]
    fn execute_checks_without_subcommand_lists_runtime_checks() {
        // Arrange
        let command = args(&["ralph-engine", "checks"]);

        // Act
        let output = execute(command).expect("checks command should succeed");

        // Assert
        assert!(output.contains("Checks (2)"));
    }

    #[test]
    fn execute_hooks_without_subcommand_lists_runtime_hooks() {
        // Arrange
        let command = args(&["ralph-engine", "hooks"]);

        // Act
        let output = execute(command).expect("hooks command should succeed");

        // Assert
        assert!(output.contains("Runtime hooks (11)"));
    }

    #[test]
    fn execute_policies_without_subcommand_lists_runtime_policies() {
        // Arrange
        let command = args(&["ralph-engine", "policies"]);

        // Act
        let output = execute(command).expect("policies command should succeed");

        // Assert
        assert!(output.contains("Policies (1)"));
    }

    #[test]
    fn execute_providers_without_subcommand_lists_runtime_providers() {
        // Arrange
        let command = args(&["ralph-engine", "providers"]);

        // Act
        let output = execute(command).expect("providers command should succeed");

        // Assert
        assert!(output.contains("Providers (4)"));
    }

    #[test]
    fn execute_unknown_capabilities_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "capabilities", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown capabilities command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown capabilities command: doctor");
    }

    #[test]
    fn execute_unknown_agents_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "agents", "runtime"]);

        // Act
        let error = execute(command).expect_err("unknown agents command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown agents command: runtime");
    }

    #[test]
    fn execute_unknown_templates_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "templates", "runtime"]);

        // Act
        let error = execute(command).expect_err("unknown templates command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown templates command: runtime");
    }

    #[test]
    fn execute_unknown_prompts_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "prompts", "runtime"]);

        // Act
        let error = execute(command).expect_err("unknown prompts command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown prompts command: runtime");
    }

    #[test]
    fn execute_unknown_checks_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "checks", "runtime"]);

        // Act
        let error = execute(command).expect_err("unknown checks command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown checks command: runtime");
    }

    #[test]
    fn execute_unknown_hooks_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "hooks", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown hooks command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown hooks command: doctor");
    }

    #[test]
    fn execute_unknown_policies_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "policies", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown policies command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown policies command: doctor");
    }

    #[test]
    fn execute_unknown_providers_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "providers", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown providers command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown providers command: doctor");
    }

    #[test]
    fn execute_unknown_mcp_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "mcp", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown mcp command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown mcp command: doctor");
    }

    #[test]
    fn execute_plugins_without_subcommand_lists_official_plugins() {
        // Arrange
        let command = args(&["ralph-engine", "plugins"]);

        // Act
        let output = execute(command).expect("plugins command should succeed");

        // Assert
        assert!(output.contains("Official plugins (8)"));
    }

    #[test]
    fn execute_unknown_plugins_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "plugins", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown plugins command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown plugins command: doctor");
    }

    #[test]
    fn execute_unknown_command_fails() {
        // Arrange
        let command = args(&["ralph-engine", "unknown"]);

        // Act
        let error = execute(command).expect_err("unknown command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown command: unknown");
    }
}
