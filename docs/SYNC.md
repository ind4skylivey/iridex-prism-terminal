# Cloud Sync & Dotfiles

IRIDEX sync keeps themes, configs, and selected dotfiles in sync across machines.

## Components
- `sync::client`: Reqwest-based HTTP client with push/pull/status endpoints.
- `sync::auth`: Token storage under `~/.config/prism/auth` or `PRISM_SYNC_TOKEN` env.
- `sync::dotfiles`: Copy-on-write backup of tracked files.
- `sync::storage`: Metadata cache for history/rollback.

## CLI Commands
- `prism sync push` — uploads metadata (mock endpoint until backend is ready).
- `prism sync pull` — downloads data and prints summary.
- `prism sync status` — compares local timestamp to remote.
- `prism sync configure` — writes placeholder token for local dev.

## API Contract (draft)
```
POST /push { themes: [string], config: object, dotfiles: [string], timestamp: rfc3339 }
GET  /pull -> same payload
GET  /status -> { local_timestamp, remote_timestamp }
```

## Dotfiles Manager
1. Place files under `~/.config/prism/dotfiles` via future automation.
2. `sync::dotfiles::track` copies files into managed space with backup semantics.
3. Sync includes hashed metadata (TODO: S1b Fase 5.3).

## Security
- JWT tokens stored locally; never commit them.
- HTTPS required for production endpoints.
- Future: encrypt dotfiles at rest before upload.
