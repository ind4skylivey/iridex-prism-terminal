# Cyber-Noir Bash prompt
cyber_noir_battery() {
  if [[ -f /sys/class/power_supply/BAT0/capacity ]]; then
    cat /sys/class/power_supply/BAT0/capacity 2>/dev/null
  fi
}

cyber_noir_load() {
  uptime | awk -F'load average:' '{print $2}' | cut -d',' -f1 | awk '{print $1}'
}

cyber_noir_git() {
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

cyber_noir_docker() {
  if command -v docker >/dev/null 2>&1 && docker info >/dev/null 2>&1; then
    printf ""
  fi
}

cyber_noir_prompt() {
  local exit_status=$?
  local primary="\[\e[35m\]"
  local secondary="\[\e[36m\]"
  local accent="\[\e[33m\]"
  local success="\[\e[32m\]"
  local error="\[\e[91m\]"
  local status_color=$success
  local status_symbol="✔"
  if [[ $exit_status -ne 0 ]]; then
    status_color=$error
    status_symbol="✖"
  fi
  local time_seg=$(date +%H:%M)
  local load_seg=$(cyber_noir_load)
  local battery=$(cyber_noir_battery)
  local git_line=$(cyber_noir_git)
  local docker_line=$(cyber_noir_docker)
  PS1="${secondary}╭─ ${primary}time:${time_seg} ${accent}• ${secondary}load:${load_seg} ${accent}• ${secondary}${battery:+bat:${battery}} ${accent}${docker_line:+${docker_line}}\n"
  PS1+="${secondary}╰─ ${primary}\u@\h ${accent} \w ${git_line:+${secondary}(${git_line}) }${status_color}${status_symbol}${exit_status:+ ${exit_status}} ${accent}↺ ${primary}"
}
PROMPT_COMMAND=cyber_noir_prompt
