use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::core::errors::DocumentError;

use super::errors::MakepadAnalyzerError;

const MAKEPAD_DEPENDENCIES_NAME: &str = "makepad-widgets";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct MakepadManifest {
  project_name: String,
  project_version: String,
  makepad_widgets_path: String,
  makepad_platform_path: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MakepadManifestFile {
  manifest: MakepadManifest,
  path: PathBuf,
}

impl MakepadManifestFile {
  pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self, MakepadAnalyzerError> {
    // Now the manifest file is Cargo.toml
    let manifest_path = Self::find_manifest_path(path.as_ref())
      .ok_or(DocumentError::ManifestFileNotFound {
        dir: path.as_ref().to_string_lossy().to_string(),
      })?;

    // Todo: to get project name, version and widgets_path, platform_path
    let makepad_manifest = MakepadManifest {
      project_name: "Makepad".to_string(),
      project_version: "0.1.0".to_string(),
      makepad_platform_path: "".to_string(),
      makepad_widgets_path: "".to_string(),
    };

    // resolve the path to the manifest file to get project name, version and judge it is a makepad project

    todo!()
  }

  fn find_manifest_path(start_dir: &Path) -> Option<PathBuf> {
    let mut current_dir = start_dir.to_path_buf();
    while current_dir.exists() {
      let manifest_path = current_dir.join("Cargo.toml");
      if manifest_path.exists() && is_makepad_project(&manifest_path) {
        return Some(manifest_path);
      }
      current_dir = current_dir.parent()?.to_path_buf();
    }
    None
  }

}

/// This function needs to be refactor
fn is_makepad_project(manifest_path: &Path) -> bool {
  let content = match std::fs::read_to_string(manifest_path) {
      Ok(content) => content,
      Err(_) => return false,
  };

  let toml_value: toml::Value = match content.parse() {
      Ok(value) => value,
      Err(_) => return false,
  };

  let table = match toml_value.as_table() {
      Some(table) => table,
      None => return false,
  };

  if let Some(dependencies) = table.get("dependencies").and_then(|v| v.as_table()) {
      if dependencies.contains_key(MAKEPAD_DEPENDENCIES_NAME) {
          return true;
      }
  }

  if let Some(dev_dependencies) = table.get("dev-dependencies").and_then(|v| v.as_table()) {
      if dev_dependencies.contains_key(MAKEPAD_DEPENDENCIES_NAME) {
          return true;
      }
  }

  if let Some(workspace) = table.get("workspace") {
      if let Some(workspace_deps) = workspace.get("dependencies").and_then(|v| v.as_table()) {
          if workspace_deps.contains_key(MAKEPAD_DEPENDENCIES_NAME) {
              return true;
          }
      }
  }

  false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_cargo_toml() {
    let dir = Path::new("D:\\projects\\project-robius\\robrix\\src\\login\\login_screen.rs");
    let dir2 = Path::new("E:\\makepad");

    let cargo_toml = MakepadManifestFile::find_manifest_path(dir);
    let cargo_toml2 = MakepadManifestFile::find_manifest_path(dir2);

    if cargo_toml.is_some() {
      println!("Found Cargo.toml at {:?}", cargo_toml.unwrap());
    } else {
      println!("Cargo.toml not found in the directory");
    }

    if cargo_toml2.is_some() {
      println!("Found Cargo.toml at {:?}", cargo_toml2.unwrap());
    } else {
      println!("Cargo.toml not found in the directory");
    }
  }
}