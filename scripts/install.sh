#!/usr/bin/env bash
# ralph-engine installer — works on Linux, macOS, Windows (WSL2).
# Usage: curl -fsSL https://raw.githubusercontent.com/diegorodrigo90/ralph-engine/main/scripts/install.sh | bash
# Or locally: ./scripts/install.sh

set -euo pipefail

REPO="diegorodrigo90/ralph-engine"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
BINARY="ralph-engine"

# Colors.
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}[info]${NC} $*"; }
warn()  { echo -e "${YELLOW}[warn]${NC} $*"; }
error() { echo -e "${RED}[error]${NC} $*" >&2; exit 1; }

# Detect OS and arch.
detect_platform() {
  OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
  ARCH="$(uname -m)"

  case "$OS" in
    linux)  OS="linux" ;;
    darwin) OS="darwin" ;;
    *mingw*|*msys*|*cygwin*) OS="windows" ;;
    *) error "Unsupported OS: $OS" ;;
  esac

  case "$ARCH" in
    x86_64|amd64)  ARCH="amd64" ;;
    aarch64|arm64) ARCH="arm64" ;;
    *) error "Unsupported architecture: $ARCH" ;;
  esac

  info "Detected: ${OS}/${ARCH}"
}

# Try downloading a pre-built binary from GitHub Releases.
install_from_release() {
  LATEST=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null | grep '"tag_name"' | sed -E 's/.*"v([^"]+)".*/\1/' || echo "")

  if [ -z "$LATEST" ]; then
    warn "No releases found. Will build from source."
    return 1
  fi

  info "Latest release: v${LATEST}"

  EXT="tar.gz"
  [ "$OS" = "windows" ] && EXT="zip"

  URL="https://github.com/${REPO}/releases/download/v${LATEST}/${BINARY}_${LATEST}_${OS}_${ARCH}.${EXT}"
  info "Downloading: $URL"

  TMPDIR=$(mktemp -d)
  trap "rm -rf $TMPDIR" EXIT

  if curl -fsSL "$URL" -o "$TMPDIR/archive.${EXT}" 2>/dev/null; then
    if [ "$EXT" = "tar.gz" ]; then
      tar -xzf "$TMPDIR/archive.${EXT}" -C "$TMPDIR"
    else
      unzip -q "$TMPDIR/archive.${EXT}" -d "$TMPDIR"
    fi

    if [ -f "$TMPDIR/$BINARY" ]; then
      sudo install -m 755 "$TMPDIR/$BINARY" "$INSTALL_DIR/$BINARY" 2>/dev/null \
        || install -m 755 "$TMPDIR/$BINARY" "$INSTALL_DIR/$BINARY"
      info "Installed: $INSTALL_DIR/$BINARY (v${LATEST})"
      return 0
    fi
  fi

  warn "Download failed. Will build from source."
  return 1
}

# Build from source using Go.
install_from_source() {
  if ! command -v go &>/dev/null; then
    echo ""
    warn "Go is required to build from source."
    echo ""
    echo "Install Go:"
    echo "  Arch Linux:   sudo pacman -S go"
    echo "  Ubuntu/Debian: sudo apt install golang-go"
    echo "  macOS:         brew install go"
    echo "  Windows (WSL): sudo apt install golang-go"
    echo "  Manual:        https://go.dev/dl/"
    echo ""
    echo "Or download a pre-built binary from:"
    echo "  https://github.com/${REPO}/releases"
    exit 1
  fi

  GO_VERSION=$(go version | grep -oE '[0-9]+\.[0-9]+')
  info "Found Go ${GO_VERSION}"

  info "Installing from source..."
  go install "github.com/${REPO}/cmd/${BINARY}@latest"

  # Check GOPATH/bin is in PATH.
  GOBIN=$(go env GOPATH)/bin
  if ! echo "$PATH" | grep -q "$GOBIN"; then
    warn "Add to your PATH: export PATH=\$PATH:$GOBIN"
    warn "Or add to ~/.bashrc / ~/.zshrc"
  fi

  info "Installed: $GOBIN/$BINARY"
}

# Main.
main() {
  echo "ralph-engine installer"
  echo ""

  detect_platform

  # Try binary download first, fall back to source.
  if ! install_from_release; then
    install_from_source
  fi

  echo ""
  info "Done! Run: ralph-engine version"
  info "Quick start: ralph-engine init --preset basic"
}

main "$@"
