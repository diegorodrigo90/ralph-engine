#!/usr/bin/env bash
set -euo pipefail

MODE="local"
CHECKS=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --mode)
      MODE="$2"
      shift 2
      ;;
    --checks)
      CHECKS="$2"
      shift 2
      ;;
    *)
      echo "unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

default_checks_for_mode() {
  case "$1" in
    hook)
      echo "fmt,clippy,test,rustdoc,deny,audit,gitleaks,trivy,docs"
      ;;
    ci | local)
      echo "fmt,clippy,test,coverage,rustdoc,deny,audit,gitleaks,trivy,docs"
      ;;
    release)
      echo "fmt,clippy,test,coverage,rustdoc,deny,audit,gitleaks,trivy,docs,release"
      ;;
    *)
      echo "unknown validation mode: $1" >&2
      exit 1
      ;;
  esac
}

if [[ -z "$CHECKS" ]]; then
  CHECKS="$(default_checks_for_mode "$MODE")"
fi

contains_check() {
  local needle="$1"
  [[ ",${CHECKS}," == *",${needle},"* ]]
}

timestamp() {
  date +"%H:%M:%S"
}

run_check() {
  local name="$1"
  shift

  local started_at
  started_at="$(date +%s)"
  echo
  echo "==> [$MODE] $name"
  echo "    started $(timestamp)"

  "$@"

  local finished_at
  finished_at="$(date +%s)"
  echo "    finished $(timestamp) in $((finished_at - started_at))s"
}

if contains_check fmt; then
  run_check fmt cargo fmt --all --check
fi

if contains_check clippy; then
  run_check clippy cargo clippy --workspace --all-targets --all-features -- -D warnings
fi

if contains_check test; then
  run_check test cargo test --workspace --all-targets --all-features
fi

if contains_check coverage; then
  mkdir -p coverage
  run_check coverage cargo llvm-cov --workspace --all-features --lcov --output-path coverage/lcov.info
fi

if contains_check rustdoc; then
  run_check rustdoc env RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
fi

if contains_check deny; then
  run_check deny cargo deny check
fi

if contains_check audit; then
  run_check audit cargo audit
fi

if contains_check gitleaks; then
  run_check gitleaks gitleaks git --redact --exit-code 1 --config .gitleaks.toml
fi

if contains_check trivy; then
  run_check trivy trivy fs --no-progress --scanners vuln,misconfig --severity HIGH,CRITICAL --exit-code 1 --skip-dirs docs/node_modules --skip-dirs target .
fi

if contains_check docs; then
  run_check docs npm --prefix docs run build
fi

if contains_check release; then
  run_check release cargo build --workspace --release
fi

echo
echo "Validation completed for mode '$MODE'."
