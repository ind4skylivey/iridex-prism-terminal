# =============================================================================
# PRISM TERMINAL: Eclipse-Protocol
# Description: Solar corona flares erupting from the absolute void.
# Generated for: Fish Shell
# =============================================================================

# Colors
set -gx PRISM_BG '#000000'
set -gx PRISM_FG '#e0e0e0'
set -gx PRISM_YELLOW '#ffcc00'
set -gx PRISM_ACCENT_PRIMARY '#ffd700'
set -gx PRISM_ACCENT_SECONDARY '#ffa500'
set -gx PRISM_ACCENT_TERTIARY '#ffffff'
set -gx PRISM_ACCENT_ERROR '#ff0000'

function _ep_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    set_color $PRISM_ACCENT_PRIMARY
    echo -n " ☾ $branch"
    
    if test -n "$dirty"
        set_color $PRISM_ACCENT_ERROR
        echo -n " 🩸"
    end
end

function fish_prompt
    set -l last_status $status
    
    echo
    
    # Solar Corona
    set_color $PRISM_ACCENT_PRIMARY
    echo -n "🌑 "
    
    set_color $PRISM_ACCENT_SECONDARY
    echo -n "──"
    
    set_color $PRISM_ACCENT_TERTIARY
    echo -n " "(prompt_pwd)" "
    
    set_color $PRISM_ACCENT_SECONDARY
    echo -n "──"
    
    _ep_git_status
    
    echo
    
    # Ray Input
    if test $last_status -eq 0
        set_color $PRISM_ACCENT_PRIMARY
        echo -n "☀ "
    else
        set_color $PRISM_ACCENT_ERROR
        echo -n "⚡ "
    end
    
    set_color normal
end
