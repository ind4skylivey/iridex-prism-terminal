# =============================================================================
# PRISM TERMINAL: Lavender-Core
# Description: Elegant anime-tech with powerline bubbles
# Generated for: Zsh 5.8+
# =============================================================================

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -n "%K{#b57edc}%F{#1a1a2e}  $branch %k%f"
    
    if [[ -n "$dirty" ]]; then
      echo -n "%F{#ff69b4}✦ %f"
    fi
  fi
}

prism_prompt() {
  local right_sep=""
  
  echo
  echo -n "%F{#6a4c93}$right_sep%K{#6a4c93}%F{white} ⚜ %k%f"
  echo -n "%K{#b57edc}%F{#6a4c93}$right_sep%K{#b57edc}%F{#1a1a2e} %n %k%f"
  echo -n "%K{#ff9aa2}%F{#b57edc}$right_sep%K{#ff9aa2}%F{#1a1a2e} %1~ %k%f"
  echo -n "%K{#b57edc}%F{#ff9aa2}$right_sep$(prism_git_status)"
  echo -n "%F{#b57edc}$right_sep%f"
  echo
  echo -n "%(?.%F{#b57edc}⚛.%F{red}⚛) %f"
}

setopt PROMPT_SUBST
PROMPT='$(prism_prompt)'
