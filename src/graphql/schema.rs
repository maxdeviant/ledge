use juniper::{Context as JuniperContext, RootNode};

use crate::database::DatabaseConnection;

pub struct Database {
    conn: DatabaseConnection
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
});

pub struct Mutation;

graphql_object!(Mutation: Database |&self| {
    description: "The root mutation object."
});

pub type Schema = RootNode<'static, Query, Mutation>;
