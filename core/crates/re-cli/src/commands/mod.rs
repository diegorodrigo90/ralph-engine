//! Modular CLI command routing.

mod agents;
mod capabilities;
mod checks;
mod config;
mod doctor;
mod embedded_assets;
mod grouped_surfaces;
mod hooks;
mod locales;
mod mcp;
mod plugins;
mod policies;
mod prompts;
mod providers;
mod runtime;
mod runtime_state;
mod templates;

use crate::{CliError, i18n};

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
        name: "runtime",
        subcommands: &["show", "status", "issues"],
        handler: runtime::execute,
    },
    CommandDescriptor {
        name: "templates",
        subcommands: &["list", "show", "asset", "scaffold", "materialize"],
        handler: templates::execute,
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
    let mut lines = vec![re_core::banner(), String::new()];

    if i18n::is_pt_br(locale) {
        lines.push("Uso: ralph-engine [--locale <id>] <comando> [argumentos]".to_owned());
        lines.push(String::new());
        lines.push("Comandos:".to_owned());
    } else {
        lines.push("Usage: ralph-engine [--locale <id>] <command> [arguments]".to_owned());
        lines.push(String::new());
        lines.push("Commands:".to_owned());
    }

    for command in COMMANDS {
        lines.push(format!("  {}", command.name));
    }

    lines.push(String::new());
    if i18n::is_pt_br(locale) {
        lines.push("Flags:".to_owned());
        lines.push("  --locale <id>, -L <id>   Define o idioma (en, pt-br)".to_owned());
        lines.push("  --version, -V            Mostra a versão".to_owned());
        lines.push("  --help, -h               Mostra esta ajuda".to_owned());
    } else {
        lines.push("Flags:".to_owned());
        lines.push("  --locale <id>, -L <id>   Set the locale (en, pt-br)".to_owned());
        lines.push("  --version, -V            Show version".to_owned());
        lines.push("  --help, -h               Show this help".to_owned());
    }

    lines.join("\n")
}

fn dispatch_command(command_name: &str, args: &[String], locale: &str) -> Result<String, CliError> {
    match COMMANDS.iter().find(|command| command.name == command_name) {
        Some(command) => {
            // Handle per-command help
            if args.first().map(String::as_str) == Some("--help")
                || args.first().map(String::as_str) == Some("-h")
            {
                return Ok(render_command_help(command, locale));
            }
            (command.handler)(args, locale)
        }
        None => Err(CliError::new(i18n::unknown_command(locale, command_name))),
    }
}

fn render_command_help(command: &CommandDescriptor, locale: &str) -> String {
    let mut lines = Vec::new();

    if i18n::is_pt_br(locale) {
        lines.push(format!(
            "Uso: ralph-engine {} <subcomando> [argumentos]",
            command.name
        ));
        lines.push(String::new());
        lines.push("Subcomandos:".to_owned());
    } else {
        lines.push(format!(
            "Usage: ralph-engine {} <subcommand> [arguments]",
            command.name
        ));
        lines.push(String::new());
        lines.push("Subcommands:".to_owned());
    }

    for sub in command.subcommands {
        lines.push(format!("  {sub}"));
    }

    lines.join("\n")
}
