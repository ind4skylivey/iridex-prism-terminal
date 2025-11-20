# =============================================================================
# PRISM TERMINAL: Arch-Vortex
# Description: A swirling fusion of Arch Linux blue and Catppuccin lavender.
# Generated for: Zsh 5.8+
# =============================================================================

# Colors
export PRISM_BLUE='#1793d1'
export PRISM_MAGENTA='#cba6f7'
export PRISM_ACCENT_PRIMARY='#1793d1'
export PRISM_ACCENT_SECONDARY='#cba6f7'
export PRISM_ACCENT_TERTIARY='#89b4fa'
export PRISM_ACCENT_ERROR='#f38ba8'

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  [[ -n "$branch" ]] && echo " %F{$PRISM_ACCENT_SECONDARY} $branch%f"
}

prism_prompt() {
  echo
  echo "%F{$PRISM_ACCENT_PRIMARY}🌀 %F{$PRISM_ACCENT_SECONDARY}%n %F{$PRISM_ACCENT_TERTIARY}in %F{$PRISM_ACCENT_PRIMARY}%1~$(prism_git_status)"
  echo "%(?.%F{$PRISM_ACCENT_SECONDARY}❯.%F{$PRISM_ACCENT_ERROR}❯) %f"
}

setopt PROMPT_SUBST
PROMPT='$(prism_prompt)'
