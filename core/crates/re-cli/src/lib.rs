//! CLI command execution for Ralph Engine.

use std::fmt;

use re_mcp::{McpServerDescriptor, render_mcp_server_listing};
use re_plugin::{PluginDescriptor, render_plugin_listing};

/// CLI execution errors.
#[derive(Debug, Eq, PartialEq)]
pub struct CliError(String);

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CliError {}

/// Executes the current CLI foundation command set.
pub fn execute<I>(args: I) -> Result<String, CliError>
where
    I: IntoIterator<Item = String>,
{
    let collected: Vec<String> = args.into_iter().collect();
    let command = collected.get(1).map(String::as_str);

    match command {
        None => Ok(format!(
            "{}\n\nRust foundation bootstrapped.",
            re_core::banner()
        )),
        Some("--version") => Ok(env!("CARGO_PKG_VERSION").to_owned()),
        Some("mcp") => execute_mcp_command(&collected[2..]),
        Some("plugins") => execute_plugins_command(&collected[2..]),
        Some(other) => Err(CliError(format!("unknown command: {other}"))),
    }
}

fn execute_mcp_command(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_mcp_server_listing(&official_mcp_servers())),
        Some(other) => Err(CliError(format!("unknown mcp command: {other}"))),
    }
}

fn execute_plugins_command(args: &[String]) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_plugin_listing(&official_plugins())),
        Some(other) => Err(CliError(format!("unknown plugins command: {other}"))),
    }
}

fn official_plugins() -> [PluginDescriptor; 8] {
    [
        re_plugin_basic::descriptor(),
        re_plugin_bmad::descriptor(),
        re_plugin_claude::descriptor(),
        re_plugin_claudebox::descriptor(),
        re_plugin_codex::descriptor(),
        re_plugin_github::descriptor(),
        re_plugin_ssh::descriptor(),
        re_plugin_tdd_strict::descriptor(),
    ]
}

fn official_mcp_servers() -> [McpServerDescriptor; 4] {
    [
        re_plugin_claude::mcp_servers()[0],
        re_plugin_claudebox::mcp_servers()[0],
        re_plugin_codex::mcp_servers()[0],
        re_plugin_github::mcp_servers()[0],
    ]
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
