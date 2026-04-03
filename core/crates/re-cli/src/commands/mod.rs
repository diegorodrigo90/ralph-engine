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
    handler: fn(&[String], &str) -> Result<String, CliError>,
}

const COMMANDS: &[CommandDescriptor] = &[
    CommandDescriptor {
        name: "agents",
        handler: agents::execute,
    },
    CommandDescriptor {
        name: "capabilities",
        handler: capabilities::execute,
    },
    CommandDescriptor {
        name: "checks",
        handler: checks::execute,
    },
    CommandDescriptor {
        name: "config",
        handler: config::execute,
    },
    CommandDescriptor {
        name: "doctor",
        handler: doctor::execute,
    },
    CommandDescriptor {
        name: "hooks",
        handler: hooks::execute,
    },
    CommandDescriptor {
        name: "locales",
        handler: locales::execute,
    },
    CommandDescriptor {
        name: "mcp",
        handler: mcp::execute,
    },
    CommandDescriptor {
        name: "policies",
        handler: policies::execute,
    },
    CommandDescriptor {
        name: "prompts",
        handler: prompts::execute,
    },
    CommandDescriptor {
        name: "providers",
        handler: providers::execute,
    },
    CommandDescriptor {
        name: "plugins",
        handler: plugins::execute,
    },
    CommandDescriptor {
        name: "runtime",
        handler: runtime::execute,
    },
    CommandDescriptor {
        name: "templates",
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
        Some(command) => (command.handler)(args, locale),
        None => Err(CliError::new(i18n::unknown_command(locale, command_name))),
    }
}
