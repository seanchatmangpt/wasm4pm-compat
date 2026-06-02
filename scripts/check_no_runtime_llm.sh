#!/bin/bash
#
# check_no_runtime_llm.sh
# AGENT 9 — Runtime LLM Integration Detector
#
# Purpose: Detect runtime LLM integrations, API clients, and language model
# dependencies in source code and manifests.
#
# Patterns checked:
# - FORBIDDEN: openai, anthropic, claude, llm, chatcompletion, messages.create, api key (runtime)
#
# Classification:
# - SOURCE_RISK: Found in Cargo.toml dependencies or runtime .rs code
# - DOCS_ONLY: Found in documentation or comments only
# - ABSENT: Not found anywhere
#
# Exit: 0 if ABSENT or DOCS_ONLY; 1 if SOURCE_RISK
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Forbidden patterns for runtime LLM
declare -a FORBIDDEN_LLM_PATTERNS=(
  "openai"
  "anthropic"
  "claude"
  "llm"
  "chatcompletion"
  "messages.create"
)

# Result tracking
declare -A PATTERN_FINDINGS
RISK_LEVEL="ABSENT"

##############################################################################
# Helper Functions
##############################################################################

check_llm_in_dependencies() {
  local pattern="$1"
  local dep_hits=0

  # Check ONLY in Cargo.toml [dependencies] sections
  dep_hits=$(find "$PROJECT_ROOT" \
    -type f -name "Cargo.toml" \
    ! -path "*/target/*" \
    ! -path "*/.git/*" \
    -exec grep -A 50 "^\[dependencies\]" {} \; 2>/dev/null \
    | grep -i "$pattern" | wc -l)

  if [ "$dep_hits" -gt 0 ]; then
    echo "Found in dependencies: $dep_hits" >&2
    return 1
  fi
  return 0
}

check_llm_in_runtime_code() {
  local pattern="$1"
  local code_hits=0

  # Check in source code, excluding comments and test code
  code_hits=$(find "$PROJECT_ROOT" \
    -type f -name "*.rs" \
    ! -path "*/target/*" \
    ! -path "*/.git/*" \
    ! -path "*/tests/*" \
    ! -path "*/examples/*" \
    -exec grep -i "$pattern" {} \; 2>/dev/null \
    | grep -v "^[[:space:]]*//\|^[[:space:]]*/\*" \
    | grep -v "doctest\|example\|comment" | wc -l)

  if [ "$code_hits" -gt 0 ]; then
    echo "Found in runtime code: $code_hits" >&2
    return 1
  fi
  return 0
}

check_llm_pattern() {
  local pattern="$1"
  local risk_in_deps=0
  local risk_in_code=0
  local doc_hits=0

  # Check dependencies
  check_llm_in_dependencies "$pattern" || risk_in_deps=1

  # Check runtime code
  check_llm_in_runtime_code "$pattern" || risk_in_code=1

  # Check documentation (acceptable context)
  doc_hits=$(find "$PROJECT_ROOT" \
    -type f -name "*.md" \
    ! -path "*/target/*" \
    ! -path "*/.git/*" \
    -exec grep -il "$pattern" {} \; 2>/dev/null | wc -l)

  PATTERN_FINDINGS["$pattern"]="deps_risk:$risk_in_deps,code_risk:$risk_in_code,docs:$doc_hits"

  if [ "$risk_in_deps" -eq 1 ] || [ "$risk_in_code" -eq 1 ]; then
    RISK_LEVEL="SOURCE_RISK"
    return 1
  elif [ "$doc_hits" -gt 0 ]; then
    if [ "$RISK_LEVEL" != "SOURCE_RISK" ]; then
      RISK_LEVEL="DOCS_ONLY"
    fi
  fi

  return 0
}

##############################################################################
# Main Check
##############################################################################

main() {
  echo "[CHECK] No Runtime LLM Integration Detection" >&2

  for pattern in "${FORBIDDEN_LLM_PATTERNS[@]}"; do
    check_llm_pattern "$pattern" || true
  done

  echo "Risk Level: ${RISK_LEVEL}" >&2

  case "$RISK_LEVEL" in
    "ABSENT")
      echo "✓ No runtime LLM patterns detected" >&2
      return 0
      ;;
    "DOCS_ONLY")
      echo "⚠ LLM patterns found in documentation only (acceptable for examples/references)" >&2
      return 0
      ;;
    "SOURCE_RISK")
      echo "✗ Runtime LLM integrations or dependencies detected in source code" >&2
      for pattern in "${!PATTERN_FINDINGS[@]}"; do
        IFS=',' read -r deps_part code_part docs_part <<< "${PATTERN_FINDINGS[$pattern]}"
        deps_risk="${deps_part##*:}"
        code_risk="${code_part##*:}"
        if [ "$deps_risk" -eq 1 ]; then
          echo "  - Pattern '$pattern': Found in Cargo.toml dependencies" >&2
        fi
        if [ "$code_risk" -eq 1 ]; then
          echo "  - Pattern '$pattern': Found in runtime source code" >&2
        fi
      done
      return 1
      ;;
  esac
}

main "$@"
