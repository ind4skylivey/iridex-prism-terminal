# Terminal-Ghost: ultra-cool monochrome prompt
terminal_ghost_prompt() {
  local exit_status=$?
  local primary='%F{#b3c1d4}'
  local secondary='%F{#899ca6}'
  local accent='%F{#5dd2ff}'
  local fg='%F{#dfe6f0}'
  local success='%F{#7ef79d}'
  local error='%F{#ff5f87}'
  local status_color=$success
  local symbol='●'
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
    symbol='✦'
  fi
  local branch
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  local git_line=''
  if [[ -n $branch ]]; then
    git_line="${accent}[${branch}]"
  fi
  PS1="${primary}%n@%m ${secondary}%~${git_line:+ ${git_line}} ${status_color}${symbol}${exit_status:+ ${exit_status}} ${accent}› ${fg}"
}
autoload -U add-zsh-hook
add-zsh-hook precmd terminal_ghost_prompt
