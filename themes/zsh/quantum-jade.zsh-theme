# =============================================================================
# PRISM TERMINAL: Quantum-Jade
# Description: Imperial jade tones meeting quantum field theory.
# Generated for: Zsh 5.8+
# =============================================================================

# Colors
export PRISM_GREEN='#3fb950'
export PRISM_CYAN='#39c5bb'
export PRISM_ACCENT_PRIMARY='#00a86b'
export PRISM_ACCENT_SECONDARY='#00cc99'
export PRISM_ACCENT_TERTIARY='#20b2aa'
export PRISM_ACCENT_ERROR='#ff5555'

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    if [[ -n "$dirty" ]]; then
      echo " %F{$PRISM_ACCENT_PRIMARY}⟁ $branch%F{$PRISM_ACCENT_ERROR} ☢%f"
    else
      echo " %F{$PRISM_ACCENT_PRIMARY}⟁ $branch%f"
    fi
  fi
}

prism_prompt() {
  echo
  echo "%F{$PRISM_ACCENT_SECONDARY}⚛ %F{$PRISM_FG}[%F{$PRISM_ACCENT_TERTIARY}%1~%F{$PRISM_FG}]$(prism_git_status)"
  echo "%(?.%F{$PRISM_ACCENT_PRIMARY}❇.%F{$PRISM_ACCENT_ERROR}⚠) %f"
}

setopt PROMPT_SUBST
PROMPT='$(prism_prompt)'
