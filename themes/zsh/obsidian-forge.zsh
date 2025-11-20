# =============================================================================
# PRISM TERMINAL: Obsidian-Forge
# Description: A volcanic theme forging molten copper accents against obsidian rock.
# Generated for: Zsh 5.8+
# =============================================================================

# Colors
export PRISM_BG='#0b0c0e'
export PRISM_FG='#a8a8b2'
export PRISM_ACCENT_PRIMARY='#d65d0e'
export PRISM_ACCENT_SECONDARY='#af3a03'
export PRISM_ACCENT_TERTIARY='#fabd2f'
export PRISM_ACCENT_ERROR='#cc241d'

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    if [[ -n "$dirty" ]]; then
      echo " %F{$PRISM_ACCENT_PRIMARY}⚒ $branch%F{$PRISM_ACCENT_ERROR} 🔥%f"
    else
      echo " %F{$PRISM_ACCENT_PRIMARY}⚒ $branch%f"
    fi
  fi
}

prism_prompt() {
  echo
  echo "%F{$PRISM_ACCENT_SECONDARY}🌋 %F{$PRISM_FG}[%F{$PRISM_ACCENT_TERTIARY}%n%F{$PRISM_FG}]──[%F{$PRISM_ACCENT_PRIMARY}%1~%F{$PRISM_FG}]$(prism_git_status)"
  echo "%F{$PRISM_ACCENT_SECONDARY}╰─%(?.%F{$PRISM_ACCENT_PRIMARY}.%F{$PRISM_ACCENT_ERROR})► %f"
}

setopt PROMPT_SUBST
PROMPT='$(prism_prompt)'
