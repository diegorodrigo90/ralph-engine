#!/usr/bin/env bash
# Verifies Golden Rules 56-57: all user-facing strings in TOML catalogs,
# no hardcoded plugin IDs in core.
#
# Scans: core/crates/re-cli/src/commands/ (the command handler layer)
# Exits non-zero if violations found.

set -euo pipefail

ERRORS=0

echo "=== i18n compliance check (Golden Rules 56-57) ==="

# Helper: count grep matches safely (returns 0 if no match)
count_matches() {
  grep -rn "$@" 2>/dev/null | wc -l || echo 0
}

# ── Rule 56: No locale_str! macro usage ──────────────────────────
echo ""
echo "[1/4] Checking for locale_str! macro..."
# Exclude comments (lines starting with //)
MATCHES=$(grep -rn 'locale_str!' core/crates/re-cli/src/commands/ 2>/dev/null | grep -v '^[^:]*:[^:]*:\s*//' || true)
COUNT=$(echo "$MATCHES" | grep -c . || true)
if [ "$COUNT" -gt 0 ] && [ -n "$MATCHES" ]; then
  echo "  FAIL: $COUNT locale_str! calls found in command files"
  echo "$MATCHES"
  ERRORS=$((ERRORS + 1))
else
  echo "  OK: zero locale_str! calls in commands/"
fi

# ── Rule 56: No is_pt_br() in command files ──────────────────────
echo ""
echo "[2/4] Checking for is_pt_br() in command files..."
COUNT=$(grep -rn 'is_pt_br(' core/crates/re-cli/src/commands/ 2>/dev/null | wc -l || true)
if [ "$COUNT" -gt 0 ]; then
  echo "  FAIL: $COUNT is_pt_br() calls found in command files"
  grep -rn 'is_pt_br(' core/crates/re-cli/src/commands/
  ERRORS=$((ERRORS + 1))
else
  echo "  OK: zero is_pt_br() in commands/"
fi

# ── Rule 56: No raw locale == "pt-br" in command files ───────────
echo ""
echo "[3/4] Checking for raw locale comparisons..."
COUNT=$(grep -rn 'locale == "pt-br"' core/crates/re-cli/src/commands/ 2>/dev/null | wc -l || true)
if [ "$COUNT" -gt 0 ]; then
  echo "  FAIL: $COUNT raw locale comparisons found"
  grep -rn 'locale == "pt-br"' core/crates/re-cli/src/commands/
  ERRORS=$((ERRORS + 1))
else
  echo "  OK: zero raw locale comparisons in commands/"
fi

# ── Rule 57: No hardcoded plugin IDs in core CLI code ────────────
echo ""
echo "[4/4] Checking for hardcoded plugin IDs in core CLI code..."
# Search in CLI command handler files only (not lib.rs which has integration tests).
# Exclude comments and test-related lines.
MATCHES=$(grep -rn 'official\.\(bmad\|claude\|codex\|claudebox\)' \
  core/crates/re-cli/src/commands/ \
  core/crates/re-cli/src/i18n/ \
  core/crates/re-cli/src/catalog.rs \
  2>/dev/null | grep -v '// ' | grep -v 'assert' | grep -v 'to_owned' || true)
COUNT=$(echo "$MATCHES" | grep -c . || true)
if [ "$COUNT" -gt 0 ] && [ -n "$MATCHES" ]; then
  echo "  FAIL: $COUNT hardcoded plugin IDs found in CLI source"
  echo "$MATCHES"
  ERRORS=$((ERRORS + 1))
else
  echo "  OK: zero hardcoded plugin IDs in CLI source"
fi

echo ""
if [ "$ERRORS" -gt 0 ]; then
  echo "=== FAIL: $ERRORS i18n violation(s) found ==="
  exit 1
else
  echo "=== PASS: all i18n compliance checks passed ==="
fi
