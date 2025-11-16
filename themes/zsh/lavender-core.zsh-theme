# Lavender-Core: ethereal anime-tech prompt
lavender_core_colors() {
  local primary='%F{#f6d0ff}'
  local secondary='%F{#c9b6ff}'
  local accent='%F{#82f7ff}'
  local bg='%K{#1c1830}%f'
  local fg='%F{#f8f6ff}'
  local success='%F{#a8ff8c}'
  local error='%F{#ff6f91}'
  echo "$primary" "$secondary" "$accent" "$fg" "$success" "$error"
}
lavender_core_git() {
  local branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n $branch ]]; then
    local dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
    if [[ -n $dirty ]]; then
      echo "❧ ${branch} ⚡"
    else
      echo "❧ ${branch}"
    fi
  fi
}
lavender_core_prompt() {
  local -a colors=($(lavender_core_colors))
  local primary=${colors[1]}
  local secondary=${colors[2]}
  local accent=${colors[3]}
  local fg=${colors[4]}
  local success=${colors[5]}
  local error=${colors[6]}
  local exit_status=$?
  local status_color=$success
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
  fi
  local branch=$(lavender_core_git)
  PS1="%{$primary%}${fg}◆ %{$secondary%}%n@%m %{$accent%}⟆ %{$fg%}%~"
  PS1+="\n%{$secondary%}${branch:+${branch} }%{$status_color%}${exit_status} %{$primary%}→ %{$fg%}"
}
autoload -U add-zsh-hook
add-zsh-hook precmd lavender_core_prompt
