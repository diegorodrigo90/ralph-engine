#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DOCS_DIR="$ROOT_DIR/docs"
CONTENT_DIR="$DOCS_DIR/.content"
SOURCE_DIR="$DOCS_DIR/locales"

if [[ ! -d "$SOURCE_DIR/en" || ! -d "$SOURCE_DIR/pt-br" ]]; then
  echo "docs locale sources are missing under $SOURCE_DIR" >&2
  exit 1
fi

rm -rf "$CONTENT_DIR"
mkdir -p "$CONTENT_DIR/pt-br" "$CONTENT_DIR/.vitepress" "$CONTENT_DIR/public"

cp "$DOCS_DIR/.vitepress/config.mts" "$CONTENT_DIR/.vitepress/config.mts"
cp -R "$DOCS_DIR/.vitepress/theme" "$CONTENT_DIR/.vitepress/theme"
cp -R "$DOCS_DIR/public/." "$CONTENT_DIR/public"

cp "$SOURCE_DIR/en/index.md" "$CONTENT_DIR/index.md"
cp -R "$SOURCE_DIR/en/getting-started" "$CONTENT_DIR/getting-started"
cp -R "$SOURCE_DIR/en/guides" "$CONTENT_DIR/guides"
cp -R "$SOURCE_DIR/en/reference" "$CONTENT_DIR/reference"
cp -R "$SOURCE_DIR/en/development" "$CONTENT_DIR/development"

cp "$SOURCE_DIR/pt-br/index.md" "$CONTENT_DIR/pt-br/index.md"
cp -R "$SOURCE_DIR/pt-br/getting-started" "$CONTENT_DIR/pt-br/getting-started"
cp -R "$SOURCE_DIR/pt-br/guides" "$CONTENT_DIR/pt-br/guides"
cp -R "$SOURCE_DIR/pt-br/reference" "$CONTENT_DIR/pt-br/reference"
cp -R "$SOURCE_DIR/pt-br/development" "$CONTENT_DIR/pt-br/development"
