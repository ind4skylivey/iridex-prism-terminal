# Midnight-Warp Fish Prompt
# A deep space travel theme

function _mw_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    set_color blue
    echo -n " ☄ $branch"
    
    if test -n "$dirty"
        set_color red
        echo -n " 💥"
    end
end

function fish_prompt
    set -l last_status $status
    
    # Palette
    set -l c_star (set_color white)
    set -l c_void (set_color blue)
    set -l c_warp (set_color cyan)
    set -l c_dim (set_color 555)
    
    echo
    
    # Line 1: Starfield
    set_color blue
    echo -n "✨ "
    
    set_color cyan
    echo -n "╭─"
    
    set_color blue
    echo -n " $USER "
    
    set_color cyan
    echo -n "─"
    
    set_color white
    echo -n " 🚀 "
    
    set_color cyan
    echo -n "─"
    
    set_color blue
    echo -n " "(prompt_pwd)" "
    
    _mw_git_status
    
    echo
    
    # Line 2: Warp trail
    set_color cyan
    echo -n "╰─"
    
    if test $last_status -eq 0
        set_color --bold white
        echo -n "🪐 "
    else
        set_color red
        echo -n "🌑 "
    end
    
    set_color normal
end

function fish_right_prompt
    set_color 555
    date +%H:%M
    set_color normal
end

