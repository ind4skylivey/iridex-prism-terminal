# Mono-Quiet Fish Prompt
# A minimalist, high-tech monochrome theme

function _mq_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    set_color 808080
    echo -n " :: $branch"
    
    if test -n "$dirty"
        set_color ffffff
        echo -n " •"
    end
end

function fish_prompt
    set -l last_status $status
    
    # Palette
    set -l c_white (set_color ffffff)
    set -l c_grey (set_color 808080)
    set -l c_dark (set_color 333333)
    
    echo
    
    # Clean Path
    set_color -o ffffff
    echo -n (prompt_pwd)
    
    # Git
    _mq_git_status
    
    echo
    
    # Minimal Prompt
    if test $last_status -eq 0
        set_color 808080
        echo -n "──○ "
    else
        set_color ffffff
        echo -n "──● "
    end
    
    set_color normal
end
