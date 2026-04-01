//! End-to-end smoke tests for the Ralph Engine CLI binary.

#![allow(clippy::expect_used)]

use std::process::Command;

#[test]
fn binary_without_args_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Rust foundation bootstrapped."));
}

#[test]
fn binary_with_unknown_command_fails() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.arg("doctor");

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(stderr.contains("unknown command: doctor"));
}

#[test]
fn binary_plugins_list_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["plugins", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Official plugins (8)"));
    assert!(stdout.contains("official.codex"));
}

#[test]
fn binary_plugins_show_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["plugins", "show", "official.github"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Plugin: official.github"));
    assert!(stdout.contains("Lifecycle: discover -> configure -> load"));
    assert!(stdout.contains("Load boundary: in_process"));
    assert!(stdout.contains(
        "Runtime hooks: mcp_registration, data_source_registration, context_provider_registration, forge_provider_registration"
    ));
    assert!(stdout.contains("Resolved activation: disabled"));
    assert!(stdout.contains("Resolved from: built_in_defaults"));
}

#[test]
fn binary_capabilities_list_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["capabilities", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Capabilities (11)"));
    assert!(stdout.contains("mcp_contribution"));
}

#[test]
fn binary_hooks_list_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["hooks", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime hooks (11)"));
    assert!(stdout.contains("mcp_registration"));
}

#[test]
fn binary_mcp_list_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["mcp", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Official MCP servers (4)"));
    assert!(stdout.contains("official.github.repository"));
}

#[test]
fn binary_mcp_show_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["mcp", "show", "official.codex.session"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("MCP server: official.codex.session"));
    assert!(stdout.contains("Process model: plugin_managed"));
    assert!(stdout.contains("Launch policy: plugin_runtime"));
    assert!(stdout.contains("Availability: on_demand"));
    assert!(stdout.contains("Command: managed by plugin runtime"));
}

#[test]
fn binary_runtime_show_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["runtime", "show"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime phase: ready"));
    assert!(stdout.contains("Plugins (8)"));
    assert!(stdout.contains("Capabilities (18)"));
    assert!(stdout.contains("Runtime hooks (18)"));
    assert!(stdout.contains("MCP servers (4)"));
}

#[test]
fn binary_runtime_status_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["runtime", "status"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime health: degraded"));
    assert!(stdout.contains("Plugins: enabled=1, disabled=7"));
}

#[test]
fn binary_runtime_issues_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["runtime", "issues"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime issues (28)"));
    assert!(stdout.contains("plugin_disabled"));
    assert!(stdout.contains("mcp_server_disabled"));
}

#[test]
fn binary_runtime_plan_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["runtime", "plan"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime action plan (28)"));
    assert!(stdout.contains("enable_plugin"));
    assert!(stdout.contains("enable_capability_provider"));
    assert!(stdout.contains("enable_mcp_server"));
}

#[test]
fn binary_config_show_defaults_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["config", "show-defaults"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("schema_version: 1"));
    assert!(stdout.contains("default_locale: en"));
}

#[test]
fn binary_config_show_plugin_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["config", "show-plugin", "official.basic"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("id: official.basic"));
    assert!(stdout.contains("activation: enabled"));
    assert!(stdout.contains("resolved_from: built_in_defaults"));
}
