use std::path::Path;

use crate::application::error::AppError;

pub trait PromptLoader {
    fn load(&self, workspace_root: &Path, relative_path: &str) -> Result<String, AppError>;
}
