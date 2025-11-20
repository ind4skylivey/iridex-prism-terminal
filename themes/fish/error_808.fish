# ERROR_808 Fish prompt
function error_808_git
    set branch (command git -C $PWD symbolic-ref --short HEAD 2>/dev/null)
    if test -n "$branch"
        set dirty (command git -C $PWD status --porcelain 2>/dev/null)
        if test -n "$dirty"
            echo "⚠ $branch ✗"
        else
            echo "⚠ $branch"
        end
    end
end

function fish_prompt
    set -l exit_status $status
    set -l primary (set_color --bold red)
    set -l warning (set_color yellow)
    set -l glitch (set_color cyan)
    set -l success (set_color green)
    set -l bars "▒▓▒▓"
    set -l git_line (error_808_git)
    printf '%s %s%s %s\n' $primary"⚠" (set_color white)"$(whoami)@$(hostname)" $glitch$bars
    printf '%s %s%s' $warning"⟆" $glitch"$(pwd)"
    if test -n "$git_line"
        printf ' %s' $git_line
    end
    if test $exit_status -eq 0
        printf ' %s✓' $success
    else
        printf ' %s✖ (%d)' $primary $exit_status
    end
    printf '\n'
end
