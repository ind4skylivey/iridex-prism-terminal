# IRIDEX (PRISM) – Terminal Aesthetic Manager

IRIDEX, code-named **PRISM**, is a Rust-powered terminal aesthetic manager with live previews, context-aware theming, animated widgets, and secure cloud sync. The binary currently ships as `prism` to align with Cargo naming.

## Features
- Terminal Theme Manager + Ratatui live preview.
- Context detectors for git/project/time/system/docker with rule engine.
- Animated widgets (git, system, clock, docker) with async trait runtime.
- Cloud sync + dotfiles manager scaffolding with JWT auth stubs.
- Daemon/IPC groundwork for auto theme switching.
- Optional AI bridge via `ai_small_model` feature + `PRISM_AI` env flag (Ollama endpoint assumed).

## Quickstart
```bash
cargo build --release
./target/release/prism list
./target/release/prism preview
./target/release/prism apply cyberpunk
```

## Repository Layout
See `docs/ARCHITECTURE.md` for module details and `docs/PLAN.md` for the sprint checklist.

## License
Dual-licensed under MIT or Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE`.
