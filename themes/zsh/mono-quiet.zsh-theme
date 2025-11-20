# Mono-Quiet: ultra-minimal, restrained focus
mono_quiet_prompt() {
  local exit_status=$?
  local fg='%F{#f3f3f3}'
  local accent='%F{#9ad8ff}'
  local secondary='%F{#a3a3a3}'
  local success='%F{#80ffab}'
  local error='%F{#ff5c63}'
  local status_color=$success
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
  fi
  local branch
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  local dirty
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  local git_line=''
  if [[ -n $branch ]]; then
    git_line="${secondary}[${branch}${dirty:+*}]"
  fi
  PS1="${fg}%~ ${git_line:+${git_line} }${status_color}${exit_status:+${exit_status}} ${accent}» ${fg}"
}
autoload -U add-zsh-hook
add-zsh-hook precmd mono_quiet_prompt
