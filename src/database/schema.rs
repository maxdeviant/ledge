table! {
    event_entry (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        ledger_id -> Uuid,
        name -> Text,
        timestamp -> Timestamptz,
    }
}

table! {
    ledger (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Text,
    }
}

table! {
    project (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Text,
    }
}

joinable!(event_entry -> ledger (ledger_id));

allow_tables_to_appear_in_same_query!(
    event_entry,
    ledger,
    project,
);
