use uuid::Uuid;

use super::Entry;

#[derive(Debug)]
pub struct LedgerId(Uuid);

#[derive(Debug)]
pub struct Ledger {
    pub id: LedgerId,
    pub name: String,
    pub entries: Vec<Entry>,
}
