CREATE TABLE event_entry (
    id UUID PRIMARY KEY,
    ledger_id UUID NOT NULL REFERENCES ledger (id),
    name TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('event_entry');
