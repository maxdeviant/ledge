CREATE TABLE event_entry (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ledger_id UUID NOT NULL REFERENCES ledger (id),
    name TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);

SELECT diesel_manage_updated_at('event_entry');
