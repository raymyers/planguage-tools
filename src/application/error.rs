use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Walk(#[from] ignore::Error),
    #[error("prompt template not found: {0}")]
    PromptNotFound(String),
}
