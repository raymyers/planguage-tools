use std::fs;
use std::path::Path;

use crate::application::error::AppError;
use crate::ports::prompts::PromptLoader;

#[derive(Debug, Default)]
pub struct FsPromptLoader;

impl PromptLoader for FsPromptLoader {
    fn load(&self, workspace_root: &Path, relative_path: &str) -> Result<String, AppError> {
        let path = workspace_root.join(relative_path);

        if !path.is_file() {
            return Err(AppError::PromptNotFound(relative_path.to_owned()));
        }

        Ok(fs::read_to_string(path)?)
    }
}
