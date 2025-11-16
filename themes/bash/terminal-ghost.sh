# Terminal-Ghost Bash prompt
terminal_ghost_git() {
  local branch
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n $branch ]]; then
    printf "[%s]" "$branch"
  fi
}

terminal_ghost_prompt() {
  local exit_status=$?
  local status_color="\[\e[32m\]"
  local symbol="●"
  if [[ $exit_status -ne 0 ]]; then
    status_color="\[\e[91m\]"
    symbol="✦"
  fi
  PS1="\[\e[37m\]\u@\h \[\e[35m\]\w\n"
  PS1+="\[\e[36m\]$(terminal_ghost_git) ${status_color}${symbol}${exit_status:+ ${exit_status}} \[\e[94m\]› \[\e[0m\]"
}
PROMPT_COMMAND=terminal_ghost_prompt
