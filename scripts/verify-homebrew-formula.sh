#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "verify-homebrew-formula: $*" >&2
  exit 1
}

FORMULA_PATH="${1:-}"
EXPECTED_VERSION="${2:-}"

if [[ -z "${FORMULA_PATH}" ]]; then
  fail "formula path is required as the first argument"
fi

if [[ -z "${EXPECTED_VERSION}" ]]; then
  fail "expected version is required as the second argument"
fi

if [[ ! -f "${FORMULA_PATH}" ]]; then
  fail "formula file does not exist: ${FORMULA_PATH}"
fi

if ! command -v brew >/dev/null 2>&1; then
  fail "brew is required to validate the formula"
fi

export HOMEBREW_NO_AUTO_UPDATE=1
export HOMEBREW_NO_INSTALL_CLEANUP=1
export HOMEBREW_NO_ENV_HINTS=1

formula_name="$(basename "${FORMULA_PATH}" .rb)"

cleanup() {
  brew uninstall --force "${formula_name}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

brew audit --strict --formula "${FORMULA_PATH}"
brew install --formula "${FORMULA_PATH}"
brew test "${formula_name}"

installed_version="$(ralph-engine --version | tr -d '\r' | tr -d '\n')"
if [[ "${installed_version}" != "${EXPECTED_VERSION}" ]]; then
  fail "expected ralph-engine --version to print '${EXPECTED_VERSION}', got '${installed_version}'"
fi

echo "verify-homebrew-formula: ok (${FORMULA_PATH})"
