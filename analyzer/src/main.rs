#![allow(dead_code)]

mod core;
mod handlers {
  pub mod notification;
  pub mod request;
}

use core::context::{self, AnalyzerContext};
use handlers::{request, notification};
use tower_lsp::{jsonrpc::Result, lsp_types::{DidOpenTextDocumentParams, InitializeParams, InitializeResult}, Client, LanguageServer, LspService, Server};

#[tokio::main]
async fn main() {
  MakepadAnalyzer::start().await;
}

struct MakepadAnalyzer {
  pub(crate) context: AnalyzerContext,
}

#[tower_lsp::async_trait]
impl LanguageServer for MakepadAnalyzer {

  async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
    request::handle_initialize(&self.context, params)
  }

  async fn did_open(&self, params: DidOpenTextDocumentParams) {
    if let Err(err) = notification::handle_did_open_text_document(&self.context, params).await {
      tracing::error!("Error handling didOpen notification: {:?}", err);
    }
  }

  async fn shutdown(&self) -> Result<()> {
    Ok(())
  }

}

impl MakepadAnalyzer {
  fn init(client: Client) -> MakepadAnalyzer {
    MakepadAnalyzer {
      context: AnalyzerContext::new(client),
    }
  }

  pub async fn start() {
    let (service, socket) =
      LspService::build(MakepadAnalyzer::init).finish();
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
      .serve(service)
      .await;
  }
}