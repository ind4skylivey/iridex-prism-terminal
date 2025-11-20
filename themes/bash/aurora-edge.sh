# Aurora-Edge Bash prompt
aurora_edge_git() {
  local branch dirty
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  if [[ -n $branch ]]; then
    if [[ -n $dirty ]]; then
      printf "[%s*]" "$branch"
    else
      printf "[%s]" "$branch"
    fi
  else
    printf "[no-git]"
  fi
}

aurora_edge_docker() {
  if command -v docker >/dev/null 2>&1 && docker info >/dev/null 2>&1; then
    printf "🐳"
  else
    printf "no-docker"
  fi
}

aurora_edge_k8s() {
  if command -v kubectl >/dev/null 2>&1; then
    local ctx
    ctx=$(kubectl config current-context 2>/dev/null)
    if [[ -n $ctx ]]; then
      printf "☸ %s" "$ctx"
    else
      printf "no-k8s"
    fi
  else
    printf "no-k8s"
  fi
}

aurora_edge_prompt() {
  local exit_status=$?
  local primary="\[\e[36m\]"
  local secondary="\[\e[34m\]"
  local accent="\[\e[35m\]"
  local success="\[\e[32m\]"
  local error="\[\e[91m\]"
  local status_color=$success
  local symbol="✔"
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
    symbol="✖"
  fi
  PS1="${secondary}╭ ${primary}\u@\h ${accent}| ${secondary}\w ${accent}| git ${secondary}$(aurora_edge_git)\n"
  PS1+="${secondary}╰ ${accent}docker ${secondary}$(aurora_edge_docker) ${accent}k8s ${secondary}$(aurora_edge_k8s) ${status_color}${symbol}${exit_status:+ ${exit_status}} ${primary}» \[\e[0m\]"
}
PROMPT_COMMAND=aurora_edge_prompt
