# =============================================================================
# PRISM TERMINAL:# Arch-Vortex Fish Prompt
# A swirling fusion of Arch Linux blue and Catppuccin lavender
# Generated for: Fish Shell
# =============================================================================

# Colors
set -gx PRISM_BG '#14161b'
set -gx PRISM_FG '#cdd6f4'
set -gx PRISM_BLUE '#1793d1'
set -gx PRISM_MAGENTA '#cba6f7'
set -gx PRISM_ACCENT_PRIMARY '#1793d1'
set -gx PRISM_ACCENT_SECONDARY '#cba6f7'
set -gx PRISM_ACCENT_TERTIARY '#89b4fa'
set -gx PRISM_ACCENT_ERROR '#f38ba8'

function _av_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    # Git segment with background
    set_color -b cba6f7
    set_color 14161b
    echo -n "  $branch "
    
    if test -n "$dirty"
        set_color f38ba8
        echo -n "🌪 "
    end
    
    set_color -b normal
end

function fish_prompt
    set -l last_status $status
    
    echo
    
    # Vortex icon with Arch blue background
    set_color -b 1793d1
    set_color 14161b
    echo -n " 🌀 "
    set_color -b normal
    
    echo -n " "
    
    # User in lavender
    set_color cba6f7
    echo -n $USER
    
    set_color 89b4fa
    echo -n " in "
    
    # Directory with subtle background
    set_color -b 89b4fa
    set_color 14161b
    echo -n " "(prompt_pwd)" "
    set_color -b normal
    
    _av_git_status
    
    echo
    
    # Spiral Input
    if test $last_status -eq 0
        set_color cba6f7
        echo -n "❯ "
    else
        set_color f38ba8
        echo -n "❯ "
    end
    
    set_color normal
end
