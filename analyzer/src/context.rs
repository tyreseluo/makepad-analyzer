use std::sync::Arc;

use parking_lot::RwLock;
use tower_lsp::Client;

use crate::core::config::Config;

#[derive(Debug)]
pub struct AnalyzerContext {
  pub (crate) client: Option<Client>,
  pub config: Arc<RwLock<Config>>
}

impl AnalyzerContext {
  pub fn new(client: Client) -> AnalyzerContext {
    AnalyzerContext {
      client: Some(client),
      ..Default::default()
    }
  }
}

impl Default for AnalyzerContext {
  fn default() -> Self {
    let context = AnalyzerContext {
      client: None,
      config: Arc::new(RwLock::new(Config::default())),
    };
    context
  }
}