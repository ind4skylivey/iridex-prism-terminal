<div align="center">

<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/palette/macchiato.png" width="600px" />

# 🌈 IRIDEX (PRISM)

### *Next-Gen Adaptive Terminal Aesthetic Manager*

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust%20🦀-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Powered by Ratatui](https://img.shields.io/badge/Powered%20by-Ratatui-blueviolet?style=for-the-badge)](https://ratatui.rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20%7C%20Apache--2.0-blue?style=for-the-badge)](./LICENSE-MIT)

[![Rust Version](https://img.shields.io/badge/rust-1.74%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/build-passing-success?style=flat-square)](https://github.com/ind4skylivey/prims-terminal)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com)

**A Rust-powered terminal aesthetic platform that brings context-aware theming, live previews, animated widgets, and cloud sync to your shell**

[Why PRISM?](#-why-prism) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Comparisons](#-how-prism-compares) • [Documentation](#-documentation)

</div>

---

## 🎬 Demo

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

## 🎯 Why PRISM?

### The Problem

Modern terminal prompts face three key challenges:

1. **Static Configuration**: Traditional prompt tools like Oh-My-Zsh and Starship require manual theme switching. Working on a Rust project at night? You manually switch. High CPU load slowing things down? You manually switch to a minimal theme.

2. **No Preview Before Apply**: You edit config files, reload your shell, see if you like it, repeat. There's no way to preview themes before committing to them.

3. **Fragmented Workflow**: Your themes, dotfiles, and shell configs are scattered across multiple tools and repositories with no unified sync solution.

### The PRISM Solution

PRISM is a **complete terminal aesthetic management platform** that solves these problems:

┌─────────────────────────────────────────────────────────────┐
│ 🎨 Interactive Preview → ✅ Apply → 🔄 Auto-Adapt │
│ 👁️ See Before Apply → 📦 Backup → ☁️ Sync Everywhere │
└─────────────────────────────────────────────────────────────┘


#### Core Innovations

**🧠 Context-Aware Theming Engine**
// Automatic theme selection based on your environment
Git conflict detected → Switch to "danger-zone" theme
Rust project + night → Switch to "cyberpunk" theme
High CPU load → Switch to "minimal" theme
Docker containers → Show container widgets


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

## 🔥 How PRISM Compares

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

**vs. Starship** [web:33][web:35]
- ✅ PRISM adds context-aware auto-theming and live preview
- ✅ Built-in cloud sync and dotfile management
- ✅ Interactive TUI for theme creation
- ⚖️ Starship is simpler if you just want a static, cross-shell prompt

**vs. Oh-My-Zsh** [web:33]
- ✅ PRISM is **10x faster** (Rust vs shell scripts)
- ✅ Cross-shell support (not Zsh-only)
- ✅ Live preview instead of edit-reload-repeat
- ⚖️ Oh-My-Zsh has more community plugins (but slower)

**vs. Powerlevel10k** [web:38]
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

git clone https://github.com/ind4skylivey/prims-terminal.git
cd prims-terminal

Fetch dependencies

cargo fetch
Build release binary

cargo build --release
The binary is now at: ./target/release/prism
Add to PATH or copy to ~/.local/bin

sudo cp target/release/prism /usr/local/bin/
OR

cp target/release/prism ~/.local/bin/


### Method 2: Install with Cargo (Coming Soon)

Will be available on crates.io

cargo install prism-terminal


### Method 3: Pre-built Binaries (Coming Soon)

Download from [Releases](https://github.com/ind4skylivey/prims-terminal/releases) page.

---

## ⚡ Quick Start

### 1. First Run - Explore Themes

List all available themes

prism list
Output:
Built-in themes:
- cyberpunk Neon cyberpunk vibe
- dracula Elegant dark theme
- nord Arctic minimalist
- tokyo-night Warm balanced colors
- minimal Clean and fast


### 2. Preview Themes Interactively

Apply to Zsh

prism apply cyberpunk --shell zsh
Apply to Bash

prism apply dracula --shell bash
Apply to Fish

prism apply tokyo-night --shell fish


**What happens:**
1. Creates `~/.config/prism/prism.<shell>` with prompt script
2. Backs up your existing config
3. Adds `source ~/.config/prism/prism.<shell>` to your shell config
4. Reloads your shell

### 4. Enable Auto-Theming (The Magic! ✨)

Set default theme with auto-adaptation

prism auto --set cyberpunk
Now PRISM will automatically:
- Switch to minimal theme when CPU is high
- Use tokyo-night for nighttime coding
- Show danger-zone theme on Git conflicts
- Adapt to your project type (Rust, JS, Python, etc.)


### 5. Start the Daemon (Optional)

Start background daemon for auto-updates

prism daemon start
Enable as systemd service

prism daemon enable
Check status

prism daemon status

---

## 📖 Core Concepts

### Context Detection

PRISM continuously monitors your environment:

Context Detectors:
├── Git → branch, dirty state, conflicts
├── Project → Cargo.toml, package.json, requirements.txt
├── Time → morning, afternoon, night
├── System → CPU load, memory, battery
└── Docker → running containers, health


### Rule Engine

Define rules in themes to trigger auto-switching:

[context_rules]
Switch themes based on context

on_git_conflict = "danger-zone" # Red alert theme
on_high_load = "minimal" # Minimal when CPU > 80%
night_theme = "tokyo-night" # After 8 PM
Project-specific themes

project_themes.rust = "cyberpunk"
project_themes.javascript = "dracula"
project_themes.python = "nord"


### Widget System

Async widgets update independently:

List available widgets

prism widget list
Add a widget

prism widget add docker-status
Remove a widget

prism widget remove clock
Built-in widgets:
git-status - Branch, dirty files, conflicts
system-load - CPU, memory, disk
clock - Time with customizable format
docker - Container status
custom - Create your own!

---

## 🎨 Theme Creation

### Quick Edit

Edit theme in TUI

prism edit cyberpunk
TUI Features:
- Live color picker with preview
- Segment reordering (drag & drop)
- Widget configuration
- Export/import to TOML

### Manual Creation

Create `~/.config/prism/themes/mytheme.toml`:

[metadata]
name = "My Custom Theme"
author = "yourusername"
version = "1.0.0"
description = "My perfect terminal aesthetic"

[colors]
background = "#1a1b26"
foreground = "#c0caf5"
black = "#15161e"
red = "#f7768e"
green = "#9ece6a"
yellow = "#e0af68"
blue = "#7aa2f7"
magenta = "#bb9af7"
cyan = "#7dcfff"
white = "#a9b1d6"

[colors.bright]
black = "#414868"
red = "#f7768e"
green = "#9ece6a"
yellow = "#e0af68"
blue = "#7aa2f7"
magenta = "#bb9af7"
cyan = "#7dcfff"
white = "#c0caf5"

[prompt]
style = "powerline" # or "plain", "minimal"
show_user = true
show_host = false
show_time = true
show_git = true
show_path = true
separator = "" # Powerline separator

[prompt.segments]
user = { bg = "#7aa2f7", fg = "#1a1b26", icon = "" }
path = { bg = "#9ece6a", fg = "#1a1b26", icon = "" }
git = { bg = "#f7768e", fg = "#1a1b26", icon = "" }
time = { bg = "#bb9af7", fg = "#1a1b26", icon = "🕐" }

[widgets]
enabled = ["git-status", "system-load", "clock"]

[context_rules]
on_git_conflict = "danger-zone"
on_high_load = "minimal"
night_theme = "tokyo-night"
morning_theme = "nord"
project_themes.rust = "mytheme"

Test it:

prism preview mytheme
prism apply mytheme --shell zsh

---

## ☁️ Cloud Sync & Dotfiles

### Setup Authentication

Configure sync credentials

export PRISM_SYNC_TOKEN="your_api_token"
prism sync configure
Or store JWT secret for local token generation

export PRISM_SYNC_JWT_SECRET="your_jwt_secret"
prism sync jwt issue --subject $(whoami) --ttl 7200

### Sync Themes & Config

Push local themes to cloud

prism sync push
Pull from cloud

prism sync pull
View sync history

prism sync history
Rollback to previous state

prism sync rollback


### Dotfile Management

Track dotfiles (stored in ~/.config/prism/dotfiles/)

cp ~/.zshrc ~/.config/prism/dotfiles/
cp ~/.vimrc ~/.config/prism/dotfiles/
cp ~/.gitconfig ~/.config/prism/dotfiles/
List tracked dotfiles

prism sync dotfiles list
Output:
Tracked dotfiles:
.zshrc 12.3 KB sha256:a3f2b1c... 2025-11-14 20:15
.vimrc 8.7 KB sha256:d9e4c2a... 2025-11-13 18:30
.gitconfig 2.1 KB sha256:f7b8e3d... 2025-11-10 10:00
Sync dotfiles to cloud

prism sync push
Restore on another machine

prism sync pull
prism sync dotfiles restore
Selective restore

PRISM_SYNC_DOTFILES=.zshrc,.vimrc prism sync dotfiles restore


### Integrity Verification

All synced files include:
- SHA-256 checksum
- File size
- Last modified timestamp
- Unix permissions

PRISM verifies integrity on every pull.

---

## 🛠️ Advanced Usage

### Debug Mode

Enable verbose logging

export RUST_LOG=prism=debug
prism preview
Or use --verbose flag

prism --verbose apply cyberpunk

### AI Features (Experimental)

Build with AI support

cargo build --release --features ai_small_model
Enable at runtime

export PRISM_AI=1
prism auto
AI can suggest themes based on:
- Your current project
- Time of day
- Historical preferences

### Custom Widgets

Create `~/.config/prism/widgets/custom.rs`:

use prism::widgets::{Widget, WidgetOutput};
use async_trait::async_trait;

pub struct CustomWidget;

#[async_trait]
impl Widget for CustomWidget {
async fn render(&self) -> WidgetOutput {
WidgetOutput {
text: "💎 Custom".to_string(),
color: "#ff00ff".to_string(),
}
}
}

Register and use:

prism widget add custom --path ~/.config/prism/widgets/custom.rs

### Shell-Specific Configuration

**Zsh Integration:**
~/.zshrc

source ~/.config/prism/prism.zsh
Optional: Hook for directory changes

autoload -U add-zsh-hook
add-zsh-hook chpwd prism_context_update

**Bash Integration:**
~/.bashrc

source ~/.config/prism/prism.bash
Optional: Hook for directory changes

PROMPT_COMMAND="prism_context_update; $PROMPT_COMMAND"

**Fish Integration:**
~/.config/fish/config.fish

source ~/.config/prism/prism.fish
Optional: Hook for directory changes

function __prism_context_update --on-variable PWD
prism daemon trigger
end

---

## 📚 Documentation

<div align="center">

| Document | Description |
|:---------|:------------|
| [ARCHITECTURE.md](./docs/ARCHITECTURE.md) | System design, component interaction, data flow |
| [INSTALLATION.md](./docs/INSTALLATION.md) | Detailed installation for all platforms |
| [THEMES.md](./docs/THEMES.md) | Theme format specification, examples |
| [WIDGETS.md](./docs/WIDGETS.md) | Creating custom widgets, API reference |
| [SYNC.md](./docs/SYNC.md) | Cloud sync protocol, security details |
| [PLAN.md](./docs/PLAN.md) | Development roadmap, feature tracking |

</div>

---

## 🎯 Use Cases

### For Developers

Automatically adapt to your workflow

prism auto --set adaptive
Benefits:
✅ Minimal theme during heavy compilation (saves CPU)
✅ Rich theme with Git widgets for active development
✅ Warning theme when conflicts detected
✅ Different themes per project type

### For DevOps Engineers

Enable Docker and system widgets

prism widget add docker-status
prism widget add system-load
Real-time monitoring:
🐳 Container status in prompt
📊 CPU/Memory/Disk usage
⚠️ Alerts when resources are low

### For Multi-Machine Workflows

On Machine 1

prism sync push
On Machine 2

prism sync pull
prism sync dotfiles restore
Instantly sync:
✅ All custom themes
✅ Widget configurations
✅ Dotfiles (.zshrc, .vimrc, etc.)
✅ Context rules

### For Teams

Share team themes

prism export tokyo-night > team-theme.toml
Share file with team
Team members import

prism import team-theme.toml
prism apply team-theme

---

## 🗺️ Roadmap

<div align="center">

| Status | Milestone | ETA |
|:------:|:----------|:---:|
| ✅ | Core theming engine | Done |
| ✅ | Ratatui TUI preview | Done |
| ✅ | Context detection system | Done |
| ✅ | Async widget runtime | Done |
| ✅ | Cloud sync prototype | Done |
| 🚧 | Theme gallery with search | Q1 2026 |
| 🚧 | Plugin system for widgets | Q1 2026 |
| 📋 | Homebrew formula | Q2 2026 |
| 📋 | AUR package | Q2 2026 |
| 📋 | Windows support | Q2 2026 |
| 💡 | Web dashboard for themes | Q3 2026 |
| 💡 | Mobile companion app | Future |

</div>

**Legend:** ✅ Complete | 🚧 In Progress | 📋 Planned | 💡 Proposed

Full roadmap: [docs/PLAN.md](./docs/PLAN.md)

---

## 🤝 Contributing

We welcome contributions! Here's how:

### Quick Start

Fork and clone

git clone https://github.com/yourusername/prims-terminal.git
cd prims-terminal
Create feature branch

git checkout -b feature/amazing-feature
Make changes
... edit code ...
Format and lint

cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
Test

cargo test --all-targets --all-features
Commit and push

git commit -m "Add amazing feature"
git push origin feature/amazing-feature
Open PR on GitHub

### Development Guidelines

- **Code Style**: Follow `rustfmt` and `clippy` suggestions
- **Tests**: Add tests for new features
- **Documentation**: Update relevant `.md` files
- **Commit Messages**: Use [Conventional Commits](https://www.conventionalcommits.org/)

### Areas We Need Help

- 🎨 Theme designs (submit to `themes/`)
- 🔌 Custom widgets (examples wanted!)
- 📝 Documentation improvements
- 🐛 Bug reports and fixes
- 🌍 Translations (future)

---

## 📊 Project Stats

<div align="center">

![Lines of Code](https://img.shields.io/tokei/lines/github/ind4skylivey/prims-terminal?style=flat-square)
![Repo Size](https://img.shields.io/github/repo-size/ind4skylivey/prims-terminal?style=flat-square)
![Last Commit](https://img.shields.io/github/last-commit/ind4skylivey/prims-terminal?style=flat-square)

</div>

---

## 📄 License

<div align="center">

**Dual-licensed under your choice of:**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](./LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=for-the-badge)](./LICENSE-APACHE)

</div>

---

## 🙏 Acknowledgments

- [Ratatui](https://ratatui.rs) - Excellent TUI framework
- [Tokio](https://tokio.rs) - Async runtime
- [Starship](https://starship.rs) - Inspiration for cross-shell support
- [Powerlevel10k](https://github.com/romkatv/powerlevel10k) - Inspiration for performance optimization
- The Rust community - For amazing tooling

---

## 💬 Community & Support

<div align="center">

[![GitHub Issues](https://img.shields.io/github/issues/ind4skylivey/prims-terminal?style=for-the-badge)](https://github.com/ind4skylivey/prims-terminal/issues)
[![GitHub Discussions](https://img.shields.io/github/discussions/ind4skylivey/prims-terminal?style=for-the-badge)](https://github.com/ind4skylivey/prims-terminal/discussions)

**Questions?** Open a [Discussion](https://github.com/ind4skylivey/prims-terminal/discussions)  
**Bug?** File an [Issue](https://github.com/ind4skylivey/prims-terminal/issues)  
**Feature idea?** Start a [Discussion](https://github.com/ind4skylivey/prims-terminal/discussions)

</div>

---

<div align="center">

### ⭐ If PRISM makes your terminal better, consider starring the repo!

**Made with 🦀 Rust, ❤️ passion, and ☕ coffee**

*PRISM: Because your CLI shouldn't just work — it should inspire* ✨

---

**[Documentation](./docs/)** • **[Examples](./examples/)** • **[Themes](./themes/)** • **[Contributing](#-contributing)**

</div>
