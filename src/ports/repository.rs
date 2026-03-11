use std::path::Path;

use crate::application::error::AppError;
use crate::domain::document::{DocumentSummary, SearchQuery};

pub trait DocumentRepository {
    fn list_markdown(&self, root: &Path) -> Result<Vec<DocumentSummary>, AppError>;
    fn search_markdown(
        &self,
        root: &Path,
        query: &SearchQuery,
    ) -> Result<Vec<DocumentSummary>, AppError>;
}
