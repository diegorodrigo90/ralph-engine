#!/usr/bin/env bash
set -euo pipefail

MODE="local"
CHECKS=""
BASE_SHA="${GIT_BASE_SHA:-}"
HEAD_SHA="${GIT_HEAD_SHA:-HEAD}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --mode)
      MODE="$2"
      shift 2
      ;;
    --checks)
      CHECKS="$2"
      shift 2
      ;;
    --base)
      BASE_SHA="$2"
      shift 2
      ;;
    --head)
      HEAD_SHA="$2"
      shift 2
      ;;
    *)
      echo "unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

default_checks_for_mode() {
  case "$1" in
    hook)
      echo "fmt,clippy,test,contracts,rustdoc,deny,audit,gitleaks,trivy,public"
      ;;
    ci | local)
      echo "fmt,clippy,test,contracts,coverage,rustdoc,deny,audit,gitleaks,trivy,public"
      ;;
    release)
      echo "fmt,clippy,test,contracts,coverage,rustdoc,deny,audit,gitleaks,trivy,public,release"
      ;;
    *)
      echo "unknown validation mode: $1" >&2
      exit 1
      ;;
  esac
}

if [[ -z "$CHECKS" ]]; then
  CHECKS="$(default_checks_for_mode "$MODE")"
fi

contains_requested_check() {
  local needle="$1"
  [[ ",${CHECKS}," == *",${needle},"* ]]
}

timestamp() {
  date +"%H:%M:%S"
}

run_check() {
  local name="$1"
  shift

  local started_at
  started_at="$(date +%s)"
  echo
  echo "==> [$MODE] $name"
  echo "    started $(timestamp)"

  "$@"

  local finished_at
  finished_at="$(date +%s)"
  echo "    finished $(timestamp) in $((finished_at - started_at))s"
}

skip_check() {
  local name="$1"
  local reason="$2"
  echo
  echo "==> [$MODE] $name"
  echo "    skipped: $reason"
}

detect_base_sha() {
  if [[ -n "$BASE_SHA" ]]; then
    if git rev-parse --verify "$BASE_SHA" >/dev/null 2>&1; then
      git merge-base "$BASE_SHA" "$HEAD_SHA"
      return 0
    fi

    echo ""
    return 0
  fi

  if git rev-parse --verify HEAD~1 >/dev/null 2>&1; then
    git rev-parse HEAD~1
    return 0
  fi

  echo ""
}

collect_working_tree_files() {
  {
    git diff --name-only HEAD
    git ls-files --others --exclude-standard
  } | awk 'NF { print }' | sort -u
}

collect_changed_files() {
  if [[ -z "$BASE_SHA" && "$MODE" != "ci" && "$MODE" != "release" ]]; then
    collect_working_tree_files
    return 0
  fi

  local effective_base
  effective_base="$(detect_base_sha)"

  if [[ -z "$effective_base" ]]; then
    return 0
  fi

  git diff --name-only "$effective_base" "$HEAD_SHA"
}

CHANGED_FILES="$(collect_changed_files)"

has_any_changes() {
  [[ -n "$CHANGED_FILES" ]]
}

