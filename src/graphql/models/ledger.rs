use uuid::Uuid;

#[derive(GraphQLObject)]
#[graphql(description = "A ledger.")]
pub struct Ledger {
    #[graphql(description = "The ID of the ledger.")]
    pub id: Uuid,

    #[graphql(description = "The name of the ledger.")]
    pub name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A new ledger.")]
pub struct NewLedger {
    #[graphql(description = "The name of the ledger.")]
    pub name: String,
}
