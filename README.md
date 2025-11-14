# IRIDEX (PRISM) – Adaptive Terminal Aesthetic Manager

IRIDEX, code-named **PRISM**, is a Rust terminal aesthetic platform that fuses live previews, context-aware rule sets, animated widgets, and secure sync tooling into a single CLI. The project targets fast feedback, zero guesswork theming, and production-ready ergonomics across Zsh, Bash, and Fish.

## Why PRISM?
- **Context-native themes:** Git/project/time/system/docker detectors feed a rule engine so prompts react to the work you are doing.
- **Ratatui previews & editor groundwork:** See a theme before touching your shell, with a future-facing editor stubbed out for inline edits.
- **Widget runtime:** Async trait-based widgets (git, system, clock, docker) animate inside prompts and can be extended via examples.
- **Live prompt streaming + caching:** Shell scripts watch the prompt stream file while the daemon caches widget output and throttles updates to keep terminals responsive.
- **Cloud & dotfiles sync:** Reqwest-based client, token storage, and dotfile tracking (size/hash/perms metadata) set the stage for encrypted push/pull workflows and safer restores.
- **Daemon + IPC:** Background watcher/IPC server keep shells aligned without manual `apply` loops.
- **AI-ready feature flag:** `ai_small_model` + `PRISM_AI=1` opt-in enables small-model experiments without impacting the default binary.
- **Adaptive cadence:** The watcher dynamically stretches intervals when load spikes or containers idle, trimming laptop battery usage.
- **Adaptive cadence:** The watcher dynamically lengthens intervals when system load spikes or containers are quiet to conserve power.

## Quickstart
Prerequisites: Rust 1.74+ (stable), a modern terminal, and optional Ollama endpoint if experimenting with the AI feature.

```bash
# Install dependencies
cargo fetch

# Build & try the CLI
cargo build --release
./target/release/prism list
./target/release/prism preview
./target/release/prism apply cyberpunk --shell zsh
```

To run in debug mode with verbose logging:
```bash
cargo run -- --verbose preview
```

## CLI Overview
| Command | Purpose |
| --- | --- |
| `prism list` | Enumerate built-in + user themes pulled from `themes/` and `~/.config/prism/themes`. |
| `prism preview [name]` | Open the Ratatui preview carousel to inspect themes before applying. |
| `prism apply <theme> [--shell {zsh,bash,fish}]` | Generate shell-specific prompt scripts and source hooks safely (backups included). |
| `prism edit [theme]` | Launch the theme editor TUI to adjust metadata, toggles, segments, and colors; saves to `~/.config/prism/themes/`. |
| `prism auto [--set THEME|--clear]` | Capture a context snapshot, honor rule priorities, or pin/clear a manual override (`rules.toml`). |
| `prism widget <add|remove|list|configure>` | Manage persisted widget selections stored under `~/.config/prism/widgets.json`. |
| `prism sync <push|pull|status|configure|history|rollback>` | Exercise the sync client, token store, dotfile scaffolding, and snapshot history/rollback helpers. |
| `prism sync dotfiles <list|restore>` | Inspect tracked dotfiles or copy a specific one back to your home directory. |
| `prism sync jwt issue [--subject ... --ttl ...]` | Mint JWTs locally using the stored secret so sync commands always have a fresh bearer token. |
| `prism daemon <start|stop|status|enable>` | Run the context watcher + IPC server loop. |
| `prism config <get|set|edit|reset>` | Inspect or mutate CLI configuration JSON in the config directory. |
| `prism revert [--shell ...]` | Remove sourced prompt scripts for the selected shell. |

## Architecture at a Glance
- `src/core`: Theme parsing/validation, prompt generation, and shell apply logic (with automatic backup + sourcing).
- `src/context`: Git/project/time/system/docker detectors plus the rule engine driving auto suggestions and daemon behavior.
- `src/prompt_stream`: Prompt stream path utilities + formatter with caching-friendly helpers for the live shell snippets.
- `src/widgets`: Async widget trait, manager, and stock widgets (git/system/clock/docker) ready for composition.
- `src/tui`: Ratatui preview and editor scaffold, including reusable terminal frame + color picker components.
- `src/sync`: HTTP client, token store, dotfiles helper, and metadata persistence.
- `src/daemon`: Context watcher loop, widget updater heartbeat, and lightweight TCP IPC server.
- `docs/`: Architecture notes, plan checkpoints, widget/theme guides, installation recipes, and sync reference material.

See `docs/ARCHITECTURE.md` for diagrams and `docs/PLAN.md` for the sprint checklist that tracks what is production-ready vs. planned.

## Context & Widgets Pipeline
1. **Detection:** `ContextDetector` inspects git status, project manifests, local time-of-day, system load, and docker hints.
2. **Rule evaluation:** `RuleEngine` maps those signals to a named theme (e.g., night work → `tokyo-night`, heavy CPU → `minimal`).
3. **Widgets:** `WidgetManager` spins through registered async widgets, rendering colored glyphs that are stitched into prompt segments.
4. **Prompt output:** `core::prompt` emits shell-specific scripts that read the live prompt stream, so widgets/context refresh without reapplying.

The daemon reuses this pipeline in a tokio loop to reapply themes whenever context drift is detected.

## Cloud Sync & Tokens
- Configure credentials via `PRISM_SYNC_TOKEN` or `prism sync configure` (writes `~/.config/prism/auth.token`).
- Provide JWT material through `PRISM_SYNC_JWT_SECRET` so the client can validate and attach bearer tokens automatically.
- Use `prism sync jwt issue --subject <you> --ttl 7200` to sign and store a short-lived token without leaving the CLI.
- `SyncClient` currently talks to a placeholder endpoint but already serializes theme lists, widget config, and dotfile manifests (plus dotfile payloads as base64 blobs).
- Dotfiles dropped into `~/.config/prism/dotfiles/` can be tracked via `sync::dotfiles`, streamed in sync snapshots, and selectively restored with `prism sync dotfiles`. Snapshots now include file size, last-modified timestamps, SHA-256 digests, and permission hints for integrity checks.
- `prism sync history` and `prism sync rollback` read/write compressed metadata snapshots so you can inspect prior pushes or revert to the last pull.
- Set `PRISM_SYNC_FORCE=1` to override conflict detection or `PRISM_SYNC_DOTFILES=<name1,name2|all|none>` to control which dotfiles apply during pull.

## Development Workflow
```bash
# Format + lint + test everything
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Recommended tooling:
- Set `RUST_LOG=debug` when iterating on daemon/watchers.
- Use `cargo run --features ai_small_model -- auto` to verify feature-gated code stays tidy.
- Regenerate docs or diagrams in `docs/` whenever you land major architectural changes.
- Use `prism daemon enable` to write + auto-enable the user service (the command now runs `systemctl --user daemon-reload` + `enable --now` and prints instructions if it cannot).

## Roadmap Highlights
- Add prompt segment reordering + gallery search to the TUI experience.
- Implement JWT auth + conflict resolution in the sync client.
- Surface selective/differential dotfile sync plus backup policies.
- Bring Systemd unit files + installers for macOS/Linux shells.
- Expand daemon IPC so shells can request context snapshots on demand.

Track progress in `docs/PLAN.md` and the translated work plan under `../🚀 PRISM - PROYECTO TERMINAL AESTHETIC REVOLUTION.md`.

## License
Dual-licensed under MIT or Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE` for details.
