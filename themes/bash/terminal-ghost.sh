# =============================================================================
# PRISM TERMINAL: Terminal-Ghost
# Description: Minimal ghostly theme with subtle bubble segments
# Generated for: Bash 5.0+
# =============================================================================

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -en "\033[48;2;74;74;74m\033[38;2;224;224;224m  $branch \033[0m"
    
    if [[ -n "$dirty" ]]; then
      echo -en "\033[38;2;255;170;0m● \033[0m"
    fi
  fi
}

prism_prompt() {
  local right_sep=""
  
  PS1="\n"
  
  # Segment 1: Ghost Icon (Dark bg)
  PS1+="\[\033[38;2;42;42;42m\]$right_sep"
  PS1+="\[\033[48;2;42;42;42m\033[38;2;224;224;224m\] 👻 \033[0m"
  
  # Segment 2: Directory (Grey bg)
  PS1+="\[\033[48;2;74;74;74m\033[38;2;42;42;42m\]$right_sep"
  PS1+="\[\033[48;2;74;74;74m\033[38;2;240;240;240m\] \W \033[0m"
  
  # Segment 3: Git (Darker grey bg)
  PS1+="\[\033[48;2;58;58;58m\033[38;2;74;74;74m\]$right_sep"
  PS1+="\[\033[48;2;58;58;58m\]"
  PS1+="\$(prism_git_status)"
  
  # End
  PS1+="\[\033[38;2;58;58;58m\]$right_sep\033[0m"
  
  PS1+="\n"
  
  # Prompt line
  PS1+="\[\$(if [ \$? -eq 0 ]; then echo '\033[38;2;128;128;128m'; else echo '\033[38;2;255;85;85m'; fi)\]❯ \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
