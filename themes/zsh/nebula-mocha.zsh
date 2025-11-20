# =============================================================================
# PRISM TERMINAL: Nebula-Mocha
# Description: Cozy cosmic theme with powerline bubbles
# Generated for: Zsh 5.8+
# =============================================================================

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -n "%K{#967bb6}%F{#f5f5dc} 🌌 $branch %k%f"
    
    if [[ -n "$dirty" ]]; then
      echo -n "%F{#d2b48c}☕ %f"
    fi
  fi
}

prism_prompt() {
  local right_sep=""
  
  echo
  echo -n "%F{#6f4e37}$right_sep%K{#6f4e37}%F{#f5f5dc} ☁ %k%f"
  echo -n "%K{#967bb6}%F{#6f4e37}$right_sep%K{#967bb6}%F{#f5f5dc} %n %k%f"
  echo -n "%K{#d2b48c}%F{#967bb6}$right_sep%K{#d2b48c}%F{#6f4e37} %1~ %k%f"
  echo -n "%K{#967bb6}%F{#d2b48c}$right_sep$(prism_git_status)"
  echo -n "%F{#967bb6}$right_sep%f"
  echo
  echo -n "%(?.%F{#967bb6}⋆｡°✩.%F{#6f4e37}☾) %f"
}

setopt PROMPT_SUBST
PROMPT='$(prism_prompt)'
