//! End-to-end smoke tests for the Ralph Engine CLI binary.

#![allow(clippy::expect_used)]

use std::{
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

/// Creates a Command with `RALPH_ENGINE_LOCALE=en` set so smoke tests
/// produce predictable English output regardless of the host OS locale.
fn english_command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ralph-engine"));
    command.env("RALPH_ENGINE_LOCALE", "en");
    command
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("current time should be after unix epoch")
        .as_nanos();
    path.push(format!(
        "ralph-engine-{prefix}-{}-{nanos}",
        std::process::id()
    ));
    path
}

#[test]
fn binary_without_args_succeeds() {
    // Arrange
    let mut command = english_command();

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Rust foundation bootstrapped."));
}

#[test]
fn binary_without_args_succeeds_in_pt_br() {
    // Arrange
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Fundação Rust inicializada."));
}

#[test]
fn binary_without_args_accepts_global_locale_flag() {
    let mut command = english_command();
    command.args(["--locale", "pt-br"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Fundação Rust inicializada."));
}

#[test]
fn binary_global_locale_flag_overrides_environment() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "en");
    command.args(["--locale", "pt-br", "plugins", "show", "official.basic"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Nome: Básico"));
    assert!(stdout.contains("Resumo: Plugin base para templates iniciais."));
}

#[test]
fn binary_rejects_unsupported_locale() {
    // Arrange
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "es");

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(stderr.contains("unsupported locale: es. supported locales: en, pt-br"));
}

#[test]
fn binary_with_unknown_command_fails() {
    // Arrange
    let mut command = english_command();
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
    let mut command = english_command();
    command.args(["plugins", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Official plugins ("));
    assert!(stdout.contains("official.codex"));
}

#[test]
fn binary_plugins_show_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["plugins", "show", "official.github"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Plugin: official.github"));
    assert!(stdout.contains("Summary: GitHub data, context, forge, and MCP integration."));
    assert!(stdout.contains("Kind: data_source"));
    assert!(stdout.contains("Lifecycle: discover -> configure -> load"));
    assert!(stdout.contains("Load boundary: in_process"));
    assert!(stdout.contains(
        "Runtime hooks: mcp_registration, data_source_registration, context_provider_registration, forge_provider_registration"
    ));
    assert!(stdout.contains("Resolved activation: disabled"));
    assert!(stdout.contains("Resolved from: built_in_defaults"));
}

#[test]
fn binary_plugins_show_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["plugins", "show", "official.basic"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Nome: Básico"));
    assert!(stdout.contains("Resumo: Plugin base para templates iniciais."));
    assert!(stdout.contains("Ativação resolvida: enabled"));
    assert!(stdout.contains("Resolvido de: built_in_defaults"));
}

#[test]
fn binary_agents_list_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["agents", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Agent runtimes ("));
    assert!(stdout.contains("official.claude.session"));
    assert!(stdout.contains("official.codex.session"));
}

#[test]
fn binary_agents_list_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["agents", "list"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtimes de agente ("));
    assert!(stdout.contains("official.codex.session"));
}

#[test]
fn binary_agents_show_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["agents", "show", "official.codex.session"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Agent runtime: official.codex.session"));
    assert!(stdout.contains("Runtime hook: agent_bootstrap"));
}

#[test]
fn binary_agents_show_rejects_unknown_plugin_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["agents", "show", "official.missing"]);

    let output = command.output().expect("binary should run");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(stderr.contains("runtime de agente desconhecido: official.missing"));
}

#[test]
fn binary_templates_list_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["templates", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Templates ("));
    assert!(stdout.contains("official.basic.starter"));
    assert!(stdout.contains("official.bmad.starter"));
}

#[test]
fn binary_templates_show_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["templates", "show", "official.basic.starter"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Template: official.basic.starter"));
    assert!(stdout.contains("Plugin: official.basic"));
    assert!(stdout.contains("Runtime hook: scaffold"));
}

#[test]
fn binary_templates_show_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["templates", "show", "official.basic.starter"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Template: official.basic.starter"));
    assert!(stdout.contains("Nome: Starter básico"));
}

#[test]
fn binary_templates_show_rejects_unknown_plugin_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["templates", "show", "official.missing"]);

    let output = command.output().expect("binary should run");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(stderr.contains("template desconhecido: official.missing"));
}

#[test]
fn binary_templates_asset_succeeds() {
    let mut command = english_command();
    command.args([
        "templates",
        "asset",
        "official.basic.starter",
        ".ralph-engine/config.yaml",
    ]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("# ralph-engine basic template"));
}

#[test]
fn binary_templates_asset_rejects_unknown_asset_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args([
        "templates",
        "asset",
        "official.basic.starter",
        ".ralph-engine/missing.yaml",
    ]);

    let output = command.output().expect("binary should run");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(stderr.contains("asset de template desconhecido: .ralph-engine/missing.yaml"));
}

#[test]
fn binary_templates_materialize_writes_embedded_assets() {
    let output_dir = unique_temp_dir("templates-materialize");
    let output_dir_str = output_dir.to_string_lossy().into_owned();

    let mut command = english_command();
    command.args([
        "templates",
        "materialize",
        "official.basic.starter",
        &output_dir_str,
    ]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Materialized assets (4)"));
    assert!(output_dir.join(".ralph-engine/config.yaml").is_file());
    assert!(output_dir.join(".ralph-engine/README.md").is_file());

    std::fs::remove_dir_all(&output_dir).expect("temporary directory should be removable");
}

#[test]
fn binary_templates_scaffold_is_alias_for_materialize() {
    let output_dir = std::env::temp_dir().join("re-smoke-scaffold-alias");
    let _ = std::fs::remove_dir_all(&output_dir);

    let mut command = english_command();
    command.args([
        "templates",
        "scaffold",
        "official.basic.starter",
        output_dir
            .to_str()
            .expect("temp path should be valid UTF-8"),
    ]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    assert!(output_dir.join(".ralph-engine/config.yaml").is_file());

    let _ = std::fs::remove_dir_all(&output_dir);
}

#[test]
fn binary_prompts_list_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["prompts", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Prompts (1)"));
    assert!(stdout.contains("official.bmad.workflow"));
}

#[test]
fn binary_prompts_show_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["prompts", "show", "official.bmad.workflow"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Prompt: official.bmad.workflow"));
    assert!(stdout.contains("Plugin: official.bmad"));
    assert!(stdout.contains("Runtime hook: prompt_assembly"));
}

#[test]
fn binary_prompts_show_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["prompts", "show", "official.bmad.workflow"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Prompt: official.bmad.workflow"));
    assert!(stdout.contains("Nome: Prompt de workflow BMAD"));
}

#[test]
fn binary_prompts_show_rejects_unknown_plugin_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["prompts", "show", "official.missing"]);

    let output = command.output().expect("binary should run");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(stderr.contains("prompt desconhecido: official.missing"));
}

#[test]
fn binary_prompts_asset_succeeds() {
    let mut command = english_command();
    command.args([
        "prompts",
        "asset",
        "official.bmad.workflow",
        "prompts/workflow.md",
    ]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("# Ralph Engine — BMAD Template"));
}

#[test]
fn binary_prompts_asset_rejects_unknown_asset_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args([
        "prompts",
        "asset",
        "official.bmad.workflow",
        "prompts/missing.md",
    ]);

    let output = command.output().expect("binary should run");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(stderr.contains("asset de prompt desconhecido: prompts/missing.md"));
}

#[test]
fn binary_prompts_materialize_writes_embedded_assets() {
    let output_dir = unique_temp_dir("prompts-materialize");
    let output_dir_str = output_dir.to_string_lossy().into_owned();

    let mut command = english_command();
    command.args([
        "prompts",
        "materialize",
        "official.bmad.workflow",
        &output_dir_str,
    ]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Materialized assets (1)"));
    assert!(output_dir.join("prompts/workflow.md").is_file());

    std::fs::remove_dir_all(&output_dir).expect("temporary directory should be removable");
}

#[test]
fn binary_capabilities_list_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["capabilities", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Capabilities ("));
    assert!(stdout.contains("mcp_contribution"));
}

#[test]
fn binary_capabilities_show_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["capabilities", "show", "template"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Capacidade: template"));
    assert!(stdout.contains("Provedores ("));
}

#[test]
fn binary_capabilities_show_rejects_unknown_capability_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["capabilities", "show", "missing"]);

    let output = command.output().expect("binary should run");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(stderr.contains("capacidade desconhecida: missing"));
}

#[test]
fn binary_hooks_list_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["hooks", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime hooks ("));
    assert!(stdout.contains("mcp_registration"));
}

#[test]
fn binary_hooks_list_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["hooks", "list"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Hooks de runtime ("));
    assert!(stdout.contains("mcp_registration"));
}

#[test]
fn binary_checks_list_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["checks", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Checks ("));
    assert!(stdout.contains("prepare"));
    assert!(stdout.contains("doctor"));
}

#[test]
fn binary_checks_show_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["checks", "show", "prepare"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Verificação: prepare"));
    assert!(stdout.contains("Provedores ("));
}

#[test]
fn binary_checks_show_succeeds() {
    // Arrange
    let mut command = english_command();
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
fn binary_checks_show_by_id_succeeds() {
    let mut command = english_command();
    command.args(["checks", "show", "official.bmad.prepare"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Check: official.bmad.prepare"));
    assert!(stdout.contains("Name: BMAD prepare check"));
    assert!(stdout.contains("Kind: prepare"));
}

#[test]
fn binary_checks_run_succeeds() {
    let mut command = english_command();
    command.args(["checks", "run", "prepare"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime check: prepare"));
    assert!(stdout.contains("Outcome: failed"));
    assert!(stdout.contains("Runtime issues ("));
}

#[test]
fn binary_checks_run_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["checks", "run", "official.bmad.prepare"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Verificação de runtime: prepare"));
    assert!(stdout.contains("Resultado: reprovada"));
    assert!(stdout.contains("Problemas do runtime ("));
}

#[test]
fn binary_policies_show_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["policies", "show", "official.tdd-strict.guardrails"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Política: official.tdd-strict.guardrails"));
    assert!(stdout.contains("Hook de aplicação de política: policy_enforcement"));
}

#[test]
fn binary_policies_list_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["policies", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Policies (1)"));
    assert!(stdout.contains("official.tdd-strict.guardrails"));
}

#[test]
fn binary_policies_show_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["policies", "show", "official.tdd-strict.guardrails"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Policy: official.tdd-strict.guardrails"));
    assert!(stdout.contains("Provider: official.tdd-strict"));
    assert!(stdout.contains("Policy enforcement hook: policy_enforcement"));
}

#[test]
fn binary_providers_list_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["providers", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Providers ("));
    assert!(stdout.contains("data_source"));
    assert!(stdout.contains("remote_control"));
}

#[test]
fn binary_providers_show_succeeds() {
    // Arrange
    let mut command = english_command();
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
fn binary_providers_show_by_id_succeeds() {
    let mut command = english_command();
    command.args(["providers", "show", "official.github.data"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Provider: official.github.data"));
    assert!(stdout.contains("Name: GitHub data source"));
    assert!(stdout.contains("Kind: data_source"));
}

#[test]
fn binary_providers_show_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["providers", "show", "data_source"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Provedor: data_source"));
    assert!(stdout.contains("Provedores ("));
}

#[test]
fn binary_mcp_list_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["mcp", "list"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Official MCP servers ("));
    assert!(stdout.contains("official.github.repository"));
}

#[test]
fn binary_mcp_show_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["mcp", "show", "official.codex.session"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Servidor MCP: official.codex.session"));
    assert!(stdout.contains("Nome: Sessão Codex"));
    assert!(stdout.contains("Política de execução: plugin_runtime"));
    assert!(stdout.contains("Comando: gerenciado pelo runtime do plugin"));
}

#[test]
fn binary_mcp_show_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["mcp", "show", "official.codex.session"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("MCP server: official.codex.session"));
    assert!(stdout.contains("Name: Codex Session"));
    assert!(stdout.contains("Process model: plugin_managed"));
    assert!(stdout.contains("Launch policy: plugin_runtime"));
    assert!(stdout.contains("Availability: on_demand"));
    assert!(stdout.contains("Command: managed by plugin runtime"));
}

#[test]
fn binary_mcp_plan_succeeds() {
    let mut command = english_command();
    command.args(["mcp", "plan", "official.codex.session"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("MCP launch plan: official.codex.session"));
    assert!(stdout.contains("Launch step: plugin_runtime_bootstrap"));
    assert!(stdout.contains("Command: managed by plugin runtime"));
}

#[test]
fn binary_mcp_plan_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["mcp", "plan", "official.codex.session"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Plano de lançamento MCP: official.codex.session"));
    assert!(stdout.contains("Etapa de lançamento: plugin_runtime_bootstrap"));
    assert!(stdout.contains("Comando: gerenciado pelo runtime do plugin"));
}

#[test]
fn binary_mcp_status_succeeds() {
    let mut command = english_command();
    command.args(["mcp", "status"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("MCP server statuses ("));
}

#[test]
fn binary_mcp_status_single_server_succeeds() {
    let mut command = english_command();
    command.args(["mcp", "status", "official.codex.session"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("MCP server status: official.codex.session"));
    assert!(stdout.contains("Readiness:"));
    assert!(stdout.contains("Transport: stdio"));
}

#[test]
fn binary_mcp_status_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["mcp", "status"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Status dos servidores MCP ("));
}

#[test]
fn binary_mcp_status_rejects_unknown_server() {
    let mut command = english_command();
    command.args(["mcp", "status", "unknown.server"]);

    let output = command.output().expect("binary should run");

    assert!(!output.status.success());
}

#[test]
fn binary_runtime_show_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["runtime", "show"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime phase: ready"));
    assert!(stdout.contains("Plugins ("));
    assert!(stdout.contains("Capabilities ("));
    assert!(stdout.contains("Templates ("));
    assert!(stdout.contains("Prompts (1)"));
    assert!(stdout.contains("Agent runtimes ("));
    assert!(stdout.contains("Checks ("));
    assert!(stdout.contains("Providers ("));
    assert!(stdout.contains("Policies (1)"));
    assert!(stdout.contains("Runtime hooks ("));
    assert!(stdout.contains("MCP servers (4)"));
}

#[test]
fn binary_runtime_show_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["runtime", "show"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Fase do runtime: ready"));
    assert!(stdout.contains("Capacidades ("));
    assert!(stdout.contains("Verificações ("));
    assert!(stdout.contains("Políticas (1)"));
}

#[test]
fn binary_runtime_status_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["runtime", "status"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime health: degraded"));
    assert!(stdout.contains("Plugins: enabled=1, disabled="));
    assert!(stdout.contains("Templates: enabled=1, disabled="));
    assert!(stdout.contains("Prompts: enabled=0, disabled="));
    assert!(stdout.contains("Agent runtimes: enabled=0, disabled="));
    assert!(stdout.contains("Checks: enabled=0, disabled="));
    assert!(stdout.contains("Providers: enabled=0, disabled="));
    assert!(stdout.contains("Policies: enabled=0, disabled="));
    assert!(stdout.contains("Runtime hooks: enabled=1, disabled="));
}

#[test]
fn binary_runtime_status_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["runtime", "status"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Fase do runtime: ready"));
    assert!(stdout.contains("Saúde do runtime: degraded"));
    assert!(stdout.contains("Runtimes de agente: enabled=0, disabled="));
    assert!(stdout.contains("Hooks de runtime: enabled=1, disabled="));
}

#[test]
fn binary_runtime_issues_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["runtime", "issues"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime issues ("));
    assert!(stdout.contains("template_disabled"));
    assert!(stdout.contains("prompt_provider_disabled"));
    assert!(stdout.contains("agent_runtime_disabled"));
    assert!(stdout.contains("check_disabled"));
    assert!(stdout.contains("provider_disabled"));
    assert!(stdout.contains("plugin_disabled"));
    assert!(stdout.contains("policy_disabled"));
    assert!(stdout.contains("hook_disabled"));
    assert!(stdout.contains("mcp_server_disabled"));
}

#[test]
fn binary_runtime_issues_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["runtime", "issues"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Problemas do runtime ("));
    assert!(stdout.contains("ative o plugin na configuração tipada do projeto"));
    assert!(stdout.contains("ative o plugin provedor responsável por esta capacidade"));
}

#[test]
fn binary_runtime_plan_succeeds() {
    // Arrange
    let mut command = english_command();
    command.args(["runtime", "plan"]);

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime action plan ("));
    assert!(stdout.contains("enable_prompt_provider"));
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
fn binary_runtime_plan_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["runtime", "plan"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Plano de ação do runtime ("));
    assert!(stdout.contains("reason=o provedor ainda desabilita a capacidade forge_provider"));
    assert!(stdout.contains("reason=o provedor ainda desabilita a política official.tdd-strict"));
}

#[test]
fn binary_runtime_agent_plans_succeeds() {
    let mut command = english_command();
    command.args(["runtime", "agent-plans"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(stdout.trim(), "Runtime agent bootstrap plans (0)");
}

#[test]
fn binary_runtime_agent_plans_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["runtime", "agent-plans"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(
        stdout.trim(),
        "Planos de bootstrap de agentes do runtime (0)"
    );
}

#[test]
fn binary_runtime_provider_plans_succeeds() {
    let mut command = english_command();
    command.args(["runtime", "provider-plans"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(stdout.trim(), "Runtime provider registration plans (0)");
}

#[test]
fn binary_runtime_provider_plans_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["runtime", "provider-plans"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(
        stdout.trim(),
        "Planos de registro de providers do runtime (0)"
    );
}

#[test]
fn binary_runtime_check_plans_succeeds() {
    let mut command = english_command();
    command.args(["runtime", "check-plans"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(stdout.trim(), "Runtime check execution plans (0)");
}

#[test]
fn binary_runtime_check_plans_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["runtime", "check-plans"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(
        stdout.trim(),
        "Planos de execução de verificações do runtime (0)"
    );
}

#[test]
fn binary_runtime_policy_plans_succeeds() {
    let mut command = english_command();
    command.args(["runtime", "policy-plans"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(stdout.trim(), "Runtime policy enforcement plans (0)");
}

#[test]
fn binary_runtime_policy_plans_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["runtime", "policy-plans"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(
        stdout.trim(),
        "Planos de enforcement de políticas do runtime (0)"
    );
}

#[test]
fn binary_runtime_mcp_plans_succeeds() {
    let mut command = english_command();
    command.args(["runtime", "mcp-plans"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(stdout.trim(), "Runtime MCP launch plans (0)");
}

#[test]
fn binary_runtime_mcp_plans_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["runtime", "mcp-plans"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(stdout.trim(), "Planos de lançamento MCP do runtime (0)");
}

#[test]
fn binary_runtime_patch_succeeds() {
    let mut command = english_command();
    command.args(["runtime", "patch"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("plugins:"));
    assert!(stdout.contains("- id: official.github"));
    assert!(stdout.contains("activation: enabled"));
    assert!(stdout.contains("mcp:"));
    assert!(stdout.contains("servers:"));
    assert!(stdout.contains("- id: official.github.repository"));
    assert!(stdout.contains("enabled: true"));
}

#[test]
fn binary_runtime_patched_config_succeeds() {
    let mut command = english_command();
    command.args(["runtime", "patched-config"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("schema_version: 1"));
    assert!(stdout.contains("default_locale: en"));
    assert!(stdout.contains("- id: official.basic"));
    assert!(stdout.contains("- id: official.github"));
    assert!(stdout.contains("mcp:"));
    assert!(stdout.contains("- id: official.github.repository"));
    assert!(stdout.contains("enabled: true"));
}

#[test]
fn binary_runtime_write_patched_config_succeeds() {
    let output_path = unique_temp_dir("runtime-write-config").with_extension("yaml");
    let output_path_str = output_path.to_string_lossy().into_owned();
    let _ = std::fs::remove_file(&output_path);

    let mut command = english_command();
    command.args(["runtime", "write-patched-config", &output_path_str]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(stdout.trim(), format!("Wrote output: {output_path_str}"));
    let persisted = std::fs::read_to_string(&output_path).expect("config file should exist");
    assert!(persisted.contains("schema_version: 1"));
    assert!(persisted.contains("- id: official.github"));

    std::fs::remove_file(&output_path).expect("temporary config file should be removable");
}

#[test]
fn binary_config_show_defaults_succeeds() {
    // Arrange
    let mut command = english_command();
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
    let mut command = english_command();
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
fn binary_config_locale_succeeds() {
    let mut command = english_command();
    command.args(["config", "locale"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(stdout.trim(), "default_locale: en");
}

#[test]
fn binary_locales_list_succeeds() {
    let mut command = english_command();
    command.args(["locales", "list"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("supported_locales:"));
    assert!(stdout.contains("  - id: en"));
    assert!(stdout.contains("  - id: pt-br"));
}

#[test]
fn binary_locales_show_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.args(["locales", "show", "pt-br"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("id: pt-br"));
    assert!(stdout.contains("native_name: Português (Brasil)"));
}

#[test]
fn binary_config_budgets_succeeds() {
    // Arrange
    let mut command = english_command();
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
    let mut command = english_command();
    command.arg("doctor");

    // Act
    let output = command.output().expect("binary should run");

    // Assert
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Runtime doctor"));
    assert!(stdout.contains("Runtime issues ("));
    assert!(stdout.contains("Runtime action plan ("));
}

#[test]
fn binary_doctor_succeeds_in_pt_br() {
    let mut command = english_command();
    command.env("RALPH_ENGINE_LOCALE", "pt-br");
    command.arg("doctor");

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Diagnóstico do runtime"));
    assert!(stdout.contains("Problemas do runtime ("));
    assert!(stdout.contains("Plano de ação do runtime ("));
    assert!(stdout.contains("ative o plugin provedor responsável por esta contribuição"));
}

#[test]
fn binary_doctor_config_succeeds() {
    let mut command = english_command();
    command.args(["doctor", "config"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("schema_version: 1"));
    assert!(stdout.contains("- id: official.github"));
    assert!(stdout.contains("- id: official.github.repository"));
    assert!(stdout.contains("enabled: true"));
}

#[test]
fn binary_doctor_write_config_succeeds() {
    let output_path = unique_temp_dir("doctor-write-config").with_extension("yaml");
    let output_path_str = output_path.to_string_lossy().into_owned();
    let _ = std::fs::remove_file(&output_path);

    let mut command = english_command();
    command.args(["doctor", "write-config", &output_path_str]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert_eq!(stdout.trim(), format!("Wrote output: {output_path_str}"));
    let persisted = std::fs::read_to_string(&output_path).expect("config file should exist");
    assert!(persisted.contains("schema_version: 1"));
    assert!(persisted.contains("- id: official.github"));

    std::fs::remove_file(&output_path).expect("temporary config file should be removable");
}

#[test]
fn binary_config_show_plugin_succeeds() {
    // Arrange
    let mut command = english_command();
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

#[test]
fn binary_config_show_mcp_server_succeeds() {
    let mut command = english_command();
    command.args(["config", "show-mcp-server", "official.github.repository"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("id: official.github.repository"));
    assert!(stdout.contains("enabled: false"));
    assert!(stdout.contains("resolved_from: built_in_defaults"));
}

#[test]
fn binary_mcp_launch_probes_plugin_runtime_server() {
    let mut command = english_command();
    command.args(["mcp", "launch", "official.claude.session"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("MCP launch probe"));
    assert!(stdout.contains("plugin_runtime"));
}

#[test]
fn binary_mcp_launch_probes_spawn_process_server() {
    let mut command = english_command();
    command.args(["mcp", "launch", "official.github.repository"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("ralph-engine-github-mcp"));
}

#[test]
fn binary_agents_launch_probes_bootstrap() {
    let mut command = english_command();
    command.args(["agents", "launch", "official.claude.session"]);

    let output = command.output().expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("Agent bootstrap probe"));
    assert!(stdout.contains("official.claude"));
}

#[test]
fn binary_mcp_launch_rejects_unknown_server() {
    let mut command = english_command();
    command.args(["mcp", "launch", "unknown.server"]);

    let output = command.output().expect("binary should run");

    assert!(!output.status.success());
}

#[test]
fn binary_agents_launch_rejects_unknown_agent() {
    let mut command = english_command();
    command.args(["agents", "launch", "unknown.agent"]);

    let output = command.output().expect("binary should run");

    assert!(!output.status.success());
}

// ── Project config file loading tests ────────────────────────────

#[test]
fn doctor_reads_project_config_and_enables_plugins() {
    // Without config: only official.basic is enabled (1 plugin).
    // With config enabling bmad+claude: doctor should show more enabled.
    let tmp = unique_temp_dir("config-load");
    std::fs::create_dir_all(tmp.join(".ralph-engine")).expect("should create .ralph-engine dir");
    std::fs::write(
        tmp.join(".ralph-engine/config.yaml"),
        "\
schema_version: 1
default_locale: en
plugins:
  - id: official.basic
    activation: enabled
  - id: official.bmad
    activation: enabled
  - id: official.claude
    activation: enabled
  - id: official.claudebox
    activation: disabled
  - id: official.codex
    activation: disabled
  - id: official.github
    activation: enabled
  - id: official.hello-world
    activation: disabled
  - id: official.ssh
    activation: disabled
  - id: official.tdd-strict
    activation: enabled
mcp:
  enabled: true
  discovery: official_only
  servers:
    - id: official.claude.session
      enabled: true
    - id: official.claudebox.session
      enabled: false
    - id: official.codex.session
      enabled: false
    - id: official.github.repository
      enabled: true
budgets:
  prompt_tokens: 8192
  context_tokens: 32768
",
    )
    .expect("should write config");

    let mut command = english_command();
    command.current_dir(&tmp);
    command.args(["doctor"]);

    let output = command.output().expect("binary should run");
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");

    // With config: 5 plugins enabled (basic, bmad, claude, github, tdd-strict)
    assert!(
        stdout.contains("enabled=5"),
        "doctor should show 5 enabled plugins when project config is present.\nGot: {stdout}"
    );

    std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn doctor_without_config_shows_default_enabled_count() {
    // In a directory without .ralph-engine/config.yaml, only basic is enabled.
    let tmp = unique_temp_dir("no-config");
    std::fs::create_dir_all(&tmp).expect("should create temp dir");

    let mut command = english_command();
    command.current_dir(&tmp);
    command.args(["doctor"]);

    let output = command.output().expect("binary should run");
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");

    // Default: only official.basic enabled (1 plugin)
    assert!(
        stdout.contains("enabled=1"),
        "doctor without config should show 1 enabled plugin.\nGot: {stdout}"
    );

    std::fs::remove_dir_all(&tmp).ok();
}

// ── Run command smoke tests ──────────────────────────────────────

#[test]
fn binary_run_without_args_shows_usage_error() {
    let tmp = unique_temp_dir("run-no-args");
    std::fs::create_dir_all(&tmp).expect("should create temp dir");

    let mut command = english_command();
    command.current_dir(&tmp);
    command.args(["run"]);

    let output = command.output().expect("binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(
        stderr.contains("Work item ID required"),
        "run without args should show usage error.\nGot: {stderr}"
    );

    std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn binary_run_without_config_shows_config_error() {
    let tmp = unique_temp_dir("run-no-config");
    std::fs::create_dir_all(&tmp).expect("should create temp dir");

    let mut command = english_command();
    command.current_dir(&tmp);
    command.args(["run", "5.3"]);

    let output = command.output().expect("binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(
        stderr.contains("config not found") || stderr.contains("materialize"),
        "run without config should suggest creating config.\nGot: {stderr}"
    );

    std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn binary_run_without_run_section_shows_missing_field_error() {
    let tmp = unique_temp_dir("run-no-run-section");
    std::fs::create_dir_all(tmp.join(".ralph-engine")).expect("should create dir");
    std::fs::write(
        tmp.join(".ralph-engine/config.yaml"),
        "schema_version: 1\ndefault_locale: en\nplugins:\n  - id: official.basic\n    activation: enabled\nmcp:\n  enabled: true\n  discovery: official_only\n  servers:\nbudgets:\n  prompt_tokens: 8192\n  context_tokens: 32768\n",
    )
    .expect("should write config");

    let mut command = english_command();
    command.current_dir(&tmp);
    command.args(["run", "5.3"]);

    let output = command.output().expect("binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(
        stderr.contains("workflow_plugin"),
        "run without run: section should mention workflow_plugin.\nGot: {stderr}"
    );

    std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn binary_run_list_without_config_shows_config_error() {
    let tmp = unique_temp_dir("run-list-no-config");
    std::fs::create_dir_all(&tmp).expect("should create temp dir");

    let mut command = english_command();
    command.current_dir(&tmp);
    command.args(["run", "--list"]);

    let output = command.output().expect("binary should run");

    assert!(!output.status.success());

    std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn binary_run_plan_without_id_shows_error() {
    let mut command = english_command();
    command.args(["run", "plan"]);

    let output = command.output().expect("binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(
        stderr.contains("work item ID"),
        "run plan without ID should ask for one.\nGot: {stderr}"
    );
}
