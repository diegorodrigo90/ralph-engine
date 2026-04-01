#!/usr/bin/env bash
set -euo pipefail

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required" >&2
  exit 1
fi

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"

python3 - <<'PY' "$repo_root"
from pathlib import Path
import re
import sys

repo_root = Path(sys.argv[1])
cargo_toml = (repo_root / "Cargo.toml").read_text(encoding="utf-8")
match = re.search(r'^\s*version\s*=\s*"([^"]+)"\s*$', cargo_toml, re.MULTILINE)

if not match:
    print("workspace version not found in Cargo.toml", file=sys.stderr)
    sys.exit(1)

version = match.group(1)
print(version)
PY
