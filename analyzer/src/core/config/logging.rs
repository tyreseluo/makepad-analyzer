use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoggingConfig {
  #[serde(with = "LevelFilterDef")]
  pub level: LevelFilter,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(remote = "LevelFilter")]
#[allow(clippy::upper_case_acronyms)]
enum LevelFilterDef {
  OFF,
  ERROR,
  WARN,
  INFO,
  DEBUG,
  TRACE,
}

impl Default for LoggingConfig {
  fn default() -> Self {
      Self {
        level: LevelFilter::OFF
      }
  }
}