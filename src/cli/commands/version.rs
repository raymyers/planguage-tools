use std::process::ExitCode;

use crate::application::App;

pub fn run(app: &App) -> Result<ExitCode, crate::application::error::AppError> {
    println!("{} {}", app.metadata().name, app.metadata().version);
    Ok(ExitCode::SUCCESS)
}
