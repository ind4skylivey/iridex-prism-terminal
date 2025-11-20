# =============================================================================
# PRISM TERMINAL: Matrix-Shade
# Description: Elite hacker terminal - cybersecurity aesthetics
# Generated for: Zsh 5.8+
# =============================================================================

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -n "%K{black}%F{green}[GIT::%B$branch%b%F{green}"
    
    if [[ -n "$dirty" ]]; then
      echo -n "%F{red}::MODIFIED"
    else
      echo -n "%F{green}::CLEAN"
    fi
    
    echo -n "]%k%f"
  fi
}

prism_prompt() {
  echo
  echo -n "%K{black}%F{green}"
  
  # Status indicator
  echo -n "%(?.%F{green}[●SECURE] .%F{red}[✖BREACH] %F{green})"
  
  # User@Host
  echo -n "[ROOT::%B%n@%m%b] "
  
  # Directory
  echo -n "[PATH::%B%F{white}%1~%F{green}%b] "
  
  # Git
  echo -n "$(prism_git_status)"
  
  echo -n "%k%f"
  echo
  
  # Line 2: Command Input
  echo -n "%F{green}┌─[%B%F{green}TERMINAL%b%F{green}]"
  echo
  echo -n "└─► %f"
}

setopt PROMPT_SUBST
PROMPT='$(prism_prompt)'
