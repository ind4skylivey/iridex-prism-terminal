# Lavender-Core Bash prompt
lavender_core_git() {
  local branch
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n $branch ]]; then
    local dirty
    dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
    if [[ -n $dirty ]]; then
      printf "❧ %s ⚡" "$branch"
    else
      printf "❧ %s" "$branch"
    fi
  fi
}

lavender_core_prompt() {
  local exit_status=$?
  local status_color="\[\e[32m\]"
  if [[ $exit_status -ne 0 ]]; then
    status_color="\[\e[91m\]"
  fi
  local branch=$(lavender_core_git)
  PS1="\[\e[35m\]◆ \[\e[96m\]\u@\h \[\e[94m\]⟆ \[\e[97m\]\w\n"
  PS1+="\[\e[95m\]${branch:+${branch} }${status_color}(${exit_status})\[\e[95m\] → \[\e[97m\]"
}
PROMPT_COMMAND=lavender_core_prompt
