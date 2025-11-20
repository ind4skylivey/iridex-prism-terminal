# Prism Theme Palette Schema

Prism-Terminal treats `themes/shared-palettes/<theme>.json` as the canonical record for each personality. Every palette file must include the same set of keys so tooling, validators, and downstream generators can rely on consistent metadata.

## Required keys and their meaning
- `name` (string) – formal theme name (e.g., "Lavender-Core"). Used in docs, listings, and UI labels.
- `description` (string) – short flavor text describing the personality, vibe, or narrative.
- `primary` (hex color) – main accent color for symbols, highlights, and active segments.
- `secondary` (hex color) – supporting accent or softer gradient used in secondary segments.
- `accent` (hex color) – contrasting color reserved for important status, separators, and warnings.
- `bg` (hex color) – background tone that pairs with the prompt layout; should match the terminal wallpaper or base color.
- `fg` (hex color) – default text color for prompt characters and general output.
- `error` (hex color) – dedicated hue for failure, flee, or panic states (exit codes, warnings).
- `success` (hex color) – dedicated hue for success states (ok signs, clean git branches, etc.).

The three current palettes (`Lavender-Core`, `Tokyo-Ghost`, `ERROR_808`) already follow this template, and new personalities should mirror their structure exactly, filling in the hex values and descriptive text that best represent the character.
