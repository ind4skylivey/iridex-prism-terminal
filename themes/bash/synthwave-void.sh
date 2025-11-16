# Synthwave-Void Bash prompt
synthwave_void_git() {
  local branch dirty
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  if [[ -n $branch ]]; then
    if [[ -n $dirty ]]; then
      printf " %s *" "$branch"
    else
      printf " %s" "$branch"
    fi
  fi
}

synthwave_void_prompt() {
  local exit_status=$?
  local primary="\[\e[35m\]"
  local secondary="\[\e[36m\]"
  local accent="\[\e[33m\]"
  local success="\[\e[32m\]"
  local error="\[\e[91m\]"
  local status_color=$success
  local symbol="★"
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
    symbol="✖"
  fi
  local branch_line=$(synthwave_void_git)
  PS1="${secondary}╭${accent}═${primary}◈ ${secondary}%n@%m ${accent} \w\n"
  PS1+="${secondary}╰${accent}═ ${branch_line:+${branch_line} }${status_color}${symbol}${exit_status:+ ${exit_status}} ${accent}↝ ${primary}"
}
PROMPT_COMMAND=synthwave_void_prompt
