# Tokyo-Ghost Fish prompt
function tokyo_ghost_git
    set branch (command git -C $PWD symbolic-ref --short HEAD ^/dev/null)
    if test -n "$branch"
        set dirty (command git -C $PWD status --porcelain ^/dev/null)
        if test -n "$dirty"
            echo "󱩎 $branch ●"
        else
            echo "󱩎 $branch"
        end
    end
end

function fish_prompt
    set -l exit_status $status
    set -l status_color (set_color --bold.cyan)
    set -l symbol "❯"
    if test $exit_status -ne 0
        set status_color (set_color --bold.red)
        set symbol "⚡"
    end
    set -l user (whoami)
    set -l host (hostname)
    set -l git_line (tokyo_ghost_git)
    printf '%s %s%s %s %s%s ' (set_color --bold.blue)"⟐" (set_color --bold.white)"$user@$host" (set_color --reset)
    printf '%s%s%s' (set_color --bold.cyan)"❯" (set_color --blue)"$(pwd)" (set_color --reset)
    printf '%s %s ' (set_color --magenta)"$git_line" (set_color --reset)
    printf '%s%d %s\n' $status_color $exit_status (set_color --blue)"$symbol" (set_color --reset)
end
