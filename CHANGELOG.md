# Changelog

All notable changes to this project will be documented in this file.

## v0.1.0-beta - 2025-11-15

- Completed Sprint 6.3 validation gates: `cargo fmt`, `cargo clippy`, `cargo test`, and trimmed prompt benchmarks must all pass before release.
- Captured prompt benchmark baselines plus the regression helper (`scripts/bench-compare.sh`) to guard terminal prompt performance.
- Unified the CLI/theme-loader/sync TestEnv harness so release validation covers CLI parsing, loader behaviors, and sync push/pull flows end-to-end.
- Produced signed release tarballs from `cargo build --release` to keep artifacts reproducible and verifiable.
