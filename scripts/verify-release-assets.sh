#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ASSET_DIR="${1:-${ROOT_DIR}/target/distrib}"
REQUIRE_ALL_TARGETS="false"

if [[ "${ASSET_DIR}" == "--require-all-targets" ]]; then
  REQUIRE_ALL_TARGETS="true"
  ASSET_DIR="${2:-${ROOT_DIR}/target/distrib}"
elif [[ "${2:-}" == "--require-all-targets" ]]; then
  REQUIRE_ALL_TARGETS="true"
fi

if command -v python3 >/dev/null 2>&1; then
  PYTHON_BIN="python3"
elif command -v python >/dev/null 2>&1; then
  PYTHON_BIN="python"
else
  echo "verify-release-assets: python interpreter not found" >&2
  exit 1
fi

if [[ ! -d "${ASSET_DIR}" ]]; then
  echo "verify-release-assets: missing directory '${ASSET_DIR}'" >&2
  exit 1
fi

mapfile -t REQUIRED_TARGETS < <("${PYTHON_BIN}" - "${ROOT_DIR}/dist-workspace.toml" <<'PY'
import pathlib
import sys
import tomllib

payload = tomllib.loads(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8"))
for target in payload["dist"]["targets"]:
    print(target)
PY
)

require_file() {
  local path="$1"
  if [[ ! -f "${path}" ]]; then
    echo "verify-release-assets: missing required file '${path}'" >&2
    exit 1
  fi
}

verify_checksum_file() {
  local checksum_path="$1"
  local base_name checksum asset_path

  require_file "${checksum_path}"

  checksum="$(awk '{print $1}' "${checksum_path}")"
  base_name="$(awk '{print $2}' "${checksum_path}" | sed 's#^\*##')"

  if [[ ! "${checksum}" =~ ^[0-9a-fA-F]{64}$ ]]; then
    echo "verify-release-assets: invalid checksum payload in '${checksum_path}'" >&2
    exit 1
  fi

  if [[ -z "${base_name}" ]]; then
    echo "verify-release-assets: missing asset name in '${checksum_path}'" >&2
    exit 1
  fi

  asset_path="${ASSET_DIR}/${base_name}"
  require_file "${asset_path}"

  (
    cd "${ASSET_DIR}"
    sha256sum -c "$(basename "${checksum_path}")" >/dev/null
  )
}

require_file "${ASSET_DIR}/source.tar.gz"
verify_checksum_file "${ASSET_DIR}/source.tar.gz.sha256"
require_file "${ASSET_DIR}/sha256.sum"

if ! find "${ASSET_DIR}" -maxdepth 1 -type f \( -name 'ralph-engine-installer.sh' -o -name 'ralph-engine-installer.ps1' \) | grep -q .; then
  echo "verify-release-assets: expected at least one installer script in '${ASSET_DIR}'" >&2
  exit 1
fi

mapfile -t binary_checksum_files < <(find "${ASSET_DIR}" -maxdepth 1 -type f -name 'ralph-engine-*.sha256' ! -name 'source.tar.gz.sha256' | sort)

if [[ "${#binary_checksum_files[@]}" -eq 0 ]]; then
  echo "verify-release-assets: expected at least one binary checksum file in '${ASSET_DIR}'" >&2
  exit 1
fi

for checksum_file in "${binary_checksum_files[@]}"; do
  verify_checksum_file "${checksum_file}"
done

if [[ "${REQUIRE_ALL_TARGETS}" == "true" ]]; then
  for target in "${REQUIRED_TARGETS[@]}"; do
    mapfile -t checksum_candidates < <(find "${ASSET_DIR}" -maxdepth 1 -type f -name "ralph-engine-${target}.*.sha256" | sort)

    if [[ "${#checksum_candidates[@]}" -eq 0 ]]; then
      echo "verify-release-assets: missing checksum for target '${target}' in '${ASSET_DIR}'" >&2
      exit 1
    fi

    if [[ "${#checksum_candidates[@]}" -ne 1 ]]; then
      echo "verify-release-assets: expected one checksum file for target '${target}', found ${#checksum_candidates[@]}" >&2
      exit 1
    fi
  done
fi

(
  cd "${ASSET_DIR}"
  sha256sum -c sha256.sum >/dev/null
)

echo "verify-release-assets: ok (${ASSET_DIR})"
