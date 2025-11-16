# Terminal-Ghost Fish prompt
function terminal_ghost_git
    set branch (command git -C $PWD symbolic-ref --short HEAD ^/dev/null)
    if test -n "$branch"
        printf "[%s]" $branch
    end
end

function fish_prompt
    set -l exit_status $status
    set -l accent (set_color --cyan)
    set -l error (set_color --red)
    set -l success (set_color --green)
    set -l status_color $success
    set -l symbol '●'
    if test $exit_status -ne 0
        set status_color $error
        set symbol '✦'
    end
    set -l git_line (terminal_ghost_git)
    set -l user (whoami)
    set -l host (hostname)
    printf '%s %s %s ' (set_color --white)"$user@$host" (set_color --magenta)"%c" (set_color --reset)
    if test -n "$git_line"
        printf '%s ' $accent"$git_line"
    end
    printf '%s%s %s
' $status_color $symbol $exit_status
    printf '%s› %s
' $accent (set_color --white)
end
