use uuid::Uuid;

use juniper::{Context as JuniperContext, FieldResult, RootNode};

use crate::database::DatabaseConnection;
use crate::graphql::{Ledger, NewLedger};

pub struct Database {
    conn: DatabaseConnection,
}

impl Database {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl JuniperContext for Database {}

pub struct Query;

graphql_object!(Query: Database |&self| {
    description: "The root query object."

    field ledgers(&executor) -> FieldResult<Vec<Ledger>>
        as "Returns a list of ledgers."
    {
        Ok(vec![
            Ledger {
                id: Uuid::new_v4(),
                name: "My Ledger".to_string(),
            }
        ])
    }
});

pub struct Mutation;

graphql_object!(Mutation: Database |&self| {
    description: "The root mutation object."

    field create_ledger(&executor, new_ledger: NewLedger) -> FieldResult<Ledger> {
        Ok(Ledger {
            id: Uuid::new_v4(),
            name: "New Ledger".to_string(),
        })
    }
});

pub type Schema = RootNode<'static, Query, Mutation>;
