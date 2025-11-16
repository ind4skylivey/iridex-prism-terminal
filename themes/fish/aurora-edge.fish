# Aurora-Edge Fish prompt
function aurora_edge_git
    set branch (command git -C $PWD symbolic-ref --short HEAD ^/dev/null)
    if test -n "$branch"
        set dirty (command git -C $PWD status --porcelain ^/dev/null)
        if test -n "$dirty"
            echo "[$branch*]"
        else
            echo "[$branch]"
        end
    end
end

function aurora_edge_docker
    command -v docker >/dev/null 2>&1 && docker info >/dev/null 2>&1 && echo '🐳'
end

function aurora_edge_k8s
    command -v kubectl >/dev/null 2>&1 && kubectl config current-context 2>/dev/null && echo '☸'
end

function fish_prompt
    set -l exit_status $status
    set -l accent (set_color --cyan)
    set -l secondary (set_color --blue)
    set -l error (set_color --red)
    set -l success (set_color --green)
    set -l status_color $success
    set -l symbol '✔'
    if test $exit_status -ne 0
        set status_color $error
        set symbol '✖'
    end
    set -l git_line (aurora_edge_git)
    set -l docker_line (aurora_edge_docker)
    set -l k8s_line (aurora_edge_k8s)
    printf '%s %s %s %s %s\n' $secondary'╭' $accent'%n@%m' $secondary'%c' $accent'| git' $secondary"${git_line:-no-git}"
    printf '%s %s %s %s %s %s\n' $secondary'╰' $accent'docker' $secondary"${docker_line:-none}" $accent'k8s' $secondary"${k8s_line:-none}" $status_color"${symbol}${exit_status:+ ${exit_status}}"
end
