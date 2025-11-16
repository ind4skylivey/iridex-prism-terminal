# Theme Template

This stub shows the files and metadata every Prism personality must provide:

- `themes/shared-palettes/<name>.json` – defines the palette following the canonical schema (`name`, `description`, `primary`, `secondary`, `accent`, `bg`, `fg`, `error`, `success`). Duplicate `theme-template.json` and update the values for your new character.
- `themes/zsh/<name>.zsh-theme`, `themes/fish/<name>.fish`, `themes/bash/<name>.sh` – bespoke prompt scripts that share no layout with existing personalities. Each script should include unique icons, colors, Git status, user@host formatting, prefix/suffix characters, and success/error handling. Use this template file to copy/modify the structure.
- `docs/<name>.md` – short write-up describing the vibe, mood, prompt example, and recommended terminal settings (transparency, blur, shadow).

To add a new theme:
1. Copy `themes/shared-palettes/theme-template.json` → `themes/shared-palettes/<your-name>.json` and fill in the hex values.
2. Copy `themes/skeleton/theme-template.*` and adjust the symbols, spacing and Git handling to match the new personality.
3. Document the character in `docs/<your-name>.md`, emphasizing what makes it unique.
4. Run `cargo test --test palette_schema` to ensure the palette file still satisfies the schema.
