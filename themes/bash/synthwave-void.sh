# =============================================================================
# PRISM TERMINAL: Synthwave-Void
# Description: Retro-cyberpunk sunset with neon grid aesthetics
# Generated for: Bash 5.0+
# =============================================================================

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -en "\033[48;2;255;0;255m\033[38;2;0;0;0m  $branch \033[0m"
    
    if [[ -n "$dirty" ]]; then
      echo -en "\033[38;2;255;255;0m⚡ \033[0m"
    fi
  fi
}

prism_prompt() {
  PS1="\n"
  
  # Line 1: Retro Grid with Sunset
  # Neon pink grid
  PS1+="\[\033[38;2;255;0;255m\]▓▒░ "
  
  # Sunset emoji on dark bg
  PS1+="\[\033[48;2;26;0;51m\033[38;2;255;255;0m\] 🌆 \033[0m"
  
  # User segment (cyan neon)
  PS1+="\[\033[48;2;0;255;255m\033[38;2;0;0;0m\] \u \033[0m"
  
  PS1+="\[\033[38;2;255;0;255m\] ▸ "
  
  # Directory (magenta neon)
  PS1+="\[\033[48;2;255;0;255m\033[38;2;0;0;0m\] \W \033[0m"
  
  # Git
  PS1+="\$(prism_git_status)"
  
  # Grid decoration
  PS1+="\[\033[38;2;255;0;255m\] ░▒▓"
  
  PS1+="\n"
  
  # Line 2: Neon Prompt
  PS1+="\[\033[38;2;255;0;255m\]╰─"
  PS1+="\[\033[38;2;0;255;255m\]═"
  PS1+="\[\033[38;2;255;255;0m\]═"
  PS1+="\[\033[38;2;255;0;255m\]► "
  
  # Status indicator
  PS1+="\[\$(if [ \$? -eq 0 ]; then echo '\033[38;2;0;255;255m◆'; else echo '\033[38;2;255;0;0m✖'; fi)\] \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
