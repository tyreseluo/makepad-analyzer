use std::{ffi::OsStr, fs, path::{Path, PathBuf}, sync::atomic::{AtomicBool, Ordering::Relaxed}};

use tower_lsp::lsp_types::Url;

use crate::core::{documents::{Document, Documents}, errors::MakepadAnalyzerError};

use super::sync_workspace::SyncWorkspace;

pub type ProjectDirectory = PathBuf;

#[derive(Debug)]
pub struct Session {
  documents: Documents,
  workspace: SyncWorkspace,
  is_active: AtomicBool,
}

impl Session {
  pub fn new() -> Self {
    Session {
      documents: Documents::default(),
      workspace: SyncWorkspace::new(),
      is_active: AtomicBool::new(true),
    }
  }

  pub async fn init(
    &self,
    uri: &Url,
  ) -> Result<ProjectDirectory, MakepadAnalyzerError> {
    let manifest_dir = PathBuf::from(uri.path());

    // create a temp directory from the workspace
    self.workspace.create_temp_dir_from_workspace(&manifest_dir)?;
    // clone the manifest directory to the temp directory
    self.workspace.clone_manifest_dir_to_temp()?;

    // store all project files in the documents (workspace)
    let _ = self.store_project_files().await?;
    // self.sync.watch_and_sync_manifest();

    // return the manifest directory
    self.workspace.manifest_dir().map_err(Into::into)
  }


  pub fn workspace(&self) -> &SyncWorkspace {
    &self.workspace
  }

  pub fn documents(&self) -> &Documents {
    &self.documents
  }

  pub fn status(&self) -> bool {
    self.is_active.load(Relaxed)
  }

  pub fn mark_inactived(&self) {
    self.is_active.store(false, Relaxed);
  }

  async fn store_project_files(&self) -> Result<(), MakepadAnalyzerError> {
    let temp_dir = self.workspace.temp_dir()?;
    for path in get_project_files(temp_dir).iter().filter_map(|fp| fp.to_str()) {
      self.documents.store_document(Document::build_from_path(path).await?)?;
    }

    Ok(())
  }

}

fn get_project_files(path: PathBuf) -> Vec<PathBuf> {
  let mut files = vec![];
  let mut dir_entries = vec![path];

  while let Some(next_dir) = dir_entries.pop() {
    if let Ok(read_dir) = fs::read_dir(&next_dir) {
      for entry in read_dir.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
          dir_entries.push(path);
        } else if is_rust_file(&path) {
          files.push(path);
        }
      }
    }
  }
  files
}

pub fn is_rust_file(file: &Path) -> bool {
  file.is_file() && file.extension() == Some(OsStr::new("rs"))
}


#[cfg(test)]
mod test {
  use std::sync::Arc;

  use tracing::info;

use super::*;

  #[tracing_test::traced_test]
  #[tokio::test(flavor = "multi_thread")]
  async fn test_init_session() {
    let session = Arc::new(Session::new());
    let path = PathBuf::from("E:\\makepad\\examples\\simple\\src\\app.rs");
    let uri = Url::from_file_path(path).unwrap();

    session.init(&uri).await.unwrap();
    let worskpace = session.workspace();
    info!("workspace: {:?}", worskpace);
  }
}