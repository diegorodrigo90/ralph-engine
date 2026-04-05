//! Modular CLI command routing.

use crate::{CliError, catalog, i18n};

// All locale strings are now in TOML catalogs (locales/*.toml).
// The locale_str! macro has been removed — use i18n::* accessors instead.

/// Standard status markers used in command output.
const STATUS_OK: &str = "[OK]";
const STATUS_NOT_READY: &str = "[NOT READY]";
const STATUS_MISSING: &str = "[MISSING]";
const STATUS_UNSUPPORTED: &str = "[UNSUPPORTED]";

mod agents;
mod capabilities;
mod checks;
mod config;
mod doctor;
mod embedded_assets;
pub(crate) mod format;
mod hooks;
mod init;
mod install;
mod locales;
mod mcp;
mod plugins;
mod policies;
mod prompts;
mod providers;
mod run;
mod runtime;
pub(crate) mod runtime_state;
mod templates;
mod tui;

struct CommandDescriptor {
    name: &'static str,
    /// i18n key for the command description (resolved at render time).
    description_key: &'static str,
    subcommands: &'static [&'static str],
    handler: fn(&[String], &str) -> Result<String, CliError>,
}

const COMMANDS: &[CommandDescriptor] = &[
    CommandDescriptor {
        name: "run",
        description_key: "cmd_run",
        subcommands: &["plan"],
        handler: run::execute,
    },
    CommandDescriptor {
        name: "exec",
        description_key: "cmd_exec",
        subcommands: &["plan"],
        handler: run::execute_headless,
    },
    CommandDescriptor {
        name: "tui",
        description_key: "cmd_tui",
        subcommands: &[],
        handler: tui::execute,
    },
    CommandDescriptor {
        name: "init",
        description_key: "cmd_init",
        subcommands: &[],
        handler: init::execute,
    },
    CommandDescriptor {
        name: "doctor",
        description_key: "cmd_doctor",
        subcommands: &["config", "apply-config"],
        handler: doctor::execute,
    },
    CommandDescriptor {
        name: "plugins",
        description_key: "cmd_plugins",
        subcommands: &["list", "show"],
        handler: plugins::execute,
    },
    CommandDescriptor {
        name: "install",
        description_key: "cmd_install",
        subcommands: &[],
        handler: install::execute,
    },
    CommandDescriptor {
        name: "uninstall",
        description_key: "cmd_uninstall",
        subcommands: &[],
        handler: install::execute_uninstall,
    },
    CommandDescriptor {
        name: "agents",
        description_key: "cmd_agents",
        subcommands: &["list", "show", "plan", "launch"],
        handler: agents::execute,
    },
    CommandDescriptor {
        name: "mcp",
        description_key: "cmd_mcp",
        subcommands: &["list", "show", "plan", "launch", "status"],
        handler: mcp::execute,
    },
    CommandDescriptor {
        name: "checks",
        description_key: "cmd_checks",
        subcommands: &["list", "show", "plan", "run", "asset", "materialize"],
        handler: checks::execute,
    },
    CommandDescriptor {
        name: "templates",
        description_key: "cmd_templates",
        subcommands: &["list", "show", "asset", "scaffold", "materialize"],
        handler: templates::execute,
    },
    CommandDescriptor {
        name: "prompts",
        description_key: "cmd_prompts",
        subcommands: &["list", "show", "asset", "materialize"],
        handler: prompts::execute,
    },
    CommandDescriptor {
        name: "policies",
        description_key: "cmd_policies",
        subcommands: &["list", "show", "plan", "run", "asset", "materialize"],
        handler: policies::execute,
    },
    CommandDescriptor {
        name: "hooks",
        description_key: "cmd_hooks",
        subcommands: &["list", "show", "plan"],
        handler: hooks::execute,
    },
    CommandDescriptor {
        name: "config",
        description_key: "cmd_config",
        subcommands: &[
            "show-defaults",
            "locale",
            "budgets",
            "layers",
            "show-plugin",
            "show-mcp-server",
        ],
        handler: config::execute,
    },
    CommandDescriptor {
        name: "runtime",
        description_key: "cmd_runtime",
        subcommands: &["show", "status", "issues"],
        handler: runtime::execute,
    },
    CommandDescriptor {
        name: "capabilities",
        description_key: "cmd_capabilities",
        subcommands: &["list", "show"],
        handler: capabilities::execute,
    },
    CommandDescriptor {
        name: "providers",
        description_key: "cmd_providers",
        subcommands: &["list", "show", "plan"],
        handler: providers::execute,
    },
    CommandDescriptor {
        name: "locales",
        description_key: "cmd_locales",
        subcommands: &["list", "show"],
        handler: locales::execute,
    },
];

/// Executes the CLI command tree from collected process arguments.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    let invocation = i18n::resolve_cli_invocation(args)?;
    let locale = invocation.locale;

    // --demo flag: launch TUI with demo mode (debug builds only)
    #[cfg(debug_assertions)]
    if args.iter().any(|a| a == "--demo") {
        return tui::execute(&["--demo".to_owned()], locale);
    }

    match args.get(invocation.command_index).map(String::as_str) {
        None => dispatch_default(locale),
        Some("--version" | "-V") => Ok(env!("CARGO_PKG_VERSION").to_owned()),
        Some("--help" | "-h") => Ok(render_help(locale)),
        Some(command_name) => dispatch_command(
            command_name,
            &args[(invocation.command_index + 1)..],
            locale,
        ),
    }
}

