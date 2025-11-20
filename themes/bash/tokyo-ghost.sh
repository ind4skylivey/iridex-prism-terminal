# =============================================================================
# PRISM TERMINAL: Tokyo-Ghost
# Description: ZEN Japanese aesthetic inspired by anime and traditional culture
# Generated for: Bash 5.0+
# =============================================================================

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    echo -en "\033[48;2;255;183;197m\033[38;2;26;26;26m ⛩ $branch \033[0m"
    
    if [[ -n "$dirty" ]]; then
      echo -en "\033[38;2;255;107;157m🌸 \033[0m"
    fi
  fi
}

prism_prompt() {
  local moon="月"
  local bamboo="竹"
  
  PS1="\n"
  
  # Ghost with dark blue background (night)
  PS1+="\[\033[48;2;26;35;126m\033[38;2;225;245;254m\] 👻 \033[0m"
  
  PS1+="\[\033[38;2;129;199;132m\] $bamboo "
  
  # User in soft blue
  PS1+="\[\033[48;2;100;181;246m\033[38;2;13;71;161m\] \u \033[0m"
  
  PS1+="\[\033[38;2;129;199;132m\] › "
  
  # Directory with sakura pink background
  PS1+="\[\033[48;2;255;183;197m\033[38;2;136;14;79m\] \W \033[0m"
  
  # Git
  PS1+="\$(prism_git_status)"
  
  # Moon decoration
  PS1+="\[\033[38;2;92;107;192m\] $moon\033[0m"
  
  PS1+="\n"
  
  # Prompt line
  PS1+="\[\$(if [ \$? -eq 0 ]; then echo '\033[38;2;100;181;246m'; else echo '\033[38;2;255;107;157m'; fi)\]❯ \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
