use std::process::ExitCode;

use crate::application::App;
use crate::cli::args::GetArgs;

pub fn run(args: GetArgs, app: &App) -> Result<ExitCode, crate::application::error::AppError> {
    let documents = app.list_documents()?;

    for document in documents.into_iter().filter(|document| {
        args.path_prefix
            .as_ref()
            .is_none_or(|prefix| document.path.to_string_lossy().starts_with(prefix))
    }) {
        println!("{}", document.path.display());
    }

    Ok(ExitCode::SUCCESS)
}
