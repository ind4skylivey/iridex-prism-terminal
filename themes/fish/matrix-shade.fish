# Matrix-Shade Fish prompt
function matrix_shade_git
    set branch (command git -C $PWD symbolic-ref --short HEAD ^/dev/null)
    if test -n "$branch"
        set dirty (command git -C $PWD status --porcelain ^/dev/null)
        if test -n "$dirty"
            echo "$branch*"
        else
            echo "$branch"
        end
    end
end

function matrix_shade_duration
    if test -n "$LAST_COMMAND_STARTED_AT"
        echo (math "scale=0; ($FISH_TIMER - $LAST_COMMAND_STARTED_AT)")s
    else
        echo 0s
    end
end

function fish_prompt
    set -l exit_status $status
    set -l accent (set_color --cyan)
    set -l secondary (set_color --green)
    set -l error (set_color --red)
    set -l success (set_color --green)
    set -l status_color $success
    set -l symbol '■'
    if test $exit_status -ne 0
        set status_color $error
        set symbol '✖'
    end
    set -l duration (matrix_shade_duration)
    set -l git_line (matrix_shade_git)
    set -l vi_mode (set -q KEYMAP; and echo $KEYMAP; or echo main)
    printf '%s %s %s %s\n' $secondary'╔═' $accent'%~' $secondary'|' $accent"${git_line:-no-git}"
    printf '%s %s %s %s %s\n' $secondary'╚═' $status_color"${symbol} ${exit_status}" $accent"[${vi_mode}]" $secondary"${duration}" (set_color --reset)
end
