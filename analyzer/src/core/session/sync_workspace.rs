use std::{fs, path::{Path, PathBuf}};

use dashmap::DashMap;
use tempfile::Builder;
use tower_lsp::lsp_types::Url;

use crate::core::{errors::{DirectoryError, DocumentError, MakepadAnalyzerError}, manifest::MakepadManifestFile};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Directory {
  /// The root directory of the project.
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

  pub(crate) fn create_temp_dir_from_workspace(&self, manifest_dir: &PathBuf) -> Result<(), MakepadAnalyzerError> {
    let manifest = MakepadManifestFile::from_dir(manifest_dir).map_err(|_| {
      DocumentError::ManifestFileNotFound {
        dir: manifest_dir.to_string_lossy().to_string(),
      }
    })?;

    let manifest_dir = manifest
      .path()
      .parent()
      .ok_or(DirectoryError::ManifestDirNotFound)?;

    let project_name = manifest
      .manifest()
      .project_name
      .as_ref()
      .ok_or(DirectoryError::CantExtractProjectName {
        dir: manifest_dir.to_string_lossy().to_string(),
      })?;

    let temp_dir = Builder::new()
      .prefix("makepad-analyzer")
      .tempdir()
      .map_err(|_| DirectoryError::TempDirFailed)?;

    let temp_path = temp_dir
      .keep()
      .canonicalize()
      .map_err(|_| DirectoryError::CanonicalizeFailed)?
      .join(project_name);

    self.directories
      .insert(Directory::Manifest, manifest_dir.to_path_buf());
    self.directories.insert(Directory::Temp, temp_path.clone());

    Ok(())
  }

  pub(crate) fn clone_manifest_dir_to_temp(&self) -> Result<(), DirectoryError> {
    copy_dir_contents(self.manifest_dir()?, self.temp_dir()?)
    .map_err(|_| DirectoryError::CopyContentsFailed)?;

    Ok(())
  }

  pub fn workspace_to_temp_url(&self, uri: &Url) -> Result<Url, DirectoryError> {
    convert_url(&uri, &self.temp_dir()?, &self.manifest_dir()?)
  }

  pub(crate) fn temp_to_workspace_url(&self, uri: &Url) -> Result<Url, DirectoryError> {
    convert_url(uri, &self.manifest_dir()?, &self.temp_dir()?)
  }

  pub(crate) fn temp_manifest_path(&self) -> Option<PathBuf> {
    self.temp_dir()
      .map(|dir| dir.join("Cargo.toml"))
      .ok()
  }

  pub fn manifest_path(&self) -> Option<PathBuf> {
    self.manifest_dir()
      .map(|dir| dir.join("Cargo.toml"))
      .ok()
  }

  pub(crate) fn manifest_dir(&self) -> Result<PathBuf, DirectoryError> {
    self.directories
      .try_get(&Directory::Manifest)
      .try_unwrap()
      .map(|item| item.value().clone())
      .ok_or(DirectoryError::ManifestDirNotFound)
  }

  pub(crate) fn temp_dir(&self) -> Result<PathBuf, DirectoryError> {
    self.directories
      .try_get(&Directory::Temp)
      .try_unwrap()
      .map(|item| item.value().clone())
      .ok_or(DirectoryError::TempDirNotFound)
  }

}

fn convert_url(uri: &Url, from: &Path, to: &PathBuf) -> Result<Url, DirectoryError> {
  let path = from.join(
    PathBuf::from(uri.path())
      .strip_prefix(to)
      .map_err(DirectoryError::StripPrefixError)?,
  );

  get_url_from_path(&path)
}


fn copy_dir_contents(
  src_dir: impl AsRef<Path>,
  target_dir: impl AsRef<Path>,
) -> std::io::Result<bool> {
  let mut has_relevant_files = false;
  for entry in fs::read_dir(&src_dir)? {
    let entry = entry?;
    let path = entry.path();
    let ty = entry.file_type()?;

    if ty.is_dir() {
      if copy_dir_contents(&path, target_dir.as_ref().join(entry.file_name()))? {
        has_relevant_files = true;
      }
    } else if let Some(file_name_os) = path.file_name() {
      if let Some(file_name) = file_name_os.to_str() {
        if file_name.ends_with(".rs")
        || file_name == "Cargo.toml"
        || file_name == "Cargo.lock" {
          if !has_relevant_files {
            fs::create_dir_all(&target_dir)?;
            has_relevant_files = true;
          }
          fs::copy(&path, target_dir.as_ref().join(file_name))?;
        }
      }
    }
  }
  Ok(has_relevant_files)
}


/// Create a [Url] from a [`PathBuf`].
pub fn get_url_from_path(path: &PathBuf) -> Result<Url, DirectoryError> {
  Url::from_file_path(path).map_err(|()| DirectoryError::UrlFromPathFailed {
    path: path.to_string_lossy().to_string(),
  })
}

/// Create a [`PathBuf`] from a [Url].
pub fn get_path_from_url(url: &Url) -> Result<PathBuf, DirectoryError> {
  url.to_file_path()
    .map_err(|()| DirectoryError::PathFromUrlFailed {
      url: url.to_string(),
    })
}


