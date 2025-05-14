use std::ops::Deref;

use dashmap::DashMap;
use tower_lsp::lsp_types::{Position, Range, TextDocumentContentChangeEvent, Url};
use tracing::info;
use super::errors::DocumentError;


#[derive(Debug, Clone)]
pub struct Document {
  version: i32,
  uri: String,
  content: String,
  line_offsets: Vec<usize>,
}

impl Document {
  pub async fn build_from_path(path: &str) -> Result<Self, DocumentError> {
    tokio::fs::read_to_string(path)
    .await
    .map(|content| {
      let line_offsets = Document::calculate_line_offsets(&content);
      Self {
        version: 1,
        uri: path.into(),
        content,
        line_offsets,
      }
    })
    .map_err(|e| match e.kind() {
      std::io::ErrorKind::NotFound => {
        DocumentError::DocumentNotFound { path: path.into() }
      }
      std::io::ErrorKind::PermissionDenied => {
        DocumentError::PermissionDenied { path: path.into() }
      }
      _ => DocumentError::IOError {
        path: path.into(),
        error: e.to_string(),
      },
    })
  }

  pub fn apply_change(
    &mut self,
    change: &TextDocumentContentChangeEvent,
  ) -> Result<(), DocumentError> {
    if let Some(range) = change.range {
      self.validate_range(range)?;
      let start_index = self.position_to_index(range.start);
      let end_index = self.position_to_index(range.end);
      self.content
          .replace_range(start_index..end_index, &change.text);
    } else {
      self.content.clone_from(&change.text);
    }
    self.line_offsets = Self::calculate_line_offsets(&self.content);
    self.version += 1;
    Ok(())
  }

  pub fn get_uri(&self) -> &str {
    &self.uri
  }

  pub fn get_text(&self) -> &str {
    &self.content
  }

  pub fn get_line(&self, line: usize) -> &str {
    let start = self
      .line_offsets
      .get(line)
      .copied()
      .unwrap_or(self.content.len());
    let end = self
      .line_offsets
      .get(line + 1)
      .copied()
      .unwrap_or(self.content.len());
    &self.content[start..end]
  }

  fn validate_range(&self, range: Range) -> Result<(), DocumentError> {
    let start = self.position_to_index(range.start);
    let end = self.position_to_index(range.end);
    if start > end || end > self.content.len() {
      return Err(DocumentError::InvalidRange { range });
    }
    Ok(())
  }

  fn position_to_index(&self, position: Position) -> usize {
    let line_offset = self
      .line_offsets
      .get(position.line as usize)
      .copied()
      .unwrap_or(self.content.len());
    line_offset + position.character as usize
  }

  fn calculate_line_offsets(text: &str) -> Vec<usize> {
    let mut offsets = vec![0];
    for (i, c) in text.char_indices() {
      if c == '\n' {
        offsets.push(i + 1);
      }
    }
    offsets
  }
}

#[derive(Debug)]
pub struct Documents(DashMap<String, Document>);

impl Documents {
  pub fn new() -> Self {
    Documents(DashMap::new())
  }

  pub async fn handle_open_file(&self, uri: &Url) {
    if !self.contains_key(uri.path()) {
      if let Ok(document) = Document::build_from_path(uri.path()).await {
        info!("Building document from path: {:?}", uri.path());
        let _ = self.store_document(document);
      }
    }
  }

  pub fn store_document(&self, document: Document) -> Result<(), DocumentError> {
    let uri =  document.get_uri().to_string();
    self.insert(uri.clone(), document).map_or(Ok(()), |_| {
      Err(DocumentError::DocumentAlreadyStored { path: uri })
    })
  }

  pub fn remove_document(&self, url: &Url) -> Result<Document, DocumentError> {
    self.remove(url.path())
        .ok_or_else(|| DocumentError::DocumentNotFound {
          path: url.path().to_string()
        })
        .map(|(_, doc)| doc)
  }

  pub fn update_document(
    &self,
    url: &Url,
    changes: &[TextDocumentContentChangeEvent],
  ) -> Result<String, DocumentError> {
    self.try_get_mut(url.path())
        .try_unwrap()
        .ok_or_else(|| DocumentError::DocumentNotFound {
          path: url.path().to_string(),
        })
        .and_then(|mut document| {
          for change in changes {
            document.apply_change(change)?;
          }
          Ok(document.get_text().to_string())
        })
  }

  pub fn get_document(&self, url: &Url) -> Result<Document, DocumentError> {
    self.try_get(url.path())
        .try_unwrap()
        .ok_or_else(|| DocumentError::DocumentNotFound {
          path: url.path().to_string()
        })
        .map(|doc| doc.clone())
  }
}

impl Default for Documents {
  fn default() -> Self {
    Self::new()
  }
}

impl Deref for Documents {
  type Target = DashMap<String, Document>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn build_from_path_returns_text_document() {
    let path = "E:\\makepad-analyzer\\Cargo.toml";
    let result = Document::build_from_path(path).await;

    if result.is_ok() {
      eprintln!("Result: {:?}", result);
    }

    let document = result.unwrap();
    assert_eq!(document.version, 1);
    assert_eq!(document.uri, path);
    assert!(!document.content.is_empty());
    assert!(!document.line_offsets.is_empty());
  }

  #[tokio::test]
  async fn build_from_path_returns_document_not_found_error() {
    let path = "E:\\makepad-analyzer\\non-existent-file.txt";
    let result = Document::build_from_path(path).await.expect_err("expected DocumentNotFound");
    assert_eq!(result, DocumentError::DocumentNotFound { path: path.into() });
  }

  #[test]
  fn get_line_returns_correct_line() {
      let content = "line1\nline2\nline3".to_string();
      let line_offsets = Document::calculate_line_offsets(&content);
      let document = Document {
          version: 1,
          uri: "test.rs".into(),
          content,
          line_offsets,
      };
      assert_eq!(document.get_line(0), "line1\n");
      assert_eq!(document.get_line(1), "line2\n");
      assert_eq!(document.get_line(2), "line3");
  }
}
