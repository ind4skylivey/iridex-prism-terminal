# =============================================================================
# PRISM TERMINAL: Nebula-Mocha
# Description: Cozy cosmic theme with powerline bubbles
# Generated for: Bash 5.0+
# =============================================================================

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -en "\033[48;2;150;123;182m\033[38;2;245;245;220m 🌌 $branch \033[0m"
    
    if [[ -n "$dirty" ]]; then
      echo -en "\033[38;2;210;180;140m☕ \033[0m"
    fi
  fi
}

prism_prompt() {
  local right_sep=""
  
  PS1="\n"
  
  # Segment 1: Cloud Icon (Coffee bg)
  PS1+="\[\033[38;2;111;78;55m\]$right_sep"
  PS1+="\[\033[48;2;111;78;55m\033[38;2;245;245;220m\] ☁ \033[0m"
  
  # Segment 2: User (Nebula bg)
  PS1+="\[\033[48;2;150;123;182m\033[38;2;111;78;55m\]$right_sep"
  PS1+="\[\033[48;2;150;123;182m\033[38;2;245;245;220m\] \u \033[0m"
  
  # Segment 3: Directory (Cream bg)
  PS1+="\[\033[48;2;210;180;140m\033[38;2;150;123;182m\]$right_sep"
  PS1+="\[\033[48;2;210;180;140m\033[38;2;111;78;55m\] \W \033[0m"
  
  # Segment 4: Git (Nebula bg)
  PS1+="\[\033[48;2;150;123;182m\033[38;2;210;180;140m\]$right_sep"
  PS1+="\$(prism_git_status)"
  
  # End
  PS1+="\[\033[38;2;150;123;182m\]$right_sep\033[0m"
  
  PS1+="\n"
  
  # Prompt line
  PS1+="\[\$(if [ \$? -eq 0 ]; then echo '\033[38;2;150;123;182m⋆｡°✩'; else echo '\033[38;2;111;78;55m☾'; fi)\] \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
