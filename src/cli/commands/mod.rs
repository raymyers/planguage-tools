use std::process::ExitCode;

use crate::application::App;
use crate::cli::args::Command;

mod convert;
mod get;
mod qa;
mod search;
mod stats;
mod tree;
mod version;

pub fn run(command: Command, app: &App) -> Result<ExitCode, crate::application::error::AppError> {
    match command {
        Command::Convert(args) => convert::run(args, app),
        Command::Get(args) => get::run(args, app),
        Command::Qa(args) => qa::run(args, app),
        Command::Search(args) => search::run(args, app),
        Command::Stats(args) => stats::run(args, app),
        Command::Tree(args) => tree::run(args, app),
        Command::Version => version::run(app),
    }
}
