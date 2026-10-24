# Shared by release.sh and store-publish.sh (sourced, not run on its own).

set -euo pipefail
# Without this, a failing command inside $(...) doesn't stop the script.
shopt -s inherit_errexit

# Always work from the repository's root, wherever the script is started from.
cd "$(dirname "${BASH_SOURCE[0]}")/.."

REPO="mmsaffari/minesweeper-rust"

step() { printf '\n\033[1m==> %s\033[0m\n' "$*"; }
info() { printf '    %s\n' "$*"; }
warn() { printf '\033[33mwarning:\033[0m %s\n' "$*" >&2; }
die() {
    printf '\033[31merror:\033[0m %s\n' "$*" >&2
    exit 1
}

# Asks a yes/no question; anything but y/yes is no. Without a terminal, the answer is no.
confirm() {
    [[ -t 0 ]] || return 1
    local answer
    read -r -p "$1 [y/N] " answer
    [[ "$answer" =~ ^[Yy](es)?$ ]]
}

require_gh() {
    command -v gh >/dev/null || die "GitHub CLI not found. On Debian: sudo apt install gh"
    gh auth status >/dev/null 2>&1 || die "GitHub CLI isn't signed in. Run: gh auth login"
}

# The version in Cargo.toml, e.g. 1.4.2.
cargo_version() {
    sed -n 's/^version = "\([0-9]*\.[0-9]*\.[0-9]*\)"$/\1/p' Cargo.toml | head -n 1
}

# The id of a workflow's run for a commit, waiting up to 2 minutes for GitHub to start it.
# Call it as `id=$(find_run ...)` on its own line, so a failure stops the script.
#   find_run <workflow file> <commit sha> [<branch or tag>]
find_run() {
    local workflow=$1 sha=$2 ref=${3:-} id="" tries=0
    local args=(--repo "$REPO" --workflow "$workflow" --commit "$sha" --limit 1 --json databaseId --jq '.[0].databaseId // empty')
    [[ -n "$ref" ]] && args+=(--branch "$ref")
    while [[ -z "$id" ]]; do
        # A gh error (offline, signed out) stops here; only "no run yet" waits.
        id=$(gh run list "${args[@]}") || die "couldn't ask GitHub for $workflow runs (see gh's message above)"
        if [[ -z "$id" ]]; then
            ((++tries > 24)) && die "$workflow didn't start for ${sha:0:7} within 2 minutes"
            sleep 5
        fi
    done
    echo "$id"
}

# Waits for a run to finish; fails the script if it failed.
#   watch_run <run id> <what it is>
watch_run() {
    local id=$1 what=$2
    [[ -n "$id" ]] || die "no run to wait for ($what)"
    info "$what: https://github.com/$REPO/actions/runs/$id"
    if gh run watch "$id" --repo "$REPO" --exit-status --interval 15 >/dev/null 2>&1; then
        info "$what: passed"
    else
        die "$what failed: https://github.com/$REPO/actions/runs/$id"
    fi
}
