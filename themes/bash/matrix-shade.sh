# =============================================================================
# PRISM TERMINAL: Matrix-Shade
# Description: Elite hacker terminal - cybersecurity aesthetics
# Generated for: Bash 5.0+
# =============================================================================

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -en "\033[48;2;0;0;0m\033[38;2;0;255;0m[GIT::"
    echo -en "\033[1;38;2;0;255;0m$branch\033[0m"
    echo -en "\033[48;2;0;0;0m\033[38;2;0;255;0m"
    
    if [[ -n "$dirty" ]]; then
      echo -en "\033[38;2;255;0;0m::MODIFIED"
    else
      echo -en "\033[38;2;0;255;0m::CLEAN"
    fi
    
    echo -en "]\033[0m"
  fi
}

prism_prompt() {
  PS1="\n"
  
  # Line 1: System Status Bar
  PS1+="\[\033[48;2;0;0;0m\033[38;2;0;255;0m\]"
  
  # Status indicator
  PS1+="\$(if [ \$? -eq 0 ]; then echo '[●SECURE] '; else echo '\[\033[38;2;255;0;0m\][✖BREACH] \[\033[38;2;0;255;0m\]'; fi)"
  
  # User@Host
  PS1+="[ROOT::\[\033[1;38;2;0;255;0m\]\u@\h\[\033[0;48;2;0;0;0m\033[38;2;0;255;0m\]] "
  
  # Directory
  PS1+="[PATH::\[\033[1;38;2;255;255;255m\]\W\[\033[0;48;2;0;0;0m\033[38;2;0;255;0m\]] "
  
  # Git
  PS1+="\$(prism_git_status)"
  
  PS1+="\[\033[0m\]"
  
  PS1+="\n"
  
  # Line 2: Command Input
  PS1+="\[\033[38;2;0;255;0m\]┌─[\[\033[1;38;2;0;255;0m\]TERMINAL\[\033[0;38;2;0;255;0m\]]\n"
  PS1+="└─► \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
