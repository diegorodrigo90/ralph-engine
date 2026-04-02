//! Modular CLI command routing.

mod agents;
mod capabilities;
mod checks;
mod config;
mod doctor;
mod hooks;
mod locales;
mod mcp;
mod plugins;
mod policies;
mod prompts;
mod providers;
mod runtime;
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
    let locale = i18n::resolve_cli_locale()?;

    match args.get(1).map(String::as_str) {
        None => Ok(format!(
            "{}\n\n{}",
            re_core::banner(),
            i18n::root_bootstrapped(locale)
        )),
        Some("--version") => Ok(env!("CARGO_PKG_VERSION").to_owned()),
        Some(command_name) => dispatch_command(command_name, &args[2..], locale),
    }
}

fn dispatch_command(command_name: &str, args: &[String], locale: &str) -> Result<String, CliError> {
    match COMMANDS.iter().find(|command| command.name == command_name) {
        Some(command) => (command.handler)(args, locale),
        None => Err(CliError::new(i18n::unknown_command(locale, command_name))),
    }
}
