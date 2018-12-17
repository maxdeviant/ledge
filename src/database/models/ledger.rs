use std::convert::TryFrom;

use chrono::prelude::*;
use uuid_diesel_0_7_pg::Uuid;

use crate::database::schema::ledger;
use crate::domain;

#[derive(Queryable, Identifiable, Insertable)]
#[table_name = "ledger"]
pub struct Ledger {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<domain::Ledger> for Ledger {
    type Error = &'static str;

    fn try_from(ledger: domain::Ledger) -> Result<Self, Self::Error> {
        let now = Utc::now();
        Ok(Self {
            id: ledger.id.value().into(),
            name: ledger.name,
            created_at: now,
            updated_at: now,
        })
    }
}

impl TryFrom<Ledger> for domain::Ledger {
    type Error = &'static str;

    fn try_from(ledger: Ledger) -> Result<Self, Self::Error> {
        Ok(Self {
            id: domain::LedgerId::from(ledger.id.0),
            name: ledger.name,
        })
    }
}
