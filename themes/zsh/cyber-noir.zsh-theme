# =============================================================================
# PRISM TERMINAL: Cyber-Noir
# Description: Neon-soaked cyberpunk theme with bubble segments
# Generated for: Zsh 5.8+
# =============================================================================

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -n " %K{magenta}%F{black}  $branch %k%f"
    if [[ -n "$dirty" ]]; then
      echo -n "%F{yellow}⚡ %f"
    fi
  fi
}

prism_prompt() {
  local right_sep=""
  
  echo
  echo -n "%F{#5f00af}$right_sep%K{#5f00af}%F{cyan} %T %k%f"
  echo -n "%K{magenta}%F{#5f00af}$right_sep%K{magenta}%F{black} %n@%m %k%f"
  echo -n "%K{cyan}%F{magenta}$right_sep%K{cyan}%F{black} %1~ %k%f"
  echo -n "%K{magenta}%F{cyan}$right_sep$(prism_git_status)"
  echo -n "%F{magenta}$right_sep%f"
  echo
  echo -n "%(?.%F{cyan}❯.%F{red}❯) %f"
}

setopt PROMPT_SUBST
PROMPT='$(prism_prompt)'
