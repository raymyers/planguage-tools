use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSummary {
    pub path: PathBuf,
}
