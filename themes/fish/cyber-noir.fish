# Cyber-Noir Fish prompt
function cyber_noir_battery
    if test -f /sys/class/power_supply/BAT0/capacity
        cat /sys/class/power_supply/BAT0/capacity 2>/dev/null | tr -d '\n' && printf '%%'
    end
end

function cyber_noir_git
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

function cyber_noir_load
    uptime | awk -F'load average:' '{print $2}' | cut -d',' -f1 | awk '{print $1}'
end

function cyber_noir_docker
    if command -v docker >/dev/null 2>&1 && docker info >/dev/null 2>&1
        echo ''
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
    set -l symbol '✔'
    if test $exit_status -ne 0
        set status_color $error
        set symbol '✖'
    end
    set -l time_seg (date +%H:%M)
    set -l load_seg (cyber_noir_load)
    set -l batt_seg (cyber_noir_battery)
    set -l git_line (cyber_noir_git)
    set -l docker_line (cyber_noir_docker)
    set -l user (whoami)
    set -l host (hostname)
    printf '%s %s %s %s %s%s\n' $primary"╭─" $secondary"time:${time_seg}" $accent"load:${load_seg}" $secondary"${batt_seg:+bat:${batt_seg}}" $accent"${docker_line:+${docker_line}}"
    printf '%s %s %s %s %s\n' $primary"╰─" $secondary"${user}@${host}" $accent"%c" $secondary"${git_line:-no-git}" $status_color"${symbol}${exit_status:+ ${exit_status}}"
end
