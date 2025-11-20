# =============================================================================
# PRISM TERMINAL: Cyber-Noir
# Description: Neon-soaked cyberpunk theme with bubble segments
# Generated for: Bash 5.0+
# =============================================================================

# Colors
PRISM_PRIMARY='#ff5fe0'
PRISM_SECONDARY='#44ddff'
PRISM_ACCENT='#ffc94f'

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -en " \033[48;2;255;0;255m\033[38;2;0;0;0m  $branch \033[0m"
    if [[ -n "$dirty" ]]; then
      echo -en "\033[38;2;255;255;0m⚡ \033[0m"
    fi
  fi
}

prism_prompt() {
  local right_sep=""
  
  PS1="\n"
  # Segment 1: Time (Deep Purple bg)
  PS1+="\[\033[38;2;95;0;175m\]$right_sep"
  PS1+="\[\033[48;2;95;0;175m\033[38;2;0;255;255m\] \t \033[0m"
  
  # Segment 2: User@Host (Magenta bg)
  PS1+="\[\033[48;2;255;0;255m\033[38;2;95;0;175m\]$right_sep"
  PS1+="\[\033[48;2;255;0;255m\033[38;2;0;0;0m\] \u@\h \033[0m"
  
  # Segment 3: Directory (Cyan bg)
  PS1+="\[\033[48;2;0;255;255m\033[38;2;255;0;255m\]$right_sep"
  PS1+="\[\033[48;2;0;255;255m\033[38;2;0;0;0m\] \W \033[0m"
  
  # Segment 4: Git (Magenta bg)
  PS1+="\[\033[48;2;255;0;255m\033[38;2;0;255;255m\]$right_sep"
  PS1+="\$(prism_git_status)"
  
  # End
  PS1+="\[\033[38;2;255;0;255m\]$right_sep\033[0m"
  
  PS1+="\n"
  
  # Prompt line
  PS1+="\[\$(if [ \$? -eq 0 ]; then echo '\033[38;2;0;255;255m'; else echo '\033[38;2;255;0;0m'; fi)\]❯ \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
