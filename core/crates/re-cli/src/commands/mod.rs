//! Modular CLI command routing.

use crate::{CliError, i18n};

/// Selects between English and Portuguese text based on locale.
/// Eliminates the `if locale == "pt-br" { ... } else { ... }` pattern
/// that was duplicated 20+ times across command handlers.
macro_rules! locale_str {
    ($locale:expr, $en:expr, $pt_br:expr) => {
        if i18n::is_pt_br($locale) { $pt_br } else { $en }
    };
}

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
mod locales;
mod mcp;
mod plugins;
mod policies;
mod prompts;
mod providers;
mod runtime;
mod runtime_state;
mod templates;

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
    let usage = locale_str!(
        locale,
        "Usage: ralph-engine [--locale <id>] <command> [arguments]",
        "Uso: ralph-engine [--locale <id>] <comando> [argumentos]"
    );
    let commands_label = locale_str!(locale, "Commands:", "Comandos:");

    let mut lines = vec![
        re_core::banner(),
        String::new(),
        usage.to_owned(),
        String::new(),
        commands_label.to_owned(),
    ];

    for command in COMMANDS {
        lines.push(format!("  {}", command.name));
    }

    lines.push(String::new());
    lines.push("Flags:".to_owned());
    lines.push(format!(
        "  --locale <id>, -L <id>   {}",
        locale_str!(
            locale,
            "Set the locale (en, pt-br)",
            "Define o idioma (en, pt-br)"
        )
    ));
    lines.push(format!(
        "  --version, -V            {}",
        locale_str!(locale, "Show version", "Mostra a versão")
    ));
    lines.push(format!(
        "  --help, -h               {}",
        locale_str!(locale, "Show this help", "Mostra esta ajuda")
    ));

    lines.join("\n")
}

fn dispatch_command(command_name: &str, args: &[String], locale: &str) -> Result<String, CliError> {
    match COMMANDS.iter().find(|command| command.name == command_name) {
        Some(command) => {
            if matches!(args.first().map(String::as_str), Some("--help" | "-h")) {
                return Ok(render_command_help(command, locale));
            }
            (command.handler)(args, locale)
        }
        None => Err(CliError::new(i18n::unknown_command(locale, command_name))),
    }
}

fn render_command_help(command: &CommandDescriptor, locale: &str) -> String {
    let sub_label = locale_str!(locale, "subcommand", "subcomando");
    let subs_label = locale_str!(locale, "Subcommands:", "Subcomandos:");
    let usage_prefix = locale_str!(locale, "Usage", "Uso");

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
