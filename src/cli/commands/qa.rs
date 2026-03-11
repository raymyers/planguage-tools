use std::fs;
use std::io::{self, IsTerminal, Read};
use std::process::ExitCode;

use crate::application::App;
use crate::cli::args::PromptArgs;

const TEMPLATE_PATH: &str = "prompts/planguage_spec_quality_control.md";

pub fn run(args: PromptArgs, app: &App) -> Result<ExitCode, crate::application::error::AppError> {
    let template = app.load_prompt(TEMPLATE_PATH)?;
    let input = load_input(args, app)?;

    println!("{template}");
    println!();
    println!("---");
    println!("Input:");
    println!();
    println!("{input}");

    Ok(ExitCode::SUCCESS)
}

fn load_input(args: PromptArgs, app: &App) -> Result<String, crate::application::error::AppError> {
    if let Some(path) = args.file {
        return Ok(fs::read_to_string(app.workspace_root().join(path))?);
    }

    if let Some(text) = args.text {
        return Ok(text);
    }

    if io::stdin().is_terminal() {
        return Err(crate::application::error::AppError::PromptInputRequired);
    }

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}
