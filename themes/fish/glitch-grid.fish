# Glitch-Grid Fish Prompt
# A chaotic, cyberpunk glitch theme

function _gg_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    set_color cyan
    echo -n "  "
    set_color --reverse cyan
    echo -n " $branch "
    set_color normal
    
    if test -n "$dirty"
        set_color red
        echo -n " ▓▒░ ERROR"
    end
end

function fish_prompt
    set -l last_status $status
    
    # Palette
    set -l c_prim (set_color magenta)
    set -l c_sec (set_color cyan)
    set -l c_err (set_color red)
    set -l c_reset (set_color normal)
    
    echo
    
    # Hostname with "glitch" brackets
    set_color magenta
    echo -n "█▓▒░ "
    set_color --bold white
    echo -n (hostname)
    set_color magenta
    echo -n " ░▒▓█"
    
    echo
    
    # Directory
    set_color cyan
    echo -n "  └─► "
    set_color --bold cyan
    echo -n (prompt_pwd)
    
    # Git
    _gg_git_status
    
    echo
    
    # Prompt Symbol
    if test $last_status -eq 0
        set_color --bold yellow
        echo -n "⚡ "
    else
        set_color --bold red
        echo -n "💀 "
    end
    
    set_color normal
end

