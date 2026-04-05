//! Runtime-hook command handlers.

use re_core::{RuntimeCheckKind, RuntimeHookRegistration, RuntimeProviderKind};
use re_plugin::parse_plugin_runtime_hook;

use super::format;
use crate::{CliError, catalog, i18n};

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
    let mut seen = Vec::new();
    let mut grouped_rows: Vec<Vec<String>> = Vec::new();

    for reg in registrations {
        let key = reg.hook.as_str();
        if seen.contains(&key) {
            continue;
        }
        seen.push(key);

        let all = registrations
            .iter()
            .filter(|r| r.hook.as_str() == key)
            .collect::<Vec<_>>();
        let enabled = all.iter().filter(|r| r.is_enabled()).count();

        grouped_rows.push(vec![
            key.to_owned(),
            all.len().to_string(),
            enabled.to_string(),
        ]);
    }

    let label = i18n::hooks_label(locale);
    let heading = i18n::list_heading(locale, label, label, grouped_rows.len());

    if grouped_rows.is_empty() {
        return heading;
    }

    let headers = &["HOOK", "PROVIDERS", "ENABLED"];
    format!(
        "{heading}\n\n{}",
        format::render_table(headers, &grouped_rows)
    )
}

fn render_hook_detail(
    hook_id: &str,
    providers: &[RuntimeHookRegistration],
    locale: &str,
) -> String {
    let label = i18n::hook_label(locale);
    let heading = i18n::detail_heading(locale, label, label, hook_id);
    let providers_heading = i18n::providers_heading(locale, providers.len());

    let headers = &["PLUGIN", "STATUS", "BOUNDARY"];
    let rows: Vec<Vec<String>> = providers
        .iter()
        .map(|p| {
            vec![
                p.plugin_id.to_owned(),
                p.activation.as_str().to_owned(),
                p.load_boundary.as_str().to_owned(),
            ]
        })
        .collect();

    format!(
        "{heading}\n{providers_heading}\n\n{}",
        format::render_table(headers, &rows)
    )
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
            let headers = &["ID", "PLUGIN", "STATUS", "HOOK"];
            let rows: Vec<Vec<String>> = templates
                .iter()
                .map(|t| {
                    vec![
                        t.descriptor.id.to_owned(),
                        t.descriptor.plugin_id.to_owned(),
                        t.activation.as_str().to_owned(),
                        t.scaffold_hook_registered.to_string(),
                    ]
                })
                .collect();
            if !rows.is_empty() {
                lines.push(format::render_table(headers, &rows));
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
            let headers = &["ID", "PLUGIN", "STATUS", "HOOK"];
            let rows: Vec<Vec<String>> = prompts
                .iter()
                .map(|p| {
                    vec![
                        p.descriptor.id.to_owned(),
                        p.descriptor.plugin_id.to_owned(),
                        p.activation.as_str().to_owned(),
                        p.prompt_hook_registered.to_string(),
                    ]
                })
                .collect();
            if !rows.is_empty() {
                lines.push(format::render_table(headers, &rows));
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
            let headers = &["ID", "PLUGIN", "STATUS", "HOOK"];
            let rows: Vec<Vec<String>> = agents
                .iter()
                .map(|a| {
                    vec![
                        a.descriptor.id.to_owned(),
                        a.descriptor.plugin_id.to_owned(),
                        a.activation.as_str().to_owned(),
                        a.bootstrap_hook_registered.to_string(),
                    ]
                })
                .collect();
            if !rows.is_empty() {
                lines.push(format::render_table(headers, &rows));
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
            let headers = &["ID", "PLUGIN", "STATUS", "HOOK"];
            let rows: Vec<Vec<String>> = policies
                .iter()
                .map(|p| {
                    vec![
                        p.descriptor.id.to_owned(),
                        p.descriptor.plugin_id.to_owned(),
                        p.activation.as_str().to_owned(),
                        p.enforcement_hook_registered.to_string(),
                    ]
                })
                .collect();
            if !rows.is_empty() {
                lines.push(format::render_table(headers, &rows));
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
            let headers = &["ID", "ENABLED", "TRANSPORT"];
            let rows: Vec<Vec<String>> = servers
                .iter()
                .map(|s| {
                    vec![
                        s.descriptor.id.to_owned(),
                        s.enabled.to_string(),
                        s.descriptor.transport.to_string(),
                    ]
                })
                .collect();
            if !rows.is_empty() {
                lines.push(format::render_table(headers, &rows));
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
    let headers = &["ID", "PLUGIN", "STATUS", "HOOK"];
    let rows: Vec<Vec<String>> = checks
        .iter()
        .map(|c| {
            vec![
                c.descriptor.id.to_owned(),
                c.descriptor.plugin_id.to_owned(),
                c.activation.as_str().to_owned(),
                c.runtime_hook_registered.to_string(),
            ]
        })
        .collect();
    if !rows.is_empty() {
        lines.push(format::render_table(headers, &rows));
    }
}

fn render_provider_section(lines: &mut Vec<String>, kind: RuntimeProviderKind) {
    let providers = catalog::find_official_provider_contributions(kind);
    lines.push(format!("Providers ({})", providers.len()));
    let headers = &["ID", "PLUGIN", "STATUS", "HOOK"];
    let rows: Vec<Vec<String>> = providers
        .iter()
        .map(|p| {
            vec![
                p.descriptor.id.to_owned(),
                p.descriptor.plugin_id.to_owned(),
                p.activation.as_str().to_owned(),
                p.registration_hook_registered.to_string(),
            ]
        })
        .collect();
    if !rows.is_empty() {
        lines.push(format::render_table(headers, &rows));
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
        let registrations = [];

        let rendered = render_hook_listing(&registrations, "en");

        assert!(rendered.contains("Runtime hooks (0)"));
    }

    #[test]
    fn render_hook_detail_is_human_readable() {
        let providers = [RuntimeHookRegistration::new(
            PluginRuntimeHook::Scaffold,
            PLUGIN_ID,
            PluginActivation::Enabled,
            PluginLoadBoundary::InProcess,
        )];

        let rendered = render_hook_detail("scaffold", &providers, "en");

        assert!(rendered.contains("Runtime hook: scaffold"));
        assert!(rendered.contains("Providers (1)"));
        assert!(rendered.contains("fixture.templates"));
        assert!(rendered.contains("enabled"));
        assert!(rendered.contains("in_process"));
    }

    #[test]
    fn render_hook_plan_lists_owned_surfaces() {
        let rendered = render_hook_plan(PluginRuntimeHook::AgentBootstrap, "en");

        assert!(rendered.contains("Runtime hook plan: agent_bootstrap"));
        assert!(rendered.contains("Agent runtimes ("));
        // Table should contain hook column with true/false values
        assert!(rendered.contains("HOOK"));
    }
}
