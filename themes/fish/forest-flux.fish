# Forest-Flux Fish prompt
function forest_flux_git
    set branch (command git -C $PWD symbolic-ref --short HEAD ^/dev/null)
    if test -n "$branch"
        set dirty (command git -C $PWD status --porcelain ^/dev/null)
        if test -n "$dirty"
            echo "$branch *"
        else
            echo "$branch"
        end
    end
end

function fish_prompt
    set -l exit_status $status
    set -l accent (set_color --yellow)
    set -l secondary (set_color --green)
    set -l error (set_color --red)
    set -l success (set_color --green)
    set -l status_color $success
    set -l symbol '✔'
    if test $exit_status -ne 0
        set status_color $error
        set symbol '✖'
    end
    set -l time_seg (date +%H:%M)
    set -l git_line (forest_flux_git)
    printf '%s %s %s\n' $secondary'╭' $accent"time:${time_seg}" $secondary'%n@%m'
    printf '%s %s %s ' $secondary'╰' $accent"%c" $secondary"${git_line:+${git_line}}"
    printf '%s%s %s\n' $status_color $symbol $exit_status
end
