# servitor

Multi-tenant double-entry ledger API. See `1. servitor.md` for the full brief
and `docs/requirements.md` for the freeze.

## Prerequisites

- Rust stable (edition 2024), rustfmt, clippy
- Docker Desktop or Podman

## Boot local dependencies

```powershell
docker compose -f deploy/compose.yaml up -d