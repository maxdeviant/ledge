table! {
    event_entry (id) {
        id -> Uuid,
        ledger_id -> Uuid,
        name -> Text,
        timestamp -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    ledger (id) {
        id -> Uuid,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(event_entry -> ledger (ledger_id));

allow_tables_to_appear_in_same_query!(
    event_entry,
    ledger,
);
