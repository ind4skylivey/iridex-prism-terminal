# Cyber-Noir Fish Prompt
# A neon-soaked cyberpunk theme with bubble segments

function _cn_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    set_color -b magenta
    set_color black
    echo -n "  $branch "
    
    if test -n "$dirty"
        set_color yellow
        echo -n "⚡ "
    end
    
    set_color normal
end

function fish_prompt
    set -l last_status $status
    
    # Powerline symbols
    set -l right_sep ""
    
    echo
    
    # Segment 1: Time (Deep Purple bg)
    set_color 5f00af
    echo -n ""
    set_color -b 5f00af
    set_color cyan
    echo -n " "(date +%H:%M:%S)" "
    
    # Segment 2: User@Host (Magenta bg)
    set_color -b magenta
    set_color 5f00af
    echo -n "$right_sep"
    set_color -b magenta
    set_color black
    echo -n " $USER@"(hostname)" "
    
    # Segment 3: Directory (Cyan bg)
    set_color -b cyan
    set_color magenta
    echo -n "$right_sep"
    set_color -b cyan
    set_color black
    echo -n " "(prompt_pwd)" "
    
    # Segment 4: Git (Magenta bg)
    set_color -b magenta
    set_color cyan
    echo -n "$right_sep"
    
    _cn_git_status
    
    # End
    set_color -b normal
    set_color magenta
    echo -n "$right_sep"
    
    echo
    
    # Prompt line
    if test $last_status -eq 0
        set_color cyan
        echo -n "❯ "
    else
        set_color red
        echo -n "❯ "
    end
    
    set_color normal
end
