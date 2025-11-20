# Cyber-Noir: neon-heavy multi-segment prompt
cyber_noir_battery() {
  if [[ -f /sys/class/power_supply/BAT0/capacity ]]; then
    cat /sys/class/power_supply/BAT0/capacity 2>/dev/null | tr -d '\n' && echo '%'
  elif command -v pmset >/dev/null 2>&1; then
    pmset -g batt | awk '/%/ {print $3}'
  fi
}

cyber_noir_docker() {
  if command -v docker >/dev/null 2>&1 && docker info >/dev/null 2>&1; then
    echo '🐳'
  fi
}

cyber_noir_load() {
  uptime | awk -F'load average:' '{print $2}' | cut -d',' -f1 | awk '{print $1}'
}

cyber_noir_prompt() {
  local exit_status=$?
  local primary='%F{#ff5fe0}'
  local secondary='%F{#44ddff}'
  local accent='%F{#ffc94f}'
  local fg='%F{#f0f5ff}'
  local success='%F{#8dff6e}'
  local error='%F{#ff3860}'
  local status_color=$success
  local status_symbol='✔'
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
    status_symbol='✖'
  fi
  local time_seg=$(date +%H:%M)
  local load_seg=$(cyber_noir_load)
  local battery=$(cyber_noir_battery)
  local docker_line=$(cyber_noir_docker)
  local branch
  branch=$(git -C "$PWD" symbolic-ref --short HEAD 2>/dev/null)
  local dirty
  dirty=$(git -C "$PWD" status --porcelain 2>/dev/null)
  local git_line=''
  if [[ -n $branch ]]; then
    git_line="${secondary}${branch}${dirty:+*}"
  fi
  PS1="${secondary}╭─ ${primary}${time_seg} ${accent}• ${secondary}load:${load_seg}${accent}• ${secondary}${battery:+bat:${battery}}${accent}${docker_line:+ • ${docker_line}}${fg}\n"
  PS1+="${secondary}╰─ ${primary}%n@%m ${accent} ${fg}%~ ${git_line:+(${git_line}) }${status_color}${status_symbol}${exit_status:+ ${exit_status}} ${primary}↺ ${fg}"
}
add-zsh-hook precmd cyber_noir_prompt
