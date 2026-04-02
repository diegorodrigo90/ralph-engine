#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_WORKSPACE_PATH="${1:-${ROOT_DIR}/dist-workspace.toml}"

if command -v python3 >/dev/null 2>&1; then
  PYTHON_BIN="python3"
elif command -v python >/dev/null 2>&1; then
  PYTHON_BIN="python"
else
  echo "verify-dist-workspace: python interpreter not found" >&2
  exit 1
fi

if [[ ! -f "${DIST_WORKSPACE_PATH}" ]]; then
  echo "verify-dist-workspace: missing file '${DIST_WORKSPACE_PATH}'" >&2
  exit 1
fi

"${PYTHON_BIN}" - "${DIST_WORKSPACE_PATH}" <<'PY'
import pathlib
import sys
import tomllib

path = pathlib.Path(sys.argv[1])
payload = tomllib.loads(path.read_text(encoding="utf-8"))

workspace = payload.get("workspace", {})
members = workspace.get("members")
if members != ["cargo:."]:
    raise SystemExit(
        f"verify-dist-workspace: expected workspace.members=['cargo:.'], got {members!r}"
    )

dist = payload.get("dist")
if not isinstance(dist, dict):
    raise SystemExit("verify-dist-workspace: missing [dist] table")

expected_targets = [
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
]

targets = dist.get("targets")
if targets != expected_targets:
    raise SystemExit(
        f"verify-dist-workspace: expected targets {expected_targets!r}, got {targets!r}"
    )

installers = dist.get("installers")
if installers != ["shell", "powershell"]:
    raise SystemExit(
        "verify-dist-workspace: expected installers ['shell', 'powershell'], "
        f"got {installers!r}"
    )

if dist.get("hosting") != "github":
    raise SystemExit(
        f"verify-dist-workspace: expected hosting='github', got {dist.get('hosting')!r}"
    )

if dist.get("install-path") != "CARGO_HOME":
    raise SystemExit(
        "verify-dist-workspace: expected install-path='CARGO_HOME', "
        f"got {dist.get('install-path')!r}"
    )

if dist.get("install-updater") is not False:
    raise SystemExit(
        "verify-dist-workspace: expected install-updater=false, "
        f"got {dist.get('install-updater')!r}"
    )

version = dist.get("cargo-dist-version")
if not isinstance(version, str) or not version:
    raise SystemExit("verify-dist-workspace: missing non-empty cargo-dist-version")

allow_dirty = dist.get("allow-dirty")
if allow_dirty != ["ci"]:
    raise SystemExit(
        f"verify-dist-workspace: expected allow-dirty=['ci'], got {allow_dirty!r}"
    )

print(f"verify-dist-workspace: ok ({path})")
PY
