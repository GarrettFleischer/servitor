CREATE TABLE tenants (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE api_keys (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants (id),
    key_hash BYTEA NOT NULL,
    prefix TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    revoked_at TIMESTAMPTZ
);

CREATE INDEX api_keys_prefix_idx ON api_keys (prefix);

CREATE TABLE accounts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants (id),
    name TEXT NOT NULL,
    currency CHAR(3) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (tenant_id, name),
    CONSTRAINT accounts_currency_usd CHECK (currency = 'USD')
);

CREATE INDEX accounts_tenant_id_idx ON accounts (tenant_id, id);

CREATE TABLE entries (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants (id),
    idempotency_key TEXT NOT NULL,
    request_hash CHAR(64) NOT NULL,
    reverses_entry_id UUID REFERENCES entries (id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (tenant_id, idempotency_key)
);

CREATE INDEX entries_tenant_id_idx ON entries (tenant_id, id);

CREATE TABLE entry_lines (
    id UUID PRIMARY KEY,
    entry_id UUID NOT NULL REFERENCES entries (id) ON DELETE RESTRICT,
    account_id UUID NOT NULL REFERENCES accounts (id),
    amount_cents BIGINT NOT NULL,
    side TEXT NOT NULL,
    CONSTRAINT entry_lines_amount_positive CHECK (amount_cents > 0),
    CONSTRAINT entry_lines_side CHECK (side IN ('debit', 'credit'))
);

CREATE INDEX entry_lines_account_entry_idx ON entry_lines (account_id, entry_id);

CREATE TABLE outbox (
    id UUID PRIMARY KEY,
    topic TEXT NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    published_at TIMESTAMPTZ
);

CREATE INDEX outbox_unpublished_idx ON outbox (created_at) WHERE published_at IS NULL;

CREATE TABLE inbox (
    event_id UUID PRIMARY KEY,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE account_balances (
    account_id UUID PRIMARY KEY REFERENCES accounts (id),
    tenant_id UUID NOT NULL,
    posted_cents BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);