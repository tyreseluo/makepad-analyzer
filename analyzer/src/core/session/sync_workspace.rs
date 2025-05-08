use std::path::PathBuf;

use dashmap::DashMap;
use tower_lsp::lsp_types::Url;

use crate::core::errors::DirectoryError;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Directory {
  Manifest,
  Temp,
}

// ./target/debug/makepad-widgets.path

#[derive(Debug)]
pub struct SyncWorkspace {
  pub directories: DashMap<Directory, PathBuf>
}

impl SyncWorkspace {
  pub(crate) fn new() -> Self {
    Self {
      directories: DashMap::new(),
    }
  }

  pub fn to_temp_url(&self, uri: &Url) -> Result<Url, DirectoryError> {
    todo!()
  }
}