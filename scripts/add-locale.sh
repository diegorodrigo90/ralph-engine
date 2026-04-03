#!/usr/bin/env bash
# add-locale.sh — Guide for adding a new locale to Ralph Engine.
#
# Usage: ./scripts/add-locale.sh <locale-id>
# Example: ./scripts/add-locale.sh es  (Spanish)
#          ./scripts/add-locale.sh fr  (French)
#
# This script lists every file that needs to be created or modified
# when adding a new locale to the codebase. It does NOT make changes
# automatically — the developer must translate strings manually.

set -euo pipefail

if [ $# -lt 1 ]; then
  echo "Usage: $0 <locale-id>"
  echo "Example: $0 es"
  exit 1
fi

LOCALE="$1"
LOCALE_MOD="${LOCALE//-/_}"  # e.g., "pt-br" → "pt_br", "es" → "es"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "=== Add locale: ${LOCALE} (module: ${LOCALE_MOD}) ==="
echo ""

# Step 1: Register the locale in re-config
echo "--- Step 1: Register locale in re-config ---"
echo "  File: core/crates/re-config/src/lib.rs"
echo "  Actions:"
echo "    1. Add variant to SupportedLocale enum (e.g., Es)"
echo "    2. Add locale descriptor in SUPPORTED_LOCALES array"
echo "    3. Add match arm in parse_supported_locale()"
echo "    4. Add match arm in parse_os_locale() for OS patterns (e.g., 'es_ES')"
echo "    5. Update supported_locales() tests"
echo ""

# Step 2: Create i18n files in each crate and plugin
echo "--- Step 2: Create i18n locale files ---"
echo "  For each i18n directory, copy en.rs as ${LOCALE_MOD}.rs and translate."
echo ""

I18N_DIRS=()
while IFS= read -r f; do
  dir="$(dirname "$f")"
  I18N_DIRS+=("$dir")
done < <(find "$ROOT/core/crates" "$ROOT/plugins" -path "*/i18n/en.rs" -type f 2>/dev/null | sort)

for dir in "${I18N_DIRS[@]}"; do
  rel="${dir#"$ROOT/"}"
  target="${rel}/${LOCALE_MOD}.rs"
  modfile="${rel}/mod.rs"

  if [ -f "${ROOT}/${target}" ]; then
    echo "  [EXISTS] ${target}"
  else
    echo "  [CREATE] ${target}  (copy from ${rel}/en.rs, translate values)"
  fi
  echo "  [MODIFY] ${modfile}  (add: pub mod ${LOCALE_MOD};)"
  echo "  [MODIFY] ${modfile}  (add PluginLocalizedText entries for '${LOCALE}')"
  echo ""
done

# Step 3: Documentation
echo "--- Step 3: Create documentation locale ---"
DOC_DIR="docs/locales/${LOCALE}"
if [ -d "${ROOT}/${DOC_DIR}" ]; then
  echo "  [EXISTS] ${DOC_DIR}/"
else
  echo "  [CREATE] ${DOC_DIR}/  (copy from docs/locales/en/, translate)"
fi
echo ""

# Step 4: VitePress config
echo "--- Step 4: Update VitePress docs config ---"
echo "  File: docs/.vitepress/config.mts"
echo "  Action: Add locale entry for '${LOCALE}' in locales config"
echo ""

# Step 5: Plugin manifests
echo "--- Step 5: Update plugin manifest.yaml files ---"
MANIFESTS=$(find "$ROOT/plugins" -name "manifest.yaml" -type f 2>/dev/null | sort)
for manifest in $MANIFESTS; do
  rel="${manifest#"$ROOT/"}"
  echo "  [MODIFY] ${rel}  (add ${LOCALE} translations for name/summary)"
done
echo ""

# Summary
TOTAL_CREATE=${#I18N_DIRS[@]}
echo "=== Summary ==="
echo "  Locale files to create: ${TOTAL_CREATE}"
echo "  mod.rs files to modify: ${TOTAL_CREATE}"
echo "  re-config changes: 1 file"
echo "  Documentation dir: 1"
echo "  VitePress config: 1"
echo "  Plugin manifests: $(echo "$MANIFESTS" | wc -l | tr -d ' ')"
echo ""
echo "After all changes, run:"
echo "  cargo fmt --all"
echo "  cargo clippy --workspace --all-targets --all-features -- -D warnings"
echo "  cargo test --workspace --all-targets"
echo "  npm --prefix docs run build"
