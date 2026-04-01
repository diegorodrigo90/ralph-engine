//! Binary entrypoint for the Ralph Engine CLI.

use std::process::ExitCode;

fn main() -> ExitCode {
    match re_cli::execute(std::env::args()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
