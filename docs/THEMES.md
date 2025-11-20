# Themes Reference

Themes live in `themes/*.toml` for built-ins and `~/.config/prism/themes/*.toml` for user overrides. Use TOML with the schema defined in `core::theme`.

## Schema Overview
```toml
[metadata]
name = "Example"
author = "ind4skylivey"
version = "1.0.0"
description = "Short summary"

[colors]
background = "#000000"
foreground = "#ffffff"
# ... base colors ...

[colors.bright]
# optional bright overrides

[prompt]
style = "minimal|powerline|rich"
show_user = true
show_host = false
show_time = true
show_git = true
separator = ""

[prompt.segments]
user = { bg = "#ff00ff", fg = "#000000", icon = "" }
path = { bg = "#00ffff", fg = "#000000" }

[widgets]
enabled = ["git-status", "clock", "system"]

[context_rules]
on_git_conflict = "dracula"
on_high_load = "minimal"
night_theme = "tokyo-night"
project_themes.rust = "cyberpunk"
```

## Validation
`prism list` validates every theme via `Theme::validate`. Errors highlight bad colors or missing metadata. Keep hex colors in `#rrggbb` form.

## Custom Themes
1. Copy `examples/custom_theme.toml` into `~/.config/prism/themes/<name>.toml`.
2. Run `prism list` to verify.
3. Preview with `prism preview <name>`.
4. Apply via `prism apply <name>`.

## Context Rules
Context rules map `git`, `project`, `time`, and `system` signals to theme names. They are optional but recommended for automation.
