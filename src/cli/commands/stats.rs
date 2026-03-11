use std::process::ExitCode;

use crate::application::App;
use crate::cli::args::StatsArgs;

pub fn run(args: StatsArgs, app: &App) -> Result<ExitCode, crate::application::error::AppError> {
    let stats = app.document_stats(args.path_prefix.as_deref())?;

    println!("markdown_files	{}", stats.markdown_files);
    println!(
        "directories_with_markdown	{}",
        stats.directories_with_markdown
    );

    Ok(ExitCode::SUCCESS)
}
