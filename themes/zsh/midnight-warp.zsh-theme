# Midnight-Warp: sleek modern UI with right-aligned info
midnight_warp_prompt() {
  local exit_status=$?
  local primary='%F{#8bd9ff}'
  local secondary='%F{#576bff}'
  local accent='%F{#d38cff}'
  local fg='%F{#f6f9ff}'
  local success='%F{#7ef3a2}'
  local error='%F{#ff5e7a}'
  local status_color=$success
  local symbol='✔'
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
    symbol='✖'
  fi
  local branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  local dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  local git_line=''
  if [[ -n $branch ]]; then
    git_line="(${branch}${dirty:+*})"
  fi
  local time_seg=$(date +%H:%M)
  RPS1="${secondary}[${accent}time ${time_seg}${secondary}]"
  PS1="${primary}%n@%m ${accent} ${fg}%~ ${git_line} ${status_color}${symbol}${exit_status:+ ${exit_status}} ${fg}→ \n"
}
autoload -U add-zsh-hook
add-zsh-hook precmd midnight_warp_prompt
