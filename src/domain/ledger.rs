use uuid::Uuid;

#[derive(Debug)]
pub struct LedgerId(Uuid);

impl LedgerId {
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
