mod core;
mod context;
mod handlers {
  pub mod notification;
  pub mod request;
}

use context::AnalyzerContext;
use handlers::request;
use tower_lsp::{jsonrpc::Result, lsp_types::{InitializeParams, InitializeResult}, Client, LanguageServer, LspService, Server};

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