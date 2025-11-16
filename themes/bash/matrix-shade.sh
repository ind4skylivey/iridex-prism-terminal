# Matrix-Shade Bash prompt
matrix_shade_precmd() {
  MATRIX_SHADE_START=$(date +%s)
}
trap 'matrix_shade_precmd' DEBUG

matrix_shade_git() {
  local branch dirty
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  if [[ -n $branch ]]; then
    if [[ -n $dirty ]]; then
      printf "%s*" "$branch"
    else
      printf "%s" "$branch"
    fi
  else
    printf "no-git"
  fi
}

matrix_shade_duration() {
  local now=$(date +%s)
  if [[ -n $MATRIX_SHADE_START ]]; then
    echo "${now:-0} - ${MATRIX_SHADE_START}" | bc
  else
    echo 0
  fi
}

matrix_shade_prompt() {
  local exit_status=$?
  local status_color="\[\e[32m\]"
  local symbol="■"
  if [[ $exit_status -ne 0 ]]; then
    status_color="\[\e[91m\]"
    symbol="✖"
  fi
  local duration=$(matrix_shade_duration)
  PS1="\[\e[32m\]╔═ \[\e[36m\]\u@\h \[\e[32m\]$(matrix_shade_git)\n"
  PS1+="\[\e[32m\]╚═ ${status_color}${symbol} ${exit_status} \[\e[35m\][${duration}s] \[\e[0m\]"
}
PROMPT_COMMAND=matrix_shade_prompt
