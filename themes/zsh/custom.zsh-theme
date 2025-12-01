# =============================================================================
# PRISM TERMINAL: Custom
# Description: Placeholder palette so users can override with their own tweaks.
# Generated for: Zsh 5.8+
# =============================================================================

PRISM_BG="#0b0b0b"
PRISM_FG="#f8f8f2"
PRISM_PRIMARY="#ff79c6"
PRISM_SECONDARY="#8be9fd"
PRISM_ACCENT="#bd93f9"
PRISM_ERROR="#ff5555"
PRISM_SUCCESS="#50fa7b"

prism_prompt() {
  local status=$?
  local status_color=$([[ $status -eq 0 ]] && echo "$PRISM_SUCCESS" || echo "$PRISM_ERROR")
  PROMPT="%F{$PRISM_PRIMARY}◆%f %F{$PRISM_SECONDARY}%n%f %F{$PRISM_ACCENT}in%f %F{$PRISM_PRIMARY}%~%f %F{$status_color}❯%f "
}

autoload -Uz add-zsh-hook
add-zsh-hook precmd prism_prompt
