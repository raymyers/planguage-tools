mod adapters;
mod application;
mod cli;
mod domain;
mod ports;

use std::process::ExitCode;

fn main() -> ExitCode {
    match cli::run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(1)
        }
    }
}
