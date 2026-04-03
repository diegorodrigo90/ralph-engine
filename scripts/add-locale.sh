#!/usr/bin/env bash
# add-locale.sh — Guide for adding a new locale to Ralph Engine.
#
# Usage: ./scripts/add-locale.sh <locale-id>
# Example: ./scripts/add-locale.sh es  (Spanish)
#          ./scripts/add-locale.sh fr  (French)
#
# Lists every file that needs to be created or modified. The i18n system
# uses TOML files — no Rust knowledge is needed for translations.

set -euo pipefail

if [ $# -lt 1 ]; then
  echo "Usage: $0 <locale-id>"
  echo "Example: $0 es"
  exit 1
fi

LOCALE="$1"
LOCALE_MOD="${LOCALE//-/_}"  # "pt-br" → "pt_br"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "=== Add locale: ${LOCALE} (module: ${LOCALE_MOD}) ==="
echo ""

# Step 1: Register the locale in re-config
echo "--- Step 1: Register locale in re-config ---"
echo "  File: core/crates/re-config/src/lib.rs"
echo "  Actions:"
echo "    1. Add variant to SupportedLocale enum"
echo "    2. Add locale descriptor in SUPPORTED_LOCALES array"
echo "    3. Add match arm in parse_supported_locale()"
echo "    4. Add match arm in parse_os_locale()"
echo ""

# Step 2: Create TOML locale files
echo "--- Step 2: Create TOML locale files ---"
echo "  Copy en.toml as ${LOCALE}.toml in each crate and translate the values."
echo ""

TOML_COUNT=0
while IFS= read -r f; do
  dir="$(dirname "$f")"
  rel="${dir#"$ROOT/"}"
  target="${rel}/${LOCALE}.toml"

  if [ -f "${ROOT}/${target}" ]; then
    echo "  [EXISTS] ${target}"
  else
    echo "  [CREATE] ${target}  (copy from ${rel}/en.toml, translate)"
  fi
  TOML_COUNT=$((TOML_COUNT + 1))
done < <(find "$ROOT/core/crates" "$ROOT/plugins" -path "*/locales/en.toml" -type f 2>/dev/null | sort)
echo ""

# Step 3: Hand-coded fn files (only re-core and re-cli)
echo "--- Step 3: Translation functions (re-core and re-cli only) ---"
for crate in re-core re-cli; do
  fn_file="core/crates/${crate}/src/i18n/fn_${LOCALE_MOD}.rs"
  en_file="core/crates/${crate}/src/i18n/fn_en.rs"
  if [ -f "${ROOT}/${fn_file}" ]; then
    echo "  [EXISTS] ${fn_file}"
  else
    echo "  [CREATE] ${fn_file}  (copy from ${en_file}, translate)"
  fi
done
echo ""

# Step 4: Documentation
echo "--- Step 4: Documentation locale ---"
DOC_DIR="docs/locales/${LOCALE}"
if [ -d "${ROOT}/${DOC_DIR}" ]; then
  echo "  [EXISTS] ${DOC_DIR}/"
else
  echo "  [CREATE] ${DOC_DIR}/  (copy from docs/locales/en/, translate)"
fi
echo ""

# Step 5: VitePress config
echo "--- Step 5: Update VitePress config ---"
echo "  File: docs/.vitepress/config.mts"
echo "  Action: Add locale entry for '${LOCALE}'"
echo ""

# Summary
echo "=== Summary ==="
echo "  TOML files to create: ${TOML_COUNT}"
echo "  fn files to create: 2 (re-core + re-cli)"
echo "  re-config changes: 1 file"
echo "  Documentation: 1 directory"
echo "  VitePress config: 1 file"
echo ""
echo "After all changes, run:"
echo "  cargo fmt --all"
echo "  cargo clippy --workspace --all-targets --all-features -- -D warnings"
echo "  cargo test --workspace --all-targets"
echo "  npm --prefix docs run build"
