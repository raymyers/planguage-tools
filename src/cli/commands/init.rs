use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::application::App;
use crate::cli::args::InitArgs;

const SAMPLE_DOCUMENT: &str = r#"# Example Planguage Spec

Tag: Example.Quality
Type: Performance
Version: 2026-03-11
Status: Draft
Owner: Team
Authority: Sponsor
Gist: Example starter specification.
Scale: Seconds
Meter: Measure response latency in a representative workflow.
Target: <Value TBD>
Source: <- Example, 2026-03-11
"#;

const DEFAULT_CONFIG: &str = r#"[workspace]
doc_paths = ["docs/planguage"]
"#;

pub fn run(args: InitArgs, app: &App) -> Result<ExitCode, crate::application::error::AppError> {
    let target_dir = args
        .dir
        .map(PathBuf::from)
        .unwrap_or_else(|| app.workspace_root().to_path_buf());

    fs::create_dir_all(target_dir.join("docs/planguage/templates"))?;
    fs::create_dir_all(target_dir.join("docs/planguage/fragments"))?;
    fs::create_dir_all(target_dir.join("prompts"))?;

    write_if_missing(
        target_dir.join("docs/planguage/example.md"),
        SAMPLE_DOCUMENT,
    )?;
    write_if_missing(target_dir.join("plg.toml"), DEFAULT_CONFIG)?;

    println!("initialized {}", target_dir.display());
    Ok(ExitCode::SUCCESS)
}

fn write_if_missing(
    path: PathBuf,
    content: &str,
) -> Result<(), crate::application::error::AppError> {
    if !path.exists() {
        fs::write(path, content)?;
    }

    Ok(())
}
