#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "publish-homebrew-tap: $*" >&2
  exit 1
}

TAG="${1:-}"
FORMULA_PATH="${2:-}"
TAP_REPOSITORY="${3:-${HOMEBREW_TAP_REPOSITORY:-}}"
TAP_TOKEN="${HOMEBREW_TAP_TOKEN:-}"

if [[ -z "$TAG" ]]; then
  fail "tag is required as the first argument"
fi

if [[ -z "$FORMULA_PATH" ]]; then
  fail "formula path is required as the second argument"
fi

if [[ -z "$TAP_REPOSITORY" ]]; then
  fail "tap repository is required via argument or HOMEBREW_TAP_REPOSITORY"
fi

if [[ -z "$TAP_TOKEN" ]]; then
  fail "HOMEBREW_TAP_TOKEN is required"
fi

if [[ ! -f "$FORMULA_PATH" ]]; then
  fail "formula file does not exist: $FORMULA_PATH"
fi

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

tap_url="https://x-access-token:${TAP_TOKEN}@github.com/${TAP_REPOSITORY}.git"
git clone --depth 1 "$tap_url" "$tmp_dir/tap" >/dev/null 2>&1

mkdir -p "$tmp_dir/tap/Formula"
cp "$FORMULA_PATH" "$tmp_dir/tap/Formula/ralph-engine.rb"

pushd "$tmp_dir/tap" >/dev/null

if git diff --quiet -- Formula/ralph-engine.rb; then
  echo "publish-homebrew-tap: no tap changes required"
  exit 0
fi

git config user.name "github-actions[bot]"
git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
git add Formula/ralph-engine.rb
git commit -m "chore(release): update Ralph Engine ${TAG}" >/dev/null
git push origin HEAD >/dev/null

popd >/dev/null

echo "publish-homebrew-tap: updated ${TAP_REPOSITORY} for ${TAG}"
