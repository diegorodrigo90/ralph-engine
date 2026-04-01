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
        assert!(output.contains("MCP servers: enabled=0, disabled=4"));
    }

    #[test]
    fn execute_runtime_issues_returns_typed_issue_summary() {
        // Arrange
        let command = args(&["ralph-engine", "runtime", "issues"]);

        // Act
        let output = execute(command).expect("runtime issues should succeed");

        // Assert
        assert!(output.contains("Runtime issues (28)"));
        assert!(output.contains(
            "plugin_disabled | subject=official.github | action=enable the plugin in typed project configuration"
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
        assert!(output.contains("Runtime action plan (28)"));
        assert!(output.contains(
            "enable_plugin | target=official.github | reason=the plugin is registered but disabled"
        ));
        assert!(output.contains(
            "enable_capability_provider | target=official.github | reason=the provider still disables capability forge_provider"
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
    fn execute_unknown_capabilities_subcommand_fails() {
        // Arrange
        let command = args(&["ralph-engine", "capabilities", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown capabilities command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown capabilities command: doctor");
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
        let command = args(&["ralph-engine", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown command: doctor");
    }
}
