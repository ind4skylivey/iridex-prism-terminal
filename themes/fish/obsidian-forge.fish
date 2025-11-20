# =============================================================================
# PRISM TERMINAL: Obsidian-Forge
# Description: A volcanic theme forging molten copper accents against obsidian rock.
# Generated for: Fish Shell
# =============================================================================

# Colors
set -gx PRISM_BG '#0b0c0e'
set -gx PRISM_FG '#a8a8b2'
set -gx PRISM_RED '#e06c75'
set -gx PRISM_GREEN '#98c379'
set -gx PRISM_YELLOW '#d19a66'
set -gx PRISM_BLUE '#61afef'
set -gx PRISM_MAGENTA '#c678dd'
set -gx PRISM_CYAN '#56b6c2'
set -gx PRISM_ACCENT_PRIMARY '#d65d0e'
set -gx PRISM_ACCENT_SECONDARY '#af3a03'
set -gx PRISM_ACCENT_TERTIARY '#fabd2f'
set -gx PRISM_ACCENT_ERROR '#cc241d'

function _of_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    set_color $PRISM_ACCENT_PRIMARY
    echo -n " ⚒ $branch"
    
    if test -n "$dirty"
        set_color $PRISM_ACCENT_ERROR
        echo -n " 🔥"
    end
end

function fish_prompt
    set -l last_status $status
    
    echo
    
    # Magma Line
    set_color $PRISM_ACCENT_SECONDARY
    echo -n "🌋 "
    
    set_color $PRISM_FG
    echo -n "["
    set_color $PRISM_ACCENT_TERTIARY
    echo -n $USER
    set_color $PRISM_FG
    echo -n "]──["
    set_color $PRISM_ACCENT_PRIMARY
    echo -n (prompt_pwd)
    set_color $PRISM_FG
    echo -n "]"
    
    _of_git_status
    
    echo
    
    # Ash Line
    set_color $PRISM_ACCENT_SECONDARY
    echo -n "╰─"
    
    if test $last_status -eq 0
        set_color $PRISM_ACCENT_PRIMARY
        echo -n "► "
    else
        set_color $PRISM_ACCENT_ERROR
        echo -n "► "
    end
    
    set_color normal
end
