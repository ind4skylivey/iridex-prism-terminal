# Aurora-Edge Fish Prompt
# A sleek, bubble-style theme with cool boreal colors

function _ae_git_status
    if not command -v git >/dev/null
        return
    end
    
    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)
    
    # Bubble start
    set_color -b blue
    set_color black
    echo -n "  $branch "
    
    if test -n "$dirty"
        set_color -b blue
        set_color yellow
        echo -n "● "
    end
    set_color normal
end

function _ae_k8s
    if command -v kubectl >/dev/null 2>&1
        set -l ctx (kubectl config current-context 2>/dev/null)
        if test -n "$ctx"
            set_color -b magenta
            set_color white
            echo -n " ☸ $ctx "
            set_color normal
        end
    end
end

function fish_prompt
    set -l last_status $status
    
    # Symbols
    set -l left_cap ""
    set -l right_cap ""
    
    echo
    
    # Segment 1: Host (Blue)
    set_color blue
    echo -n "$left_cap"
    set_color -b blue
    set_color black
    echo -n " ❄ "
    
    # Segment 2: Directory (Cyan)
    set_color -b cyan
    set_color blue
    echo -n "$right_cap" # Transition
    set_color -b cyan
    set_color black
    echo -n " "
    echo -n (prompt_pwd)
    echo -n " "
    
    # Segment 3: Git (Blue again)
    set_color -b blue
    set_color cyan
    echo -n "$right_cap" # Transition
    
    _ae_git_status
    
    # End Bubble
    set_color -b normal
    set_color blue
    echo -n "$right_cap"
    
    echo
    
    # Line 2: Prompt Char
    if test $last_status -eq 0
        set_color cyan
        echo -n "❯ "
    else
        set_color red
        echo -n "❯ "
    end
    
    set_color normal
end

