# Nebula-Mocha Bash prompt
nebula_mocha_git() {
  local branch dirty
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  if [[ -n $branch ]]; then
    if [[ -n $dirty ]]; then
      printf " %s ✦" "$branch"
    else
      printf " %s" "$branch"
    fi
  fi
}

nebula_mocha_prompt() {
  local exit_status=$?
  local status_color="\[\e[32m\]"
  if [[ $exit_status -ne 0 ]]; then
    status_color="\[\e[91m\]"
  fi
  local git_line=$(nebula_mocha_git)
  PS1="\[\e[35m\]╭─ \[\e[96m\]\u@\h \[\e[33m\] \[\e[97m\]\w\n"
  PS1+="\[\e[35m\]╰─ ${git_line:+\[\e[33m\]$git_line }${status_color}${exit_status} \[\e[97m\]→ \[\e[0m\]"
}
PROMPT_COMMAND=nebula_mocha_prompt
