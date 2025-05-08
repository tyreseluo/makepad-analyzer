use dashmap::DashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Directory {
  Manifest,
  Temp,
}

pub struct SyncWorkspace {
  // pub directories: DashMap<>
}