use clap::{CommandFactory, Parser};
use std::process::ExitCode;

use crate::application::App;

mod args;
pub mod commands;

pub fn run() -> Result<ExitCode, crate::application::error::AppError> {
    let cli = args::Cli::parse();
    let app = App::new()?;

    match cli.command {
        Some(command) => commands::run(command, &app),
        None => {
            let mut cmd = args::Cli::command();
            cmd.print_long_help()?;
            println!();
            Ok(ExitCode::SUCCESS)
        }
    }
}
