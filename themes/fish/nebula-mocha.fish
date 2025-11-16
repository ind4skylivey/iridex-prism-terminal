# Nebula-Mocha Fish prompt
function nebula_mocha_git
    set branch (command git -C $PWD symbolic-ref --short HEAD ^/dev/null)
    if test -n "$branch"
        set dirty (command git -C $PWD status --porcelain ^/dev/null)
        if test -n "$dirty"
            echo " $branch ✦"
        else
            echo " $branch"
        end
    end
end

function fish_prompt
    set -l exit_status $status
    set -l status_color (set_color --bold.green)
    if test $exit_status -ne 0
        set status_color (set_color --bold.red)
    end
    set -l git_line (nebula_mocha_git)
    printf '%s %s %s %s\n' (set_color --magenta)"╭─" (set_color --cyan)"%n@%m" (set_color --yellow)"" (set_color --reset)
    printf '%s %s ' (set_color --yellow)"╰─" (set_color --white)"%c"
    if test -n "$git_line"
        printf ' %s' (set_color --cyan)"$git_line"
    end
    printf ' %s%s\n' $status_color $exit_status
end
