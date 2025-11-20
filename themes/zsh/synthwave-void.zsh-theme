# Synthwave-Void: neon block prompt
synthwave_void_prompt() {
  local exit_status=$?
  local primary='%F{#f4a7ff}'
  local secondary='%F{#62e8ff}'
  local accent='%F{#ffe066}'
  local fg='%F{#f8f8ff}'
  local success='%F{#93ff88}'
  local error='%F{#ff4e75}'
  local status_color=$success
  local symbol='★'
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
    symbol='✖'
  fi
  local branch
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  local dirty
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  local git_line=''
  if [[ -n $branch ]]; then
    git_line="${secondary}${branch}${dirty:+*}"
  fi
  PS1="${secondary}╭${accent}═${primary}◈ ${secondary}%n@%m ${accent} ${fg}%~\n"
  PS1+="${secondary}╰${accent}═ ${git_line:+${git_line} }${status_color}${symbol}${exit_status:+ ${exit_status}} ${accent}↝ ${fg}"
}
autoload -U add-zsh-hook
add-zsh-hook precmd synthwave_void_prompt
