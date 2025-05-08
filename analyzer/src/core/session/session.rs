use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use crate::core::documents::Documents;

#[derive(Debug)]
pub struct Session {
  pub documents: Documents,
  // pub workspace: SyncWorkspace,
  is_active: AtomicBool,
}

impl Session {
  pub fn new() -> Self {
    Session {
      documents: Documents::new(),
      is_active: AtomicBool::new(true),
    }
  }

  pub fn status(&self) -> bool {
    self.is_active.load(Relaxed)
  }

  pub fn mark_inactived(&self) {
    self.is_active.store(false, Relaxed);
  }
}