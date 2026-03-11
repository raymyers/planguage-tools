use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::application::App;
use crate::cli::args::NewArgs;

pub fn run(args: NewArgs, app: &App) -> Result<ExitCode, crate::application::error::AppError> {
    let template = built_in_template(&args.template);
    let output_path = app.workspace_root().join(PathBuf::from(args.output));

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&output_path, template)?;
    println!("created {}", output_path.display());

    Ok(ExitCode::SUCCESS)
}

fn built_in_template(name: &str) -> &'static str {
    match name {
        "performance" => {
            "Tag: Example.Performance
Type: Performance
Version: 2026-03-11
Status: Draft
Owner: Team
Authority: Sponsor
Ambition: Improve a measurable performance attribute.
Scale: <Scale TBD>
Meter: <Meter TBD>
Target: <Value TBD>
Source: <- Example, 2026-03-11
"
        }
        _ => {
            "Tag: Example.Requirement
Type: Requirement
Version: 2026-03-11
Status: Draft
Owner: Team
Authority: Sponsor
Gist: Describe the requirement clearly.
Source: <- Example, 2026-03-11
"
        }
    }
}
