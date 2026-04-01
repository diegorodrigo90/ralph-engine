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
        assert!(output.contains("Default activation: disabled"));
    }

    #[test]
    fn execute_plugins_show_reports_enabled_default_activation() {
        // Arrange
        let command = args(&["ralph-engine", "plugins", "show", "official.basic"]);

        // Act
        let output = execute(command).expect("plugins show should succeed");

        // Assert
        assert!(output.contains("Plugin: official.basic"));
        assert!(output.contains("Default activation: enabled"));
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
        assert!(output.contains("Availability: explicit_opt_in"));
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
