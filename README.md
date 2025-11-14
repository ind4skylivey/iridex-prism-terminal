<div align="center">

# 🌈 IRIDEX • PRISM

### *Adaptive Terminal Aesthetic Manager*

<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/palette/macchiato.png" width="600px" />

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Powered by Ratatui](https://img.shields.io/badge/Powered%20by-Ratatui-blueviolet?style=for-the-badge&logo=rust)](https://ratatui.rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20%7C%20Apache--2.0-blue?style=for-the-badge)](./LICENSE-MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge)](http://makeapullrequest.com)

[![Rust Version](https://img.shields.io/badge/rust-1.74%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/build-passing-success?style=flat-square)](https://github.com/ind4skylivey/prims-terminal)
[![Code Style](https://img.shields.io/badge/code%20style-rustfmt-blue?style=flat-square)](https://github.com/rust-lang/rustfmt)
[![Stars](https://img.shields.io/github/stars/ind4skylivey/prims-terminal?style=flat-square&color=yellow)](https://github.com/ind4skylivey/prims-terminal/stargazers)

*A Rust-powered terminal aesthetic platform fusing live previews, context-aware rules, animated widgets, and secure cloud sync into a single CLI*

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Themes](#-built-in-themes) • [Architecture](#%EF%B8%8F-architecture) • [Contributing](#-contributing)

</div>

---

## 🎬 Demo

> **Note:** Record your terminal sessions with [asciinema](https://asciinema.org) and convert to GIF using [agg](https://github.com/asciinema/agg)

<div align="center">

<!-- ADD YOUR GIF DEMOS HERE -->
### Theme Preview in Action
![PRISM Preview Demo](./docs/demo/preview.gif)

### Live Widget Updates
![Widget Demo](./docs/demo/widgets.gif)

### Context-Aware Theme Switching
![Context Demo](./docs/demo/context.gif)

</div>

---

## ✨ Features

<table>
<tr>
<td width="50%">

### 🎨 **Smart Theming**
- Context-native themes with Git/project/time/system/Docker detection
- Rule engine that adapts prompts to your workflow
- Zero-config automatic theme switching
- Custom theme creation with TOML

</td>
<td width="50%">

### ⚡ **Live Preview & Editing**
- Ratatui-powered TUI with instant preview
- Interactive theme editor with color picker
- See changes before they hit your shell
- Export/import theme configurations

</td>
</tr>
<tr>
<td width="50%">

### 🔌 **Widget Runtime**
- Async trait-based widget system
- Built-in: Git status, system info, clock, Docker
- Animated and cached for performance
- Extensible with custom widgets

</td>
<td width="50%">

### ☁️ **Cloud Sync & Dotfiles**
- Secure token-based authentication
- JWT integration for encrypted workflows
- Dotfile tracking with SHA-256 integrity
- Compressed snapshots with rollback support

</td>
</tr>
<tr>
<td width="50%">

### 🔋 **Power Efficient**
- Adaptive cadence based on system load
- Smart caching to reduce CPU usage
- Battery-friendly for laptops
- Daemon with IPC for minimal overhead

</td>
<td width="50%">

### 🤖 **AI-Ready**
- Optional AI features via feature flags
- Small model experiments with `PRISM_AI=1`
- Keeps default binary lean
- Future-proof architecture

</td>
</tr>
</table>

---

## 🚀 Installation

### Prerequisites

![Rust](https://img.shields.io/badge/Rust-1.74+-orange?style=flat&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/Terminal-Modern-green?style=flat&logo=gnometerminal)
![Optional](https://img.shields.io/badge/Ollama-Optional-blue?style=flat)

