# =============================================================================
# PRISM TERMINAL: Sakura-Steel
# Description: Soft cherry blossoms falling on cold titanium steel.
# Generated for: Fish Shell
# =============================================================================

# Colors
set -gx PRISM_BG '#2e3440'
set -gx PRISM_FG '#d8dee9'
set -gx PRISM_RED '#bf616a'
set -gx PRISM_ACCENT_PRIMARY '#ffb7b2'
set -gx PRISM_ACCENT_SECONDARY '#ff9aa2'
set -gx PRISM_ACCENT_TERTIARY '#e2f0cb'
set -gx PRISM_ACCENT_ERROR '#ff6961'

function _ss_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    set_color $PRISM_ACCENT_PRIMARY
    echo -n " 🌸 $branch"
    
    if test -n "$dirty"
        set_color $PRISM_ACCENT_ERROR
        echo -n " 🥀"
    end
end

function fish_prompt
    set -l last_status $status
    
    echo
    
    # Steel Blade
    set_color $PRISM_FG
    echo -n "🗡  "
    
    set_color $PRISM_ACCENT_SECONDARY
    echo -n $USER
    
    set_color $PRISM_FG
    echo -n " :: "
    
    set_color $PRISM_ACCENT_TERTIARY
    echo -n (prompt_pwd)
    
    _ss_git_status
    
    echo
    
    # Petal Input
    if test $last_status -eq 0
        set_color $PRISM_ACCENT_PRIMARY
        echo -n "❀ "
    else
        set_color $PRISM_ACCENT_ERROR
        echo -n "✖ "
    end
    
    set_color normal
end
