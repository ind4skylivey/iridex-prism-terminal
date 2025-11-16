# Synthwave-Void Fish prompt
function synthwave_void_git
    set branch (command git -C $PWD symbolic-ref --short HEAD ^/dev/null)
    if test -n "$branch"
        set dirty (command git -C $PWD status --porcelain ^/dev/null)
        if test -n "$dirty"
            echo " $branch *"
        else
            echo " $branch"
        end
    end
end

function fish_prompt
    set -l exit_status $status
    set -l primary (set_color --magenta)
    set -l secondary (set_color --cyan)
    set -l accent (set_color --yellow)
    set -l error (set_color --red)
    set -l success (set_color --green)
    set -l status_color $success
    set -l symbol '★'
    if test $exit_status -ne 0
        set status_color $error
        set symbol '✖'
    end
    set -l git_line (synthwave_void_git)
    printf '%s %s %s %s
' $secondary'╭' $accent'═' $primary'◈' $secondary'%n@%m'
    printf '%s %s ' $accent'' (set_color --white)"%c"
    printf '%s %s %s
' $secondary'╰' $git_line $status_color"${symbol}${exit_status:+ ${exit_status}}"
end
