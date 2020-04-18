# shellcheck source=./util.sh disable=SC2155
function _dl_util_sh {
    local UTIL_VERSION="v2"
    local dir="$( dirname "$( realpath "$1" )" )"
    [ -f "${dir}/util.sh" ] || bash "${dir}/download-util.sh" "$UTIL_VERSION" || exit 1
    source "${dir}/util.sh"
}; _dl_util_sh "$0"

check "cargo"

# Wrapper for `cargo`. All arguments are passed to the `cargo` command.
# Runs the command in a new terminal, if specified.
function cargo_cmd {
    [ -z "$1" ] && err "Function \`cargo_cmd\` needs at least one argument"

    local args=
    local subcmd=
    local cmd=()
    local cmdmsg=

    subcmd="$1"
    shift
    args=( "$@" )
    cmd=( \
        cargo "$subcmd" \
        "${args[@]}" \
    )

    cmdmsg="$(clr yellow black bold)Running$(clrrs) $(clr "${CLR_CODE[@]}")${cmd[*]}$(clrrs)"

    if should_run_in_terminal; then
        run_terminal "echo ${cmdmsg}; " "${cmd[@]}"
    else
        msg "${cmdmsg}"
        "${cmd[@]}"
    fi
}
