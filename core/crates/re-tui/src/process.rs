//! Agent process control (pause/resume via OS signals).
//!
//! Core owns process lifecycle — plugins observe state changes via
//! the [`StateListener`] trait but cannot send signals directly.
//! This module is Unix-only (`SIGSTOP`/`SIGCONT`).

/// Sends `SIGSTOP` to pause a process via the `kill` command.
///
/// Uses `std::process::Command` to avoid `unsafe` blocks.
///
/// # Errors
///
/// Returns an error message if the signal fails.
#[cfg(unix)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn pause_process(pid: u32) -> Result<(), String> {
    send_signal(pid, "STOP")
}

/// Sends `SIGCONT` to resume a paused process via the `kill` command.
///
/// # Errors
///
/// Returns an error message if the signal fails.
#[cfg(unix)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn resume_process(pid: u32) -> Result<(), String> {
    send_signal(pid, "CONT")
}

/// Sends a named signal to a process using the `kill` command.
#[cfg(unix)]
#[cfg_attr(coverage_nightly, coverage(off))]
fn send_signal(pid: u32, signal: &str) -> Result<(), String> {
    let status = std::process::Command::new("kill")
        .args([&format!("-{signal}"), &pid.to_string()])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| format!("failed to execute kill -{signal} {pid}: {e}"))?;

    if status.success() {
        tracing::info!(pid, signal, "sent signal to agent process");
        Ok(())
    } else {
        let msg = format!("kill -{signal} {pid} failed (exit: {status})");
        tracing::error!(pid, signal, "signal delivery failed");
        Err(msg)
    }
}

/// Non-Unix stub — process control not available.
#[cfg(not(unix))]
pub fn pause_process(_pid: u32) -> Result<(), String> {
    Err("Process pause not supported on this platform".to_owned())
}

/// Non-Unix stub — process control not available.
#[cfg(not(unix))]
pub fn resume_process(_pid: u32) -> Result<(), String> {
    Err("Process resume not supported on this platform".to_owned())
}

/// A state change event emitted by the TUI state machine.
///
/// Plugins implement [`StateListener`] to react to these events
/// (e.g., trigger quality gates when paused, update panels on resume).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateEvent {
    /// Agent started running.
    Started,
    /// Agent was paused by user.
    Paused,
    /// Agent was resumed (with or without feedback).
    Resumed,
    /// User is entering feedback text.
    FeedbackStarted,
    /// Agent session completed successfully.
    Completed,
    /// Agent session ended with error.
    Failed,
}

/// Trait for plugins to observe TUI state changes.
///
/// Core calls `on_state_changed()` for all registered listeners
/// after every state transition. Plugins use this to update their
/// panels, trigger background work (quality gates), or log events.
pub trait StateListener: Send {
    /// Called when the TUI state changes.
    fn on_state_changed(&mut self, event: StateEvent);
}

/// Manages session state for save/restore across terminal sessions.
///
/// Serialized to `.ralph-engine/.session.json` on pause/exit.
/// Loaded on `run --resume` to continue where the user left off.
#[derive(Debug, Clone)]
pub struct SessionState {
    /// Work item ID being executed.
    pub work_item_id: String,
    /// Agent process ID (if still running).
    pub agent_pid: Option<u32>,
    /// Current TUI state when session was saved.
    pub tui_state: String,
    /// Number of tool calls completed.
    pub tool_count: usize,
    /// Last N activity lines for context restoration.
    pub recent_activity: Vec<String>,
    /// Timestamp when session was saved.
    pub saved_at: String,
}

impl SessionState {
    /// Creates a new session state snapshot.
    #[must_use]
    pub fn new(work_item_id: &str) -> Self {
        Self {
            work_item_id: work_item_id.to_owned(),
            agent_pid: None,
            tui_state: "Running".to_owned(),
            tool_count: 0,
            recent_activity: Vec::new(),
            saved_at: String::new(),
        }
    }

