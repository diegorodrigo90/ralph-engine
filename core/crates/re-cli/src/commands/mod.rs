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
mod grouped_surfaces;
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
    subcommands: &'static [&'static str],
    handler: fn(&[String], &str) -> Result<String, CliError>,
}

const COMMANDS: &[CommandDescriptor] = &[
    CommandDescriptor {
        name: "agents",
        subcommands: &["list", "show", "plan", "launch"],
        handler: agents::execute,
    },
    CommandDescriptor {
        name: "capabilities",
        subcommands: &["list", "show"],
        handler: capabilities::execute,
    },
    CommandDescriptor {
        name: "checks",
        subcommands: &["list", "show", "plan", "run", "asset", "materialize"],
        handler: checks::execute,
    },
    CommandDescriptor {
        name: "config",
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
        name: "doctor",
        subcommands: &["config", "apply-config"],
        handler: doctor::execute,
    },
    CommandDescriptor {
        name: "hooks",
        subcommands: &["list", "show", "plan"],
        handler: hooks::execute,
    },
    CommandDescriptor {
        name: "init",
        subcommands: &[],
        handler: init::execute,
    },
    CommandDescriptor {
        name: "install",
        subcommands: &[],
        handler: install::execute,
    },
    CommandDescriptor {
        name: "locales",
        subcommands: &["list", "show"],
        handler: locales::execute,
    },
    CommandDescriptor {
        name: "mcp",
        subcommands: &["list", "show", "plan", "launch", "status"],
        handler: mcp::execute,
    },
    CommandDescriptor {
        name: "policies",
        subcommands: &["list", "show", "plan", "run", "asset", "materialize"],
        handler: policies::execute,
    },
    CommandDescriptor {
        name: "prompts",
        subcommands: &["list", "show", "asset", "materialize"],
        handler: prompts::execute,
    },
    CommandDescriptor {
        name: "providers",
        subcommands: &["list", "show", "plan"],
        handler: providers::execute,
    },
    CommandDescriptor {
        name: "plugins",
        subcommands: &["list", "show"],
        handler: plugins::execute,
    },
    CommandDescriptor {
        name: "run",
        subcommands: &["plan"],
        handler: run::execute,
    },
    CommandDescriptor {
        name: "runtime",
        subcommands: &["show", "status", "issues"],
        handler: runtime::execute,
    },
    CommandDescriptor {
        name: "templates",
        subcommands: &["list", "show", "asset", "scaffold", "materialize"],
        handler: templates::execute,
    },
    CommandDescriptor {
        name: "tui",
        subcommands: &[],
        handler: tui::execute,
    },
    CommandDescriptor {
        name: "uninstall",
        subcommands: &[],
        handler: install::execute_uninstall,
    },
];

/// Executes the CLI command tree from collected process arguments.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    let invocation = i18n::resolve_cli_invocation(args)?;
    let locale = invocation.locale;

    match args.get(invocation.command_index).map(String::as_str) {
        None => Ok(format!(
            "{}\n\n{}",
            re_core::banner(),
            i18n::root_bootstrapped(locale)
        )),
        Some("--version" | "-V") => Ok(env!("CARGO_PKG_VERSION").to_owned()),
        Some("--help" | "-h") => Ok(render_help(locale)),
        Some(command_name) => dispatch_command(
            command_name,
            &args[(invocation.command_index + 1)..],
            locale,
        ),
    }
}

fn render_help(locale: &str) -> String {
    let mut lines = vec![
        re_core::banner(),
        String::new(),
        i18n::usage_help(locale).to_owned(),
        String::new(),
        i18n::commands_heading(locale).to_owned(),
    ];

    for command in COMMANDS {
        lines.push(format!("  {}", command.name));
    }

    lines.push(String::new());
    lines.push("Flags:".to_owned());
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

fn dispatch_command(command_name: &str, args: &[String], locale: &str) -> Result<String, CliError> {
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

    Err(CliError::new(i18n::unknown_command(locale, command_name)))
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
