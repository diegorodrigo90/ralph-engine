//! Binary entrypoint for the Ralph Engine CLI.

use std::process::ExitCode;

/// Sentry DSN for Ralph Engine error tracking.
///
/// Always active in release builds. Disabled in debug builds and when
/// `RALPH_ENGINE_ENV=development` is set, so developers don't pollute
/// production telemetry.
const SENTRY_DSN: &str = "https://59c405510d5c5a88cd660b3f6204dd7c@o4511127271636992.ingest.us.sentry.io/4511155075809280";

/// Returns the effective Sentry DSN based on build profile and environment.
///
/// - Debug builds → disabled (empty string makes Sentry a no-op).
/// - `RALPH_ENGINE_ENV=development` → disabled.
/// - Everything else → active with the production DSN.
fn effective_sentry_dsn() -> &'static str {
    if cfg!(debug_assertions) {
        return "";
    }

    let is_dev = std::env::var("RALPH_ENGINE_ENV").is_ok_and(|env| env == "development");

    if is_dev { "" } else { SENTRY_DSN }
}

fn main() -> ExitCode {
    // Sentry captures ALL errors automatically:
    // - Panics (via sentry-panic feature — zero manual instrumentation)
    // - CLI errors (explicit capture_message on the Err path)
    // - Plugin runtime errors (propagate through CliError → captured here)
    //
    // The guard keeps Sentry alive until process exit so all events flush.
    let _sentry_guard = sentry::init((
        effective_sentry_dsn(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            send_default_pii: true,
            attach_stacktrace: true,
            ..Default::default()
        },
    ));

    match re_cli::execute(std::env::args()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            sentry::capture_message(&error.to_string(), sentry::Level::Error);
            use owo_colors::OwoColorize as _;
            use owo_colors::Stream::Stderr;
            let styled = error
                .to_string()
                .if_supports_color(Stderr, |t| t.red())
                .to_string();
            eprintln!("{styled}");
            ExitCode::from(error.exit_code)
        }
    }
}
