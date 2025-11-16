# Changelog

All notable changes to this project will be documented in this file.

## v0.1.1 - 2025-11-16

- Solidified the core engine, theme/runtime abstractions, and multi-shell apply/rollback path so context detection, auto-switching, and prompt generation stay aligned across Zsh/Bash/Fish.
- Delivered the widget runtime, built-in git/system/clock/docker widgets, configuration hooks, animation system, and plugin SDK to stream live data without regressions.
- Shipped the Ratatui preview/editor/gallery workflow (live preview, color picker, real-time editor, gallery search/filter/community showcase) so themes can be tuned and shared from the TUI.
- Finished the sync stack (API client + JWT auth + conflict resolution) plus dotfiles manager (tracking, selective sync, restore-on-new-machine with backups/exclusions) to keep environments replicable.
- Polished the CLI and daemon surface (`apply/preview/list/edit/auto`, widget/sync/daemon/config commands, context watcher, auto-switch service, widget updater, IPC, systemd integration).
- Enforced release hygiene with fmt/clippy/tests, prompt/dotfiles/widget benchmarks, regression guarding via `scripts/bench-compare.sh`, documentation polish, and dist artifact packaging for GitHub releases.
- Declared v0.1.1 the Prism Terminal Themes milestone: 13 curated personalities (Lavender-Core, Tokyo-Ghost, ERROR_808, Nebula-Mocha, Mono-Quiet, Terminal-Ghost, Cyber-Noir, Forest-Flux, Glitch-Grid, Synthwave-Void, Midnight-Warp, Aurora-Edge, Matrix-Shade) plus the Ratatui gallery/preview/editor so users can swap them without leaving the TUI.

> Phase 2 Sprint 2.3 (Ollama integration, profile analyzer, non-LLM suggestion engine, heuristic scoring, smart recommendations) remains planned for a future release and is not part of v0.1.1.

## v0.1.0 - 2025-11-15

- Added widget animations, concurrency safeguards, and a plugin SDK so gallery/system/git widgets can stay fast with streaming outputs and third-party hooks (`src/widgets/*`).
- Introduced the Ratatui-based Theme Gallery (local + community tabs, inline filters/search, live preview) plus metadata tags so you can browse/filter/apply fast without leaving the TUI (`src/tui/preview.rs`, `docs/gallery-stub.json`).
- Hardened the dotfile workflow with exclusions, restore-all with backup snapshots under `metadata/dotfile-backups`, and new CLI controls (`sync dotfiles restore-all|exclude|include|exclusions`).
- Locked in Sprint 6.3 release gate: fmt/clippy/tests, prompt + dotfiles benches, bench-regression guard (`scripts/bench-compare.sh`), and scripted artifact creation for `dist/v0.1.0/prism-v0.1.0-*.tar.gz`.

## v0.1.0-beta - 2025-11-15

- Completed Sprint 6.3 validation gates: `cargo fmt`, `cargo clippy`, `cargo test`, and trimmed prompt benchmarks must all pass before release.
- Captured prompt benchmark baselines plus the regression helper (`scripts/bench-compare.sh`) to guard terminal prompt performance.
- Unified the CLI/theme-loader/sync TestEnv harness so release validation covers CLI parsing, loader behaviors, and sync push/pull flows end-to-end.
- Produced signed release tarballs from `cargo build --release` to keep artifacts reproducible and verifiable.
