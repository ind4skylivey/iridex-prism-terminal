# =============================================================================
# PRISM TERMINAL: Custom
# Description: Placeholder palette so users can override with their own tweaks.
# Generated for: fish 3.6+
# =============================================================================

set -g prism_bg "#0b0b0b"
set -g prism_fg "#f8f8f2"
set -g prism_primary "#ff79c6"
set -g prism_secondary "#8be9fd"
set -g prism_accent "#bd93f9"
set -g prism_error "#ff5555"
set -g prism_success "#50fa7b"

function fish_prompt
    set -l status_color $prism_success
    if test $status -ne 0
        set status_color $prism_error
    end
    printf "%s◆ %s%s%s in %s%s%s %s❯%s " \
        (set_color $prism_primary) (set_color $prism_secondary) (whoami) (set_color normal) \
        (set_color $prism_primary) (prompt_pwd) (set_color normal) \
        (set_color $status_color) (set_color normal)
end
