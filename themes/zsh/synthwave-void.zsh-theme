# =============================================================================
# PRISM TERMINAL: Synthwave-Void
# Description: Retro-cyberpunk sunset with neon grid aesthetics
# Generated for: Zsh 5.8+
# =============================================================================

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -n "%K{magenta}%F{black}  $branch %k%f"
    
    if [[ -n "$dirty" ]]; then
      echo -n "%F{yellow}⚡ %f"
    fi
  fi
}

prism_prompt() {
  echo
  
  # Line 1: Retro Grid with Sunset
  echo -n "%F{magenta}▓▒░ "
  echo -n "%K{#1a0033}%F{yellow} 🌆 %k%f"
  echo -n "%K{cyan}%F{black} %n %k%f"
  echo -n "%F{magenta} ▸ "
  echo -n "%K{magenta}%F{black} %1~ %k%f"
  echo -n "$(prism_git_status)"
  echo -n "%F{magenta} ░▒▓%f"
  
  echo
  
  # Line 2: Neon Prompt
  echo -n "%F{magenta}╰─%F{cyan}═%F{yellow}═%F{magenta}► "
  echo -n "%(?.%F{cyan}◆.%F{red}✖) %f"
}

setopt PROMPT_SUBST
PROMPT='$(prism_prompt)'
