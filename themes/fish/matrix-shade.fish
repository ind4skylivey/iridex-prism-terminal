# Matrix-Shade Fish Prompt
# Elite hacker terminal - cybersecurity aesthetics

function _ms_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    # Git segment with black background
    set_color -b 000000
    set_color 00ff00
    echo -n "[GIT::"
    set_color -o 00ff00
    echo -n "$branch"
    set_color normal
    set_color -b 000000
    set_color 00ff00
    
    if test -n "$dirty"
        set_color ff0000
        echo -n "::MODIFIED"
    else
        set_color 00ff00
        echo -n "::CLEAN"
    end
    
    echo -n "]"
    set_color -b normal
end

function fish_prompt
    set -l last_status $status
    
    echo
    
    # Line 1: System Status Bar
    set_color -b 000000
    set_color 00ff00
    
    # Status indicator
    if test $last_status -eq 0
        echo -n "[●SECURE] "
    else
        set_color ff0000
        echo -n "[✖BREACH] "
        set_color 00ff00
    end
    
    # User@Host
    echo -n "[ROOT::"
    set_color -o 00ff00
    echo -n "$USER@"(hostname)
    set_color normal
    set_color -b 000000
    set_color 00ff00
    echo -n "] "
    
    # Directory
    echo -n "[PATH::"
    set_color -o ffffff
    echo -n (prompt_pwd)
    set_color normal
    set_color -b 000000
    set_color 00ff00
    echo -n "] "
    
    # Git
    _ms_git_status
    
    set_color -b normal
    
    echo
    
    # Line 2: Command Input
    set_color 00ff00
    echo -n "┌─["
    set_color -o 00ff00
    echo -n "TERMINAL"
    set_color normal
    set_color 00ff00
    echo -n "]"
    echo
    echo -n "└─► "
    
    set_color normal
end