    /// Serializes the session state to a simple key-value format.
    ///
    /// Uses a plain text format (not JSON) to avoid adding serde
    /// as a dependency. One key=value per line.
    #[must_use]
    pub fn serialize(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("work_item_id={}", self.work_item_id));
        if let Some(pid) = self.agent_pid {
            lines.push(format!("agent_pid={pid}"));
        }
        lines.push(format!("tui_state={}", self.tui_state));
        lines.push(format!("tool_count={}", self.tool_count));
        lines.push(format!("saved_at={}", self.saved_at));
        lines.push(format!("activity_count={}", self.recent_activity.len()));
        for (i, line) in self.recent_activity.iter().enumerate() {
            lines.push(format!("activity_{i}={line}"));
        }
        lines.join("\n")
    }

    /// Deserializes session state from the key-value format.
    #[must_use]
    pub fn deserialize(input: &str) -> Option<Self> {
        let mut state = Self::new("");
        let mut activity_count = 0usize;

        for line in input.lines() {
            if let Some((key, value)) = line.split_once('=') {
                match key {
                    "work_item_id" => state.work_item_id = value.to_owned(),
                    "agent_pid" => state.agent_pid = value.parse().ok(),
                    "tui_state" => state.tui_state = value.to_owned(),
                    "tool_count" => state.tool_count = value.parse().unwrap_or(0),
                    "saved_at" => state.saved_at = value.to_owned(),
                    "activity_count" => activity_count = value.parse().unwrap_or(0),
                    k if k.starts_with("activity_") => {
                        state.recent_activity.push(value.to_owned());
                    }
                    _ => {} // ignore unknown keys for forward compatibility
                }
            }
        }

        if state.work_item_id.is_empty() {
            return None;
        }

        // Verify activity count matches
        if state.recent_activity.len() != activity_count {
            state.recent_activity.truncate(activity_count);
        }

        Some(state)
    }
}

/// Default session file path relative to project root.
pub const SESSION_FILE: &str = ".ralph-engine/.session";

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn state_event_debug_format() {
        assert_eq!(format!("{:?}", StateEvent::Paused), "Paused");
        assert_eq!(format!("{:?}", StateEvent::Resumed), "Resumed");
    }

    #[test]
    fn session_state_new_has_defaults() {
        let state = SessionState::new("5.3");
        assert_eq!(state.work_item_id, "5.3");
        assert!(state.agent_pid.is_none());
        assert_eq!(state.tui_state, "Running");
        assert_eq!(state.tool_count, 0);
        assert!(state.recent_activity.is_empty());
    }

    #[test]
    fn session_state_roundtrip() {
        let mut state = SessionState::new("5.3");
        state.agent_pid = Some(12345);
        state.tui_state = "Paused".to_owned();
        state.tool_count = 7;
        state.saved_at = "2026-04-04T14:00:00".to_owned();
        state.recent_activity = vec![">> Tool: Read".to_owned(), ">> Agent: thinking".to_owned()];

        let serialized = state.serialize();
        let deserialized = SessionState::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.work_item_id, "5.3");
        assert_eq!(deserialized.agent_pid, Some(12345));
        assert_eq!(deserialized.tui_state, "Paused");
        assert_eq!(deserialized.tool_count, 7);
        assert_eq!(deserialized.saved_at, "2026-04-04T14:00:00");
        assert_eq!(deserialized.recent_activity.len(), 2);
        assert_eq!(deserialized.recent_activity[0], ">> Tool: Read");
    }

    #[test]
    fn session_state_deserialize_empty_returns_none() {
        assert!(SessionState::deserialize("").is_none());
        assert!(SessionState::deserialize("random=garbage").is_none());
    }

    #[test]
    fn session_state_deserialize_no_pid() {
        let input = "work_item_id=1.2\ntui_state=Running\ntool_count=0\nactivity_count=0";
        let state = SessionState::deserialize(input).unwrap();
        assert_eq!(state.work_item_id, "1.2");
        assert!(state.agent_pid.is_none());
    }

    #[test]
    fn session_state_ignores_unknown_keys() {
        let input = "work_item_id=1.2\nfuture_key=value\ntui_state=Running\nactivity_count=0";
        let state = SessionState::deserialize(input).unwrap();
        assert_eq!(state.work_item_id, "1.2");
    }

    // Note: pause_process/resume_process are I/O operations that send
    // real signals — tested only via integration tests with real processes.
    // Marked coverage(off) in the implementation.
}
