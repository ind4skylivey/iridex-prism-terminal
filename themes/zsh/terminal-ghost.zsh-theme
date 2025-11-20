# =============================================================================
# PRISM TERMINAL: Terminal-Ghost
# Description: Minimal ghostly theme with subtle bubble segments
# Generated for: Zsh 5.8+
# =============================================================================

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -n "%K{#4a4a4a}%F{#e0e0e0}  $branch %k%f"
    
    if [[ -n "$dirty" ]]; then
      echo -n "%F{#ffaa00}● %f"
    fi
  fi
}

prism_prompt() {
  local right_sep=""
  
  echo
  echo -n "%F{#2a2a2a}$right_sep%K{#2a2a2a}%F{#e0e0e0} 👻 %k%f"
  echo -n "%K{#4a4a4a}%F{#2a2a2a}$right_sep%K{#4a4a4a}%F{#f0f0f0} %1~ %k%f"
  echo -n "%K{#3a3a3a}%F{#4a4a4a}$right_sep%K{#3a3a3a}$(prism_git_status)"
  echo -n "%F{#3a3a3a}$right_sep%f"
  echo
  echo -n "%(?.%F{#808080}❯.%F{#ff5555}❯) %f"
}

setopt PROMPT_SUBST
PROMPT='$(prism_prompt)'
