use std::path::Path;

use crate::application::error::AppError;
use crate::domain::document::DocumentSummary;

pub trait DocumentRepository {
    fn list_markdown(&self, root: &Path) -> Result<Vec<DocumentSummary>, AppError>;
}
