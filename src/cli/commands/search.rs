use std::process::ExitCode;

use crate::application::App;
use crate::cli::args::SearchArgs;
use crate::domain::document::SearchQuery;

pub fn run(args: SearchArgs, app: &App) -> Result<ExitCode, crate::application::error::AppError> {
    let query = SearchQuery {
        needle: args.needle,
        path_prefix: args.path_prefix,
    };

    for document in app.search_documents(&query)? {
        println!("{}", document.path.display());
    }

    Ok(ExitCode::SUCCESS)
}
