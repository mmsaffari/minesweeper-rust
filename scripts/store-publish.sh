#!/usr/bin/env bash
# Submits an existing release to the Microsoft Store again, without a new release:
#
#   scripts/store-publish.sh            # the newest release tag
#   scripts/store-publish.sh 1.4.3      # a given one
#
# release.sh already submits every release. Use this when that submission failed and you
# fixed the cause (for example, deleted a draft that was started by hand in Partner
# Center), or when the Store needs the same package once more.
#
# It re-runs only the "Submit to the Microsoft Store" job of that tag's Store workflow:
# the .msix it built is reused, nothing is rebuilt or re-released.

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

[[ $# -le 1 ]] || {
    echo "usage: scripts/store-publish.sh [<version>]" >&2
    exit 2
}

require_gh
git fetch --quiet origin --tags
if [[ $# -eq 1 ]]; then
    tag="v${1#v}"
else
    tag=$(git tag --list 'v*' --sort=-version:refname | head -n 1)
    [[ -n "$tag" ]] || die "no release tags yet"
fi
sha=$(git rev-list -n 1 "$tag" 2>/dev/null) || die "$tag doesn't exist"

step "Store workflow for $tag"
run=$(find_run store.yml "$sha" "$tag")
info "https://github.com/$REPO/actions/runs/$run"

# The publish job of that run, and how it ended last time.
read -r job conclusion < <(gh run view "$run" --repo "$REPO" --json jobs \
    --jq '.jobs[] | select(.name | startswith("Submit")) | "\(.databaseId) \(.conclusion)"')
[[ -n "${job:-}" ]] || die "that run has no Store submission job (is it older than the automation?)"
info "last submission: ${conclusion:-still running}"

case ${conclusion:-} in
"") die "it's still running: wait for it (gh run watch $run --repo $REPO)" ;;
success)
    warn "$tag was already submitted successfully."
    confirm "Submit it again anyway?" || exit 0
    ;;
esac

step "Submitting $tag again"
gh run rerun "$run" --repo "$REPO" --job "$job"
sleep 10 # give GitHub a moment to start the job again
watch_run "$run" "Microsoft Store submission"
info "submitted for certification; Partner Center emails you when it's live."
