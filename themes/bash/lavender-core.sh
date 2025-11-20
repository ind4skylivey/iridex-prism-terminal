# =============================================================================
# PRISM TERMINAL: Lavender-Core
# Description: Elegant anime-tech with powerline bubbles
# Generated for: Bash 5.0+
# =============================================================================

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -en "\033[48;2;181;126;220m\033[38;2;26;26;46m  $branch \033[0m"
    
    if [[ -n "$dirty" ]]; then
      echo -en "\033[38;2;255;105;180m✦ \033[0m"
    fi
  fi
}

prism_prompt() {
  local right_sep=""
  
  PS1="\n"
  
  # Segment 1: Icon (Deep Purple bg)
  PS1+="\[\033[38;2;106;76;147m\]$right_sep"
  PS1+="\[\033[48;2;106;76;147m\033[38;2;255;255;255m\] ⚜ \033[0m"
  
  # Segment 2: User (Lavender bg)
  PS1+="\[\033[48;2;181;126;220m\033[38;2;106;76;147m\]$right_sep"
  PS1+="\[\033[48;2;181;126;220m\033[38;2;26;26;46m\] \u \033[0m"
  
  # Segment 3: Directory (Pink bg)
  PS1+="\[\033[48;2;255;154;162m\033[38;2;181;126;220m\]$right_sep"
  PS1+="\[\033[48;2;255;154;162m\033[38;2;26;26;46m\] \W \033[0m"
  
  # Segment 4: Git (Lavender bg)
  PS1+="\[\033[48;2;181;126;220m\033[38;2;255;154;162m\]$right_sep"
  PS1+="\$(prism_git_status)"
  
  # End
  PS1+="\[\033[38;2;181;126;220m\]$right_sep\033[0m"
  
  PS1+="\n"
  
  # Prompt line
  PS1+="\[\$(if [ \$? -eq 0 ]; then echo '\033[38;2;181;126;220m'; else echo '\033[38;2;255;0;0m'; fi)\]⚛ \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
