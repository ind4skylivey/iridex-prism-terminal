# Tokyo-Ghost Fish Prompt
# ZEN Japanese aesthetic inspired by anime and traditional culture

function _tg_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    # Git with sakura background
    set_color -b ffb7c5
    set_color 1a1a1a
    echo -n " ⛩ $branch "
    
    if test -n "$dirty"
        set_color ff6b9d
        echo -n "🌸 "
    end
    
    set_color -b normal
end

function fish_prompt
    set -l last_status $status
    
    # Zen symbols
    set -l moon "月"
    set -l bamboo "竹"
    
    echo
    
    # Line 1: Japanese aesthetic with zen elements
    # Ghost with dark blue background (night)
    set_color -b 1a237e
    set_color e1f5fe
    echo -n " 👻 "
    set_color -b normal
    
    set_color 81c784
    echo -n " $bamboo "
    
    # User in soft blue
    set_color -b 64b5f6
    set_color 0d47a1
    echo -n " $USER "
    set_color -b normal
    
    set_color 81c784
    echo -n " › "
    
    # Directory with sakura pink background
    set_color -b ffb7c5
    set _color 880e4f
    echo -n " "(prompt_pwd)" "
    set_color -b normal
    
    # Git
    _tg_git_status
    
    # Moon decoration
    set_color 5c6bc0
    echo -n " $moon"
    
    echo
    
    # Line 2: Minimal zen prompt
    if test $last_status -eq 0
        set_color 64b5f6
        echo -n "❯ "
    else
        set_color ff6b9d
        echo -n "❯ "
    end
    
    set_color normal
end

function fish_right_prompt
    # Time in Japanese style
    set_color 5c6bc0
    echo -n "東京 "
    date +%H:%M
    set_color normal
end


