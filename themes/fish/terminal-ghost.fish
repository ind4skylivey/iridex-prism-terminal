# Terminal-Ghost Fish Prompt
# A minimal, ghostly theme with subtle bubble segments

function _tg_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    set_color -b 4a4a4a
    set_color e0e0e0
    echo -n "  $branch "
    
    if test -n "$dirty"
        set_color ffaa00
        echo -n "● "
    end
    
    set_color normal
end

function fish_prompt
    set -l last_status $status
    
    # Powerline symbols
    set -l right_sep ""
    
    echo
    
    # Segment 1: Ghost Icon (Dark bg)
    set_color 2a2a2a
    echo -n ""
    set_color -b 2a2a2a
    set_color e0e0e0
    echo -n " 👻 "
    
    # Segment 2: Directory (Grey bg)
    set_color -b 4a4a4a
    set_color 2a2a2a
    echo -n "$right_sep"
    set_color -b 4a4a4a
    set_color f0f0f0
    echo -n " "(prompt_pwd)" "
    
    # Segment 3: Git (Darker grey bg)
    set_color -b 3a3a3a
    set_color 4a4a4a
    echo -n "$right_sep"
    set_color -b 3a3a3a
    
    _tg_git_status
    
    # End
    set_color -b normal
    set_color 3a3a3a
    echo -n "$right_sep"
    
    echo
    
    # Prompt line
    if test $last_status -eq 0
        set_color 808080
        echo -n "❯ "
    else
        set_color ff5555
        echo -n "❯ "
    end
    
    set_color normal
end
