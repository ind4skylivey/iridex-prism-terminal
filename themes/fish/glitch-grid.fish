# Glitch-Grid Fish prompt
function glitch_grid_git
    set branch (command git -C $PWD symbolic-ref --short HEAD ^/dev/null)
    if test -n "$branch"
        set dirty (command git -C $PWD status --porcelain ^/dev/null)
        echo "${branch}${dirty:+ ●}"
    end
end

function fish_prompt
    set -l exit_status $status
    set -l primary (set_color --magenta)
    set -l accent (set_color --yellow)
    set -l secondary (set_color --cyan)
    set -l error (set_color --red --reverse)
    set -l success (set_color --green)
    set -l symbol '⚡'
    if test $exit_status -ne 0
        set symbol '✖'
        printf '%s ERROR %s
' $error $secondary
    else
        printf '%s GRID %s
' $primary $accent
    end
    printf '%s %s %s ' $secondary'▌' (set_color --white)(pwd)
    printf '%s ' (set_color --cyan)(glitch_grid_git)
    printf '%s %s
' $success"${symbol}${exit_status}" (set_color --reset)
end
