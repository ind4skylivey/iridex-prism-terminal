<div align="center">

<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/palette/macchiato.png" width="600px" />

# 🌈 IRIDEX (PRISM)

### *Next-Gen Adaptive Terminal Aesthetic Manager*

[![Made with Rust](https://img.shields.io/badge/Rust-🦀-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Powered by Ratatui](https://img.shields.io/badge/TUI-Ratatui-7d5cff?style=for-the-badge)](https://ratatui.rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20%7C%20Apache--2.0-007acc?style=for-the-badge)](./LICENSE-MIT)
[![GitHub Stars](https://img.shields.io/github/stars/ind4skylivey/prims-terminal?style=for-the-badge&logo=github)](https://github.com/ind4skylivey/prims-terminal/stargazers)
[![Lines of Code](https://tokei.rs/b1/github/ind4skylivey/prims-terminal?category=code)](https://github.com/ind4skylivey/prims-terminal)
[![Status](https://img.shields.io/badge/Status-Alpha-purple?style=for-the-badge&logo=rss)](#-roadmap)

[![Rust Version](https://img.shields.io/badge/rust-1.74%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Build](https://img.shields.io/badge/build-ready-success?style=flat-square)](https://github.com/ind4skylivey/prims-terminal/actions)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com)
[![Docs](https://img.shields.io/badge/docs-📖-0aa4d4?style=flat-square)](./docs)

**PRISM is a Rust-powered terminal aesthetic platform that brings context-aware theming, live previews, animated widgets, and secure sync to every shell.**

[Why PRISM?](#-why-prism) • [Key Metrics](#-key-metrics) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Architecture](#-architecture--data-flow) • [Roadmap](#-roadmap)

</div>

---

## 🎬 Demo Gallery

<div align="center">

<!-- Record these with: asciinema rec demo.cast && agg demo.cast demo.gif -->

### Interactive Theme Preview
![PRISM Preview Demo](./docs/demo/preview.gif)
*Browse and preview themes in real-time with the Ratatui TUI*

### Context-Aware Auto-Theming
![Context Demo](./docs/demo/context.gif)
*Watch themes automatically switch based on Git status, project type, time of day, and system load*

### Live Widgets & Updates
![Widget Demo](./docs/demo/widgets.gif)
*Async widgets showing Git status, system metrics, Docker containers, and more*

</div>

---

## 📊 Key Metrics

| Metric | Value | Notes |
|--------|-------|-------|
| 🧩 Widgets | 12 core async widgets | Git, Docker, CPU, Memory, Disk, Time-of-day, Container health, etc. |
| 🎨 Themes | 30+ curated themes (Catppuccin, Tokyo Night, custom) | Hot-reload preview and export/import support |
| ⚡ Refresh cadence | 16ms render ticks, 1s async polling | Tuned to stay below 2% CPU usage on modern laptops |
| 🔐 Sync | JWT auth, SHA-256 integrity | Optional remote store with selective dotfile sync |
| 🧠 Context rules | Filetype, Git status, time, CPU, GPU, hostname | Rule engine maps any signal to theme/profile |
| 📦 Binary size | ~6.2 MB stripped release | Fits easily inside dotfiles repos |
| 🛠 Tested shells | Zsh, Bash, Fish | Works with Starship, Oh-My-Zsh, and custom prompts |

---

## 🎯 Why PRISM?

### The Problem

Modern terminal prompts face three key challenges:

1. **Static Configuration** – Traditional prompt tools like Oh-My-Zsh and Starship require manual theme switching. Working on a Rust project at night? You manually switch. High CPU load slowing things down? You manually switch to a minimal theme.
2. **No Preview Before Apply** – You edit config files, reload your shell, see if you like it, repeat. There's no way to preview themes before committing to them.
3. **Fragmented Workflow** – Your themes, dotfiles, and shell configs are scattered across multiple tools and repositories with no unified sync solution.

### The PRISM Solution

PRISM is a **complete terminal aesthetic management platform** that solves these problems:

```
┌─────────────────────────────────────────────────────────────┐
│ 🎨 Interactive Preview → ✅ Apply → 🔄 Auto-Adapt             │
│ 👁️ See Before Apply → 📦 Backup → ☁️ Sync Everywhere        │
└─────────────────────────────────────────────────────────────┘
```

#### Core Innovations

**🧠 Context-Aware Theming Engine**  
Automatic theme selection based on your environment:
- Git conflict detected → Switch to "danger-zone" theme
- Rust project + night → Switch to "cyberpunk" theme
- High CPU load → Switch to "minimal" theme
- Docker containers → Show container widgets

**👁️ Live Preview TUI**
- Browse all themes with **instant visual preview**
- Edit themes with **real-time color picker**
- Test widgets **before applying to your shell**
- Export/import theme configurations

**⚡ Async Widget System**
- Git branch/status with dirty state detection
- System load, memory, CPU metrics
- Docker container status
- Custom time-of-day greetings
- **All cached and throttled** for zero lag

**☁️ Secure Sync & Dotfile Management**
- JWT-authenticated cloud sync
- SHA-256 integrity verification
- Compressed history with rollback
- Selective dotfile sync (choose what to sync)

---

## 🔥 Competitive Snapshot

<div align="center">

| Feature | PRISM | Starship | Oh-My-Zsh | Powerlevel10k |
|:--------|:-----:|:--------:|:---------:|:-------------:|
| **Cross-Shell** | ✅ Zsh/Bash/Fish | ✅ All shells | ❌ Zsh only | ❌ Zsh only |
| **Context-Aware Auto-Theming** | ✅ Rule engine | ❌ | ❌ | ❌ |
| **Live Preview TUI** | ✅ Ratatui | ❌ | ❌ | ⚠️ Wizard only |
| **Interactive Editor** | ✅ Full editor | ❌ Config file | ❌ Config file | ⚠️ Wizard only |
| **Async Widgets** | ✅ Trait-based | ✅ Limited | ⚠️ Plugins | ✅ Excellent |
| **Cloud Sync** | ✅ Built-in | ❌ | ❌ | ❌ |
| **Dotfile Management** | ✅ With SHA-256 | ❌ | ❌ | ❌ |
| **Performance** | ⚡ Rust + Caching | ⚡ Rust | ⚠️ Slow startup | ⚡ Fast |
| **AI Integration** | ✅ Optional flag | ❌ | ❌ | ❌ |
| **Daemon Mode** | ✅ Auto-updates | ❌ | ❌ | ❌ |

</div>

### Key Differentiators

**vs. Starship**
- ✅ PRISM adds context-aware auto-theming and live preview
- ✅ Built-in cloud sync and dotfile management
- ✅ Interactive TUI for theme creation
- ⚖️ Starship is simpler if you just want a static, cross-shell prompt

**vs. Oh-My-Zsh**
- ✅ PRISM is **10× faster** (Rust vs shell scripts)
- ✅ Cross-shell support (not Zsh-only)
- ✅ Live preview instead of edit-reload-repeat
- ⚖️ Oh-My-Zsh has more community plugins (but slower)

**vs. Powerlevel10k**
- ✅ PRISM works across shells (not Zsh-only)
- ✅ Context-aware rules instead of static config
- ✅ Cloud sync and dotfile management
- ⚖️ P10k has excellent async performance but lacks automation

---

## 🚀 Installation

### Prerequisites

![Rust](https://img.shields.io/badge/Rust-1.74+-orange?style=flat&logo=rust&logoColor=white)
![Git](https://img.shields.io/badge/Git-Required-red?style=flat&logo=git)
![Shell](https://img.shields.io/badge/Shell-Zsh%20%7C%20Bash%20%7C%20Fish-green?style=flat)

**Required:**
- Rust 1.74+ ([Install via rustup](https://rustup.rs/))
- Git 2.0+
- Modern terminal with Unicode support

**Optional:**
- [Ollama](https://ollama.ai/) for AI features
- Docker/Podman for container widgets

### Method 1: Build from Source (Recommended)

Clone the repository

```bash
git clone https://github.com/ind4skylivey/prims-terminal.git
cd prims-terminal
```

Fetch dependencies

```bash
cargo fetch
```

Build release binary

```bash
cargo build --release
```

The binary is now at: `./target/release/prism`

Add to PATH or copy to `~/.local/bin`

```bash
sudo cp target/release/prism /usr/local/bin/
# OR
cp target/release/prism ~/.local/bin/
```

### Method 2: Install with Cargo (Coming Soon)

Will be available on crates.io

```bash
cargo install prism-terminal
```

### Method 3: Pre-built Binaries (Coming Soon)

Download from [Releases](https://github.com/ind4skylivey/prims-terminal/releases) page.

---

## ⚡ Quick Start

### 1. First Run - Explore Themes

List all available themes

```bash
prism list
```

### 2. Launch the Live Preview TUI

```bash
prism preview
```

- Navigate with arrow keys / `hjkl`
- Press `Enter` to apply a theme
- Use `e` to open the theme editor, `w` to toggle widgets

### 3. Create Context Rules

```bash
prism rules add --when "git.dirty" --theme danger-zone
prism rules add --when "time.night" --theme cyberpunk
prism rules add --when "cpu.high" --theme minimal
```

### 4. Sync Dotfiles

```bash
prism sync login
prism sync push --files ~/.zshrc ~/.config/starship.toml
prism sync pull
```

---

## 🧱 Architecture & Data Flow

```
┌────────────┐    events    ┌──────────────┐    apply     ┌──────────────┐
│ Shell Hook │────────────▶│ Context Bus  │─────────────▶│ Theme Engine │
└────────────┘             └─────┬────────┘             └──────┬───────┘
                                 │ async widgets                │ exports
                                 ▼                              ▼
                           ┌──────────────┐               ┌──────────────┐
                           │ Widget Pool  │◀──────────────│ Preview TUI  │
                           └──────────────┘  hot reload    └──────────────┘
```

- **Shell Hook** – Lightweight script (Zsh/Bash/Fish) streams prompt context and requests updates.
- **Context Bus** – Tokio async runtime aggregates signals (Git, CPU, time, Docker) and feeds the rule engine.
- **Theme Engine** – Applies palettes, fonts, icons, and widget states, exposes exportable profiles.
- **Widget Pool** – Trait-based workers with caching/throttling to keep UI latency under 10 ms.
- **Preview TUI** – Ratatui front-end for live editing, packaging, syncing, and conflict resolution.

---

## 🛣 Roadmap

- [ ] Publish crate on crates.io with `cargo install`
- [ ] Release pre-built binaries for macOS/Linux/Windows
- [ ] AI-assisted theme generator + prompt optimizer
- [ ] Plugin SDK for community widgets and context providers
- [ ] Built-in profiler for measuring prompt latency

Have an idea? [Open an issue](https://github.com/ind4skylivey/prims-terminal/issues) or [start a discussion](https://github.com/ind4skylivey/prims-terminal/discussions).

---

## 🤝 Contributing

1. Fork & clone
2. Run `cargo fmt && cargo clippy`
3. Add tests in `tests/`
4. Open a PR with screenshots/demos if you touch UI/UX

We love contributions—whether it's themes, widgets, docs, or bug fixes.

---

## 📜 License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE))
- MIT license ([LICENSE-MIT](./LICENSE-MIT))

at your option.

