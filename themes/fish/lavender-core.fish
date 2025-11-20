# Lavender-Core Fish Prompt
# An elegant, anime-tech inspired theme with powerline bubbles

function _lc_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    set_color -b b57edc
    set_color 1a1a2e
    echo -n "  $branch "
    
    if test -n "$dirty"
        set_color ff69b4
        echo -n "✦ "
    end
    
    set_color normal
end

function fish_prompt
    set -l last_status $status
    
    # Powerline symbols
    set -l right_sep ""
    
    echo
    
    # Segment 1: Icon (Deep Purple bg)
    set_color 6a4c93
    echo -n ""
    set_color -b 6a4c93
    set_color ffffff
    echo -n " ⚜ "
    
    # Segment 2: User (Lavender bg)
    set_color -b b57edc
    set_color 6a4c93
    echo -n "$right_sep"
    set_color -b b57edc
    set_color 1a1a2e
    echo -n " $USER "
    
    # Segment 3: Directory (Pink bg)
    set_color -b ff9aa2
    set_color b57edc
    echo -n "$right_sep"
    set_color -b ff9aa2
    set_color 1a1a2e
    echo -n " "(prompt_pwd)" "
    
    # Segment 4: Git (Lavender bg)
    set_color -b b57edc
    set_color ff9aa2
    echo -n "$right_sep"
    
    _lc_git_status
    
    # End
    set_color -b normal
    set_color b57edc
    echo -n "$right_sep"
    
    echo
    
    # Prompt line
    if test $last_status -eq 0
        set_color b57edc
        echo -n "⚛ "
    else
        set_color ff0000
        echo -n "⚛ "
    end
    
    set_color normal
end

