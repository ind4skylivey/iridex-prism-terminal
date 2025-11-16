# Matrix-Shade: green-on-black prompt
matrix_shade_prompt() {
  local exit_status=$?
  local primary='%F{#7eff7e}'
  local secondary='%F{#2cf8b4}'
  local accent='%F{#6aecff}'
  local error='%F{#ff5f8f}'
  local success='%F{#9dfd82}'
  local status_color=$success
  local symbol='■'
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
    symbol='✖'
  fi
  local duration=$(($(date +%s) - ${LAST_COMMAND_START_TIME:-0}))
  local vi_mode=${KEYMAP:-main}
  local branch
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  local dirty
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  local git_line=''
  if [[ -n $branch ]]; then
    git_line="${secondary}${branch}${dirty:+*}"
  fi
  PS1="${primary}╔═ ${accent}%~ ${git_line:+(${git_line}) }${primary}║${symbol} ${status_color}${exit_status} ${secondary}[${vi_mode}] ${accent}${duration}s${primary}\n"
}
autoload -U add-zsh-hook
add-zsh-hook precmd matrix_shade_prompt
