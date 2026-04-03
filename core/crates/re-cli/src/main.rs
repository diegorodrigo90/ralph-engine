//! Binary entrypoint for the Ralph Engine CLI.

use std::process::ExitCode;

fn main() -> ExitCode {
    // Sentry captures panics and unhandled errors automatically.
    // The DSN is read from SENTRY_DSN at runtime — no hardcoded secrets.
    // When SENTRY_DSN is not set, sentry is disabled (zero overhead).
    let _sentry_guard = sentry::init(sentry::ClientOptions {
        release: Some(env!("CARGO_PKG_VERSION").into()),
        environment: Some(
            std::env::var("RALPH_ENGINE_ENV")
                .unwrap_or_else(|_| "production".to_owned())
                .into(),
        ),
        attach_stacktrace: true,
        ..Default::default()
    });

    match re_cli::execute(std::env::args()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            sentry::capture_message(&error.to_string(), sentry::Level::Error);
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
