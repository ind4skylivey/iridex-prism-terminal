# =============================================================================
# PRISM TERMINAL: Eclipse-Protocol
# Description: Solar corona flares erupting from the absolute void.
# Generated for: Zsh 5.8+
# =============================================================================

# Colors
export PRISM_YELLOW='#ffcc00'
export PRISM_ACCENT_PRIMARY='#ffd700'
export PRISM_ACCENT_SECONDARY='#ffa500'
export PRISM_ACCENT_TERTIARY='#ffffff'
export PRISM_ACCENT_ERROR='#ff0000'

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    if [[ -n "$dirty" ]]; then
      echo " %F{$PRISM_ACCENT_PRIMARY}☾ $branch%F{$PRISM_ACCENT_ERROR} 🩸%f"
    else
      echo " %F{$PRISM_ACCENT_PRIMARY}☾ $branch%f"
    fi
  fi
}

prism_prompt() {
  echo
  echo "%F{$PRISM_ACCENT_PRIMARY}🌑 %F{$PRISM_ACCENT_SECONDARY}──%F{$PRISM_ACCENT_TERTIARY} %1~ %F{$PRISM_ACCENT_SECONDARY}──$(prism_git_status)"
  echo "%(?.%F{$PRISM_ACCENT_PRIMARY}☀.%F{$PRISM_ACCENT_ERROR}⚡) %f"
}

setopt PROMPT_SUBST
PROMPT='$(prism_prompt)'
