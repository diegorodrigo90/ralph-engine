#!/usr/bin/env bash
set -euo pipefail

if command -v asdf >/dev/null 2>&1; then
  if asdf plugin list | grep -qx 'rust' && asdf plugin list | grep -qx 'nodejs'; then
    asdf install
  else
    echo "asdf found but missing rust and/or nodejs plugins; skipping asdf install"
  fi
else
  echo "asdf not found; continuing with the existing local toolchain"
fi

npm ci
npm --prefix docs ci
./scripts/install-dev-tools.sh
npx --no -- lefthook install
