#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required" >&2
  exit 1
fi

install_binary_from_github_tarball() {
  local owner_repo="$1"
  local version="$2"
  local asset_name="$3"
  local binary_name="$4"

  local tmp_dir
  tmp_dir="$(mktemp -d)"
  trap 'rm -rf "$tmp_dir"' RETURN

  curl -fsSL -o "$tmp_dir/archive.tar.gz" "https://github.com/${owner_repo}/releases/download/${version}/${asset_name}"
  tar -xzf "$tmp_dir/archive.tar.gz" -C "$tmp_dir"
  install -m 0755 "$tmp_dir/${binary_name}" "$HOME/.cargo/bin/${binary_name}"
}

install_binary_from_github_zip() {
  local owner_repo="$1"
  local version="$2"
  local asset_name="$3"
  local binary_name="$4"

  local tmp_dir
  tmp_dir="$(mktemp -d)"
  trap 'rm -rf "$tmp_dir"' RETURN

  curl -fsSL -o "$tmp_dir/archive.zip" "https://github.com/${owner_repo}/releases/download/${version}/${asset_name}"
  unzip -q "$tmp_dir/archive.zip" -d "$tmp_dir"
  install -m 0755 "$tmp_dir/${binary_name}" "$HOME/.cargo/bin/${binary_name}"
}

platform="$(uname -s)"
arch="$(uname -m)"

case "$platform/$arch" in
  Linux/x86_64)
    gitleaks_asset='gitleaks_8.30.1_linux_x64.tar.gz'
    trivy_asset='trivy_0.69.3_Linux-64bit.tar.gz'
    ;;
  Linux/aarch64|Linux/arm64)
    gitleaks_asset='gitleaks_8.30.1_linux_arm64.tar.gz'
    trivy_asset='trivy_0.69.3_Linux-ARM64.tar.gz'
    ;;
  Darwin/x86_64)
    gitleaks_asset='gitleaks_8.30.1_darwin_x64.tar.gz'
    trivy_asset='trivy_0.69.3_macOS-64bit.tar.gz'
    ;;
  Darwin/arm64)
    gitleaks_asset='gitleaks_8.30.1_darwin_arm64.tar.gz'
    trivy_asset='trivy_0.69.3_macOS-ARM64.tar.gz'
    ;;
  *)
    echo "unsupported platform for gitleaks/trivy install: ${platform}/${arch}" >&2
    exit 1
    ;;
esac

cargo install cargo-llvm-cov --version 0.8.5 --locked
cargo install cargo-audit --version 0.22.1 --locked
cargo install cargo-deny --version 0.19.0 --locked
cargo install cargo-dist --version 0.31.0 --locked

install_binary_from_github_tarball 'gitleaks/gitleaks' 'v8.30.1' "$gitleaks_asset" 'gitleaks'
install_binary_from_github_tarball 'aquasecurity/trivy' 'v0.69.3' "$trivy_asset" 'trivy'
