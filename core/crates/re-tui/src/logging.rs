//! Structured logging for TUI mode.
//!
//! In TUI mode, stdout/stderr are occupied by the render loop.
//! All logs go to a file appender at `~/.ralph/ralph.log`.
//! In `--no-tui` mode, logs also go to stderr.
//!
//! Uses `tracing` + `tracing-subscriber` with env-filter for level control.
//! Plugins do not need any special setup — `tracing` captures across crates
//! automatically.

use std::path::PathBuf;

use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

/// Configuration for the logging subsystem.
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Log verbosity level filter (e.g. `"info"`, `"debug"`, `"re_tui=debug"`).
    pub level: String,
    /// Whether TUI raw mode is active (disables stderr output).
    pub tui_active: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_owned(),
            tui_active: true,
        }
    }
}

/// Resolves the log directory path.
///
/// Prefers the platform-specific data directory (`~/.local/share/ralph-engine/`
/// on Linux, `~/Library/Application Support/` on macOS). Falls back to
/// `.ralph/` in the current directory if the platform directory cannot be
/// resolved.
fn resolve_log_dir() -> PathBuf {
    directories::ProjectDirs::from("", "", "ralph-engine")
        .map(|dirs| dirs.data_local_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from(".ralph"))
}

/// Initializes the logging subsystem and returns the log file path.
///
/// - Always writes to a log file (rolling, non-blocking).
/// - When `tui_active` is false (`--no-tui`), also writes to stderr.
/// - Log level controlled by `config.level` and overridden by
///   `RALPH_ENGINE_LOGLEVEL` environment variable.
///
/// # Errors
///
/// Returns an error if the log directory cannot be created.
pub fn init_logging(config: &LogConfig) -> Result<PathBuf, LoggingError> {
    let log_dir = resolve_log_dir();
    std::fs::create_dir_all(&log_dir).map_err(|err| LoggingError {
        message: format!(
            "failed to create log directory {}: {err}",
            log_dir.display()
        ),
    })?;

    let file_appender = tracing_appender::rolling::daily(&log_dir, "ralph.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Leak the guard so the appender lives for the process lifetime.
    // This is intentional — the TUI process is short-lived and the guard
    // must outlive all tracing calls.
    std::mem::forget(_guard);

    let env_filter = tracing_subscriber::EnvFilter::try_from_env("RALPH_ENGINE_LOGLEVEL")
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.level));

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_target(true);

    if config.tui_active {
        // TUI mode: file only (stderr is occupied by ratatui).
        tracing_subscriber::registry()
            .with(env_filter)
            .with(file_layer)
            .init();
    } else {
        // No-TUI mode: file + stderr.
        let stderr_layer = tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_ansi(true)
            .with_target(true);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(file_layer)
            .with(stderr_layer)
            .init();
    }

    let log_path = log_dir.join("ralph.log");
    tracing::debug!(path = %log_path.display(), "logging initialized");

    Ok(log_path)
}

/// Error returned by logging initialization.
#[derive(Debug)]
pub struct LoggingError {
    /// Human-readable error description.
    pub message: String,
}

impl std::fmt::Display for LoggingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for LoggingError {}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn resolve_log_dir_returns_valid_path() {
        let dir = resolve_log_dir();
        // Should be a non-empty path (platform-specific or fallback)
        assert!(!dir.as_os_str().is_empty());
    }

    #[test]
    fn log_config_default_is_info_tui() {
        let config = LogConfig::default();
        assert_eq!(config.level, "info");
        assert!(config.tui_active);
    }
}
