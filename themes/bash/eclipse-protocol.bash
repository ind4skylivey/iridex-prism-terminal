# =============================================================================
# PRISM TERMINAL: Eclipse-Protocol
# Description: Solar corona flares erupting from the absolute void.
# Generated for: Bash 5.0+
# =============================================================================

# Colors
PRISM_YELLOW='#ffcc00'
PRISM_ACCENT_PRIMARY='#ffd700'
PRISM_ACCENT_SECONDARY='#ffa500'
PRISM_ACCENT_TERTIARY='#ffffff'
PRISM_ACCENT_ERROR='#ff0000'

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    if [[ -n "$dirty" ]]; then
      echo -e " \033[38;2;255;215;0m☾ $branch\033[38;2;255;0;0m 🩸\033[0m"
    else
      echo -e " \033[38;2;255;215;0m☾ $branch\033[0m"
    fi
  fi
}

prism_prompt() {
  PS1="\n\[\033[38;2;255;215;0m\]🌑 \[\033[38;2;255;165;0m\]──\[\033[38;2;255;255;255m\] \W \[\033[38;2;255;165;0m\]──\$(prism_git_status)\n\[\$(if [ \$? -eq 0 ]; then echo '\033[38;2;255;215;0m☀'; else echo '\033[38;2;255;0;0m⚡'; fi)\] \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
