# Requirements freeze

Copied from `docs/1. servitor.md` section 4. Do not add a feature that is not listed here.
If you discover a hole, edit this file first, then code.

## Functional

| ID | Requirement |
| --- | --- |
| F1 | Multi-tenant isolation. Every row is scoped by `tenant_id`. A token for tenant A cannot read or write tenant B. |
| F2 | Accounts: create, get, list. Fields: id, tenant_id, name, currency (ISO-4217, start with `USD` only), created_at. |
| F3 | Journal entry post: array of lines `{account_id, amount_cents, side}` where `side` is `debit` or `credit`. Sum(debits) must equal sum(credits). At least two lines. All accounts must belong to the same tenant and share currency. |
| F4 | Entry is immutable after commit. No update/delete of posted entries. Reversals are a new entry that references `reverses_entry_id`. |
| F5 | Idempotency: `Idempotency-Key` header on POST `/v1/entries`. Same key + same tenant + same request hash returns the original entry. Different body with the same key returns `409`. |
| F6 | Balance read: `GET /v1/accounts/{id}` returns `posted_balance_cents` derived from the read model (not a full table scan of lines on every request). |
| F7 | List entries for an account with cursor pagination (`limit`, `after`). |
| F8 | REST API versioned under `/v1`. Errors are JSON `{code, message, request_id}`. |
| F9 | gRPC service `Ledger.PostEntry` and `Ledger.GetAccount` with the same invariants as REST. |
| F10 | Health: `GET /healthz` (process up). Ready: `GET /readyz` (Postgres ping + Redis ping + Kafka/NATS connectivity). |
| F11 | Auth: bearer API keys stored as hashes. Key is created via a CLI or bootstrap SQL for local use. No OAuth. |
| F12 | Outbox: inserting an entry and its outbox row is one Postgres transaction. A publisher worker drains `outbox` and writes to the log. |
| F13 | Consumer is idempotent: processed event ids live in an `inbox` table. Duplicate deliveries do not double-apply the read model. |
| F14 | Redis caches account balances. Cache key `bal:{tenant}:{account}`. Invalidate on apply. Never write Redis inside the posting transaction. |
| F15 | Structured JSON logs with `request_id`, `tenant_id`, `entry_id` where relevant. |

## Non-functional

| ID | Requirement |
| --- | --- |
| N1 | Local Compose p99 for `GET /v1/accounts/{id}` ≤ **20 ms** at 200 rps, 1 tenant, warm cache. |
| N2 | Local Compose p99 for `POST /v1/entries` ≤ **80 ms** at 50 rps (includes commit + outbox insert, not consumer lag). |
| N3 | Consumer lag under that write load: p99 < **2 s** from commit to read-model apply. |
| N4 | Crash safety: kill `-9` the API mid-post. Either the entry exists with outbox row, or neither exists. Never entry without outbox. |
| N5 | Duplicate publisher delivery must not corrupt balances. |
| N6 | Timeouts: inbound request 5 s; DB 2 s; Redis 200 ms (then fall through to Postgres). |
| N7 | No `unwrap`/`expect` on request paths. Errors map to status codes. |
| N8 | `cargo fmt`, `clippy -D warnings`, unit + integration tests in CI. |
| N9 | Images run as non-root. Secrets from env, never committed. |
| N10 | OpenAPI 3 doc generated or handwritten and kept in sync. Proto files are source of truth for gRPC. |

## Out of scope (do not build)

- Multi-currency FX, interest, or billing invoices
- Real webhooks, Stripe, or a frontend
- Multi-region, sharding, or Saga orchestration
- End-user signup / password reset
- Kubernetes operator or Helm chart beyond one Deployment + Service + ConfigMap
- Exactly-once Kafka transactions (you will document at-least-once + inbox)
