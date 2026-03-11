use std::path::{Path, PathBuf};

use crate::application::error::AppError;

pub struct WorkspaceLocator;

impl WorkspaceLocator {
    pub fn find(start: PathBuf) -> Result<PathBuf, AppError> {
        for candidate in start.ancestors() {
            if is_workspace_root(candidate) {
                return Ok(candidate.to_path_buf());
            }
        }

        Ok(start)
    }
}

fn is_workspace_root(path: &Path) -> bool {
    path.join("Cargo.toml").is_file() || path.join("prompts").is_dir()
}
