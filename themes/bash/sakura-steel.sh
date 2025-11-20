# =============================================================================
# PRISM TERMINAL: Sakura-Steel
# Description: Soft cherry blossoms falling on cold titanium steel.
# Generated for: Bash 5.0+
# =============================================================================

# Colors
PRISM_RED='#bf616a'
PRISM_ACCENT_PRIMARY='#ffb7b2'
PRISM_ACCENT_SECONDARY='#ff9aa2'
PRISM_ACCENT_TERTIARY='#e2f0cb'
PRISM_ACCENT_ERROR='#ff6961'

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    if [[ -n "$dirty" ]]; then
      echo -e " \033[38;2;255;183;178m🌸 $branch\033[38;2;255;105;97m 🥀\033[0m"
    else
      echo -e " \033[38;2;255;183;178m🌸 $branch\033[0m"
    fi
  fi
}

prism_prompt() {
  PS1="\n\[\033[38;2;216;222;233m\]🗡  \[\033[38;2;255;154;162m\]\u \[\033[38;2;216;222;233m\]:: \[\033[38;2;226;240;203m\]\W\$(prism_git_status)\n\[\$(if [ \$? -eq 0 ]; then echo '\033[38;2;255;183;178m❀'; else echo '\033[38;2;255;105;97m✖'; fi)\] \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
