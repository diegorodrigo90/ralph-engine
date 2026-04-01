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
    command.arg("unknown");

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(stderr.contains("unknown command: unknown"));
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
fn binary_agents_list_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["agents", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Agent runtimes (3)"));
    assert!(stdout.contains("official.claude"));
    assert!(stdout.contains("official.codex"));
}

#[test]
fn binary_agents_show_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["agents", "show", "official.codex"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Agent runtime: official.codex"));
    assert!(stdout.contains("bootstrap_hook=true"));
}

#[test]
fn binary_templates_list_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["templates", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Templates (3)"));
    assert!(stdout.contains("official.basic"));
    assert!(stdout.contains("official.bmad"));
}

#[test]
fn binary_templates_show_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["templates", "show", "official.basic"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Template provider: official.basic"));
    assert!(stdout.contains("scaffold_hook=true"));
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
fn binary_checks_list_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["checks", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Checks (2)"));
    assert!(stdout.contains("prepare"));
    assert!(stdout.contains("doctor"));
}

#[test]
fn binary_checks_show_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["checks", "show", "prepare"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Check: prepare"));
    assert!(stdout.contains("official.bmad"));
    assert!(stdout.contains("runtime_hook=true"));
}

#[test]
fn binary_policies_list_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["policies", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Policies (1)"));
    assert!(stdout.contains("official.tdd-strict"));
}

#[test]
fn binary_policies_show_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["policies", "show", "official.tdd-strict"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Policy: official.tdd-strict"));
    assert!(stdout.contains("Provider: official.tdd-strict"));
    assert!(stdout.contains("Policy enforcement hook: true"));
}

#[test]
fn binary_providers_list_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["providers", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Providers (4)"));
    assert!(stdout.contains("data_source"));
    assert!(stdout.contains("remote_control"));
}

#[test]
fn binary_providers_show_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["providers", "show", "data_source"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Provider: data_source"));
    assert!(stdout.contains("official.github"));
    assert!(stdout.contains("registration_hook=true"));
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
    assert!(stdout.contains("Templates (3)"));
    assert!(stdout.contains("Agent runtimes (3)"));
    assert!(stdout.contains("Checks (2)"));
    assert!(stdout.contains("Providers (4)"));
    assert!(stdout.contains("Policies (1)"));
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
    assert!(stdout.contains("Templates: enabled=1, disabled=2"));
    assert!(stdout.contains("Agent runtimes: enabled=0, disabled=3"));
    assert!(stdout.contains("Checks: enabled=0, disabled=2"));
    assert!(stdout.contains("Providers: enabled=0, disabled=4"));
    assert!(stdout.contains("Policies: enabled=0, disabled=1"));
    assert!(stdout.contains("Runtime hooks: enabled=1, disabled=17"));
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
    assert!(stdout.contains("Runtime issues (57)"));
    assert!(stdout.contains("template_disabled"));
    assert!(stdout.contains("agent_runtime_disabled"));
    assert!(stdout.contains("check_disabled"));
    assert!(stdout.contains("provider_disabled"));
    assert!(stdout.contains("plugin_disabled"));
    assert!(stdout.contains("policy_disabled"));
    assert!(stdout.contains("hook_disabled"));
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
    assert!(stdout.contains("Runtime action plan (57)"));
    assert!(stdout.contains("enable_template_provider"));
    assert!(stdout.contains("enable_agent_runtime_provider"));
    assert!(stdout.contains("enable_check_provider"));
    assert!(stdout.contains("enable_plugin"));
    assert!(stdout.contains("enable_provider"));
    assert!(stdout.contains("enable_policy_provider"));
    assert!(stdout.contains("enable_hook_provider"));
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
fn binary_config_layers_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["config", "layers"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("layers:"));
    assert!(stdout.contains("scope: built_in_defaults"));
    assert!(stdout.contains("plugin_count: 1"));
}

#[test]
fn binary_config_budgets_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.args(["config", "budgets"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("budgets:"));
    assert!(stdout.contains("prompt_tokens: 8192"));
    assert!(stdout.contains("context_tokens: 32768"));
}

#[test]
fn binary_doctor_succeeds() {
    // Arrange
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.arg("doctor");

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime doctor"));
    assert!(stdout.contains("Runtime issues (57)"));
    assert!(stdout.contains("Runtime action plan (57)"));
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
