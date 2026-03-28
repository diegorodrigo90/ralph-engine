#!/usr/bin/env bash
# Syncs ralph-engine from monorepo to the public GitHub repository.
#
# Usage (from monorepo root):
#   ./tools/ralph-engine/scripts/sync-to-public.sh
#
# This uses git subtree to push the tools/ralph-engine/ directory
# to the public repo as its own standalone project.
#
# Setup (first time only):
#   git remote add ralph-engine-public git@github.com:diegorodrigo90/ralph-engine.git

set -euo pipefail

REMOTE="ralph-engine-public"
PREFIX="tools/ralph-engine"
BRANCH="main"

# Check remote exists.
if ! git remote get-url "$REMOTE" &>/dev/null; then
  echo "Remote '$REMOTE' not found. Adding it..."
  git remote add "$REMOTE" "git@github.com:diegorodrigo90/ralph-engine.git"
fi

echo "Pushing $PREFIX to $REMOTE/$BRANCH..."
git subtree push --prefix="$PREFIX" "$REMOTE" "$BRANCH"

echo "Done. Public repo updated."
