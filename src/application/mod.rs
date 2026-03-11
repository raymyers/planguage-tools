pub mod error;

use crate::domain::metadata::BuildMetadata;

pub struct App {
    metadata: BuildMetadata,
}

impl App {
    pub fn new() -> Result<Self, error::AppError> {
        Ok(Self {
            metadata: BuildMetadata::current(),
        })
    }

    pub fn metadata(&self) -> &BuildMetadata {
        &self.metadata
    }
}
