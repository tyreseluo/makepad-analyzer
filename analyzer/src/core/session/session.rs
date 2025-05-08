use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use crate::core::documents::Documents;

use super::sync_workspace::SyncWorkspace;

#[derive(Debug)]
pub struct Session {
  documents: Documents,
  workspace: SyncWorkspace,
  is_active: AtomicBool,
}

impl Session {
  pub fn new() -> Self {
    Session {
      documents: Documents::default(),
      workspace: SyncWorkspace::new(),
      is_active: AtomicBool::new(true),
    }
  }

  pub fn workspace(&self) -> &SyncWorkspace {
    &self.workspace
  }

  pub fn documents(&self) -> &Documents {
    &self.documents
  }

  pub fn status(&self) -> bool {
    self.is_active.load(Relaxed)
  }

  pub fn mark_inactived(&self) {
    self.is_active.store(false, Relaxed);
  }
}