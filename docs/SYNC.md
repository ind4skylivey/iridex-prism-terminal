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
- `prism sync dotfiles <list|restore>` — inspect tracked files or restore one into place.
- `prism sync jwt issue [--subject ... --ttl ...]` — sign a short-lived JWT with the stored secret and print/store it.
- `prism sync serve [--listen ...]` — launch the bundled backend for local development and tests.

## API Contract (draft)
```
POST /push { themes: [string], config: object, dotfiles: [string], timestamp: rfc3339 }
GET  /pull -> same payload
GET  /status -> { local_timestamp, remote_timestamp }
```

## Dotfiles Manager
1. Place files under `~/.config/prism/dotfiles` via future automation.
2. `sync::dotfiles::track` copies files into managed space with backup semantics.
3. Sync captures file size, last-modified timestamps, SHA-256 digests, and permission hints for every tracked file so pulls can verify integrity and preserve modes.
4. `PRISM_SYNC_AUTO_RESTORE=1` copies the synced version back into its original path (falling back to `$HOME/<name>`).

## Local Backend
- Run `prism sync serve --listen 127.0.0.1:7878` to start the Axum-based backend that honors the same JWT secret as the CLI.
- The CLI targets `http://127.0.0.1:7878` by default; override with `PRISM_SYNC_ENDPOINT=<url>`.
- Backend state is persisted under `~/.config/prism/metadata/sync-backend.json`, so pushes/pulls survive restarts.

## JWT Issuance
- Secrets live in `~/.config/prism/auth.jwt` or `PRISM_SYNC_JWT_SECRET`.
- `prism sync jwt issue --subject workstation --ttl 3600` signs tokens locally via HS256 and writes them to `~/.config/prism/auth.token` unless `--no-store` is set.
- Tokens are validated before push/pull; expired tokens must be reissued.

## Security
- JWT tokens stored locally; never commit them.
- HTTPS required for production endpoints.
- Future: encrypt dotfiles at rest before upload.
