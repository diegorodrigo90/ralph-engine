#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required" >&2
  exit 1
fi

cargo install cargo-llvm-cov --version 0.8.5 --locked
cargo install cargo-audit --version 0.22.1 --locked
cargo install cargo-deny --version 0.19.0 --locked
cargo install cargo-dist --version 0.31.0 --locked
