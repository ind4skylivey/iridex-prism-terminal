<div align="center">
<img width="1024" height="1024" alt="banner" src="https://github.com/user-attachments/assets/72d0bacc-d2e5-410a-bd48-d35e71a8d77c" />

# 🌈 Prism Terminal

**IRIDEX-powered terminal persona engine**

_Neon. Glitch. Cinema. Your shell, possessed on purpose._

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-f46d25?style=flat&logo=rust&logoColor=ffffff&labelColor=0f172a)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-2563eb?style=flat&labelColor=0f172a)](LICENSE)
[![Themes](https://img.shields.io/badge/Themes-20%20ready-ec4899?style=flat&labelColor=0f172a)](themes/)
[![Shells](https://img.shields.io/badge/Shells-Fish%20%7C%20Zsh%20%7C%20Bash-22c55e?style=flat&labelColor=0f172a)]()

[Install](#installation)  • [CLI](#️-gallery--cli) • [Roadmap](#-plan--the-ai-frontier)

</div>

---

## ✨ Why Prism?

Prism Terminal is the **IRIDEX-inspired persona engine** from ind4skylivey. Every palette, prompt, and doc is drenched in neon, glitch, and cinematic energy so your shell feels **alive**, not boring.


**Personal note**

 - Completely available for homebrew [SOON] ... on progress. 

- **Personas, not palettes** — Every catalog entry is a personality with intent, mood, voice, and a surprise element (animation, glyph cluster, UI twist)
- **Gallery-grade confidence** — `prism preview` opens a Ratatui-powered gallery with filters, tabs, and hotkeys so you can **feel** a persona before applying it
- **Multi-shell mobility** — Zsh, Fish, and Bash scripts share palettes and narratives. Your identity travels across shells without rewriting the story
- **Release-grade rigor** — `tests/palette_schema.rs` validates every JSON palette, while `tests/theme_catalog.rs` guarantees palettes, prompts, docs, loader, and gallery stay synchronized before every release
- **Ready for extensions** — Drop in a `themes/shared-palettes/<name>.json`, reuse the skeleton prompts, add `docs/<name>.md`, and the loader exposes your new identity automatically

---

## Installation
Your terminal, your prism—pick the path that fits your stack.

### Homebrew (macOS & Linux)
Fastest way; keeps updates painless via `brew`.
```bash
brew tap ind4skylivey/tap
brew install ind4skylivey/tap/prism-terminal
```
If the tap is already added:
```bash
brew install ind4skylivey/tap/prism-terminal
```
Verification:
```bash
prism --version
```

### Prebuilt binaries (GitHub Releases)
Ideal when you want a zero-build drop-in binary.
1. Open the latest release: https://github.com/ind4skylivey/iridex-prism-terminal/releases/latest  
2. Download the archive for your platform.  
3. Extract it and move the `prism` binary somewhere on your `PATH`.
```bash
sudo mv prism /usr/local/bin/
prism --version
```

### Install via Cargo
Use this if you prefer Rust toolchains or pinned source builds.
From crates.io (when published):
```bash
cargo install prism-terminal
```
From the repository source:
```bash
git clone https://github.com/ind4skylivey/iridex-prism-terminal.git
cd iridex-prism-terminal/prism
cargo install --path .
```

---

## 🎭 20 Themes Showcase

<img width="866" height="654" alt="2025-12-03_16-17" src="https://github.com/user-attachments/assets/7b2db694-980c-4732-b7d8-29d1f476439e" />


Each theme is a **cinematic prompt** designed to possess your terminal:

### 🌑 Eclipse-Protocol

**Solar corona on absolute void**

```
🌑 ── ~/code ── ☾ main::CLEAN
☀
```

- **Aesthetic:** OLED-optimized high-contrast
- **Palette:** Pure black (`#000000`) + Solar gold (`#ffd700`)
- **Mood:** Cosmic minimalism for night coders

---

### 💻 Matrix-Shade

**Elite hacker terminal**

```
[●SECURE] [ROOT::user@host] [PATH::~/code] [GIT::main::CLEAN]
┌─[TERMINAL]
└─►
```

- **Aesthetic:** Cybersecurity console with system status
- **Palette:** Neon green (`#00ff00`) on black
- **Mood:** Penetration testing, ethical hacking

---

### 🌆 Synthwave-Void

**Retro-cyberpunk sunset grid**

```
▓▒░  🌆  user  ▸  ~/code   main ⚡  ░▒▓
╰─══► ◆
```

- **Aesthetic:** 80s neon grid with digital sunset
- **Palette:** Pure neon (`#ff00ff`, `#00ffff`, `#ffff00`)
- **Mood:** Arcade energy, retrowave vibes

---

### 🌐 Cyber-Noir

**Neon-soaked cyberpunk powerline**

```
 00:12:34  user@host  ~/code   main
❯
```

- **Aesthetic:** High-contrast powerline bubbles
- **Palette:** Magenta, Cyan, Deep purple
- **Mood:** Blade Runner terminal sessions

---

### 👻 Tokyo-Ghost

**ZEN Japanese aesthetic**

```
 👻  竹  user  ›  ~/code  ⛩ main 🌸  月
❯                                        東京 12:34
```

- **Aesthetic:** Anime-inspired with traditional elements
- **Palette:** Night blue, Sakura pink, Bamboo green
- **Symbols:** `月` (moon), `竹` (bamboo), `⛩` (torii), `🌸` (sakura)
- **Mood:** Zen coding, vaporwave ninja flow

---

### 🌋 Obsidian-Forge

**Volcanic power meets molten copper**

```
🌋 [user]──[~/code]  main 🔥
╰─►
```

- **Aesthetic:** Forged in digital volcanos
- **Palette:** Obsidian black + Molten copper (`#d65d0e`)
- **Mood:** Systems programming, Rust development

---

### 🌀 Arch-Vortex

**Arch Linux meets Catppuccin lavender**

```
 🌀  user in  ~/code   main 🌪
❯
```

- **Aesthetic:** Rolling release lifestyle
- **Palette:** Arch blue (`#1793d1`) + Lavender (`#cba6f7`)
- **Mood:** Ricing enthusiast, minimalist power

---

### ⚛ Quantum-Jade

**Imperial jade meets quantum physics**

```
⚛ [~/code] ⟁ main ☢
❇
```

- **Aesthetic:** Radioactive scientific precision
- **Palette:** Imperial jade (`#00a86b`), Neon green
- **Mood:** Data science, Python notebooks

---

### 🗡 Sakura-Steel

**Cherry blossoms on titanium**

```
🗡  user :: ~/code 🌸 main
❀
```

- **Aesthetic:** Warrior-poet fusion
- **Palette:** Titanium grey + Sakura pink
- **Mood:** Zen samurai coder

---

### 💜 Lavender-Core

**Elegant anime-tech powerline**

```
 ⚜  user  ~/code   main ✦
⚛
```

- **Aesthetic:** Pastel lavender gradients
- **Palette:** True lavender (`#b57edc`), Hot pink
- **Mood:** Aesthetic coding, pastel lovers

---

### 🌌 Nebula-Mocha

**Cosmic coffee shop**

```
 ☁  user  ~/code  🌌 main ☕
⋆｡°✩
```

- **Aesthetic:** Cozy cosmos with coffee tones
- **Palette:** Coffee brown, Nebula purple, Cream
- **Mood:** Late-night coding with coffee

---

### 🌲 Forest-Flux

**Nature-inspired minimalism**

```
🌿 ~ > src > cli  main 🍂
🌱
```

- **Aesthetic:** Breadcrumb paths with leaf symbols
- **Palette:** Forest greens and ambers
- **Mood:** Marathon focus sessions

---

### 🌊 Aurora-Edge

**Sleek bubble-style powerline**

```
❄  ~/code   main ●
❯
```

- **Aesthetic:** Cool boreal colors
- **Palette:** Blues with pink/cyan accents
- **Mood:** Docker/K8s workflows

---

### 🎨 Glitch-Grid

**Blocky layout with digital noise**

- **Aesthetic:** Inverted palettes on failure
- **Mood:** Aggressive visual alerts

---

### 📼 ERROR_808

**Glitch/noise aesthetics**

- **Aesthetic:** Inverted errors and alert theater
- **Mood:** Debug mode activated

---

### 🌙 Midnight-Warp

**Deep space travel**

```
✨ ╭─ user ─ 🚀 ─ ~/code  ☄ main
╰─🪐
```

- **Aesthetic:** Starfield for night coders
- **Palette:** Deep space blues and purples
- **Mood:** Warp trails through cosmos

---

### 🖤 Mono-Quiet

**Crystalline monochrome**

- **Aesthetic:** Single-line cwd/git/status
- **Palette:** Pure monochrome with crystal accents
- **Mood:** Minimal distraction coding

---

### 👾 Terminal-Ghost

**Almost monochrome transparency**

```
👻 ~/code  main ●
❯
```

- **Aesthetic:** Ghostly minimal for transparent tiling
- **Palette:** Subtle greys
- **Mood:** Clean, ethereal focus

---

## 🆚 Prism vs the usual suspects

| Feature             | Prism                 | Starship      | Powerlevel10k       | Oh-My-Posh       |
| ------------------- | --------------------- | ------------- | ------------------- | ---------------- |
| **Personas**        | 20 curated identities | Single config | Raw configurability | Windows-focused  |
| **Gallery Preview** | ✅ Ratatui TUI        | ❌            | ❌                  | ❌               |
| **Multi-shell**     | Fish, Zsh, Bash       | ✅            | Zsh only            | PowerShell focus |
| **AI Integration**  | 🔮 Planned            | ❌            | ❌                  | ❌               |
| **Test Suite**      | ✅ Schema + Catalog   | ❌            | ❌                  | ❌               |
| **Narrative Docs**  | ✅ Per-theme          | ❌            | ❌                  | ❌               |

**Why Prism?**

- **Starship:** Blazing fast but single config path. Prism ships a curated gallery with documented narratives.
- **Powerlevel10k:** Raw power, blank slate. Prism delivers ready-to-run suites instantly.
- **Oh-My-Posh:** Windows/PowerShell theatrics. Prism is built for Rust, Linux/macOS, and multi-shell pipelines.

---

## 🕹️ Gallery & CLI

Prism runs like a **playlist manager** for your terminal identity:

```bash
prism list                                      # Show full catalog
prism preview                                   # Launch Ratatui gallery (Tab, j/k, Enter, a, E)
prism apply cyber-noir --shell zsh              # Transform instantly
prism random                                    # Surprise me
prism revert --shell fish                       # Restore previous persona
prism dev obsidian-forge                        # Live-reload theme editing
```

### Gallery hotkeys

- `Tab` — Switch tabs
- `j/k` — Navigate personas
- `Enter` — Apply selected
- `a` — Apply to all shells
- `E` — Edit in $EDITOR

---

## 🧭 Plan & the AI frontier

The living plan in `docs/PLAN.md` shows that **Phase 1–6** live features are complete:

- ✅ Core theme engine
- ✅ Widget system
- ✅ Ratatui gallery
- ✅ Cloud sync
- ✅ CLI polish
- ✅ Release readiness

### Next stops (Sprint 2.3 - AI focus):

- 🤖 **Ollama integration** — AI-assisted theme suggestions
- 🧠 **Profile analyzer** — Learn your coding patterns
- 🎯 **Smart Recommendations** — Context-aware persona switching
- 🔬 **Heuristic scoring** — Non-LLM intelligent suggestions

### Post-v0.1.1 Roadmap:

- 📊 **Context signals** — Git conflicts, Docker/K8s cues, project detection
- 🔄 **Enhanced sync** — Multi-device theme persistence
- 🎨 **Export formats** — Alacritty, Kitty, iTerm2, VSCode themes
- 🎲 **Queue-based theming** — Rotate personas on schedule
- 🔌 **Plugin system** — Extend with Rust/WASM modules

---

## 🔧 Validation & release readiness

Every persona ships polished from day zero:

```bash
cargo test --test palette_schema    # Validate all JSON palettes
cargo test --test theme_catalog      # Lock palettes + prompts + docs + loader
cargo test                           # Full test suite
```

**Quality Gates:**

- ✅ Schema compliance for all 20 personas
- ✅ Synchronized palette/prompt/docs
- ✅ Multi-shell script generation verified
- ✅ Gallery integration tested
- ✅ CLI commands validated

---

## 🛠 Development

### Creating a custom persona

1. **Define palette** (`themes/shared-palettes/my-persona.json`):

```json
{
  "name": "My-Persona",
  "description": "A cinematic description of your persona",
  "primary": "#ff5fe0",
  "secondary": "#44ddff",
  "accent": "#ffc94f",
  "bg": "#05010b",
  "fg": "#f0f5ff",
  "error": "#ff3860",
  "success": "#8dff6e"
}
```

2. **Register in `src/themes.rs`**:

```rust
pub enum ThemeId {
    // ... existing
    MyPersona,
}
```

3. **Create shell scripts** (optional for custom prompts):

```fish
# themes/fish/my-persona.fish
function fish_prompt
    set_color $PRISM_PRIMARY
    echo -n "🎭 "(prompt_pwd)" ❯ "
    set_color normal
end
```

4. **Add narrative** (`docs/my-persona.md`):

```markdown
# My Persona

**Tagline:** One-line hook

## Story

Your persona's cinematic narrative...
```

5. **Test and preview**:

```bash
cargo test
prism dev my-persona
```

---

## 📊 Performance

Benchmarks on 2021 M1 MacBook Pro:

| Operation       | Time   |
| --------------- | ------ |
| Theme switch    | <50ms  |
| Prompt render   | <10ms  |
| Gallery startup | <100ms |
| Live reload     | <5ms   |

_Measured with 20 personas and full Git integration enabled._

---

## 🤝 Acknowledgments

Prism stands on the shoulders of giants:

**Inspiration:**

- [Starship](https://starship.rs) — Cross-shell blazing speed
- [Powerlevel10k](https://github.com/romkatv/powerlevel10k) — Zsh mastery
- [Oh My Fish](https://github.com/oh-my-fish/oh-my-fish) — Fish framework

**Color Palettes:**

- [Catppuccin](https://github.com/catppuccin/catppuccin) — Pastel perfection
- [Tokyo Night](https://github.com/enkia/tokyo-night-vscode-theme) — Neon nights
- [Nord](https://www.nordtheme.com/) — Arctic beauty

**Built With:**

- [Rust](https://www.rust-lang.org/) — Systems programming language
- [Ratatui](https://github.com/ratatui-org/ratatui) — TUI framework
- [clap](https://github.com/clap-rs/clap) — CLI parsing

---

## 📜 License

MIT License — See [LICENSE](LICENSE) for details.

**TL;DR:** Possess this software however you want. Just keep the license notice.

---

<div align="center">

**Crafted with 🦀 and ☕ by [ind4skylivey](https://github.com/ind4skylivey)**

_Neon. Glitch. Cinema. Your shell, possessed on purpose._

⭐ Star this repo if Prism transformed your terminal!

</div>
