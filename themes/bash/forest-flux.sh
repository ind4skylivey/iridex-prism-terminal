# Forest-Flux Bash prompt
forest_flux_git() {
  local branch dirty
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  if [[ -n $branch ]]; then
    if [[ -n $dirty ]]; then
      printf "%s *" "$branch"
    else
      printf "%s" "$branch"
    fi
  fi
}

forest_flux_prompt() {
  local exit_status=$?
  local status_color="\[\e[32m\]"
  local symbol="✔"
  if [[ $exit_status -ne 0 ]]; then
    status_color="\[\e[91m\]"
    symbol="✖"
  fi
  local time_seg=$(date +%H:%M)
  PS1="\[\e[32m\]╭ \[\e[36m\]time:${time_seg} \[\e[33m\]• \[\e[32m\]\u@\h\n"
  PS1+="\[\e[32m\]╰ \[\e[33m\]\w \[\e[34m\]$(forest_flux_git) ${status_color}${symbol}${exit_status:+ ${exit_status}} \[\e[37m\]→ \[\e[0m\]"
}
PROMPT_COMMAND=forest_flux_prompt
