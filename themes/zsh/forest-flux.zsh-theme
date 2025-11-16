# Forest-Flux: calm greens + amber separators
forest_flux_prompt() {
  local exit_status=$?
  local primary='%F{#a4c29a}'
  local secondary='%F{#6f8c4a}'
  local accent='%F{#f7c97f}'
  local fg='%F{#e8f5dd}'
  local error='%F{#ff8a65}'
  local success='%F{#8bf397}'
  local status_color=$success
  local symbol='✔'
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
    symbol='✖'
  fi
  local time_seg=$(date +%H:%M)
  local branch
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  local dirty
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  local git_line=''
  if [[ -n $branch ]]; then
    git_line="${secondary}${branch}${dirty:+*}"
  fi
  PS1="${secondary}╭ ${primary}%n@%m ${accent}• ${secondary}%~\n"
  PS1+="${secondary}╰ ${accent}${time_seg} ${git_line:+(${git_line}) }${status_color}${symbol}${exit_status:+ ${exit_status}} ${fg}→ ${fg}"
}
autoload -U add-zsh-hook
add-zsh-hook precmd forest_flux_prompt
