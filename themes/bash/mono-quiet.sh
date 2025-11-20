# Mono-Quiet Bash prompt
mono_quiet_git() {
  local branch
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n $branch ]]; then
    local dirty
    dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
    if [[ -n $dirty ]]; then
      printf "[%s*]" "$branch"
    else
      printf "[%s]" "$branch"
    fi
  fi
}

mono_quiet_prompt() {
  local exit_status=$?
  local status_color="\[\e[32m\]"
  if [[ $exit_status -ne 0 ]]; then
    status_color="\[\e[91m\]"
  fi
  PS1="\[\e[37m\]\w ${status_color}${exit_status:+${exit_status}} \[\e[36m\]» \[\e[0m\]$(mono_quiet_git)"
}
PROMPT_COMMAND=mono_quiet_prompt
