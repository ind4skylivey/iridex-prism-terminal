# Glitch-Grid Bash prompt
glitch_grid_git() {
  local branch dirty
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  if [[ -n $branch ]]; then
    if [[ -n $dirty ]]; then
      printf "%s ●" "$branch"
    else
      printf "%s" "$branch"
    fi
  fi
}

glitch_grid_prompt() {
  local exit_status=$?
  local primary="\[\e[35m\]"
  local accent="\[\e[33m\]"
  local secondary="\[\e[36m\]"
  local error="\[\e[41m\]\[\e[37m\]"
  local success="\[\e[32m\]"
  local symbol="⚡"
  if [[ $exit_status -ne 0 ]]; then
    printf '%s GLITCH ERROR %s\n' $error $primary
    symbol="✖"
  else
    printf '%s GRID MODE %s\n' $primary $accent
  fi
  PS1="${secondary}▌ ${success}%~ ${accent}$(glitch_grid_git) ${symbol}${exit_status:+ ${exit_status}} \[\e[0m\]"
}
PROMPT_COMMAND=glitch_grid_prompt
