#!/usr/bin/env bash
# Assembles Astro site + VitePress docs into a single deploy artifact.
# Usage: ./scripts/assemble-public-surfaces.sh [output-dir]

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${1:-$ROOT_DIR/.site-dist}"

rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# 1. Copy Astro build output as the base
cp -R "$ROOT_DIR/site/dist/." "$OUTPUT_DIR/"

# 2. Overlay VitePress docs into /docs/
mkdir -p "$OUTPUT_DIR/docs"
cp -R "$ROOT_DIR/docs/.vitepress/dist/." "$OUTPUT_DIR/docs/"

if [[ -f "$ROOT_DIR/scripts/normalize-docs-sitemap.mjs" ]]; then
  node "$ROOT_DIR/scripts/normalize-docs-sitemap.mjs" "$OUTPUT_DIR/docs/sitemap.xml"
fi

# 3. Copy shared static assets
cp "$ROOT_DIR/llms.txt" "$OUTPUT_DIR/llms.txt"

echo "Assembled $(find "$OUTPUT_DIR" -name '*.html' | wc -l) HTML files into $OUTPUT_DIR"
