use tower_lsp::{jsonrpc::Result, lsp_types::{InitializeParams, InitializeResult}};

use crate::{core::capablities, context::AnalyzerContext};

pub fn handle_initialize(
  cx: &AnalyzerContext,
  params: InitializeParams,
) -> Result<InitializeResult> {
  Ok(InitializeResult {
    server_info: None,
    capabilities: capablities::server_capabilities(),
    ..InitializeResult::default()
  })
}