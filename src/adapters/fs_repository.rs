use std::fs;
use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

use crate::application::error::AppError;
use crate::domain::document::{DocumentSummary, SearchQuery};
use crate::ports::repository::DocumentRepository;

#[derive(Debug, Default)]
pub struct FsDocumentRepository;

impl DocumentRepository for FsDocumentRepository {
    fn list_markdown(&self, root: &Path) -> Result<Vec<DocumentSummary>, AppError> {
        let mut documents = Vec::new();

        for entry in WalkBuilder::new(root).standard_filters(true).build() {
            let entry = entry?;
            let path = entry.path();

            if is_markdown_file(path) {
                documents.push(DocumentSummary {
                    path: normalize_path(root, path),
                });
            }
        }

        documents.sort_by(|left, right| left.path.cmp(&right.path));

        Ok(documents)
    }

    fn search_markdown(
        &self,
        root: &Path,
        query: &SearchQuery,
    ) -> Result<Vec<DocumentSummary>, AppError> {
        let mut documents = Vec::new();

        for document in self.list_markdown(root)? {
            if !matches_prefix(&document, query.path_prefix.as_deref()) {
                continue;
            }

            let full_path = root.join(&document.path);
            let content = fs::read_to_string(full_path)?;

            if content.contains(&query.needle) {
                documents.push(document);
            }
        }

        Ok(documents)
    }
}

fn is_markdown_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
}

fn normalize_path(root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(root)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| path.to_path_buf())
}

fn matches_prefix(document: &DocumentSummary, prefix: Option<&str>) -> bool {
    prefix.is_none_or(|prefix| document.path.to_string_lossy().starts_with(prefix))
}
