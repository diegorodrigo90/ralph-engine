//! CLI command execution for Ralph Engine.

use std::fmt;

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
        Some(other) => Err(CliError(format!("unknown command: {other}"))),
    }
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
    fn execute_unknown_command_fails() {
        // Arrange
        let command = args(&["ralph-engine", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown command: doctor");
    }
}