/// Resolves a command description key to the localized string.
fn resolve_command_description(key: &'static str, locale: &str) -> &'static str {
    match key {
        "cmd_run" => i18n::cmd_run(locale),
        "cmd_exec" => i18n::cmd_exec(locale),
        "cmd_tui" => i18n::cmd_tui(locale),
        "cmd_init" => i18n::cmd_init(locale),
        "cmd_doctor" => i18n::cmd_doctor(locale),
        "cmd_plugins" => i18n::cmd_plugins(locale),
        "cmd_install" => i18n::cmd_install(locale),
        "cmd_uninstall" => i18n::cmd_uninstall(locale),
        "cmd_agents" => i18n::cmd_agents(locale),
        "cmd_mcp" => i18n::cmd_mcp(locale),
        "cmd_checks" => i18n::cmd_checks(locale),
        "cmd_templates" => i18n::cmd_templates(locale),
        "cmd_prompts" => i18n::cmd_prompts(locale),
        "cmd_policies" => i18n::cmd_policies(locale),
        "cmd_hooks" => i18n::cmd_hooks(locale),
        "cmd_config" => i18n::cmd_config(locale),
        "cmd_runtime" => i18n::cmd_runtime(locale),
        "cmd_capabilities" => i18n::cmd_capabilities(locale),
        "cmd_providers" => i18n::cmd_providers(locale),
        "cmd_locales" => i18n::cmd_locales(locale),
        _ => key,
    }
}

/// Default when no command is given — show help.
///
/// Future: smart dispatch based on .ralph-engine/ presence
/// (chat TUI if configured, suggest init if not).
fn dispatch_default(locale: &str) -> Result<String, CliError> {
    // Launch TUI dashboard when no command given (the product experience).
    // Falls back to help text if --no-tui flag or non-interactive terminal.
    if std::io::IsTerminal::is_terminal(&std::io::stdout()) {
        tui::execute(&[], locale)
    } else {
        Ok(render_help(locale))
    }
}

fn render_help(locale: &str) -> String {
    let mut lines = vec![
        re_core::banner_with_tagline(i18n::product_tagline(locale)),
        String::new(),
        i18n::usage_help(locale).to_owned(),
        String::new(),
        i18n::commands_heading(locale).to_owned(),
    ];

    // Find max command name length for alignment
    let max_len = COMMANDS.iter().map(|c| c.name.len()).max().unwrap_or(12);

    for command in COMMANDS {
        let padding = " ".repeat(max_len - command.name.len() + 2);
        let desc = resolve_command_description(command.description_key, locale);
        lines.push(format!("  {}{}{desc}", command.name, padding));
    }

    // Plugin-contributed commands (auto-discovered)
    let plugin_cmds = catalog::collect_cli_contributions_from_plugins();
    if !plugin_cmds.is_empty() {
        lines.push(String::new());
        lines.push(format!("  {} (plugins):", i18n::commands_heading(locale)));
        for (_plugin_id, contrib) in &plugin_cmds {
            let padding = " ".repeat(max_len.saturating_sub(contrib.name.len()) + 2);
            lines.push(format!(
                "  {}{}{}",
                contrib.name, padding, contrib.description
            ));
        }
    }

    lines.push(String::new());
    lines.push(i18n::flags_heading(locale).to_owned());
    lines.push(format!(
        "  --locale <id>, -L <id>   {}",
        i18n::set_locale_help(locale)
    ));
    lines.push(format!(
        "  --version, -V            {}",
        i18n::show_version_help(locale)
    ));
    lines.push(format!(
        "  --help, -h               {}",
        i18n::show_help_help(locale)
    ));

    lines.join("\n")
}

pub(crate) fn dispatch_command(
    command_name: &str,
    args: &[String],
    locale: &str,
) -> Result<String, CliError> {
    // 1. Try static core commands first.
    if let Some(command) = COMMANDS.iter().find(|command| command.name == command_name) {
        if matches!(args.first().map(String::as_str), Some("--help" | "-h")) {
            return Ok(render_command_help(command, locale));
        }
        return (command.handler)(args, locale);
    }

    // 2. Try plugin-contributed subcommands (auto-discovered).
    let plugin_commands = catalog::collect_cli_contributions_from_plugins();
    if let Some((plugin_id, _contrib)) =
        plugin_commands.iter().find(|(_, c)| c.name == command_name)
    {
        // Plugin commands are dispatched to the plugin runtime.
        if let Some(runtime) = catalog::official_plugin_runtime(plugin_id) {
            // Pass the command name + remaining args to the plugin.
            let mut full_args = vec![command_name.to_owned()];
            full_args.extend(args.iter().cloned());
            return runtime
                .handle_cli_command(command_name, args)
                .map_err(|e| CliError::new(e.to_string()));
        }
    }

    Err(CliError::usage(i18n::unknown_command(locale, command_name)))
}

fn render_command_help(command: &CommandDescriptor, locale: &str) -> String {
    let sub_label = i18n::subcommand_label(locale);
    let subs_label = i18n::subcommands_heading(locale);
    let usage_prefix = i18n::usage_label(locale);

    let mut lines = vec![
        format!(
            "{usage_prefix}: ralph-engine {} <{sub_label}> [arguments]",
            command.name
        ),
        String::new(),
        subs_label.to_owned(),
    ];

    for sub in command.subcommands {
        lines.push(format!("  {sub}"));
    }

    lines.join("\n")
}
