use std::{fs, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};
use toml::Value;

use crate::core::errors::DocumentError;

use super::errors::MakepadAnalyzerError;

/// When opening a makepad project, we need to find the Cargo.toml file in the directory.
/// Example:
/// Open `examples/simple/app.rs`
/// 1. Check this file it include `live_design!`
/// 2. Proceed from the inside out to find the Cargo.toml file, check if it is a makepad project (find makepad-widgets keyworks).
/// 3. If it is a makepad project, prase the Cargo.toml file and get the project name and version.
/// 4. Check wether the project has already compiled. if not, compile it and get widgets_path and platform_path.
/// 5. Bypass with the widgets_path to build the widgets tree for code completion.
/// 6. If the project has already compiled, check if the widgets_path and platform_path are the same.

const MAKEPAD_DEPENDENCIES_NAME: &str = "makepad-widgets";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct MakepadManifest {
  pub project_name: Option<String>,
  pub widgets_path: Option<String>,
  pub platform_path: Option<String>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct MakepadManifestFile {
  manifest: MakepadManifest,
  path: PathBuf,
}

impl MakepadManifestFile {
  // TODO:
  pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self, MakepadAnalyzerError> {
    // We need to look from the inside out.
    // Find the Cargo.toml file in the directory and then check if it is a makepad project.
    let manifest_path = Self::find_manifest_file(path.as_ref())
      .ok_or(DocumentError::ManifestFileNotFound {
        dir: path.as_ref().to_string_lossy().to_string(),
      })?;

    // Parse the manifest file.
    let manifest_content = Self::parse_manifest_file(&manifest_path)?;

    // Check if the manifest file is a makepad project.
    if !Self::is_makepad_project(&manifest_content)? {
      Err(DocumentError::NotMakepadProject {
        path: path.as_ref().to_string_lossy().to_string()
      })?
    }

    // Get the project name and version.
    let project_name = manifest_content
      .get("package")
      .and_then(|pkg| pkg.get("name"))
      .and_then(|name| name.as_str())
      .map(|s| s.to_string());

    // Check whether the project has already compiled, if not, compile it and get widgets_path and platform_path.

    // TODO:
    let manifest = MakepadManifest {
      project_name,
      widgets_path: None,
      platform_path: None,
    };

    Ok(Self {
      manifest,
      path: manifest_path,
    })
  }

  pub fn path(&self) -> &Path {
    &self.path
  }

  pub fn manifest(&self) -> &MakepadManifest {
    &self.manifest
  }

  pub fn update(&mut self, manifest: MakepadManifest) {
    self.manifest = manifest;
  }

  fn parse_manifest_file(manifest_path: &Path) -> Result<Value, MakepadAnalyzerError> {
    // Parse the Cargo.toml file and check if it is a makepad project.
    fn parse_cargo_toml(cargo_toml_path: &Path) -> Result<Value, MakepadAnalyzerError> {
      let contents = fs::read_to_string(cargo_toml_path).map_err(|_| {
        DocumentError::IOError {
          path: cargo_toml_path.to_string_lossy().to_string(),
          error: "Parse Cargo.toml failed!".to_string(),
        }
      })?;

      let parsed_toml: Value = toml::from_str(&contents).map_err(|e| {
        DocumentError::InvalidManifestFormat {
          err: e.to_string(),
        }
      })?;

      Ok(parsed_toml)
    }

    // Current makepad project config file is `Cargo.toml`
    parse_cargo_toml(manifest_path)
  }

  fn find_manifest_file(start_dir: &Path) -> Option<PathBuf> {
    let mut current_dir = start_dir.to_path_buf();
    while current_dir.exists() {
      let cargo_toml = current_dir.join("Cargo.toml");
      if cargo_toml.exists() {
        return Some(cargo_toml);
      }
      // This directory does not contain a Cargo.toml file, so we need to go up one level.
      current_dir = current_dir.parent()?.to_path_buf();
    }
    None
  }

  fn is_makepad_project(cargo_toml_content: &Value) -> Result<bool, MakepadAnalyzerError> {
    // read the Cargo.toml file and check if it contains the makepad-widgets dependency.
    let has_makepad_dependencies = cargo_toml_content
      .get("dependencies")
      .and_then(|deps| deps.get(MAKEPAD_DEPENDENCIES_NAME))
      .or_else(|| cargo_toml_content.get("dev-dependencies")
        .and_then(|deps| deps.get(MAKEPAD_DEPENDENCIES_NAME)))
      .is_some();

    Ok(has_makepad_dependencies)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_manifest_file() {
    let path = PathBuf::from("E:\\makepad\\examples\\simple\\src\\app.rs");
    let manifest_file = MakepadManifestFile::find_manifest_file(&path);
    assert!(manifest_file.is_some());
    assert_eq!(
      manifest_file.unwrap(),
      PathBuf::from("E:\\makepad\\examples\\simple\\Cargo.toml")
    );
  }

  #[test]
  fn test_is_makepad_project() {
    let path = PathBuf::from("E:\\makepad\\examples\\simple\\src\\app.rs");
    let manifest_file = MakepadManifestFile::find_manifest_file(&path);
    if let Some(manifest_file) = manifest_file {
      let manifest_content = MakepadManifestFile::parse_manifest_file(&manifest_file).unwrap();
      let project_name = manifest_content
        .get("package")
        .and_then(|pkg| pkg.get("name"))
        .and_then(|name| name.as_str())
        .map(|s| s.to_string());
      println!("Project name: {:?}", project_name);
    } else {
      panic!("Manifest file not found");
    }
  }
}