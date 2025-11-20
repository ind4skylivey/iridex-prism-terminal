# IRIDEX (PRISM) Terminal Aesthetic Manager

IRIDEX (project code name PRISM) is a production-grade terminal aesthetic manager that brings live previews, dynamic theming, animated widgets, and secure cloud sync to any shell. This document summarizes the vision and provides quick navigation to deeper docs.

## Highlights
- 🎨 Context-aware themes that adapt to git status, project type, time of day, system load, and Docker activity.
- 🖼️ Ratatui-powered preview UI for safe experimentation before applying to your shell.
- 🧩 Widget runtime with async refresh loops and animated output.
- ☁️ Cloud sync plus dotfiles manager with JWT auth and timestamp-based conflict detection.
- ⚙️ Zero-config CLI with sensible defaults, ready for Bash/Zsh/Fish.

## Documentation Map
- [INSTALLATION](INSTALLATION.md)
- [THEMES](THEMES.md)
- [WIDGETS](WIDGETS.md)
- [SYNC](SYNC.md)
- [ARCHITECTURE](ARCHITECTURE.md)

## Naming Note
The strategic brand is **IRIDEX - Adaptive Terminal Iris System**. The crate/binary currently ships as `prism` for compatibility with Cargo ecosystems. References to PRISM and IRIDEX are interchangeable until GA rename.
