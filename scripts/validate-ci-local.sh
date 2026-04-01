#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKFLOW_FILE="${WORKFLOW_FILE:-.github/workflows/ci.yml}"
JOB_NAME="${JOB_NAME:-quality}"
ACT_EVENT="${ACT_EVENT:-push}"
ACT_IMAGE="${ACT_IMAGE:-ghcr.io/catthehacker/ubuntu:act-latest}"

if ! command -v act >/dev/null 2>&1; then
  cat >&2 <<'EOF'
act is required for local GitHub Actions smoke runs.

Install it first, then retry:
  - https://github.com/nektos/act

The canonical validation contract remains:
  ./scripts/validate.sh --mode local
EOF
  exit 1
fi

cd "$ROOT_DIR"

echo "Running local GitHub Actions smoke check with act"
echo "  workflow: $WORKFLOW_FILE"
echo "  job:      $JOB_NAME"
echo "  event:    $ACT_EVENT"
echo "  image:    $ACT_IMAGE"

act "$ACT_EVENT" \
  --workflows "$WORKFLOW_FILE" \
  --job "$JOB_NAME" \
  --container-architecture linux/amd64 \
  -P ubuntu-latest="$ACT_IMAGE"
