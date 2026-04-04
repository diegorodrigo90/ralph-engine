//! Runtime-hook command handlers.

use re_core::{RuntimeCheckKind, RuntimeHookRegistration, RuntimeProviderKind};
use re_plugin::parse_plugin_runtime_hook;

use crate::{
    CliError, catalog,
    commands::grouped_surfaces::{render_grouped_surface_detail, render_grouped_surface_listing},
    i18n,
};

/// Executes the hooks command tree.
pub fn execute(args: &[String], locale: &str) -> Result<String, CliError> {
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(render_hook_listing(
            &catalog::official_runtime_hooks(),
            locale,
        )),
        Some("show") => show_hook(args.get(1).map(String::as_str), locale),
        Some("plan") => show_hook_plan(args.get(1).map(String::as_str), locale),
        Some(other) => Err(CliError::new(i18n::unknown_subcommand(
            locale, "hooks", other,
        ))),
    }
}

fn show_hook(hook_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let hook_id = hook_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "hooks",
            i18n::hook_id_entity_label(locale),
        ))
    })?;
    let hook = parse_plugin_runtime_hook(hook_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::hook_entity_label(locale),
            hook_id,
        ))
    })?;
    let providers = catalog::find_official_runtime_hooks(hook);

    Ok(render_hook_detail(hook_id, &providers, locale))
}

fn show_hook_plan(hook_id: Option<&str>, locale: &str) -> Result<String, CliError> {
    let hook_id = hook_id.ok_or_else(|| {
        CliError::new(i18n::missing_id(
            locale,
            "hooks",
            i18n::hook_id_entity_label(locale),
        ))
    })?;
    let hook = parse_plugin_runtime_hook(hook_id).ok_or_else(|| {
        CliError::new(i18n::unknown_entity(
            locale,
            i18n::hook_entity_label(locale),
            hook_id,
        ))
    })?;

    Ok(render_hook_plan(hook, locale))
}

fn render_hook_listing(registrations: &[RuntimeHookRegistration], locale: &str) -> String {
    render_grouped_surface_listing(
        registrations,
        locale,
        i18n::hooks_label,
        |registration| registration.hook.as_str(),
        |registration| registration.is_enabled(),
    )
}

fn render_hook_detail(
    hook_id: &str,
    providers: &[RuntimeHookRegistration],
    locale: &str,
) -> String {
    render_grouped_surface_detail(hook_id, providers, locale, i18n::hook_label, |provider| {
        format!(
            "- {} | activation={} | boundary={}",
            provider.plugin_id,
            provider.activation.as_str(),
            provider.load_boundary.as_str()
        )
    })
}

