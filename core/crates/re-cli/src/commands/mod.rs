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
    description: &'static str,
    subcommands: &'static [&'static str],
    handler: fn(&[String], &str) -> Result<String, CliError>,
}

const COMMANDS: &[CommandDescriptor] = &[
    CommandDescriptor {
        name: "run",
        description: "Execute work items with TUI dashboard",
        subcommands: &["plan"],
        handler: run::execute,
    },
    CommandDescriptor {
        name: "tui",
        description: "Launch TUI demo with simulated events",
        subcommands: &[],
        handler: tui::execute,
    },
    CommandDescriptor {
        name: "init",
        description: "Initialize a new project (.ralph-engine/)",
        subcommands: &[],
        handler: init::execute,
    },
    CommandDescriptor {
        name: "doctor",
        description: "Check project health and fix issues",
        subcommands: &["config", "apply-config"],
        handler: doctor::execute,
    },
    CommandDescriptor {
        name: "plugins",
        description: "List and inspect installed plugins",
        subcommands: &["list", "show"],
        handler: plugins::execute,
    },
    CommandDescriptor {
        name: "install",
        description: "Install a community plugin",
        subcommands: &[],
        handler: install::execute,
    },
    CommandDescriptor {
        name: "uninstall",
        description: "Remove an installed plugin",
        subcommands: &[],
        handler: install::execute_uninstall,
    },
    CommandDescriptor {
        name: "agents",
        description: "List and launch agent runtimes",
        subcommands: &["list", "show", "plan", "launch"],
        handler: agents::execute,
    },
    CommandDescriptor {
        name: "mcp",
        description: "List and manage MCP servers",
        subcommands: &["list", "show", "plan", "launch", "status"],
        handler: mcp::execute,
    },
    CommandDescriptor {
        name: "checks",
        description: "Run prepare and doctor checks",
        subcommands: &["list", "show", "plan", "run", "asset", "materialize"],
        handler: checks::execute,
    },
    CommandDescriptor {
        name: "templates",
        description: "List and scaffold project templates",
        subcommands: &["list", "show", "asset", "scaffold", "materialize"],
        handler: templates::execute,
    },
    CommandDescriptor {
        name: "prompts",
        description: "List and inspect prompt contributions",
        subcommands: &["list", "show", "asset", "materialize"],
        handler: prompts::execute,
    },
    CommandDescriptor {
        name: "policies",
        description: "List and enforce policy rules",
        subcommands: &["list", "show", "plan", "run", "asset", "materialize"],
        handler: policies::execute,
    },
    CommandDescriptor {
        name: "hooks",
        description: "List runtime hooks and surfaces",
        subcommands: &["list", "show", "plan"],
        handler: hooks::execute,
    },
    CommandDescriptor {
        name: "config",
        description: "Show configuration and defaults",
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
        description: "Inspect runtime state and topology",
        subcommands: &["show", "status", "issues"],
        handler: runtime::execute,
    },
    CommandDescriptor {
        name: "capabilities",
        description: "List plugin capabilities",
        subcommands: &["list", "show"],
        handler: capabilities::execute,
    },
    CommandDescriptor {
        name: "providers",
        description: "List data/context/forge providers",
        subcommands: &["list", "show", "plan"],
        handler: providers::execute,
    },
    CommandDescriptor {
        name: "locales",
        description: "List supported locales",
        subcommands: &["list", "show"],
        handler: locales::execute,
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

    // Find max command name length for alignment
    let max_len = COMMANDS.iter().map(|c| c.name.len()).max().unwrap_or(12);

    for command in COMMANDS {
        let padding = " ".repeat(max_len - command.name.len() + 2);
        lines.push(format!(
            "  {}{}{}",
            command.name, padding, command.description
        ));
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
