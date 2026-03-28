#!/usr/bin/env bash
# Build ralph-engine from source.
# Usage: ./scripts/build-local.sh
# Binary: ./bin/ralph-engine

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

GREEN='\033[0;32m'
NC='\033[0m'

info() { echo -e "${GREEN}[info]${NC} $*"; }

if ! command -v go &>/dev/null; then
  echo "Go is required. Install:"
  echo "  Arch Linux:    sudo pacman -S go"
  echo "  Ubuntu/Debian: sudo apt install golang-go"
  echo "  macOS:         brew install go"
  echo "  Manual:        https://go.dev/dl/"
  exit 1
fi

info "Building ralph-engine..."
cd "$PROJECT_DIR"
mkdir -p bin

VERSION=$(git describe --tags --always --dirty 2>/dev/null || echo "dev")
go build \
  -ldflags "-s -w -X github.com/diegorodrigo90/ralph-engine/internal/cli.Version=${VERSION}" \
  -o bin/ralph-engine \
  ./cmd/ralph-engine/

info "Built: bin/ralph-engine (${VERSION})"
info "Run:   ./bin/ralph-engine --help"
