# Nebula-Mocha: pastel powerline stream
nebula_mocha_prompt() {
  local exit_status=$?
  local primary='%F{#ddb6f2}'
  local secondary='%F{#a6adcd}'
  local accent='%F{#f5c2e7}'
  local fg='%F{#f5e0dc}'
  local success='%F{#c3e88d}'
  local error='%F{#ff6f91}'
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
    git_line="${accent} ${branch}${dirty:+ ✦}"
  fi
  PS1="${secondary}╭─ ${primary}%n@%m ${accent} ${fg}%~\n"
  PS1+="${secondary}╰─ ${git_line:+${git_line} }${status_color}${exit_status} ${fg}→ ${fg}"
}
add-zsh-hook precmd nebula_mocha_prompt