file_is_rust_only_safe() {
  local changed_file="$1"

  [[ "$changed_file" == core/* ]] ||
    [[ "$changed_file" == plugins/official/* ]] ||
    [[ "$changed_file" == Cargo.toml ]] ||
    [[ "$changed_file" == Cargo.lock ]] ||
    [[ "$changed_file" == rust-toolchain.toml ]]
}

file_is_public_surface_safe() {
  local changed_file="$1"

  [[ "$changed_file" == docs/* ]] ||
    [[ "$changed_file" == site/* ]] ||
    [[ "$changed_file" == llms.txt ]]
}

all_changes_match() {
  local predicate_name="$1"
  local saw_any="false"

  if [[ -z "$CHANGED_FILES" ]]; then
    return 1
  fi

  while IFS= read -r changed_file; do
    [[ -z "$changed_file" ]] && continue
    saw_any="true"
    if ! "$predicate_name" "$changed_file"; then
      return 1
    fi
  done <<<"$CHANGED_FILES"

  [[ "$saw_any" == "true" ]]
}

validation_scope() {
  if ! has_any_changes; then
    echo "full"
    return 0
  fi

  if all_changes_match file_is_rust_only_safe; then
    echo "rust-only"
    return 0
  fi

  if all_changes_match file_is_public_surface_safe; then
    echo "public-only"
    return 0
  fi

  echo "full"
}

SCOPE="$(validation_scope)"

should_run_check() {
  local check_name="$1"

  if ! contains_requested_check "$check_name"; then
    return 1
  fi

  if ! has_any_changes; then
    return 0
  fi

  case "$check_name" in
    fmt | clippy | test | coverage | rustdoc)
      [[ "$SCOPE" == "rust-only" || "$SCOPE" == "full" ]]
      ;;
    contracts)
      [[ "$SCOPE" == "rust-only" || "$SCOPE" == "full" ]]
      ;;
    deny | audit | trivy)
      [[ "$SCOPE" == "rust-only" || "$SCOPE" == "full" ]]
      ;;
    public)
      [[ "$SCOPE" == "public-only" || "$SCOPE" == "full" ]]
      ;;
    release)
      [[ "$SCOPE" == "rust-only" || "$SCOPE" == "full" ]]
      ;;
    gitleaks)
      return 0
      ;;
    *)
      return 0
      ;;
  esac
}

print_changed_files_context() {
  echo "Validation mode: $MODE"
  echo "Validation scope: $SCOPE"

  if has_any_changes; then
    echo "Changed files considered for selective validation:"
    while IFS= read -r changed_file; do
      [[ -z "$changed_file" ]] && continue
      echo "  - $changed_file"
    done <<<"$CHANGED_FILES"
  else
    echo "Changed files: none detected or no diff base available; running requested checks conservatively."
  fi
}

print_changed_files_context

if should_run_check fmt; then
  run_check fmt cargo fmt --all --check
elif contains_requested_check fmt; then
  skip_check fmt "public-only change set"
fi

if should_run_check clippy; then
  run_check clippy cargo clippy --workspace --all-targets --all-features -- -D warnings
elif contains_requested_check clippy; then
  skip_check clippy "public-only change set"
fi

if should_run_check test; then
  run_check test cargo test --workspace --all-targets --all-features
elif contains_requested_check test; then
  skip_check test "public-only change set"
fi

if should_run_check contracts; then
  run_check contracts npm run contracts:verify
elif contains_requested_check contracts; then
  skip_check contracts "public-only change set"
fi

if should_run_check coverage; then
  mkdir -p coverage
  run_check coverage cargo llvm-cov --workspace --all-features --lcov --output-path coverage/lcov.info
elif contains_requested_check coverage; then
  skip_check coverage "public-only change set"
fi

if should_run_check rustdoc; then
  run_check rustdoc env RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
elif contains_requested_check rustdoc; then
  skip_check rustdoc "public-only change set"
fi

if should_run_check deny; then
  run_check deny cargo deny check
elif contains_requested_check deny; then
  skip_check deny "public-only change set"
fi

if should_run_check audit; then
  run_check audit cargo audit
elif contains_requested_check audit; then
  skip_check audit "public-only change set"
fi

if should_run_check gitleaks; then
  run_check gitleaks gitleaks git --redact --exit-code 1 --config .gitleaks.toml
fi

if should_run_check trivy; then
  run_check trivy trivy fs --no-progress --scanners vuln,misconfig --severity HIGH,CRITICAL --exit-code 1 --skip-dirs docs/node_modules --skip-dirs target .
elif contains_requested_check trivy; then
  skip_check trivy "public-only change set"
fi

if should_run_check public; then
  run_check public bash -lc "cd site && npm run build"
elif contains_requested_check public; then
  skip_check public "rust-only change set"
fi

if should_run_check release; then
  run_check release cargo build --workspace --release
elif contains_requested_check release; then
  skip_check release "public-only change set"
fi

echo
echo "Validation completed for mode '$MODE'."
