# Aurora-Edge: project-state focus
aurora_edge_docker() {
  command -v docker >/dev/null 2>&1 && docker info >/dev/null 2>&1 && echo '🐳'
}

aurora_edge_k8s() {
  command -v kubectl >/dev/null 2>&1 && kubectl config current-context 2>/dev/null && echo '☸'
}

aurora_edge_prompt() {
  local exit_status=$?
  local primary='%F{#a5eaff}'
  local secondary='%F{#8fb5ff}'
  local accent='%F{#ff9ece}'
  local fg='%F{#f0f8ff}'
  local success='%F{#80ffb8}'
  local error='%F{#ff4c7c}'
  local status_color=$success
  local symbol='✔'
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
  local docker_line=$(aurora_edge_docker)
  local k8s_line=$(aurora_edge_k8s)
  PS1="${secondary}╭ ${primary}%n@%m ${accent} ${fg}%~\n"
  PS1+="${secondary}╰ (${git_line:-no-git}) ${docker_line:+${accent}${docker_line}} ${k8s_line:+${accent}${k8s_line}} ${status_color}${symbol}${exit_status:+ ${exit_status}} ${fg}→"
}
autoload -U add-zsh-hook
add-zsh-hook precmd aurora_edge_prompt
