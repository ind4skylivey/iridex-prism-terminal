# Cloud Sync & Dotfiles

IRIDEX sync keeps themes, configs, and selected dotfiles in sync across machines.

## Components
- `sync::client`: Reqwest-based HTTP client with push/pull/status endpoints, optimistic concurrency, and conflict surfacing.
- `sync::server`: Axum backend that stores versioned snapshots + deltas and merges non-conflicting pushes.
- `sync::auth`: Token storage under `~/.config/prism/auth` or `PRISM_SYNC_TOKEN` env.
- `sync::dotfiles`: Copy-on-write backup of tracked files.
- `sync::storage`: Metadata cache for history/rollback.

## CLI Commands
- `prism sync push` — uploads the latest snapshot, sending your last known remote version for conflict detection.
- `prism sync pull` — downloads data, applies it locally, and records the new remote version.
- `prism sync status` — compares local timestamp/version to remote.
- `prism sync configure` — manage auth *and* connection settings (`--endpoint`, `--ca-bundle`, `--insecure`, `--show`).
- `prism sync dotfiles <list|restore>` — inspect tracked files or restore one into place.
- `prism sync jwt issue [--subject ... --ttl ...]` — sign a short-lived JWT with the stored secret and print/store it.
- `prism sync serve [--listen ...]` — launch the bundled backend for local development and tests.
- `prism sync journal <list|prune --keep N>` — inspect or trim the on-disk backend delta journal.

## API Contract (simplified)
```
POST /push {
  base_version: Option<u64>,
  payload: SyncData
}
-> { version: u64, timestamp: rfc3339 }

GET /pull -> SyncData (includes `version` when present)
GET /status -> { local_timestamp, remote_timestamp, remote_version }

409 Conflict -> { error: "conflict", message, conflicts: { dotfiles: [], config: [] } }
```

## Dotfiles Manager
1. Place files under `~/.config/prism/dotfiles` via future automation.
2. `sync::dotfiles::track` copies files into managed space with backup semantics.
3. Sync captures file size, last-modified timestamps, SHA-256 digests, and permission hints for every tracked file so pulls can verify integrity and preserve modes.
4. `PRISM_SYNC_AUTO_RESTORE=1` copies the synced version back into its original path (falling back to `$HOME/<name>`).

## Local Backend
- Run `prism sync serve --listen 127.0.0.1:7878` to start the Axum-based backend that honors the same JWT secret as the CLI.
- The CLI targets `http://127.0.0.1:7878` by default; override with `prism sync configure --endpoint https://sync.example.tld` (or `PRISM_SYNC_ENDPOINT=<url>`).
- Backend state (current snapshot + delta journal) is persisted under `~/.config/prism/metadata/sync-backend.json`, so pushes/pulls survive restarts and you can inspect/prune the history with `prism sync journal`.

## Versioning & Conflict Handling
- Every pull records the remote `version` locally; subsequent pushes send that number as `base_version`.
- The backend computes deltas relative to your base version, applies them on top of the latest snapshot, and automatically merges non-overlapping changes (e.g., different dotfiles or config keys).
- If another machine edited the same dotfile or config key, the server responds with HTTP 409 and names the conflicting entries so you can pull, merge locally, and retry.
- Set `PRISM_SYNC_FORCE=1` to bypass local pre-checks (still subject to server conflict validation).

## Remote Endpoint & TLS
- Connection settings live in `~/.config/prism/metadata/sync-config.json` (managed via `prism sync configure`).
- Flags:
  - `--endpoint https://sync.example.tld` — point the CLI at your production backend (TLS strongly recommended).
  - `--ca-bundle /path/to/company-ca.pem` — add a custom root certificate; stored as an absolute path.
  - `--insecure` / `--no-insecure` — toggle `danger_accept_invalid_certs` for lab environments.
  - `--show` — print the resolved configuration alongside the environment overrides.
- Environment variables still take precedence at runtime: `PRISM_SYNC_ENDPOINT`, `PRISM_SYNC_CA_BUNDLE`, and `PRISM_SYNC_INSECURE` (plus `PRISM_SYNC_TOKEN` / `PRISM_SYNC_JWT_SECRET`).
- Use `prism sync journal list` to audit the embedded backend’s history and `prism sync journal prune --keep 10` to trim old deltas.

## JWT Issuance
- Secrets live in `~/.config/prism/auth.jwt` or `PRISM_SYNC_JWT_SECRET`.
- `prism sync jwt issue --subject workstation --ttl 3600` signs tokens locally via HS256 and writes them to `~/.config/prism/auth.token` unless `--no-store` is set.
- Tokens are validated before push/pull; expired tokens must be reissued.

## Security
- JWT tokens stored locally; never commit them.
- HTTPS required for production endpoints.
- Future: encrypt dotfiles at rest before upload.
