# Mono-Quiet Fish prompt
function mono_quiet_git
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

function fish_prompt
    set -l exit_status $status
    set -l accent (set_color --cyan)
    set -l error (set_color --red)
    set -l success (set_color --green)
    set -l status_color $success
    if test $exit_status -ne 0
        set status_color $error
    end
    set -l git_line (mono_quiet_git)
    printf '%s ' (set_color --white)(pwd)
    if test -n "$git_line"
        printf '%s ' $git_line
    end
    printf '%s%d %s\n' $status_color $exit_status $accent'»'
end
