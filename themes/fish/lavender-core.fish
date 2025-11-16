# Lavender-Core Fish prompt
function lavender_core_git
    set branch (command git -C $PWD symbolic-ref --short HEAD ^/dev/null)
    if test -n "$branch"
        set dirty (command git -C $PWD status --porcelain ^/dev/null)
        if test -n "$dirty"
            echo "❧ $branch ⚡"
        else
            echo "❧ $branch"
        end
    end
end

function lavender_core_prompt
    set -l exit_status $status
    set -l status_color (set_color --bold.green)
    if test $exit_status -ne 0
        set status_color (set_color --bold.red)
    end
    set -l git_line (lavender_core_git)
    set -l user (whoami)
    set -l host (hostname)
    printf "%s %s %s %s %s %s\n" (set_color --magenta)"◆" (set_color --bold.cyan)"$user@$host" (set_color --reset) (set_color --bold.white)"•" (set_color --reset)
    printf "%s %s %s %s\n" (set_color --italic.white)"⟆" (set_color --cyan)"$(pwd)" (set_color --reset) "$git_line"
    printf "%s%s %s" $status_color"(${exit_status})" (set_color --magenta)"→ " (set_color --reset)
end

function fish_prompt
    lavender_core_prompt
end
