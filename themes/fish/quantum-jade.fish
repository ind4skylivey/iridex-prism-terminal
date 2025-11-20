# =============================================================================
# PRISM TERMINAL: Quantum-Jade
# Description: Imperial jade tones meeting quantum field theory.
# Generated for: Fish Shell
# =============================================================================

# Colors
set -gx PRISM_BG '#0d1117'
set -gx PRISM_FG '#b3b9b8'
set -gx PRISM_GREEN '#3fb950'
set -gx PRISM_CYAN '#39c5bb'
set -gx PRISM_ACCENT_PRIMARY '#00a86b'
set -gx PRISM_ACCENT_SECONDARY '#00cc99'
set -gx PRISM_ACCENT_TERTIARY '#20b2aa'
set -gx PRISM_ACCENT_ERROR '#ff5555'

function _qj_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    set_color $PRISM_ACCENT_PRIMARY
    echo -n " ⟁ $branch"
    
    if test -n "$dirty"
        set_color $PRISM_ACCENT_ERROR
        echo -n " ☢"
    end
end

function fish_prompt
    set -l last_status $status
    
    echo
    
    # Quantum Field
    set_color $PRISM_ACCENT_SECONDARY
    echo -n "⚛ "
    
    set_color $PRISM_FG
    echo -n "["
    set_color $PRISM_ACCENT_TERTIARY
    echo -n (prompt_pwd)
    set_color $PRISM_FG
    echo -n "]"
    
    _qj_git_status
    
    echo
    
    # Particle Input
    if test $last_status -eq 0
        set_color $PRISM_ACCENT_PRIMARY
        echo -n "❇ "
    else
        set_color $PRISM_ACCENT_ERROR
        echo -n "⚠ "
    end
    
    set_color normal
end
