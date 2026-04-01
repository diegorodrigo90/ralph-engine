//! Shared CLI error definitions.

use std::fmt;

/// CLI execution errors.
#[derive(Debug, Eq, PartialEq)]
pub struct CliError(pub(crate) String);

impl CliError {
    /// Creates a new CLI error.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CliError {}
