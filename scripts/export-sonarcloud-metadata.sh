#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROPERTIES_FILE="${ROOT_DIR}/sonar-project.properties"

fail() {
  echo "SonarCloud metadata export failed: $*" >&2
  exit 1
}

read_property() {
  local property_name="$1"

  awk -F= -v key="$property_name" '
    $1 == key {
      sub(/^[^=]*=/, "", $0)
      print $0
      exit
    }
  ' "$PROPERTIES_FILE"
}

if [[ ! -f "$PROPERTIES_FILE" ]]; then
  fail "missing sonar-project.properties"
fi

PROJECT_KEY="$(read_property sonar.projectKey)"
ORGANIZATION_KEY="$(read_property sonar.organization)"

if [[ -z "$PROJECT_KEY" ]]; then
  fail "sonar.projectKey is missing in sonar-project.properties"
fi

if [[ -z "$ORGANIZATION_KEY" ]]; then
  fail "sonar.organization is missing in sonar-project.properties"
fi

if [[ -n "${GITHUB_ENV:-}" ]]; then
  {
    printf 'SONAR_PROJECT_KEY=%s\n' "$PROJECT_KEY"
    printf 'SONAR_ORGANIZATION=%s\n' "$ORGANIZATION_KEY"
  } >>"$GITHUB_ENV"
fi

echo "Resolved SonarCloud metadata:"
echo "  project key: ${PROJECT_KEY}"
echo "  organization: ${ORGANIZATION_KEY}"
