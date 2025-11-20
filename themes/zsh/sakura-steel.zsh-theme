# =============================================================================
# PRISM TERMINAL: Sakura-Steel
# Description: Soft cherry blossoms falling on cold titanium steel.
# Generated for: Zsh 5.8+
# =============================================================================

# Colors
export PRISM_RED='#bf616a'
export PRISM_ACCENT_PRIMARY='#ffb7b2'
export PRISM_ACCENT_SECONDARY='#ff9aa2'
export PRISM_ACCENT_TERTIARY='#e2f0cb'
export PRISM_ACCENT_ERROR='#ff6961'

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    if [[ -n "$dirty" ]]; then
      echo " %F{$PRISM_ACCENT_PRIMARY}🌸 $branch%F{$PRISM_ACCENT_ERROR} 🥀%f"
    else
      echo " %F{$PRISM_ACCENT_PRIMARY}🌸 $branch%f"
    fi
  fi
}

prism_prompt() {
  echo
  echo "%F{$PRISM_FG}🗡  %F{$PRISM_ACCENT_SECONDARY}%n %F{$PRISM_FG}:: %F{$PRISM_ACCENT_TERTIARY}%1~$(prism_git_status)"
  echo "%(?.%F{$PRISM_ACCENT_PRIMARY}❀.%F{$PRISM_ACCENT_ERROR}✖) %f"
}

setopt PROMPT_SUBST
PROMPT='$(prism_prompt)'
