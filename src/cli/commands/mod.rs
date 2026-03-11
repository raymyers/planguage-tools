use std::process::ExitCode;

use crate::application::App;
use crate::cli::args::Command;

pub fn run(command: Command, app: &App) -> Result<ExitCode, crate::application::error::AppError> {
    match command {
        Command::Version => version::run(app),
    }
}

mod version;
