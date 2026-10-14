#!/usr/bin/env bash
# Rebuild the "full" branch: master with every game branch's primary search merged in.
#
# The branch is recreated from master each time, so never commit to it directly.
# Game branches only touch faeries/src/primary/<game>/ and history_<game>.md, so the
# merge shouldn't conflict. If it does, a game branch has changed shared code that
# belongs on master.
#
# The binary is built to target/full/release/faeries.
set -euo pipefail

# Game branches with a primary search
BRANCHES=(Tak ataxx)

repo=$(git rev-parse --show-toplevel)
cd "$repo"

dir=$(mktemp -d)
git worktree add -q --detach "$dir" master
trap 'git -C "$repo" worktree remove --force "$dir"' EXIT

git -C "$dir" checkout -q -B full master
git -C "$dir" merge -q --no-edit -m "Merge primary searches: ${BRANCHES[*]}" "${BRANCHES[@]}"

export CARGO_TARGET_DIR="$repo/target/full"
(cd "$dir" && cargo build --release && cargo test --release)

echo "full is now $(git rev-parse --short full): master + ${BRANCHES[*]}"
