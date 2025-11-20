# Nebula-Mocha Fish Prompt
# A cozy, cosmic theme with powerline bubble segments

function _nm_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    set_color -b 967bb6
    set_color f5f5dc
    echo -n " 🌌 $branch "
    
    if test -n "$dirty"
        set_color d2b48c
        echo -n "☕ "
    end
    
    set_color normal
end

function fish_prompt
    set -l last_status $status
    
    # Powerline symbols
    set -l right_sep ""
    
    echo
    
    # Segment 1: Cloud Icon (Coffee bg)
    set_color 6f4e37
    echo -n ""
    set_color -b 6f4e37
    set_color f5f5dc
    echo -n " ☁ "
    
    # Segment 2: User (Nebula bg)
    set_color -b 967bb6
    set_color 6f4e37
    echo -n "$right_sep"
    set_color -b 967bb6
    set_color f5f5dc
    echo -n " $USER "
    
    # Segment 3: Directory (Cream bg)
    set_color -b d2b48c
    set_color 967bb6
    echo -n "$right_sep"
    set_color -b d2b48c
    set_color 6f4e37
    echo -n " "(prompt_pwd)" "
    
    # Segment 4: Git (Nebula bg)
    set_color -b 967bb6
    set_color d2b48c
    echo -n "$right_sep"
    
    _nm_git_status
    
    # End
    set_color -b normal
    set_color 967bb6
    echo -n "$right_sep"
    
    echo
    
    # Prompt line
    if test $last_status -eq 0
        set_color 967bb6
        echo -n "⋆｡°✩ "
    else
        set_color 6f4e37
        echo -n "☾ "
    end
    
    set_color normal
end

