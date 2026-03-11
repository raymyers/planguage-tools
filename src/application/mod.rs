pub mod error;

use std::path::{Path, PathBuf};

use crate::adapters::fs_repository::FsDocumentRepository;
use crate::domain::document::{DocumentSummary, SearchQuery};
use crate::domain::metadata::BuildMetadata;
use crate::ports::repository::DocumentRepository;

pub struct App {
    metadata: BuildMetadata,
    workspace_root: PathBuf,
    documents: Box<dyn DocumentRepository>,
}

impl App {
    pub fn new() -> Result<Self, error::AppError> {
        Ok(Self {
            metadata: BuildMetadata::current(),
            workspace_root: std::env::current_dir()?,
            documents: Box::<FsDocumentRepository>::default(),
        })
    }

    pub fn metadata(&self) -> &BuildMetadata {
        &self.metadata
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub fn list_documents(&self) -> Result<Vec<DocumentSummary>, error::AppError> {
        self.documents.list_markdown(self.workspace_root())
    }

    pub fn search_documents(
        &self,
        query: &SearchQuery,
    ) -> Result<Vec<DocumentSummary>, error::AppError> {
        self.documents.search_markdown(self.workspace_root(), query)
    }
}
