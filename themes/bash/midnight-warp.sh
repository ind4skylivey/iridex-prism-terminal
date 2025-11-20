# Midnight-Warp Bash prompt
midnight_warp_git() {
  local branch dirty
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  if [[ -n $branch ]]; then
    if [[ -n $dirty ]]; then
      printf "[%s*]" "$branch"
    else
      printf "[%s]" "$branch"
    fi
  fi
}

midnight_warp_prompt() {
  local exit_status=$?
  local status_color="\[\e[32m\]"
  local symbol="✔"
  if [[ $exit_status -ne 0 ]]; then
    status_color="\[\e[91m\]"
    symbol="✖"
  fi
  local time_seg=$(date +%H:%M)
  PS1="\[\e[36m\]╭ \[\e[35m\]time:${time_seg} \[\e[34m\]\u@\h\n"
  PS1+="\[\e[36m\]╰ \[\e[37m\]\w \[\e[33m\]$(midnight_warp_git) ${status_color}${symbol}${exit_status:+ ${exit_status}} \[\e[36m\]› \[\e[0m\]"
}
PROMPT_COMMAND=midnight_warp_prompt