fn render_hook_plan(hook: re_plugin::PluginRuntimeHook, locale: &str) -> String {
    let mut lines = vec![format!(
        "{}: {}",
        i18n::hooks_plan_heading(locale),
        hook.as_str()
    )];

    match hook {
        re_plugin::PluginRuntimeHook::Scaffold => {
            let templates = catalog::official_template_contributions();
            lines.push(i18n::list_heading(
                locale,
                "Templates",
                "Templates",
                templates.len(),
            ));
            for template in templates {
                lines.push(format!(
                    "- {} | plugin={} | activation={} | scaffold_hook={}",
                    template.descriptor.id,
                    template.descriptor.plugin_id,
                    template.activation.as_str(),
                    template.scaffold_hook_registered,
                ));
            }
        }
        re_plugin::PluginRuntimeHook::PromptAssembly => {
            let prompts = catalog::official_prompt_contributions();
            lines.push(i18n::list_heading(
                locale,
                "Prompts",
                "Prompts",
                prompts.len(),
            ));
            for prompt in prompts {
                lines.push(format!(
                    "- {} | plugin={} | activation={} | prompt_hook={}",
                    prompt.descriptor.id,
                    prompt.descriptor.plugin_id,
                    prompt.activation.as_str(),
                    prompt.prompt_hook_registered,
                ));
            }
        }
        re_plugin::PluginRuntimeHook::AgentBootstrap => {
            let agents = catalog::official_agent_contributions();
            lines.push(i18n::list_heading(
                locale,
                "Agent runtimes",
                "Runtimes de agente",
                agents.len(),
            ));
            for agent in agents {
                lines.push(format!(
                    "- {} | plugin={} | activation={} | bootstrap_hook={}",
                    agent.descriptor.id,
                    agent.descriptor.plugin_id,
                    agent.activation.as_str(),
                    agent.bootstrap_hook_registered,
                ));
            }
        }
        re_plugin::PluginRuntimeHook::Prepare => {
            render_check_section(&mut lines, RuntimeCheckKind::Prepare);
        }
        re_plugin::PluginRuntimeHook::Doctor => {
            render_check_section(&mut lines, RuntimeCheckKind::Doctor);
        }
        re_plugin::PluginRuntimeHook::DataSourceRegistration => {
            render_provider_section(&mut lines, RuntimeProviderKind::DataSource);
        }
        re_plugin::PluginRuntimeHook::ContextProviderRegistration => {
            render_provider_section(&mut lines, RuntimeProviderKind::ContextProvider);
        }
        re_plugin::PluginRuntimeHook::ForgeProviderRegistration => {
            render_provider_section(&mut lines, RuntimeProviderKind::ForgeProvider);
        }
        re_plugin::PluginRuntimeHook::RemoteControlBootstrap => {
            render_provider_section(&mut lines, RuntimeProviderKind::RemoteControl);
        }
        re_plugin::PluginRuntimeHook::PolicyEnforcement => {
            let policies = catalog::official_policy_contributions();
            lines.push(i18n::list_heading(
                locale,
                "Policies",
                "Políticas",
                policies.len(),
            ));
            for policy in policies {
                lines.push(format!(
                    "- {} | plugin={} | activation={} | enforcement_hook={}",
                    policy.descriptor.id,
                    policy.descriptor.plugin_id,
                    policy.activation.as_str(),
                    policy.enforcement_hook_registered,
                ));
            }
        }
        re_plugin::PluginRuntimeHook::McpRegistration => {
            let servers = catalog::official_runtime_mcp_registrations();
            lines.push(i18n::list_heading(
                locale,
                "MCP servers",
                "Servidores MCP",
                servers.len(),
            ));
            for server in servers {
                lines.push(format!(
                    "- {} | enabled={} | transport={}",
                    server.descriptor.id, server.enabled, server.descriptor.transport,
                ));
            }
        }
        _ => {
            lines.push(format!("(unknown hook: {})", hook.as_str()));
        }
    }

    lines.join("\n")
}

fn render_check_section(lines: &mut Vec<String>, kind: RuntimeCheckKind) {
    let checks = catalog::find_official_check_contributions(kind);
    lines.push(format!("Checks ({})", checks.len()));
    for check in checks {
        lines.push(format!(
            "- {} | plugin={} | activation={} | runtime_hook={}",
            check.descriptor.id,
            check.descriptor.plugin_id,
            check.activation.as_str(),
            check.runtime_hook_registered,
        ));
    }
}

fn render_provider_section(lines: &mut Vec<String>, kind: RuntimeProviderKind) {
    let providers = catalog::find_official_provider_contributions(kind);
    lines.push(format!("Providers ({})", providers.len()));
    for provider in providers {
        lines.push(format!(
            "- {} | plugin={} | activation={} | registration_hook={}",
            provider.descriptor.id,
            provider.descriptor.plugin_id,
            provider.activation.as_str(),
            provider.registration_hook_registered,
        ));
    }
}

#[cfg(test)]
mod tests {
    use re_config::PluginActivation;
    use re_core::RuntimeHookRegistration;
    use re_plugin::{PluginLoadBoundary, PluginRuntimeHook};

    use super::{render_hook_detail, render_hook_listing, render_hook_plan};

    const PLUGIN_ID: &str = "fixture.templates";

    #[test]
    fn render_hook_listing_handles_empty_sets() {
        // Arrange
        let registrations = [];

        // Act
        let rendered = render_hook_listing(&registrations, "en");

        // Assert
        assert_eq!(rendered, "Runtime hooks (0)");
    }

    #[test]
    fn render_hook_detail_is_human_readable() {
        // Arrange
        let providers = [RuntimeHookRegistration::new(
            PluginRuntimeHook::Scaffold,
            PLUGIN_ID,
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
        )];

        // Act
        let rendered = render_hook_detail("scaffold", &providers, "en");

        // Assert
        assert!(rendered.contains("Runtime hook: scaffold"));
        assert!(rendered.contains("Providers (1)"));
        assert!(
            rendered.contains("- fixture.templates | activation=enabled | boundary=in_process")
        );
    }

    #[test]
    fn render_hook_plan_lists_owned_surfaces() {
        let rendered = render_hook_plan(PluginRuntimeHook::AgentBootstrap, "en");

        assert!(rendered.contains("Runtime hook plan: agent_bootstrap"));
        assert!(rendered.contains("Agent runtimes ("));
        assert!(rendered.contains("bootstrap_hook="));
    }
}
