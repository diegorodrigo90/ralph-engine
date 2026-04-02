#!/usr/bin/env bash

set -euo pipefail

publish_github_release="${1:-}"
publish_npm="${2:-}"
publish_homebrew="${3:-}"

if [[ -z "$publish_github_release" || -z "$publish_npm" || -z "$publish_homebrew" ]]; then
  echo "usage: verify-release-channel-contract.sh <publish_github_release> <publish_npm> <publish_homebrew>" >&2
  exit 1
fi

normalize_bool() {
  local value="${1,,}"
  case "$value" in
    true|false)
      printf '%s\n' "$value"
      ;;
    *)
      echo "invalid boolean value: $1" >&2
      exit 1
      ;;
  esac
}

publish_github_release="$(normalize_bool "$publish_github_release")"
publish_npm="$(normalize_bool "$publish_npm")"
publish_homebrew="$(normalize_bool "$publish_homebrew")"

if [[ "$publish_github_release" == "false" && "$publish_npm" == "true" ]]; then
  echo "publish_npm=true requires publish_github_release=true" >&2
  exit 1
fi

if [[ "$publish_github_release" == "false" && "$publish_homebrew" == "true" ]]; then
  echo "publish_homebrew=true requires publish_github_release=true" >&2
  exit 1
fi
