#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${1:-$ROOT_DIR/.site-dist}"

rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR" "$OUTPUT_DIR/docs" "$OUTPUT_DIR/plugins" "$OUTPUT_DIR/pt-br/plugins"

cp -R "$ROOT_DIR/docs/public/." "$OUTPUT_DIR/"
cp "$ROOT_DIR/site/ui/styles.css" "$OUTPUT_DIR/styles.css"
cp "$ROOT_DIR/site/ui/public-shell.js" "$OUTPUT_DIR/public-shell.js"
cp "$ROOT_DIR/site/landing/CNAME" "$OUTPUT_DIR/CNAME"
cp "$ROOT_DIR/llms.txt" "$OUTPUT_DIR/llms.txt"
node "$ROOT_DIR/site/ui/render-public-pages.mjs" "$OUTPUT_DIR"

cp -R "$ROOT_DIR/docs/.vitepress/dist/." "$OUTPUT_DIR/docs/"
node "$ROOT_DIR/scripts/normalize-docs-sitemap.mjs" "$OUTPUT_DIR/docs/sitemap.xml"

if [[ -f "$ROOT_DIR/site/landing/404.html" ]]; then
  cp "$ROOT_DIR/site/landing/404.html" "$OUTPUT_DIR/404.html"
fi
