//! CLI command execution for Ralph Engine.

mod catalog;
mod commands;
mod error;
mod i18n;

pub use error::CliError;

/// Executes the current CLI foundation command set.
pub fn execute<I>(args: I) -> Result<String, CliError>
where
    I: IntoIterator<Item = String>,
{
    let collected: Vec<String> = args.into_iter().collect();
    commands::execute(&collected)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{catalog, execute};
    use re_config::PluginActivation;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    /// Builds a command with `--locale en` injected after the binary name
    /// so tests produce predictable English output regardless of the host OS.
    fn args_en(values: &[&str]) -> Vec<String> {
        let mut result = vec![values[0].to_owned(), "--locale".to_owned(), "en".to_owned()];
        for value in &values[1..] {
            result.push((*value).to_owned());
        }
        result
    }

    fn sample_plugin_id() -> &'static str {
        catalog::official_plugins()[0].id
    }

    fn sample_disabled_plugin_id() -> &'static str {
        catalog::official_plugins()
            .into_iter()
            .find(|plugin| {
                matches!(
                    catalog::official_runtime_plugins()
                        .into_iter()
                        .find(|registration| registration.descriptor.id == plugin.id)
                        .map(|registration| registration.activation),
                    Some(PluginActivation::Disabled)
                )
            })
            .map(|plugin| plugin.id)
            .expect("expected at least one disabled plugin")
    }

    fn sample_template_id() -> &'static str {
        catalog::official_template_contributions()[0].descriptor.id
    }

    fn sample_prompt_id() -> &'static str {
        catalog::official_prompt_contributions()[0].descriptor.id
    }

    fn sample_agent_id() -> &'static str {
        catalog::official_agent_contributions()[0].descriptor.id
    }

    fn sample_policy_id() -> &'static str {
        catalog::official_policy_contributions()[0].descriptor.id
    }

    fn sample_mcp_id() -> &'static str {
        catalog::official_mcp_servers()[0].id
    }

    #[test]
    fn execute_without_subcommand_prints_foundation_banner() {
        // Arrange
        let command = args_en(&["ralph-engine"]);

        // Act
        let output = execute(command).expect("default command should succeed");

        // Assert
        assert!(output.contains("Rust foundation bootstrapped."));
        assert!(output.contains("Ralph Engine"));
    }

    #[test]
    fn execute_version_returns_package_version() {
        // Arrange
        let command = args_en(&["ralph-engine", "--version"]);

        // Act
        let output = execute(command).expect("version should succeed");

        // Assert
        assert_eq!(output, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn execute_version_accepts_global_locale_flag() {
        let command = args(&["ralph-engine", "--locale", "pt-br", "--version"]);

        let output = execute(command).expect("version with locale flag should succeed");

        assert_eq!(output, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn execute_agents_lists_runtime_agents() {
        // Arrange
        let command = args_en(&["ralph-engine", "agents", "list"]);
        let agents = catalog::official_agent_contributions();

        // Act
        let output = execute(command).expect("agents list should succeed");

        // Assert
        assert!(output.contains(&format!("Agent runtimes ({})", agents.len())));
        for agent in agents {
            assert!(output.contains(agent.descriptor.id));
        }
    }

    #[test]
    fn execute_agents_show_returns_agent_detail() {
        // Arrange
        let agent = catalog::find_official_agent_contribution(sample_agent_id())
            .expect("sample agent should exist");
        let command = args_en(&["ralph-engine", "agents", "show", agent.descriptor.id]);

        // Act
        let output = execute(command).expect("agents show should succeed");

        // Assert
        assert!(output.contains(&format!("Agent runtime: {}", agent.descriptor.id)));
        assert!(output.contains(&format!(
            "Name: {}",
            agent.descriptor.display_name_for_locale("en")
        )));
        assert!(output.contains(&format!("Plugin: {}", agent.descriptor.plugin_id)));
        assert!(output.contains(&format!("Activation: {}", agent.activation.as_str())));
        assert!(output.contains("Runtime hook:"));
    }

    #[test]
    fn execute_agents_show_requires_agent_id() {
        // Arrange
        let command = args_en(&["ralph-engine", "agents", "show"]);

        // Act
        let error = execute(command).expect_err("missing agent id should fail");

        // Assert
        assert_eq!(error.to_string(), "agents show requires an agent id");
    }

    #[test]
    fn execute_agents_show_rejects_unknown_agent() {
        // Arrange
        let command = args_en(&["ralph-engine", "agents", "show", "fixture.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown agent runtime should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown agent runtime: fixture.unknown");
    }

    #[test]
    fn execute_agents_plan_returns_bootstrap_plan() {
        let agent = catalog::find_official_agent_contribution(sample_agent_id())
            .expect("sample agent should exist");
        let command = args_en(&["ralph-engine", "agents", "plan", agent.descriptor.id]);

        let output = execute(command).expect("agents plan should succeed");

        assert!(output.contains(&format!("Agent bootstrap plan: {}", agent.descriptor.id)));
        assert!(output.contains(&format!("Plugin: {}", agent.descriptor.plugin_id)));
    }

    #[test]
    fn execute_locales_lists_supported_locale_catalog() {
        let command = args_en(&["ralph-engine", "locales", "list"]);

        let output = execute(command).expect("locales list should succeed");

        assert!(output.contains("supported_locales:"));
        assert!(output.contains("  - id: en"));
        assert!(output.contains("  - id: pt-br"));
    }

    #[test]
    fn execute_locales_show_returns_locale_detail() {
        let command = args_en(&["ralph-engine", "locales", "show", "pt-br"]);

        let output = execute(command).expect("locales show should succeed");

        assert!(output.contains("id: pt-br"));
        assert!(output.contains("english_name: Portuguese (Brazil)"));
        assert!(output.contains("native_name: Português (Brasil)"));
    }

    #[test]
    fn execute_locales_show_requires_locale_id() {
        let command = args_en(&["ralph-engine", "locales", "show"]);

        let error = execute(command).expect_err("missing locale id should fail");

        assert_eq!(error.to_string(), "locales show requires a locale id");
    }

    #[test]
    fn execute_locales_show_rejects_unknown_locale() {
        let command = args_en(&["ralph-engine", "locales", "show", "es"]);

        let error = execute(command).expect_err("unknown locale should fail");

        assert_eq!(error.to_string(), "unknown locale: es");
    }

    #[test]
    fn execute_global_locale_flag_switches_command_output() {
        let command = args(&["ralph-engine", "--locale", "pt-br"]);

        let output = execute(command).expect("global locale flag should localize command output");

        assert!(output.contains("Fundação Rust inicializada."));
    }

    #[test]
    fn execute_short_global_locale_flag_switches_command_output() {
        let command = args(&[
            "ralph-engine",
            "-L",
            "pt-br",
            "providers",
            "show",
            "official.github.data",
        ]);

        let output = execute(command).expect("short locale flag should localize command output");

        assert!(output.contains("Provedor: official.github.data"));
        assert!(output.contains("Ativação:"));
    }

    #[test]
    fn execute_global_locale_flag_requires_locale_id() {
        let command = args(&["ralph-engine", "--locale"]);

        let error = execute(command).expect_err("missing locale flag value should fail");

        assert_eq!(error.to_string(), "--locale requires a locale id");
    }

    #[test]
    fn execute_templates_list_runtime_templates() {
        // Arrange
        let command = args_en(&["ralph-engine", "templates", "list"]);
        let templates = catalog::official_template_contributions();

        // Act
        let output = execute(command).expect("templates list should succeed");

        // Assert
        assert!(output.contains(&format!("Templates ({})", templates.len())));
        for template in templates {
            assert!(output.contains(template.descriptor.id));
        }
    }

    #[test]
    fn execute_templates_show_returns_template_detail() {
        // Arrange
        let template = catalog::find_official_template_contribution(sample_template_id())
            .expect("sample template should exist");
        let command = args_en(&["ralph-engine", "templates", "show", template.descriptor.id]);

        // Act
        let output = execute(command).expect("templates show should succeed");

        // Assert
        assert!(output.contains(&format!("Template: {}", template.descriptor.id)));
        assert!(output.contains(&format!(
            "Name: {}",
            template.descriptor.display_name_for_locale("en")
        )));
        assert!(output.contains(&format!("Plugin: {}", template.descriptor.plugin_id)));
        assert!(output.contains(&format!("Activation: {}", template.activation.as_str())));
        assert!(output.contains("Runtime hook:"));
    }

    #[test]
    fn execute_templates_show_requires_template_id() {
        // Arrange
        let command = args_en(&["ralph-engine", "templates", "show"]);

        // Act
        let error = execute(command).expect_err("missing template id should fail");

        // Assert
        assert_eq!(error.to_string(), "templates show requires a template id");
    }

    #[test]
    fn execute_templates_show_rejects_unknown_template() {
        // Arrange
        let command = args_en(&["ralph-engine", "templates", "show", "fixture.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown template should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown template: fixture.unknown");
    }

    #[test]
    fn execute_prompts_list_runtime_prompts() {
        // Arrange
        let command = args_en(&["ralph-engine", "prompts", "list"]);
        let prompts = catalog::official_prompt_contributions();

        // Act
        let output = execute(command).expect("prompts list should succeed");

        // Assert
        assert!(output.contains(&format!("Prompts ({})", prompts.len())));
        for prompt in prompts {
            assert!(output.contains(prompt.descriptor.id));
        }
    }

    #[test]
    fn execute_prompts_show_returns_prompt_detail() {
        // Arrange
        let prompt = catalog::find_official_prompt_contribution(sample_prompt_id())
            .expect("sample prompt should exist");
        let command = args_en(&["ralph-engine", "prompts", "show", prompt.descriptor.id]);

        // Act
        let output = execute(command).expect("prompts show should succeed");

        // Assert
        assert!(output.contains(&format!("Prompt: {}", prompt.descriptor.id)));
        assert!(output.contains(&format!(
            "Name: {}",
            prompt.descriptor.display_name_for_locale("en")
        )));
        assert!(output.contains(&format!("Plugin: {}", prompt.descriptor.plugin_id)));
        assert!(output.contains(&format!("Activation: {}", prompt.activation.as_str())));
        assert!(output.contains("Runtime hook:"));
    }

    #[test]
    fn execute_prompts_show_requires_prompt_id() {
        // Arrange
        let command = args_en(&["ralph-engine", "prompts", "show"]);

        // Act
        let error = execute(command).expect_err("missing prompt id should fail");

        // Assert
        assert_eq!(error.to_string(), "prompts show requires a prompt id");
    }

    #[test]
    fn execute_prompts_show_rejects_unknown_prompt() {
        // Arrange
        let command = args_en(&["ralph-engine", "prompts", "show", "fixture.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown prompt should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown prompt: fixture.unknown");
    }

    #[test]
    fn execute_capabilities_lists_runtime_capabilities() {
        // Arrange
        let command = args_en(&["ralph-engine", "capabilities", "list"]);

        // Act
        let output = execute(command).expect("capabilities list should succeed");

        // Assert
        // Dynamic — count changes when plugins are added
        assert!(output.contains("Capabilities ("));
        assert!(output.contains("- agent_runtime |"));
        assert!(output.contains("- template |"));
    }

    #[test]
    fn execute_capabilities_show_returns_provider_detail() {
        // Arrange
        let command = args_en(&["ralph-engine", "capabilities", "show", "mcp_contribution"]);

        // Act
        let output = execute(command).expect("capabilities show should succeed");

        // Assert
        assert!(output.contains("Capability: mcp_contribution"));
        assert!(output.contains("Providers (4)"));
        assert!(output.contains("- official.claude | activation=disabled | boundary=in_process"));
        assert!(output.contains("- official.github | activation=disabled | boundary=in_process"));
    }

    #[test]
    fn execute_capabilities_show_requires_capability_id() {
        // Arrange
        let command = args_en(&["ralph-engine", "capabilities", "show"]);

        // Act
        let error = execute(command).expect_err("missing capability id should fail");

        // Assert
        assert_eq!(
            error.to_string(),
            "capabilities show requires a capability id"
        );
    }

    #[test]
    fn execute_capabilities_show_rejects_unknown_capability() {
        // Arrange
        let command = args_en(&["ralph-engine", "capabilities", "show", "unknown"]);

        // Act
        let error = execute(command).expect_err("unknown capability should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown capability: unknown");
    }

    #[test]
    fn execute_checks_lists_runtime_checks() {
        // Arrange
        let command = args_en(&["ralph-engine", "checks", "list"]);

        // Act
        let output = execute(command).expect("checks list should succeed");

        // Assert
        assert!(output.contains("Checks (2)"));
        assert!(output.contains("- prepare |"));
        assert!(output.contains("- doctor |"));
    }

    #[test]
    fn execute_checks_show_returns_provider_detail() {
        // Arrange
        let command = args_en(&["ralph-engine", "checks", "show", "prepare"]);

        // Act
        let output = execute(command).expect("checks show should succeed");

        // Assert
        assert!(output.contains("Check: prepare"));
        assert!(output.contains("Providers (1)"));
        assert!(output.contains(
            "- official.bmad.prepare | plugin=official.bmad | name=BMAD prepare check | summary=Runs typed prepare-time validation for BMAD workflows. | activation=disabled | boundary=in_process | runtime_hook=true"
        ));
    }

    #[test]
    fn execute_checks_show_accepts_check_contribution_id() {
        let command = args_en(&["ralph-engine", "checks", "show", "official.bmad.prepare"]);

        let output = execute(command).expect("checks show by id should succeed");

        assert!(output.contains("Check: official.bmad.prepare"));
        assert!(output.contains("Name: BMAD prepare check"));
        assert!(output.contains("Kind: prepare"));
    }

    #[test]
    fn execute_checks_run_returns_typed_check_result() {
        let command = args_en(&["ralph-engine", "checks", "run", "prepare"]);

        let output = execute(command).expect("checks run should succeed");

        assert!(output.contains("Runtime check: prepare"));
        assert!(output.contains("Outcome: failed"));
        assert!(output.contains("Runtime issues ("));
        assert!(output.contains("Runtime action plan ("));
    }

    #[test]
    fn execute_checks_run_accepts_check_contribution_id() {
        let command = args_en(&["ralph-engine", "checks", "run", "official.bmad.prepare"]);

        let output = execute(command).expect("checks run by id should succeed");

        assert!(output.contains("Runtime check: prepare"));
    }

    #[test]
    fn execute_checks_show_requires_check_id() {
        // Arrange
        let command = args_en(&["ralph-engine", "checks", "show"]);

        // Act
        let error = execute(command).expect_err("missing check id should fail");

        // Assert
        assert_eq!(error.to_string(), "checks show requires a check id");
    }

    #[test]
    fn execute_checks_run_requires_check_id() {
        let command = args_en(&["ralph-engine", "checks", "run"]);

        let error = execute(command).expect_err("missing check id should fail");

        assert_eq!(error.to_string(), "checks run requires a check id");
    }

    #[test]
    fn execute_checks_plan_returns_execution_plan() {
        let command = args_en(&["ralph-engine", "checks", "plan", "official.bmad.prepare"]);

        let output = execute(command).expect("checks plan should succeed");

        assert!(output.contains("Runtime check plan: official.bmad.prepare"));
        assert!(output.contains("Plugin: official.bmad"));
        assert!(output.contains("prepare"));
    }

    #[test]
    fn execute_checks_show_rejects_unknown_check() {
        // Arrange
        let command = args_en(&["ralph-engine", "checks", "show", "unknown"]);

        // Act
        let error = execute(command).expect_err("unknown check should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown check: unknown");
    }

    #[test]
    fn execute_hooks_lists_runtime_hooks() {
        // Arrange
        let command = args_en(&["ralph-engine", "hooks", "list"]);

        // Act
        let output = execute(command).expect("hooks list should succeed");

        // Assert
        assert!(output.contains("Runtime hooks (13)"));
        assert!(output.contains("- scaffold |"));
        assert!(output.contains("- mcp_registration | providers=4 | enabled=0"));
    }

    #[test]
    fn execute_hooks_show_returns_provider_detail() {
        // Arrange
        let command = args_en(&["ralph-engine", "hooks", "show", "mcp_registration"]);

        // Act
        let output = execute(command).expect("hooks show should succeed");

        // Assert
        assert!(output.contains("Runtime hook: mcp_registration"));
        assert!(output.contains("Providers (4)"));
        assert!(output.contains("- official.claude | activation=disabled | boundary=in_process"));
        assert!(output.contains("- official.github | activation=disabled | boundary=in_process"));
    }

    #[test]
    fn execute_hooks_show_requires_hook_id() {
        // Arrange
        let command = args_en(&["ralph-engine", "hooks", "show"]);

        // Act
        let error = execute(command).expect_err("missing hook id should fail");

        // Assert
        assert_eq!(error.to_string(), "hooks show requires a hook id");
    }

    #[test]
    fn execute_hooks_show_rejects_unknown_hook() {
        // Arrange
        let command = args_en(&["ralph-engine", "hooks", "show", "unknown"]);

        // Act
        let error = execute(command).expect_err("unknown hook should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown hook: unknown");
    }

    #[test]
    fn execute_hooks_plan_returns_owned_surface_map() {
        let command = args_en(&["ralph-engine", "hooks", "plan", "agent_bootstrap"]);

        let output = execute(command).expect("hooks plan should succeed");

        assert!(output.contains("Runtime hook plan: agent_bootstrap"));
        assert!(output.contains("Agent runtimes ("));
        assert!(output.contains("bootstrap_hook="));
    }

    #[test]
    fn execute_policies_lists_runtime_policies() {
        // Arrange
        let command = args_en(&["ralph-engine", "policies", "list"]);
        let policies = catalog::official_policy_contributions();

        // Act
        let output = execute(command).expect("policies list should succeed");

        // Assert
        assert!(output.contains(&format!("Policies ({})", policies.len())));
        for policy in policies {
            assert!(output.contains(policy.descriptor.id));
        }
    }

    #[test]
    fn execute_policies_show_returns_policy_detail() {
        // Arrange
        let policy = catalog::find_official_policy_contribution(sample_policy_id())
            .expect("sample policy should exist");
        let command = args_en(&["ralph-engine", "policies", "show", policy.descriptor.id]);

        // Act
        let output = execute(command).expect("policies show should succeed");

        // Assert
        assert!(output.contains(&format!("Policy: {}", policy.descriptor.id)));
        assert!(output.contains(&format!(
            "Name: {}",
            policy.descriptor.display_name_for_locale("en")
        )));
        assert!(output.contains(&format!("Provider: {}", policy.descriptor.plugin_id)));
        assert!(output.contains("Policy enforcement hook:"));
    }

    #[test]
    fn execute_policies_show_requires_policy_id() {
        // Arrange
        let command = args_en(&["ralph-engine", "policies", "show"]);

        // Act
        let error = execute(command).expect_err("missing policy id should fail");

        // Assert
        assert_eq!(error.to_string(), "policies show requires a policy id");
    }

    #[test]
    fn execute_policies_show_rejects_unknown_policy() {
        // Arrange
        let command = args_en(&["ralph-engine", "policies", "show", "fixture.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown policy should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown policy: fixture.unknown");
    }

    #[test]
    fn execute_policies_run_returns_typed_policy_result() {
        let policy = catalog::find_official_policy_contribution(sample_policy_id())
            .expect("sample policy should exist");
        let command = args_en(&["ralph-engine", "policies", "run", policy.descriptor.id]);

        let output = execute(command).expect("policies run should succeed");

        assert!(output.contains(&format!("Runtime policy: {}", policy.descriptor.id)));
        assert!(output.contains(&format!("Provider: {}", policy.descriptor.plugin_id)));
        assert!(output.contains("Outcome:"));
        assert!(output.contains("Policy enforcement hook:"));
    }

    #[test]
    fn execute_policies_run_requires_policy_id() {
        let command = args_en(&["ralph-engine", "policies", "run"]);

        let error = execute(command).expect_err("missing policy id should fail");

        assert_eq!(error.to_string(), "policies run requires a policy id");
    }

    #[test]
    fn execute_policies_plan_returns_enforcement_plan() {
        let command = args_en(&[
            "ralph-engine",
            "policies",
            "plan",
            "official.tdd-strict.guardrails",
        ]);

        let output = execute(command).expect("policies plan should succeed");

        assert!(output.contains("Policy enforcement plan: official.tdd-strict.guardrails"));
        assert!(output.contains("Plugin: official.tdd-strict"));
        assert!(output.contains("policy_enforcement"));
    }

    #[test]
    fn execute_providers_lists_runtime_providers() {
        // Arrange
        let command = args_en(&["ralph-engine", "providers", "list"]);

        // Act
        let output = execute(command).expect("providers list should succeed");

        // Assert
        assert!(output.contains("Providers (4)"));
        assert!(output.contains("- data_source |"));
        assert!(output.contains("- remote_control |"));
    }

    #[test]
    fn execute_providers_show_returns_provider_detail() {
        // Arrange
        let command = args_en(&["ralph-engine", "providers", "show", "data_source"]);

        // Act
        let output = execute(command).expect("providers show should succeed");

        // Assert
        assert!(output.contains("Provider: data_source"));
        assert!(output.contains("Providers (1)"));
        assert!(output.contains(
            "- official.github.data | plugin=official.github | name=GitHub data source | summary=Exposes typed repository data to Ralph Engine workflows. | activation=disabled | boundary=in_process | registration_hook=true"
        ));
    }

    #[test]
    fn execute_providers_show_accepts_provider_contribution_id() {
        let command = args_en(&["ralph-engine", "providers", "show", "official.github.data"]);

        let output = execute(command).expect("providers show by id should succeed");

        assert!(output.contains("Provider: official.github.data"));
        assert!(output.contains("Name: GitHub data source"));
        assert!(output.contains("Kind: data_source"));
    }

    #[test]
    fn execute_providers_show_requires_provider_id() {
        // Arrange
        let command = args_en(&["ralph-engine", "providers", "show"]);

        // Act
        let error = execute(command).expect_err("missing provider id should fail");

        // Assert
        assert_eq!(error.to_string(), "providers show requires a provider id");
    }

    #[test]
    fn execute_providers_show_rejects_unknown_provider() {
        // Arrange
        let command = args_en(&["ralph-engine", "providers", "show", "unknown"]);

        // Act
        let error = execute(command).expect_err("unknown provider should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown provider: unknown");
    }

    #[test]
    fn execute_providers_plan_returns_registration_plan() {
        let command = args_en(&["ralph-engine", "providers", "plan", "official.github.data"]);

        let output = execute(command).expect("providers plan should succeed");

        assert!(output.contains("Provider registration plan: official.github.data"));
        assert!(output.contains("Plugin: official.github"));
        assert!(output.contains("Registration hook: data_source_registration"));
    }

    #[test]
    fn execute_plugins_lists_official_plugins() {
        // Arrange
        let command = args_en(&["ralph-engine", "plugins", "list"]);
        let plugins = catalog::official_plugins();

        // Act
        let output = execute(command).expect("plugins list should succeed");

        // Assert
        assert!(output.contains(&format!("Official plugins ({})", plugins.len())));
        for plugin in plugins {
            assert!(output.contains(plugin.id));
        }
    }

    #[test]
    fn execute_plugins_show_returns_plugin_detail() {
        // Arrange
        let plugin = catalog::find_official_plugin(sample_disabled_plugin_id())
            .expect("sample plugin should exist");
        let command = args_en(&["ralph-engine", "plugins", "show", plugin.id]);

        // Act
        let output = execute(command).expect("plugins show should succeed");

        // Assert
        assert!(output.contains(&format!("Plugin: {}", plugin.id)));
        assert!(output.contains(&format!("Name: {}", plugin.display_name_for_locale("en"))));
        let lifecycle = plugin
            .lifecycle
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" -> ");
        assert!(output.contains(&format!("Lifecycle: {lifecycle}")));
        assert!(output.contains(&format!("Load boundary: {}", plugin.load_boundary.as_str())));
        assert!(output.contains("Runtime hooks:"));
        assert!(output.contains(&format!(
            "Resolved activation: {}",
            PluginActivation::Disabled.as_str()
        )));
        assert!(output.contains("Resolved from: built_in_defaults"));
    }

    #[test]
    fn execute_plugins_show_reports_enabled_default_activation() {
        // Arrange
        let plugin = catalog::find_official_plugin(sample_plugin_id())
            .expect("sample enabled plugin should exist");
        let command = args_en(&["ralph-engine", "plugins", "show", plugin.id]);

        // Act
        let output = execute(command).expect("plugins show should succeed");

        // Assert
        assert!(output.contains(&format!("Plugin: {}", plugin.id)));
        assert!(output.contains("Resolved activation: enabled"));
        assert!(output.contains("Resolved from: built_in_defaults"));
    }

    #[test]
    fn execute_plugins_show_requires_plugin_id() {
        // Arrange
        let command = args_en(&["ralph-engine", "plugins", "show"]);

        // Act
        let error = execute(command).expect_err("missing plugin id should fail");

        // Assert
        assert_eq!(error.to_string(), "plugins show requires a plugin id");
    }

    #[test]
    fn execute_plugins_show_rejects_unknown_plugin() {
        // Arrange
        let command = args_en(&["ralph-engine", "plugins", "show", "fixture.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown plugin id should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown plugin: fixture.unknown");
    }

    #[test]
    fn execute_config_show_defaults_returns_yaml_contract() {
        // Arrange
        let command = args_en(&["ralph-engine", "config", "show-defaults"]);

        // Act
        let output = execute(command).expect("config show-defaults should succeed");

        // Assert
        assert!(output.contains("schema_version: 1"));
        assert!(output.contains("default_locale: en"));
        assert!(output.contains("official.basic"));
    }

    #[test]
    fn execute_config_layers_returns_typed_resolution_stack() {
        // Arrange
        let command = args_en(&["ralph-engine", "config", "layers"]);

        // Act
        let output = execute(command).expect("config layers should succeed");

        // Assert
        assert!(output.contains("layers:"));
        assert!(output.contains("scope: built_in_defaults"));
        assert!(output.contains("schema_version: 1"));
        assert!(output.contains("plugin_count: 1"));
        assert!(output.contains("mcp_enabled: true"));
        assert!(output.contains("prompt_tokens: 8192"));
        assert!(output.contains("context_tokens: 32768"));
    }

    #[test]
    fn execute_config_locale_returns_default_locale_contract() {
        // Arrange
        let command = args_en(&["ralph-engine", "config", "locale"]);

        // Act
        let output = execute(command).expect("config locale should succeed");

        // Assert
        assert_eq!(output, "default_locale: en");
    }

    #[test]
    fn execute_config_show_locale_alias_returns_default_locale_contract() {
        // Arrange
        let command = args_en(&["ralph-engine", "config", "show-locale"]);

        // Act
        let output = execute(command).expect("config show-locale should succeed");

        // Assert
        assert_eq!(output, "default_locale: en");
    }

    #[test]
    fn execute_config_show_layers_alias_returns_typed_resolution_stack() {
        // Arrange
        let command = args_en(&["ralph-engine", "config", "show-layers"]);

        // Act
        let output = execute(command).expect("config show-layers should succeed");

        // Assert
        assert!(output.contains("layers:"));
        assert!(output.contains("scope: built_in_defaults"));
    }

    #[test]
    fn execute_config_budgets_returns_typed_budget_contract() {
        // Arrange
        let command = args_en(&["ralph-engine", "config", "budgets"]);

        // Act
        let output = execute(command).expect("config budgets should succeed");

        // Assert
        assert!(output.contains("budgets:"));
        assert!(output.contains("prompt_tokens: 8192"));
        assert!(output.contains("context_tokens: 32768"));
    }

    #[test]
    fn execute_config_show_budgets_alias_returns_typed_budget_contract() {
        // Arrange
        let command = args_en(&["ralph-engine", "config", "show-budgets"]);

        // Act
        let output = execute(command).expect("config show-budgets should succeed");

        // Assert
        assert!(output.contains("budgets:"));
        assert!(output.contains("prompt_tokens: 8192"));
    }

    #[test]
    fn execute_config_show_plugin_returns_resolved_yaml() {
        // Arrange
        let command = args_en(&["ralph-engine", "config", "show-plugin", sample_plugin_id()]);

        // Act
        let output = execute(command).expect("config show-plugin should succeed");

        // Assert
        assert!(output.contains(&format!("id: {}", sample_plugin_id())));
        assert!(output.contains("activation: enabled"));
        assert!(output.contains("resolved_from: built_in_defaults"));
    }

    #[test]
    fn execute_config_show_plugin_returns_disabled_built_in_default_for_known_plugin() {
        // Arrange
        let disabled_plugin_id = sample_disabled_plugin_id();
        let command = args_en(&["ralph-engine", "config", "show-plugin", disabled_plugin_id]);

        // Act
        let output = execute(command).expect("config show-plugin should succeed");

        // Assert
        assert!(output.contains(&format!("id: {disabled_plugin_id}")));
        assert!(output.contains("activation: disabled"));
        assert!(output.contains("resolved_from: built_in_defaults"));
    }

    #[test]
    fn execute_config_show_plugin_requires_plugin_id() {
        // Arrange
        let command = args_en(&["ralph-engine", "config", "show-plugin"]);

        // Act
        let error = execute(command).expect_err("missing plugin id should fail");

        // Assert
        assert_eq!(error.to_string(), "config show requires a plugin id");
    }

    #[test]
    fn execute_config_show_plugin_rejects_unknown_plugin() {
        // Arrange
        let command = args_en(&["ralph-engine", "config", "show-plugin", "fixture.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown plugin id should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown plugin config: fixture.unknown");
    }

    #[test]
    fn execute_config_without_subcommand_returns_yaml_contract() {
        // Arrange
        let command = args_en(&["ralph-engine", "config"]);

        // Act
        let output = execute(command).expect("config command should succeed");

        // Assert
        assert!(output.contains("mcp:"));
    }

    #[test]
    fn execute_doctor_returns_typed_runtime_report() {
        // Arrange
        let command = args_en(&["ralph-engine", "doctor"]);

        // Act
        let output = execute(command).expect("doctor should succeed");

        // Assert
        assert!(output.contains("Runtime doctor"));
        assert!(output.contains("Runtime health: degraded"));
        assert!(output.contains("Runtime issues ("));
        assert!(output.contains("Runtime action plan ("));
    }

    #[test]
    fn execute_doctor_runtime_returns_typed_runtime_report() {
        // Arrange
        let command = args_en(&["ralph-engine", "doctor", "runtime"]);

        // Act
        let output = execute(command).expect("doctor runtime should succeed");

        // Assert
        assert!(output.contains("Runtime doctor"));
        assert!(output.contains("Runtime issues ("));
    }

    #[test]
    fn execute_doctor_config_returns_merged_project_config() {
        let command = args_en(&["ralph-engine", "doctor", "config"]);

        let output = execute(command).expect("doctor config should succeed");

        assert!(output.contains("schema_version: 1"));
        assert!(output.contains("default_locale: en"));
        assert!(output.contains("- id: official.github"));
        assert!(output.contains("- id: official.github.repository"));
        assert!(output.contains("enabled: true"));
    }

    #[test]
    fn execute_doctor_write_config_persists_yaml() {
        let output_path = std::env::temp_dir().join(format!(
            "ralph-engine-doctor-write-config-{}.yaml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&output_path);
        let output_path_str = output_path.display().to_string();

        let command = args_en(&["ralph-engine", "doctor", "write-config", &output_path_str]);

        let output = execute(command).expect("doctor write-config should succeed");

        assert_eq!(output, format!("Wrote output: {output_path_str}"));
        let persisted = std::fs::read_to_string(&output_path).expect("config file should exist");
        assert!(persisted.contains("schema_version: 1"));
        assert!(persisted.contains("- id: official.github"));

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn execute_doctor_apply_config_persists_yaml() {
        let output_path = std::env::temp_dir().join(format!(
            "ralph-engine-doctor-apply-config-{}.yaml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&output_path);
        let output_path_str = output_path.display().to_string();

        let command = args_en(&["ralph-engine", "doctor", "apply-config", &output_path_str]);

        let output = execute(command).expect("doctor apply-config should succeed");

        assert_eq!(output, format!("Wrote output: {output_path_str}"));
        let persisted = std::fs::read_to_string(&output_path).expect("config file should exist");
        assert!(persisted.contains("schema_version: 1"));
        assert!(persisted.contains("- id: official.github"));

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn execute_unknown_doctor_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "doctor", "plugins"]);

        // Act
        let error = execute(command).expect_err("unknown doctor command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown doctor command: plugins");
    }

    #[test]
    fn execute_unknown_config_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "config", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown config command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown config command: doctor");
    }

    #[test]
    fn execute_mcp_lists_official_servers() {
        // Arrange
        let command = args_en(&["ralph-engine", "mcp", "list"]);
        let servers = catalog::official_mcp_servers();

        // Act
        let output = execute(command).expect("mcp list should succeed");

        // Assert
        assert!(output.contains(&format!("Official MCP servers ({})", servers.len())));
        for server in servers {
            assert!(output.contains(server.id));
        }
    }

    #[test]
    fn execute_mcp_show_returns_server_detail() {
        // Arrange
        let server =
            catalog::find_official_mcp_server(sample_mcp_id()).expect("sample server should exist");
        let command = args_en(&["ralph-engine", "mcp", "show", server.id]);

        // Act
        let output = execute(command).expect("mcp show should succeed");

        // Assert
        assert!(output.contains(&format!("MCP server: {}", server.id)));
        assert!(output.contains(&format!("Name: {}", server.display_name_for_locale("en"))));
        assert!(output.contains(&format!(
            "Process model: {}",
            server.process_model().as_str()
        )));
        assert!(output.contains(&format!("Launch policy: {}", server.launch_policy.as_str())));
        assert!(output.contains(&format!("Availability: {}", server.availability.as_str())));
        if let Some(command) = server.command() {
            assert!(output.contains(&format!("Command: {}", command.render_invocation())));
            assert!(output.contains(&format!(
                "Working directory: {}",
                command.working_directory.as_str()
            )));
            assert!(output.contains(&format!("Environment: {}", command.environment.as_str())));
        }
    }

    #[test]
    fn execute_mcp_plan_returns_launch_plan() {
        let server =
            catalog::find_official_mcp_server(sample_mcp_id()).expect("sample server should exist");
        let command = args_en(&["ralph-engine", "mcp", "plan", server.id]);

        let output = execute(command).expect("mcp plan should succeed");

        assert!(output.contains(&format!("MCP launch plan: {}", server.id)));
        assert!(output.contains(&format!("Plugin: {}", server.plugin_id)));
        assert!(output.contains("Launch step:"));
        if let Some(command) = server.command() {
            assert!(output.contains(&format!("Command: {}", command.render_invocation())));
        } else {
            assert!(output.contains("Command: managed by plugin runtime"));
        }
    }

    #[test]
    fn execute_mcp_show_requires_server_id() {
        // Arrange
        let command = args_en(&["ralph-engine", "mcp", "show"]);

        // Act
        let error = execute(command).expect_err("missing server id should fail");

        // Assert
        assert_eq!(error.to_string(), "mcp show requires a server id");
    }

    #[test]
    fn execute_mcp_show_rejects_unknown_server() {
        // Arrange
        let command = args_en(&["ralph-engine", "mcp", "show", "fixture.unknown"]);

        // Act
        let error = execute(command).expect_err("unknown server id should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown mcp server: fixture.unknown");
    }

    #[test]
    fn execute_mcp_plan_rejects_unknown_server() {
        let command = args_en(&["ralph-engine", "mcp", "plan", "fixture.unknown"]);

        let error = execute(command).expect_err("unknown mcp plan target should fail");

        assert_eq!(error.to_string(), "unknown mcp server: fixture.unknown");
    }

    #[test]
    fn execute_mcp_without_subcommand_lists_official_servers() {
        // Arrange
        let command = args_en(&["ralph-engine", "mcp"]);
        let servers = catalog::official_mcp_servers();

        // Act
        let output = execute(command).expect("mcp command should succeed");

        // Assert
        assert!(output.contains(&format!("Official MCP servers ({})", servers.len())));
    }

    #[test]
    fn execute_runtime_show_returns_resolved_topology() {
        // Arrange
        let command = args_en(&["ralph-engine", "runtime", "show"]);

        // Act
        let output = execute(command).expect("runtime show should succeed");

        // Assert
        assert!(output.contains("Runtime phase: ready"));
        assert!(output.contains("Locale: en"));
        let snapshot = catalog::official_runtime_snapshot();
        assert!(output.contains(&format!("Plugins ({})", snapshot.plugins.len())));
        assert!(output.contains(&format!("Capabilities ({})", snapshot.capabilities.len())));
        assert!(output.contains(&format!("Templates ({})", snapshot.templates.len())));
        assert!(output.contains(&format!("Prompts ({})", snapshot.prompts.len())));
        assert!(output.contains(&format!("Agent runtimes ({})", snapshot.agents.len())));
        assert!(output.contains(&format!("Checks ({})", snapshot.checks.len())));
        assert!(output.contains(&format!("Providers ({})", snapshot.providers.len())));
        assert!(output.contains(&format!("Policies ({})", snapshot.policies.len())));
        assert!(output.contains(&format!("Runtime hooks ({})", snapshot.hooks.len())));
        assert!(output.contains(&format!("MCP servers ({})", snapshot.mcp_servers.len())));
    }

    #[test]
    fn execute_runtime_status_returns_runtime_health_summary() {
        // Arrange
        let command = args_en(&["ralph-engine", "runtime", "status"]);

        // Act
        let output = execute(command).expect("runtime status should succeed");

        // Assert
        assert!(output.contains("Runtime phase: ready"));
        assert!(output.contains("Runtime health: degraded"));
        assert!(output.contains("Plugins:"));
        assert!(output.contains("Capabilities:"));
        assert!(output.contains("Templates:"));
        assert!(output.contains("Prompts:"));
        assert!(output.contains("Agent runtimes:"));
        assert!(output.contains("Checks:"));
        assert!(output.contains("Providers:"));
        assert!(output.contains("Policies:"));
        assert!(output.contains("Runtime hooks:"));
        assert!(output.contains("MCP servers:"));
    }

    #[test]
    fn execute_runtime_issues_returns_typed_issue_summary() {
        // Arrange
        let command = args_en(&["ralph-engine", "runtime", "issues"]);

        // Act
        let output = execute(command).expect("runtime issues should succeed");

        // Assert
        assert!(output.contains("Runtime issues ("));
        assert!(output.contains("plugin_disabled |"));
        assert!(output.contains("template_disabled |"));
        assert!(output.contains("prompt_provider_disabled |"));
        assert!(output.contains("agent_runtime_disabled |"));
        assert!(output.contains("check_disabled |"));
        assert!(output.contains("provider_disabled |"));
        assert!(output.contains("policy_disabled |"));
        assert!(output.contains("hook_disabled |"));
        assert!(output.contains("mcp_server_disabled |"));
    }

    #[test]
    fn execute_runtime_plan_returns_typed_action_plan() {
        // Arrange
        let command = args_en(&["ralph-engine", "runtime", "plan"]);

        // Act
        let output = execute(command).expect("runtime plan should succeed");

        // Assert
        assert!(output.contains("Runtime action plan ("));
        assert!(output.contains("enable_plugin |"));
        assert!(output.contains("enable_template_provider |"));
        assert!(output.contains("enable_prompt_provider |"));
        assert!(output.contains("enable_agent_runtime_provider |"));
        assert!(output.contains("enable_check_provider |"));
        assert!(output.contains("enable_provider |"));
        assert!(output.contains("enable_capability_provider |"));
        assert!(output.contains("enable_policy_provider |"));
        assert!(output.contains("enable_hook_provider |"));
        assert!(output.contains("enable_mcp_server |"));
    }

    #[test]
    fn execute_runtime_agent_plans_returns_bootstrap_plans() {
        let command = args_en(&["ralph-engine", "runtime", "agent-plans"]);

        let output = execute(command).expect("runtime agent-plans should succeed");

        assert_eq!(output, "Runtime agent bootstrap plans (0)");
    }

    #[test]
    fn execute_runtime_provider_plans_returns_registration_plans() {
        let command = args_en(&["ralph-engine", "runtime", "provider-plans"]);

        let output = execute(command).expect("runtime provider-plans should succeed");

        assert_eq!(output, "Runtime provider registration plans (0)");
    }

    #[test]
    fn execute_runtime_check_plans_returns_execution_plans() {
        let command = args_en(&["ralph-engine", "runtime", "check-plans"]);

        let output = execute(command).expect("runtime check-plans should succeed");

        assert_eq!(output, "Runtime check execution plans (0)");
    }

    #[test]
    fn execute_runtime_policy_plans_returns_enforcement_plans() {
        let command = args_en(&["ralph-engine", "runtime", "policy-plans"]);

        let output = execute(command).expect("runtime policy-plans should succeed");

        assert_eq!(output, "Runtime policy enforcement plans (0)");
    }

    #[test]
    fn execute_runtime_mcp_plans_returns_launch_plans() {
        let command = args_en(&["ralph-engine", "runtime", "mcp-plans"]);

        let output = execute(command).expect("runtime mcp-plans should succeed");

        assert_eq!(output, "Runtime MCP launch plans (0)");
    }

    #[test]
    fn execute_runtime_patch_returns_typed_config_patch() {
        let command = args_en(&["ralph-engine", "runtime", "patch"]);

        let output = execute(command).expect("runtime patch should succeed");

        assert!(output.contains("plugins:"));
        assert!(output.contains("- id: official.github"));
        assert!(output.contains("activation: enabled"));
        assert!(output.contains("mcp:"));
        assert!(output.contains("servers:"));
    }

    #[test]
    fn execute_runtime_patched_config_returns_merged_project_config() {
        let command = args_en(&["ralph-engine", "runtime", "patched-config"]);

        let output = execute(command).expect("runtime patched-config should succeed");

        assert!(output.contains("schema_version: 1"));
        assert!(output.contains("default_locale: en"));
        assert!(output.contains("- id: official.basic"));
        assert!(output.contains("- id: official.github"));
        assert!(output.contains("activation: enabled"));
        assert!(output.contains("mcp:"));
        assert!(output.contains("- id: official.github.repository"));
        assert!(output.contains("enabled: true"));
    }

    #[test]
    fn execute_runtime_write_patched_config_persists_yaml() {
        let output_path = std::env::temp_dir().join(format!(
            "ralph-engine-runtime-write-config-{}.yaml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&output_path);
        let output_path_str = output_path.display().to_string();

        let command = args_en(&[
            "ralph-engine",
            "runtime",
            "write-patched-config",
            &output_path_str,
        ]);

        let output = execute(command).expect("runtime write-patched-config should succeed");

        assert_eq!(output, format!("Wrote output: {output_path_str}"));
        let persisted = std::fs::read_to_string(&output_path).expect("config file should exist");
        assert!(persisted.contains("schema_version: 1"));
        assert!(persisted.contains("- id: official.github"));

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn execute_runtime_apply_config_persists_yaml() {
        let output_path = std::env::temp_dir().join(format!(
            "ralph-engine-runtime-apply-config-{}.yaml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&output_path);
        let output_path_str = output_path.display().to_string();

        let command = args_en(&["ralph-engine", "runtime", "apply-config", &output_path_str]);

        let output = execute(command).expect("runtime apply-config should succeed");

        assert_eq!(output, format!("Wrote output: {output_path_str}"));
        let persisted = std::fs::read_to_string(&output_path).expect("config file should exist");
        assert!(persisted.contains("schema_version: 1"));
        assert!(persisted.contains("- id: official.github"));

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn execute_runtime_without_subcommand_returns_resolved_topology() {
        // Arrange
        let command = args_en(&["ralph-engine", "runtime"]);

        // Act
        let output = execute(command).expect("runtime command should succeed");

        // Assert
        assert!(output.contains("Runtime phase: ready"));
    }

    #[test]
    fn execute_config_show_mcp_server_returns_resolved_server_config() {
        let command = args_en(&[
            "ralph-engine",
            "config",
            "show-mcp-server",
            "official.github.repository",
        ]);

        let output = execute(command).expect("config show-mcp-server should succeed");

        assert!(output.contains("id: official.github.repository"));
        assert!(output.contains("enabled: false"));
        assert!(output.contains("resolved_from: built_in_defaults"));
    }

    #[test]
    fn execute_unknown_runtime_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "runtime", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown runtime command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown runtime command: doctor");
    }

    #[test]
    fn execute_capabilities_without_subcommand_lists_runtime_capabilities() {
        // Arrange
        let command = args_en(&["ralph-engine", "capabilities"]);

        // Act
        let output = execute(command).expect("capabilities command should succeed");

        // Assert
        assert!(output.contains("Capabilities ("));
    }

    #[test]
    fn execute_agents_without_subcommand_lists_runtime_agents() {
        // Arrange
        let command = args_en(&["ralph-engine", "agents"]);

        // Act
        let output = execute(command).expect("agents command should succeed");

        // Assert
        assert!(output.contains("Agent runtimes (3)"));
    }

    #[test]
    fn execute_templates_without_subcommand_lists_runtime_templates() {
        // Arrange
        let command = args_en(&["ralph-engine", "templates"]);

        // Act
        let output = execute(command).expect("templates command should succeed");

        // Assert
        let count = crate::catalog::official_runtime_templates().len();
        assert!(output.contains(&format!("Templates ({count})")));
    }

    #[test]
    fn execute_prompts_without_subcommand_lists_runtime_prompts() {
        // Arrange
        let command = args_en(&["ralph-engine", "prompts"]);

        // Act
        let output = execute(command).expect("prompts command should succeed");

        // Assert
        assert!(output.contains("Prompts ("));
    }

    #[test]
    fn execute_checks_without_subcommand_lists_runtime_checks() {
        // Arrange
        let command = args_en(&["ralph-engine", "checks"]);

        // Act
        let output = execute(command).expect("checks command should succeed");

        // Assert
        assert!(output.contains("Checks (2)"));
    }

    #[test]
    fn execute_hooks_without_subcommand_lists_runtime_hooks() {
        // Arrange
        let command = args_en(&["ralph-engine", "hooks"]);

        // Act
        let output = execute(command).expect("hooks command should succeed");

        // Assert
        assert!(output.contains("Runtime hooks (13)"));
    }

    #[test]
    fn execute_policies_without_subcommand_lists_runtime_policies() {
        // Arrange
        let command = args_en(&["ralph-engine", "policies"]);

        // Act
        let output = execute(command).expect("policies command should succeed");

        // Assert
        assert!(output.contains("Policies (1)"));
    }

    #[test]
    fn execute_providers_without_subcommand_lists_runtime_providers() {
        // Arrange
        let command = args_en(&["ralph-engine", "providers"]);

        // Act
        let output = execute(command).expect("providers command should succeed");

        // Assert
        assert!(output.contains("Providers (4)"));
    }

    #[test]
    fn execute_unknown_capabilities_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "capabilities", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown capabilities command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown capabilities command: doctor");
    }

    #[test]
    fn execute_unknown_agents_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "agents", "runtime"]);

        // Act
        let error = execute(command).expect_err("unknown agents command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown agents command: runtime");
    }

    #[test]
    fn execute_unknown_templates_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "templates", "runtime"]);

        // Act
        let error = execute(command).expect_err("unknown templates command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown templates command: runtime");
    }

    #[test]
    fn execute_unknown_prompts_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "prompts", "runtime"]);

        // Act
        let error = execute(command).expect_err("unknown prompts command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown prompts command: runtime");
    }

    #[test]
    fn execute_unknown_checks_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "checks", "runtime"]);

        // Act
        let error = execute(command).expect_err("unknown checks command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown checks command: runtime");
    }

    #[test]
    fn execute_unknown_hooks_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "hooks", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown hooks command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown hooks command: doctor");
    }

    #[test]
    fn execute_unknown_policies_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "policies", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown policies command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown policies command: doctor");
    }

    #[test]
    fn execute_unknown_providers_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "providers", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown providers command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown providers command: doctor");
    }

    #[test]
    fn execute_unknown_mcp_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "mcp", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown mcp command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown mcp command: doctor");
    }

    #[test]
    fn execute_plugins_without_subcommand_lists_official_plugins() {
        // Arrange
        let command = args_en(&["ralph-engine", "plugins"]);

        // Act
        let output = execute(command).expect("plugins command should succeed");

        // Assert
        let count = crate::catalog::official_plugins().len();
        assert!(output.contains(&format!("Official plugins ({count})")));
    }

    #[test]
    fn execute_unknown_plugins_subcommand_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "plugins", "doctor"]);

        // Act
        let error = execute(command).expect_err("unknown plugins command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown plugins command: doctor");
    }

    #[test]
    fn execute_unknown_command_fails() {
        // Arrange
        let command = args_en(&["ralph-engine", "unknown"]);

        // Act
        let error = execute(command).expect_err("unknown command should fail");

        // Assert
        assert_eq!(error.to_string(), "unknown command: unknown");
    }
}
