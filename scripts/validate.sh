#!/usr/bin/env bash
set -euo pipefail

MODE="local"
CHECKS="fmt,clippy,test,coverage,rustdoc,deny,audit,gitleaks,trivy,docs"

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

contains_check() {
  local needle="$1"
  [[ ",${CHECKS}," == *",${needle},"* ]]
}

if contains_check fmt; then
  cargo fmt --all --check
fi

if contains_check clippy; then
  cargo clippy --workspace --all-targets --all-features -- -D warnings
fi

if contains_check test; then
  cargo test --workspace --all-targets --all-features
fi

if contains_check coverage; then
  mkdir -p coverage
  cargo llvm-cov --workspace --all-features --lcov --output-path coverage/lcov.info
fi

if contains_check rustdoc; then
  RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
fi

if contains_check deny; then
  cargo deny check
fi

if contains_check audit; then
  cargo audit
fi

if contains_check gitleaks; then
  gitleaks git --redact --exit-code 1 --config .gitleaks.toml
fi

if contains_check trivy; then
  trivy fs --quiet --scanners vuln,misconfig --severity HIGH,CRITICAL --exit-code 1 --skip-dirs docs/node_modules --skip-dirs target .
fi

if contains_check docs; then
  npm --prefix docs run build
fi

if [[ "$MODE" == "release" ]]; then
  cargo build --workspace --release
fi
