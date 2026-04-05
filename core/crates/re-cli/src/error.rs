//! Shared CLI error definitions.

use std::fmt;

/// CLI execution errors.
#[derive(Debug, Eq, PartialEq)]
pub struct CliError {
    pub(crate) message: String,
    /// Exit code: 1 = runtime error (default), 2 = usage/argument error.
    pub exit_code: u8,
}

impl CliError {
    /// Creates a new runtime CLI error (exit code 1).
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            exit_code: 1,
        }
    }

    /// Creates a usage/argument error (exit code 2).
    #[must_use]
    pub fn usage(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            exit_code: 2,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for CliError {}
