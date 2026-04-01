#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROPERTIES_FILE="${ROOT_DIR}/sonar-project.properties"
SONAR_HOST_URL="${SONAR_HOST_URL:-https://sonarcloud.io}"

fail() {
  echo "SonarCloud preflight failed: $*" >&2
  exit 1
}

require_command() {
  local command_name="$1"

  if ! command -v "$command_name" >/dev/null 2>&1; then
    fail "required command '${command_name}' is not available"
  fi
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

require_command curl
require_command awk

if [[ ! -f "$PROPERTIES_FILE" ]]; then
  fail "missing sonar-project.properties"
fi

if [[ -z "${SONAR_TOKEN:-}" ]]; then
  fail "SONAR_TOKEN is not set"
fi

PROJECT_KEY="$(read_property sonar.projectKey)"
ORGANIZATION_KEY="$(read_property sonar.organization)"

if [[ -z "$PROJECT_KEY" ]]; then
  fail "sonar.projectKey is missing in sonar-project.properties"
fi

if [[ -z "$ORGANIZATION_KEY" ]]; then
  fail "sonar.organization is missing in sonar-project.properties"
fi

AUTH_RESPONSE="$(curl --silent --show-error --fail --user "${SONAR_TOKEN}:" "${SONAR_HOST_URL}/api/authentication/validate")"

if [[ "$AUTH_RESPONSE" != *'"valid":true'* ]]; then
  fail "SONAR_TOKEN is invalid for ${SONAR_HOST_URL}"
fi

COMPONENT_URL="${SONAR_HOST_URL}/api/components/show?component=${PROJECT_KEY}"
HTTP_STATUS="$(
  curl \
    --silent \
    --show-error \
    --output /tmp/sonar-component-response.json \
    --write-out '%{http_code}' \
    --user "${SONAR_TOKEN}:" \
    "${COMPONENT_URL}"
)"

case "$HTTP_STATUS" in
  200)
    echo "SonarCloud preflight passed for project '${PROJECT_KEY}'."
    ;;
  401 | 403 | 404)
    cat >&2 <<EOF
SonarCloud preflight failed for project '${PROJECT_KEY}' in organization '${ORGANIZATION_KEY}'.

The token authenticated successfully, but it could not access the project metadata endpoint:
  ${COMPONENT_URL}
  HTTP status: ${HTTP_STATUS}

For SonarCloud, a 404 during analysis creation commonly means the token does not have the
required project permissions. Use a dedicated SonarCloud token with access to this project.

Expected minimum permissions:
- Browse on '${PROJECT_KEY}'
- Execute Analysis on '${PROJECT_KEY}'

Recommended setup:
- store a dedicated project analysis token or organization token in GitHub secret SONAR_TOKEN
- verify that the token still has Browse and Execute Analysis permissions after any GitHub or
  SonarCloud membership change
EOF
    exit 1
    ;;
  *)
    cat >&2 <<EOF
SonarCloud preflight failed with an unexpected response.

Project: ${PROJECT_KEY}
Organization: ${ORGANIZATION_KEY}
Endpoint: ${COMPONENT_URL}
HTTP status: ${HTTP_STATUS}
EOF
    exit 1
    ;;
esac
