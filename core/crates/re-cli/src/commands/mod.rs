//! Modular CLI command routing.

mod agents;
mod capabilities;
mod checks;
mod config;
mod doctor;
mod hooks;
mod mcp;
mod plugins;
mod policies;
mod providers;
mod runtime;

use crate::CliError;

struct CommandDescriptor {
    name: &'static str,
    handler: fn(&[String]) -> Result<String, CliError>,
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
        name: "mcp",
        handler: mcp::execute,
    },
    CommandDescriptor {
        name: "policies",
        handler: policies::execute,
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
];

/// Executes the CLI command tree from collected process arguments.
pub fn execute(args: &[String]) -> Result<String, CliError> {
    match args.get(1).map(String::as_str) {
        None => Ok(format!(
            "{}\n\nRust foundation bootstrapped.",
            re_core::banner()
        )),
        Some("--version") => Ok(env!("CARGO_PKG_VERSION").to_owned()),
        Some(command_name) => dispatch_command(command_name, &args[2..]),
    }
}

fn dispatch_command(command_name: &str, args: &[String]) -> Result<String, CliError> {
    match COMMANDS.iter().find(|command| command.name == command_name) {
        Some(command) => (command.handler)(args),
        None => Err(CliError::new(format!("unknown command: {command_name}"))),
    }
}
