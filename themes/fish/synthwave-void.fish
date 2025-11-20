# Synthwave-Void Fish Prompt
# Retro-cyberpunk sunset with neon grid aesthetics

function _sv_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    # Git with neon background
    set_color -b ff00ff
    set_color 000000
    echo -n "  $branch "
    
    if test -n "$dirty"
        set_color ffff00
        echo -n "⚡ "
    end
    
    set_color -b normal
end

function fish_prompt
    set -l last_status $status
    
    echo
    
    # Line 1: Retro Grid with Sunset
    # Neon pink grid
    set_color ff00ff
    echo -n "▓▒░ "
    
    # Sunset emoji on dark bg
    set_color -b 1a0033
    set_color ffff00
    echo -n " 🌆 "
    set_color -b normal
    
    # User segment (cyan neon)
    set_color -b 00ffff
    set_color 000000
    echo -n " $USER "
    set_color -b normal
    
    set_color ff00ff
    echo -n " ▸ "
    
    # Directory (magenta neon)
    set_color -b ff00ff
    set_color 000000
    echo -n " "(prompt_pwd)" "
    set_color -b normal
    
    # Git (if present)
    _sv_git_status
    
    # Grid decoration
    set_color ff00ff
    echo -n " ░▒▓"
    
    echo
    
    # Line 2: Neon Prompt
    set_color ff00ff
    echo -n "╰─"
    set_color 00ffff
    echo -n "═"
    set_color ffff00
    echo -n "═"
    set_color ff00ff
    echo -n "► "
    
    # Status indicator
    if test $last_status -eq 0
        set_color 00ffff
        echo -n "◆ "
    else
        set_color ff0000
        echo -n "✖ "
    end
    
    set_color normal
end


