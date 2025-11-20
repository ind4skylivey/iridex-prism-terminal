# ERROR_808: glitchy warning prompt
error_808_prompt() {
  local exit_status=$?
  local fg='%F{#f6f6f6}'
  local accent='%F{#ff3f55}'
  local warning='%F{#ffd866}'
  local success='%F{#19ff8c}'
  local branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  local dirty
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  local git_warn=""
  if [[ -n $branch ]]; then
    git_warn="${warning}⚠ ${branch}${dirty:+ ✗}"
  fi
  local signal='▒▓ '
  PS1="${accent}⚠ ${fg}%n@%m ${signal}${accent}%~\n"
  PS1+="${git_warn}${signal}${exit_status:+ ${accent}${exit_status}}"
  if [[ $exit_status -eq 0 ]]; then
    PS1+="${success} ✓"
  else
    PS1+="${accent} ✖"
  fi
  PS1+="%f "
  PS1+="${warning}⟆  "
}
add-zsh-hook precmd error_808_prompt
