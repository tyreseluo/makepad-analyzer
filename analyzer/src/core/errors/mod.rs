mod document_error;
mod directory_error;

use thiserror::Error;

pub use directory_error::DirectoryError;
pub use document_error::DocumentError;

#[derive(Debug, Error)]
pub enum MakepadAnalyzerError {
  #[error(transparent)]
  DocumentError(#[from] DocumentError),
  #[error(transparent)]
  DirectoryError(#[from] DirectoryError),
}