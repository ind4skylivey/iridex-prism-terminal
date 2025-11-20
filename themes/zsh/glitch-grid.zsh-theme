# Glitch-Grid: blocky warning prompt
glitch_grid_prompt() {
  local exit_status=$?
  local primary='%F{#ff55ff}'
  local secondary='%F{#0effd7}'
  local accent='%F{#f1ff5b}'
  local fg='%F{#f1f1f1}'
  local error='%F{#ff1a57}'
  local success='%F{#75ff8d}'
  local status_color=$success
  local symbol='⚡'
  local invert='%K{#ff1a57}%F{#000000}'
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
    symbol='✖'
    PS1="$invert GLITCH \n"
  else
    PS1="${primary} ╭─"
  fi
  local branch
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  local dirty
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  local git_line=""
  if [[ -n $branch ]]; then
    git_line="${secondary}${branch}${dirty:+ ●}"
  fi
  PS1+=" ${accent}%~ ${git_line} ${status_color}${symbol}${exit_status:+ ${exit_status}} ${fg}"
}
autoload -U add-zsh-hook
add-zsh-hook precmd glitch_grid_prompt
