use tower_lsp::lsp_types::Range;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DocumentError {
  #[error("No document found at {:?}", path)]
  DocumentNotFound { path: String },
  #[error("Missing Cargo.toml in {:?}", dir)]
  ManifestFileNotFound { dir: String },
  #[error("Invalid manifest format: {:?}", err)]
  InvalidManifestFormat { err: String },
  #[error("Invalid makepad project: to check Cargo.toml({:?}) if makepad related dependencies have been imported.", path)]
  NotMakepadProject { path: String },
  #[error("Document is already stored at {:?}", path)]
  DocumentAlreadyStored { path: String },
  #[error("File wasn't able to be created at path {:?} : {:?}", path, err)]
  UnableToCreateFile { path: String, err: String },
  #[error("Unable to write string to file at {:?} : {:?}", path, err)]
  UnableToWriteFile { path: String, err: String },
  #[error("File wasn't able to be removed at path {:?} : {:?}", path, err)]
  UnableToRemoveFile { path: String, err: String },
  #[error("Invalid path {:?}", path)]
  InvalidPath { path: String },

  #[error("Permission denied for path {:?}", path)]
  PermissionDenied { path: String },
  #[error("IO error for path {:?} : {:?}", path, error)]
  IOError { path: String, error: String },
  #[error("Invalid range {:?}", range)]
  InvalidRange { range: Range },
}
