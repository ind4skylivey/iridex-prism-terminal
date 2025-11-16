# ERROR_808 Bash prompt
error_808_git() {
  local branch
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n $branch ]]; then
    local dirty
    dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
    if [[ -n $dirty ]]; then
      printf "⚠ %s ✗" "$branch"
    else
      printf "⚠ %s" "$branch"
    fi
  fi
}

error_808_prompt() {
  local exit_status=$?
  local top='▒▓ ERROR_808 ▓▒'
  local left='⚠'
  local bars='▓▒'
  local git_line=$(error_808_git)
  PS1="\[\e[31m\]${top}\[\e[0m\]\n"
  PS1+="\[\e[33m\]${left} \[\e[97m\]\u@\h \[\e[36m\]\w \[\e[31m\]${bars} "
  PS1+="${git_line:+${git_line} }"
  if [[ $exit_status -eq 0 ]]; then
    PS1+="\[\e[32m\]✔"
  else
    PS1+="\[\e[91m\]✖ (${exit_status})"
  fi
  PS1+=" \[\e[33m\]${bars}\[\e[0m\] \n"
}
PROMPT_COMMAND=error_808_prompt
