# Theme Template (copy & rename when adding personalities)
template_git() {
  local branch
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n $branch ]]; then
    local dirty
    dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
    if [[ -n $dirty ]]; then
      printf "✦ %s ⚡" "$branch"
    else
      printf "✦ %s" "$branch"
    fi
  fi
}

template_prompt() {
  local exit_status=$?
  local status_color="\[\e[32m\]"
  if [[ $exit_status -ne 0 ]]; then
    status_color="\[\e[91m\]"
  fi
  PS1="\[\e[95m\]Template \[\e[96m\]\u@\h\n"
  PS1+="\[\e[94m\]➤ \[\e[33m\]\w \[\e[92m\]$(template_git)\n"
  PS1+="${status_color}${exit_status} \[\e[97m\]➜ \[\e[0m\]"
}
PROMPT_COMMAND=template_prompt
