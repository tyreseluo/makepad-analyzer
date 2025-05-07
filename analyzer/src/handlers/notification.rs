use tower_lsp::lsp_types::DidOpenTextDocumentParams;

use crate::{context::AnalyzerContext, core::errors::MakepadAnalyzerError};

/// Handle the `textDocument/didOpen` notification.
pub async fn handle_did_open_text_document(
  cx: &AnalyzerContext,
  params: DidOpenTextDocumentParams,
) -> Result<(), MakepadAnalyzerError> {
  tracing::info!("Opened document: {:?}", params.text_document.uri.path());

  // Get the URI and session from the workspace, if the seesion is not found, will create a new one and build the workspace vitrual filesystem.
  Ok(())
}