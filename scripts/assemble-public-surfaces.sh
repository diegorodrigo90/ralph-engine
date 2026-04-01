#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${1:-$ROOT_DIR/.site-dist}"

rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR" "$OUTPUT_DIR/docs" "$OUTPUT_DIR/plugins" "$OUTPUT_DIR/pt-br/plugins"

cp -R "$ROOT_DIR/docs/public/." "$OUTPUT_DIR/"
cp "$ROOT_DIR/site/landing/styles.css" "$OUTPUT_DIR/styles.css"
cp "$ROOT_DIR/site/landing/CNAME" "$OUTPUT_DIR/CNAME"
cp "$ROOT_DIR/llms.txt" "$OUTPUT_DIR/llms.txt"

cp "$ROOT_DIR/site/landing/locales/en/index.html" "$OUTPUT_DIR/index.html"
cp "$ROOT_DIR/site/landing/locales/pt-br/index.html" "$OUTPUT_DIR/pt-br/index.html"

cp "$ROOT_DIR/catalog/locales/en/index.html" "$OUTPUT_DIR/plugins/index.html"
cp "$ROOT_DIR/catalog/locales/pt-br/index.html" "$OUTPUT_DIR/pt-br/plugins/index.html"
cp "$ROOT_DIR/catalog/index.json" "$OUTPUT_DIR/plugins/index.json"

cp -R "$ROOT_DIR/docs/.vitepress/dist/." "$OUTPUT_DIR/docs/"

if [[ -f "$ROOT_DIR/site/landing/404.html" ]]; then
  cp "$ROOT_DIR/site/landing/404.html" "$OUTPUT_DIR/404.html"
fi
