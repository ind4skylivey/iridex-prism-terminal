# Tokyo-Ghost Bash prompt
tokyo_ghost_git() {
  local branch dirty
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  if [[ -n $branch ]]; then
    if [[ -n $dirty ]]; then
      printf "󱩎 %s ●" "$branch"
    else
      printf "󱩎 %s" "$branch"
    fi
  fi
}

tokyo_ghost_prompt() {
  local exit_status=$?
  local marker="❯"
  local status_color="\[\e[36m\]"
  if [[ $exit_status -ne 0 ]]; then
    marker="⚡"
    status_color="\[\e[91m\]"
  fi
  local git_line=$(tokyo_ghost_git)
  PS1="\[\e[34m\]⟐ \[\e[97m\]\u@\h \[\e[36m\]${marker} \[\e[35m\]\w"
  PS1+=" ${git_line:+\[\e[35m\]$git_line} ${status_color}${exit_status} \[\e[34m\]${marker} \[\e[0m\]"
}
PROMPT_COMMAND=tokyo_ghost_prompt
