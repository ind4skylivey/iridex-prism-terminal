# Installation Guide

## Prerequisites
- Rust 1.70+ (install via `rustup`)
- Git for theme/context detection
- Shell: Zsh, Bash, or Fish

## Build From Source
```bash
cargo build --release
```
The resulting binary lives at `target/release/prism`. Add it to your `PATH` or place under `~/bin`.

## First Run
1. `prism list` to view built-in themes.
2. `prism preview` for a TUI gallery (j/k navigate, a apply suggestion, q quit).
3. `prism apply cyberpunk` to generate prompt scripts and source into your shell.

## Shell Integration
- Zsh: Adds `source ~/.config/prism/prism.zsh` to `~/.zshrc`.
- Bash: Adds `source ~/.config/prism/prism.bash` to `~/.bashrc`.
- Fish: Adds `source ~/.config/prism/prism.fish` to `~/.config/fish/config.fish`.

To revert integration run `prism revert --shell zsh` (or bash/fish).

## Optional Dependencies
- `ollama` for the AI feature flag (`PRISM_AI=1 cargo run --features ai_small_model`).
- `podman`/`docker` for live container widgets.

## Troubleshooting
- Delete `~/.config/prism/prism.<shell>` and rerun apply to regenerate prompts.
- Set `RUST_LOG=prism=debug` for verbose logs.
- Use `prism daemon start` only after verifying `tokio` runtime compiles (currently experimental).
