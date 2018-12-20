use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Entry, Project};

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct LedgerId(Uuid);

impl LedgerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for LedgerId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ledger {
    /// The ID of the ledger.
    pub id: LedgerId,
    /// The name of the ledger.
    pub name: String,
    /// The list of projects in the ledger.
    pub projects: Vec<Project>,
    /// The list of entries in the ledger.
    pub entries: Vec<Entry>,
}
