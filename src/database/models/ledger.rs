use chrono::prelude::*;
use uuid_diesel_0_7_pg::Uuid;

use crate::database::schema::ledger;

#[derive(Queryable, Identifiable, Insertable)]
#[table_name = "ledger"]
pub struct Ledger {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
