use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug)]
pub struct Ledger {
    pub id: LedgerId,
    pub name: String,
}
