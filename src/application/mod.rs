pub mod error;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::adapters::fs_repository::FsDocumentRepository;
use crate::adapters::prompt_loader::FsPromptLoader;
use crate::adapters::workspace::WorkspaceLocator;
use crate::domain::document::{DocumentStats, DocumentSummary, SearchQuery};
use crate::domain::metadata::BuildMetadata;
use crate::ports::prompts::PromptLoader;
use crate::ports::repository::DocumentRepository;

pub struct App {
    metadata: BuildMetadata,
    workspace_root: PathBuf,
    source_root: PathBuf,
    documents: Box<dyn DocumentRepository>,
    prompts: Box<dyn PromptLoader>,
}

impl App {
    pub fn new() -> Result<Self, error::AppError> {
        Ok(Self {
            metadata: BuildMetadata::current(),
            workspace_root: WorkspaceLocator::find(std::env::current_dir()?)?,
            source_root: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            documents: Box::<FsDocumentRepository>::default(),
            prompts: Box::<FsPromptLoader>::default(),
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

    pub fn document_stats(
        &self,
        path_prefix: Option<&str>,
    ) -> Result<DocumentStats, error::AppError> {
        let documents = self.list_documents()?;
        let filtered = documents.into_iter().filter(|document| {
            path_prefix.is_none_or(|prefix| document.path.to_string_lossy().starts_with(prefix))
        });

        let mut markdown_files = 0;
        let mut directories = BTreeSet::new();

        for document in filtered {
            markdown_files += 1;

            if let Some(parent) = document.path.parent() {
                directories.insert(parent.to_path_buf());
            }
        }

        Ok(DocumentStats {
            markdown_files,
            directories_with_markdown: directories.len(),
        })
    }

    pub fn load_prompt(&self, relative_path: &str) -> Result<String, error::AppError> {
        self.prompts.load(self.workspace_root(), relative_path)
    }

    pub fn load_builtin_prompt(&self, relative_path: &str) -> Result<String, error::AppError> {
        self.prompts.load(&self.source_root, relative_path)
    }
}
