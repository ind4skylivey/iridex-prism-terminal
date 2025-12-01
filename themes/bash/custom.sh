# =============================================================================
# PRISM TERMINAL: Custom
# Description: Placeholder palette so users can override with their own tweaks.
# Generated for: Bash 5.0+
# =============================================================================

PRISM_BG='#0b0b0b'
PRISM_FG='#f8f8f2'
PRISM_PRIMARY='#ff79c6'
PRISM_SECONDARY='#8be9fd'
PRISM_ACCENT='#bd93f9'
PRISM_ERROR='#ff5555'
PRISM_SUCCESS='#50fa7b'

prism_git_status() {
  local branch
  branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  [[ -n "$branch" ]] && echo -e " \033[38;2;189;147;249m$branch\033[0m"
}

prism_prompt() {
  local status_seq="\[\033[38;2;80;250;123m\]"
  if [[ $? -ne 0 ]]; then
    status_seq="\[\033[38;2;255;85;85m\]"
  fi
  PS1="\n\[\033[38;2;255;121;198m\]◆ \[\033[38;2;139;233;253m\]\u \[\033[38;2;189;147;249m\]in \[\033[38;2;255;121;198m\]\W\$(prism_git_status)\n${status_seq}❯ \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
