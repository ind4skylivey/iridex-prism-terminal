# =============================================================================
# PRISM TERMINAL: Arch-Vortex
# Description: A swirling fusion of Arch Linux blue and Catppuccin lavender.
# Generated for: Bash 5.0+
# =============================================================================

# Colors
PRISM_BLUE='#1793d1'
PRISM_MAGENTA='#cba6f7'
PRISM_ACCENT_PRIMARY='#1793d1'
PRISM_ACCENT_SECONDARY='#cba6f7'
PRISM_ACCENT_TERTIARY='#89b4fa'
PRISM_ACCENT_ERROR='#f38ba8'

prism_git_status() {
  local branch=$(git symbolic-ref --short HEAD 2>/dev/null)
  [[ -n "$branch" ]] && echo -e " \033[38;2;203;166;247m $branch\033[0m"
}

prism_prompt() {
  PS1="\n\[\033[38;2;23;147;209m\]🌀 \[\033[38;2;203;166;247m\]\u \[\033[38;2;137;180;250m\]in \[\033[38;2;23;147;209m\]\W\$(prism_git_status)\n\[\$(if [ \$? -eq 0 ]; then echo '\033[38;2;203;166;247m'; else echo '\033[38;2;243;139;168m'; fi)\]❯ \[\033[0m\]"
}

PROMPT_COMMAND=prism_prompt
