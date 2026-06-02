#!/bin/bash
#
# check_public_ip_boundary.sh
# AGENT 9 — Public IP Boundary Validator
#
# Purpose: Verify no private IP ranges, internal hostnames, or sensitive
# identifiers appear in public documentation surfaces.
#
# Patterns checked:
# - Private IP ranges: 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
# - Internal hostnames: *.internal, *.local, localhost (outside examples)
#
# Allowed:
# - RFC 1918 examples clearly marked as "example" or "demonstration"
# - localhost in test documentation
# - Internal hostnames in development guides with clear "dev only" labels
#
# Classification:
# - SOURCE_RISK: Private IPs/hostnames in production docs without clear examples label
# - DOCS_ACCEPTABLE: Found in docs with "example", "demonstration", "test" context
# - ABSENT: Not found
#
# Exit: 0 if clean; 1 if SOURCE_RISK
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DOCS_DIR="${PROJECT_ROOT}/docs"

# IP patterns
declare -a PRIVATE_IP_PATTERNS=(
  "10\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}"  # 10.0.0.0/8
  "172\.1[6-9]\.[0-9]\{1,3\}\.[0-9]\{1,3\}"        # 172.16.0.0/12
  "172\.2[0-9]\.[0-9]\{1,3\}\.[0-9]\{1,3\}"
  "172\.3[01]\.[0-9]\{1,3\}\.[0-9]\{1,3\}"
  "192\.168\.[0-9]\{1,3\}\.[0-9]\{1,3\}"           # 192.168.0.0/16
)

# Hostname patterns
declare -a INTERNAL_HOSTNAME_PATTERNS=(
  "\.internal"
  "\.local"
  "localhost"
)

# Acceptable context keywords
declare -a ACCEPTABLE_CONTEXTS=(
  "example"
  "demonstration"
  "test"
  "demo"
  "dev"
  "development"
)

# Result tracking
declare -a VIOLATIONS
RISK_LEVEL="ABSENT"

##############################################################################
# Helper Functions
##############################################################################

has_acceptable_context() {
  local line="$1"
  for context in "${ACCEPTABLE_CONTEXTS[@]}"; do
    if echo "$line" | grep -qi "$context"; then
      return 0
    fi
  done
  return 1
}

check_private_ips() {
  echo "[CHECK] Private IP ranges in docs" >&2

  for pattern in "${PRIVATE_IP_PATTERNS[@]}"; do
    if [ ! -d "$DOCS_DIR" ]; then
      continue
    fi

    while IFS= read -r line; do
      if ! has_acceptable_context "$line"; then
        VIOLATIONS+=("Private IP found without context: $line")
        RISK_LEVEL="SOURCE_RISK"
      fi
    done < <(grep -rn "$pattern" "$DOCS_DIR" 2>/dev/null || true)
  done
}

check_internal_hostnames() {
  echo "[CHECK] Internal hostnames in docs" >&2

  for pattern in "${INTERNAL_HOSTNAME_PATTERNS[@]}"; do
    if [ ! -d "$DOCS_DIR" ]; then
      continue
    fi

    while IFS= read -r line; do
      # Skip localhost in test/example documentation
      if [[ "$pattern" == "localhost" ]]; then
        if ! has_acceptable_context "$line"; then
          VIOLATIONS+=("Internal hostname found without context: $line")
          RISK_LEVEL="SOURCE_RISK"
        fi
      else
        # .internal and .local should never appear in public docs
        VIOLATIONS+=("Internal hostname in public docs: $line")
        RISK_LEVEL="SOURCE_RISK"
      fi
    done < <(grep -rn "$pattern" "$DOCS_DIR" 2>/dev/null || true)
  done
}

check_environment_secrets() {
  echo "[CHECK] No secret patterns in public docs" >&2

  # Check for API keys, tokens, passwords in documentation
  local secret_patterns=("api.key" "api.secret" "password=" "token=" "credential")

  for pattern in "${secret_patterns[@]}"; do
    if [ ! -d "$DOCS_DIR" ]; then
      continue
    fi

    while IFS= read -r line; do
      if ! has_acceptable_context "$line"; then
        VIOLATIONS+=("Secret pattern in public docs: $line")
        RISK_LEVEL="SOURCE_RISK"
      fi
    done < <(grep -rni "$pattern" "$DOCS_DIR" 2>/dev/null || true)
  done
}

##############################################################################
# Main Check
##############################################################################

main() {
  echo "[CHECK] Public IP Boundary Validation" >&2

  # Only check if docs directory exists
  if [ ! -d "$DOCS_DIR" ]; then
    echo "Docs directory not found (acceptable for structure-only crates)" >&2
    return 0
  fi

  check_private_ips
  check_internal_hostnames
  check_environment_secrets

  echo "Risk Level: ${RISK_LEVEL}" >&2

  case "$RISK_LEVEL" in
    "ABSENT")
      echo "✓ Public documentation boundary clean" >&2
      return 0
      ;;
    "SOURCE_RISK")
      echo "✗ Private IP ranges or internal identifiers found in public docs" >&2
      for violation in "${VIOLATIONS[@]}"; do
        echo "  - $violation" >&2
      done
      return 1
      ;;
  esac
}

main "$@"
