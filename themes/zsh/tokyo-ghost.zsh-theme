# Tokyo-Ghost: nocturnal vaporwave ninja
zstyle ':precmd' python 'pass'

tokyo_ghost_prompt() {
  local exit_status=$?
  local foreground='%F{#e3d9ff}'
  local accent='%F{#3ae5ff}'
  local zombie='%F{#7472a2}'
  local status_symbol='❯'
  local status_color='%F{#74ffea}'
  if [[ $exit_status -ne 0 ]]; then
    status_symbol='⚡'
    status_color='%F{#ff4b6e}'
  fi
  local branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  local dirty
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  local git_icon='󱩎'
  local git_line=""
  if [[ -n $branch ]]; then
    git_line="${git_icon} ${branch}${dirty:+ ●}"
  fi
  PS1="${foreground}%n@%m ${zombie}${status_symbol}${status_color} ${accent}%~ ${git_line:+${zombie}${git_line} }${foreground}\n"
  PS1+="${accent}❯ ${status_color}${exit_status} ${foreground}"
}
add-zsh-hook precmd tokyo_ghost_prompt
