# Midnight-Warp Fish prompt
function midnight_warp_git
    set branch (command git -C $PWD symbolic-ref --short HEAD ^/dev/null)
    if test -n "$branch"
        set dirty (command git -C $PWD status --porcelain ^/dev/null)
        if test -n "$dirty"
            printf "[%s*]" $branch
        else
            printf "[%s]" $branch
        end
    end
end

function midnight_warp_prompt
    set -l exit_status $status
    set -l primary (set_color --cyan)
    set -l secondary (set_color --blue)
    set -l accent (set_color --magenta)
    set -l error (set_color --red)
    set -l success (set_color --green)
    set -l status_color $success
    set -l symbol '✔'
    if test $exit_status -ne 0
        set status_color $error
        set symbol '✖'
    end
    set -l git_line (midnight_warp_git)
    set -l time_seg (date +%H:%M)
    printf '%s %s %s %s %s\n' $primary'╭' $accent"time:${time_seg}" $secondary'%n@%m' (set_color --white)'\n'
    printf '%s %s %s ' $secondary'╰' (set_color --white)"%c" (set_color --cyan)$git_line
    printf '%s%s %s\n' $status_color $symbol $exit_status
end

function fish_right_prompt
    set -l time_seg (date +%H:%M)
    printf '%s%s' (set_color --blue)"${time_seg}" (set_color --reset)
end

function fish_prompt
    midnight_warp_prompt
end
