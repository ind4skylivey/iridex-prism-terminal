# Theme Template (copy and rename for new personality)
# Replace `Template` with the theme name, update colors, glyphs and vibes.
template_prompt() {
  local name='Template'
  local primary='%F{#FF0000}'
  local secondary='%F{#00FF00}'
  local accent='%F{#0000FF}'
  local fg='%F{#F0F0F0}'
  local error='%F{#FF4500}'
  local success='%F{#32CD32}'
  local exit_status=$?
  local status_color=$success
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
  fi
  local branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  local dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  local git_line=""
  if [[ -n $branch ]]; then
    git_line="✦ ${branch}${dirty:+ ⚡}"
  fi
  PS1="${primary}%n@%m ${secondary}» ${accent}%~"
  PS1+="\n${secondary}${git_line:+${git_line} }${status_color}${exit_status} ${primary}› ${fg}"
}
autoload -U add-zsh-hook
add-zsh-hook precmd template_prompt
