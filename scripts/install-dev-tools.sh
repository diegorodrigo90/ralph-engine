#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required" >&2
  exit 1
fi

TOOLS="cargo-llvm-cov,cargo-audit,cargo-deny,cargo-dist,gitleaks,trivy"
TOOLS_BIN_DIR="${HOME}/.cargo/bin"
TOOLS_CACHE_DIR="${HOME}/.cache/ralph-engine-tools"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --tools)
      TOOLS="$2"
      shift 2
      ;;
    *)
      echo "unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

has_tool() {
  local needle="$1"
  [[ ",${TOOLS}," == *",${needle},"* ]]
}

ensure_bin_dir() {
  mkdir -p "$TOOLS_BIN_DIR" "$TOOLS_CACHE_DIR"
}

cached_binary_path() {
  local binary_name="$1"
  local version="$2"

  printf '%s/%s/%s/%s\n' "$TOOLS_CACHE_DIR" "$binary_name" "$version" "$binary_name"
}

restore_cached_binary() {
  local binary_name="$1"
  local version="$2"

  local cached_path
  cached_path="$(cached_binary_path "$binary_name" "$version")"

  if [[ -x "$cached_path" ]]; then
    install -m 0755 "$cached_path" "$TOOLS_BIN_DIR/${binary_name}"
    return 0
  fi

  return 1
}

store_cached_binary() {
  local binary_name="$1"
  local version="$2"

  local cached_path
  cached_path="$(cached_binary_path "$binary_name" "$version")"
  mkdir -p "$(dirname "$cached_path")"
  install -m 0755 "$TOOLS_BIN_DIR/${binary_name}" "$cached_path"
}

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
  install -m 0755 "$tmp_dir/${binary_name}" "$TOOLS_BIN_DIR/${binary_name}"
  store_cached_binary "$binary_name" "$version"
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
  install -m 0755 "$tmp_dir/${binary_name}" "$TOOLS_BIN_DIR/${binary_name}"
  store_cached_binary "$binary_name" "$version"
}

ensure_cargo_installed_tool() {
  local cache_binary_name="$1"
  local version="$2"
  local version_command="$3"
  shift 3

  if eval "$version_command" 2>/dev/null | grep -Fq "$version"; then
    echo "${cache_binary_name} ${version} already available"
    return 0
  fi

  if restore_cached_binary "$cache_binary_name" "$version" && eval "$version_command" 2>/dev/null | grep -Fq "$version"; then
    echo "${cache_binary_name} ${version} restored from cache"
    return 0
  fi

  cargo install "$@" --version "$version" --locked

  if ! eval "$version_command" 2>/dev/null | grep -Fq "$version"; then
    echo "failed to install ${cache_binary_name} ${version}" >&2
    exit 1
  fi

  store_cached_binary "$cache_binary_name" "$version"
}

ensure_downloaded_binary() {
  local binary_name="$1"
  local version="$2"
  local version_command="$3"
  local version_check_output="$4"
  shift 4

  if eval "$version_command" 2>/dev/null | grep -Fq "$version_check_output"; then
    echo "${binary_name} ${version} already available"
    return 0
  fi

  if restore_cached_binary "$binary_name" "$version" && eval "$version_command" 2>/dev/null | grep -Fq "$version_check_output"; then
    echo "${binary_name} ${version} restored from cache"
    return 0
  fi

  "$@"

  if ! eval "$version_command" 2>/dev/null | grep -Fq "$version_check_output"; then
    echo "failed to install ${binary_name} ${version}" >&2
    exit 1
  fi
}

platform="$(uname -s)"
arch="$(uname -m)"
ensure_bin_dir

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

if has_tool cargo-llvm-cov; then
  ensure_cargo_installed_tool cargo-llvm-cov 0.8.5 "cargo llvm-cov --version" cargo-llvm-cov
fi

if has_tool cargo-audit; then
  ensure_cargo_installed_tool cargo-audit 0.22.1 "cargo audit --version" cargo-audit
fi

if has_tool cargo-deny; then
  ensure_cargo_installed_tool cargo-deny 0.19.0 "cargo deny --version" cargo-deny
fi

if has_tool cargo-dist; then
  ensure_cargo_installed_tool dist 0.31.0 "dist --version" cargo-dist
fi

if has_tool gitleaks; then
  ensure_downloaded_binary gitleaks v8.30.1 "gitleaks version" 8.30.1 \
    install_binary_from_github_tarball 'gitleaks/gitleaks' 'v8.30.1' "$gitleaks_asset" 'gitleaks'
fi

if has_tool trivy; then
  ensure_downloaded_binary trivy v0.69.3 "trivy --version" "Version: 0.69.3" \
    install_binary_from_github_tarball 'aquasecurity/trivy' 'v0.69.3' "$trivy_asset" 'trivy'
fi
