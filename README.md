# servitor

Multi-tenant double-entry ledger API. See `docs/1. servitor.md` for the full brief
and `docs/requirements.md` for the freeze.

## Prerequisites

- Rust nightly (edition 2024), rustfmt, clippy — pinned by `rust-toolchain.toml`
- Docker Desktop or Podman

```powershell
rustup toolchain install nightly
rustup component add rustfmt clippy --toolchain nightly
```

## Boot local dependencies

Copy `.env.example` to `.env`. Do not commit `.env`.

```powershell
docker compose -f deploy/compose.yaml up -d
```

| Service | Host endpoint | Role |
| --- | --- | --- |
| PostgreSQL 16 | `localhost:5432` | Source of truth |
| Redis 7 | `localhost:6379` | Balance cache (ephemeral) |
| Redpanda | `localhost:19092` | Outbox destination (Kafka API) |
| OTel Collector | `localhost:4317` (gRPC), `4318` (HTTP) | OTLP intake |
| Prometheus | `http://localhost:9090` | Scrapes collector; API later |

In-compose clients use `redpanda:9092` for Kafka. Host clients use `localhost:19092`.

Stop without wiping data: `docker compose -f deploy/compose.yaml down`.
Wipe volumes (dev reset): `docker compose -f deploy/compose.yaml down -v`.

## Check the workspace

```powershell
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

## What is not here yet

HTTP API, migrations, workers, and gRPC start in later phases. Do not add
features that are not in `docs/requirements.md`.
