# =============================================================================
# PRISM TERMINAL: Obsidian-Forge
# Description: A volcanic theme forging molten copper accents against obsidian rock.
# Generated for: Bash 5.0+
# =============================================================================

# Colors
PRISM_BG='#0b0c0e'
PRISM_FG='#a8a8b2'
PRISM_ACCENT_PRIMARY='#d65d0e'
PRISM_ACCENT_SECONDARY='#af3a03'
PRISM_ACCENT_TERTIARY='#fabd2f'
PRISM_ACCENT_ERROR='#cc241d'

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    if [[ -n "$dirty" ]]; then
      echo -e " \033[38;2;214;93;14m⚒ $branch\033[38;2;204;36;29m 🔥\033[0m"
    else
      echo -e " \033[38;2;214;93;14m⚒ $branch\033[0m"
    fi
  fi
}

prism_prompt() {
  PS1="\n\[\033[38;2;175;58;3m\]🌋 \[\033[38;2;168;168;178m\][\[\033[38;2;250;189;47m\]\u\[\033[38;2;168;168;178m\]]──[\[\033[38;2;214;93;14m\]\W\[\033[38;2;168;168;178m\]]\$(prism_git_status)\n\[\033[38;2;175;58;3m\]╰─\[\$(if [ \$? -eq 0 ]; then echo '\033[38;2;214;93;14m'; else echo '\033[38;2;204;36;29m'; fi)\]► \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
