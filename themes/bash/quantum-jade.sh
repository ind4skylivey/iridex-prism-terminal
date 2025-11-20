# =============================================================================
# PRISM TERMINAL: Quantum-Jade
# Description: Imperial jade tones meeting quantum field theory.
# Generated for: Bash 5.0+
# =============================================================================

# Colors
PRISM_GREEN='#3fb950'
PRISM_CYAN='#39c5bb'
PRISM_ACCENT_PRIMARY='#00a86b'
PRISM_ACCENT_SECONDARY='#00cc99'
PRISM_ACCENT_TERTIARY='#20b2aa'
PRISM_ACCENT_ERROR='#ff5555'

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null)
    if [[ -n "$dirty" ]]; then
      echo -e " \033[38;2;0;168;107m⟁ $branch\033[38;2;255;85;85m ☢\033[0m"
    else
      echo -e " \033[38;2;0;168;107m⟁ $branch\033[0m"
    fi
  fi
}

prism_prompt() {
  PS1="\n\[\033[38;2;0;204;153m\]⚛ \[\033[38;2;179;185;184m\][\[\033[38;2;32;178;170m\]\W\[\033[38;2;179;185;184m\]]\$(prism_git_status)\n\[\$(if [ \$? -eq 0 ]; then echo '\033[38;2;0;168;107m❇'; else echo '\033[38;2;255;85;85m⚠'; fi)\] \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
