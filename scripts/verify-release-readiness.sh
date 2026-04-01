#!/usr/bin/env bash
set -euo pipefail

if ! command -v gh >/dev/null 2>&1; then
  echo "gh is required" >&2
  exit 1
fi

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required" >&2
  exit 1
fi

repo="${GITHUB_REPOSITORY:-}"
sha="${GITHUB_SHA:-}"
token="${GITHUB_TOKEN:-${GH_TOKEN:-}}"

if [[ -z "$repo" ]]; then
  echo "GITHUB_REPOSITORY is required" >&2
  exit 1
fi

if [[ -z "$sha" ]]; then
  echo "GITHUB_SHA is required" >&2
  exit 1
fi

if [[ -z "$token" ]]; then
  echo "GITHUB_TOKEN is required" >&2
  exit 1
fi

export GH_TOKEN="$token"

git fetch --no-tags origin main

current_sha="$(git rev-parse HEAD)"
main_sha="$(git rev-parse origin/main)"

if [[ "$current_sha" != "$main_sha" ]]; then
  echo "release workflow must run from the current origin/main HEAD" >&2
  echo "current checkout: $current_sha" >&2
  echo "origin/main:     $main_sha" >&2
  exit 1
fi

runs_json="$(gh run list \
  --repo "$repo" \
  --workflow CI \
  --branch main \
  --commit "$sha" \
  --json databaseId,conclusion,status,event,headSha,createdAt \
  --limit 20)"

python3 - <<'PY' "$sha" "$runs_json"
import json
import sys

target_sha = sys.argv[1]
runs = json.loads(sys.argv[2])

matching = [
    run for run in runs
    if run.get("headSha") == target_sha and run.get("event") == "push"
]

if not matching:
    print("no completed CI run found for the current main SHA", file=sys.stderr)
    sys.exit(1)

matching.sort(key=lambda run: run.get("createdAt") or "", reverse=True)
latest = matching[0]

status = latest.get("status")
conclusion = latest.get("conclusion")
run_id = latest.get("databaseId")

if status != "completed" or conclusion != "success":
    print(
        f"CI run {run_id} for {target_sha} is not releasable: status={status} conclusion={conclusion}",
        file=sys.stderr,
    )
    sys.exit(1)

print(f"verified CI run {run_id} for {target_sha}")
PY
