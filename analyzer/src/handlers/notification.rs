use tower_lsp::lsp_types::DidOpenTextDocumentParams;

use crate::core::{context::AnalyzerContext, errors::MakepadAnalyzerError};

/// Handle the `textDocument/didOpen` notification.
pub async fn handle_did_open_text_document(
  cx: &AnalyzerContext,
  params: DidOpenTextDocumentParams,
) -> Result<(), MakepadAnalyzerError> {
  // Get the URI and session from the workspace, if the seesion is not found, will create a new one and build the workspace vitrual filesystem.
  let (uri, session) = cx
    .session_manager
    .uri_and_session_from_workspace(&params.text_document.uri)
    .await?;

  // Sync the document.
  session.documents().handle_open_file(&uri).await;
  Ok(())
}