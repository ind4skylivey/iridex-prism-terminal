# Theme Template (duplicate for new personality)
function template_git
    set branch (command git -C $PWD symbolic-ref --short HEAD ^/dev/null)
    if test -n "$branch"
        set dirty (command git -C $PWD status --porcelain ^/dev/null)
        if test -n "$dirty"
            printf "✦ %s ⚡" $branch
        else
            printf "✦ %s" $branch
        end
    end
end

function fish_prompt
    set -l exit_status $status
    set -l status_color (set_color --bold.green)
    if test $exit_status -ne 0
        set status_color (set_color --bold.red)
    end
    printf '%s %s %s\n' (set_color --magenta)"Template" (set_color --cyan)"%n@%m" (set_color --reset)
    printf '%s %s ' (set_color --yellow)"➤" (set_color --blue)"%c"
    printf '%s%d %s' $status_color $exit_status (set_color --white)"➜"
    printf '\n'
end
