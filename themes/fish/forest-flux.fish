# Forest-Flux Fish Prompt
# A distinct, nature-inspired theme with a unique layout

function _ff_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    # Git Branch with Leaf symbol
    set_color green
    echo -n " 🌿 $branch"
    
    if test -n "$dirty"
        set_color yellow
        echo -n " 🍂"
    end
end

function fish_prompt
    set -l last_status $status
    
    # Palette
    set -l c_path (set_color --bold green)
    set -l c_arrow (set_color yellow)
    set -l c_dim (set_color 555)
    set -l c_reset (set_color normal)
    
    echo
    
    # Unique "Path" style:  ~ > src > cli
    set_color green
    echo -n (prompt_pwd) | sed 's/\// > /g'
    
    # Git Status
    _ff_git_status
    
    # Duration (if long)
    if test "$CMD_DURATION" -gt 2000
        set_color 555
        set -l secs (math "$CMD_DURATION / 1000")
        echo -n " took $secs"s
    end
    
    # Right side info (Time)
    set_color 555
    echo -n "  "
    echo -n (date +%H:%M)
    
    echo
    
    # Prompt Symbol (Tree/Sprout)
    if test $last_status -eq 0
        set_color --bold green
        echo -n "🌱 "
    else
        set_color --bold red
        echo -n "🥀 "
    end
    
    set_color normal
end

