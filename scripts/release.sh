#!/usr/bin/env bash
# Releases a new version, on GitHub and in the Microsoft Store:
#
#   scripts/release.sh 1.4.3                     # GitHub writes the release notes
#   scripts/release.sh 1.4.3 --notes notes.md    # your own release notes
#
# Before running it, write the Store's "What's new" for this version in
# packaging/msix/whats-new.txt. Then the script:
#   1. checks everything first: on main, nothing uncommitted, up to date with GitHub,
#      a version newer than the current one, the "What's new" text for it
#   2. sets the version in Cargo.toml, runs the tests, commits "Release <version>"
#   3. pushes main and waits for CI to pass on it (a release from a red commit is never made)
#   4. tags v<version> and pushes the tag: GitHub builds the release, and the Store
#      workflow builds the .msix and submits it with the "What's new" text
#   5. waits for both, and puts your release notes on the GitHub release
#
# If the Store submission fails (for example, a draft started by hand in Partner Center
# is in the way), fix that and run scripts/store-publish.sh to try it again.

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

usage() {
    echo "usage: scripts/release.sh <version> [--notes <file>]" >&2
    exit 2
}

version=""
notes=""
while [[ $# -gt 0 ]]; do
    case $1 in
    --notes)
        [[ $# -ge 2 ]] || usage
        notes=$2
        shift 2
        ;;
    -h | --help) usage ;;
    *)
        [[ -z "$version" ]] || usage
        version=${1#v} # "v1.4.3" works too
        shift
        ;;
    esac
done
[[ -n "$version" ]] || usage
tag="v$version"

# --- 1. Check everything before changing anything --------------------------------------

step "Checking"
require_gh
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "'$version' isn't a version like 1.4.3"
[[ -z "$notes" || -f "$notes" ]] || die "release notes file not found: $notes"

[[ "$(git branch --show-current)" == "main" ]] || die "not on main"
[[ -z "$(git status --porcelain)" ]] || die "there are uncommitted changes (git status): commit or undo them first"
git fetch --quiet origin main --tags
[[ "$(git rev-parse HEAD)" == "$(git rev-parse origin/main)" ]] || die "main isn't the same as GitHub's: git pull (or push) first"
! git rev-parse --quiet --verify "refs/tags/$tag" >/dev/null || die "$tag already exists"

current=$(cargo_version)
newest=$(printf '%s\n%s\n' "$current" "$version" | sort -V | tail -n 1)
[[ "$version" != "$current" && "$newest" == "$version" ]] || die "$version isn't newer than the current version, $current"

# The first line of whats-new.txt that isn't a comment or empty: the version its text is for.
whats_new_version=$(grep -v '^[[:space:]]*#' packaging/msix/whats-new.txt | grep -m 1 '[^[:space:]]' | tr -d '[:space:]' || true)
if [[ "$whats_new_version" != "$version" ]]; then
    warn "packaging/msix/whats-new.txt is for version '$whats_new_version', not $version."
    warn "The Store would show this release with an empty \"What's new\"."
    confirm "Release without a \"What's new\" text?" || die "write the text in packaging/msix/whats-new.txt, commit it, and run this again"
fi
info "$current -> $version, tag $tag"

# --- 2. Version, tests, commit ----------------------------------------------------------

step "Setting the version to $version and running the tests"
sed -i "0,/^version = \"$current\"\$/s//version = \"$version\"/" Cargo.toml
[[ "$(cargo_version)" == "$version" ]] || die "couldn't set the version in Cargo.toml"
# The tests also bring Cargo.lock up to date with the new version.
cargo test --quiet --all-features
git add Cargo.toml Cargo.lock
git commit --quiet -m "Release $version"
info "committed: $(git log --oneline -1)"

# --- 3. Push and wait for CI ------------------------------------------------------------

step "Pushing main and waiting for CI"
git push --quiet origin main
sha=$(git rev-parse HEAD)
ci_run=$(find_run ci.yml "$sha" main)
watch_run "$ci_run" "CI"

# --- 4 and 5. Tag, then wait for the GitHub release and the Store submission ------------

step "Tagging $tag"
git tag "$tag"
git push --quiet origin "$tag"

step "Waiting for the release builds"
release_run=$(find_run release.yml "$sha" "$tag")
store_run=$(find_run store.yml "$sha" "$tag")
watch_run "$release_run" "GitHub release"
if [[ -n "$notes" ]]; then
    gh release edit "$tag" --repo "$REPO" --notes-file "$notes" >/dev/null
    info "release notes set from $notes"
fi
watch_run "$store_run" "Microsoft Store submission"

step "Released $version"
info "GitHub: https://github.com/$REPO/releases/tag/$tag"
info "Store:  submitted for certification; Partner Center emails you when it's live."
