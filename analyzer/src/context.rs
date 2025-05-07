use tower_lsp::Client;

#[derive(Debug)]
pub struct AnalyzerContext {
  pub (crate) client: Option<Client>
}

impl AnalyzerContext {
  pub fn new(client: Client) -> AnalyzerContext {
    AnalyzerContext {
      client: Some(client)
    }
  }
}

impl Default for AnalyzerContext {
  fn default() -> Self {
    let context = AnalyzerContext {
      client: None
    };
    context
  }
}