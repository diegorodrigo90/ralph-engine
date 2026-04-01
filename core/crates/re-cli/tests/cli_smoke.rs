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
    assert!(stdout.contains("Default activation: disabled"));
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
    assert!(stdout.contains("Availability: on_demand"));
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
