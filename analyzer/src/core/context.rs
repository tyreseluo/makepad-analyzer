use std::sync::Arc;

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use tower_lsp::Client;

use crate::core::config::Config;

use super::session::SessionManager;

/// Use a static session manager with a cache size of 7.
const SESSION_CACHE_SIZE: usize = 10;
static SESSION_MANAGER: Lazy<Arc<SessionManager>> = Lazy::new(|| {
  SessionManager::builder()
    .with_cache_capacity(SESSION_CACHE_SIZE)
    .build()
});

#[derive(Debug)]
pub struct AnalyzerContext {
  pub(crate) client: Option<Client>,
  pub config: Arc<RwLock<Config>>,

  pub session_manager: Arc<SessionManager>,
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
      session_manager: Arc::clone(&SESSION_MANAGER),
    };
    context
  }
}

#[cfg(test)]
mod tests {
  use super::*;


  #[tracing_test::traced_test]
  #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
  async fn test_server_context() {
    let context = AnalyzerContext::default();
    let session_manager = context.session_manager;
    let session_manager_cache_capacity = session_manager.cache().capacity();
    tracing::info!("Session manager cache capacity: {}", session_manager_cache_capacity);
    session_manager.stop();
  }
}
