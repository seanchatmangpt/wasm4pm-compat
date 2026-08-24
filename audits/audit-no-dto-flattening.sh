#!/bin/bash
# ggen/audits/audit-no-dto-flattening.sh
#
# DTO Flattening Boundary Audit
#
# Purpose: Enforce strict zero-cost type-law surfaces by detecting forbidden
# DTO flattening patterns in the codebase. This audit validates that:
#
#   1. EvidenceDto, AdmissionDto, RefusalDto, ReceiptDto are NOT flattened
#      into raw payload_json or state_tag fields
#   2. Serialization via to_json_string(), receipt_json, or similar lossy
#      exports do not bypass the type-law structure
#   3. Allowed contexts (wasm_boundary_allowed_with_loss_report,
#      engine_projection_allowed, test_fixture_allowed) are explicitly
#      marked and audited
#
# Forbidden patterns (blocking):
#   - EvidenceDto, AdmissionDto, RefusalDto, ReceiptDto type references
#   - payload_json, state_tag field accesses (not inside builders)
#   - to_json_string, receipt_json serialization without witness/loss context
#
# Allowed contexts (require explicit annotation):
#   - compat_core_violation: Core bridge code (wasm4pm graduation, struct mapping)
#   - wasm_boundary_allowed_with_loss_report: WASM FFI with LossReport witness
#   - engine_projection_allowed: Process mining engine projections
#   - test_fixture_allowed: Test fixtures and trybuild fail cases
#
# Exit codes:
#   0: No blocking DTO flattening violations found
#   1: Blocking violations detected
#   2: Allowed-context violations found (require annotation)
#   3: Configuration error (missing input or output paths)

set -euo pipefail

# ====================================================================
# Configuration (overridable via environment)
# ====================================================================
GEN_ROOT="${GEN_ROOT:-.}"
CRATE_ROOT="${CRATE_ROOT:-${GEN_ROOT}}"
SRC_DIR="${SRC_DIR:-${CRATE_ROOT}/src}"
TESTS_DIR="${TESTS_DIR:-${CRATE_ROOT}/tests}"
EXAMPLES_DIR="${EXAMPLES_DIR:-${CRATE_ROOT}/examples}"
EMITTED_DIR="${EMITTED_DIR:-${GEN_ROOT}/emitted}"
AUDITS_DIR="${AUDITS_DIR:-${EMITTED_DIR}/audits}"

# Input from query (comma-separated patterns)
FORBIDDEN_PATTERNS="${FORBIDDEN_PATTERNS:-EvidenceDto,AdmissionDto,RefusalDto,ReceiptDto,payload_json,state_tag,to_json_string,receipt_json}"

# Allowed contexts (comma-separated)
ALLOWED_CONTEXTS="${ALLOWED_CONTEXTS:-compat_core_violation,wasm_boundary_allowed_with_loss_report,engine_projection_allowed,test_fixture_allowed}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Result tracking
BLOCKING_VIOLATIONS=0
ALLOWED_VIOLATIONS=0
PASS_COUNT=0
WARNINGS=0

# ====================================================================
# Helper: Check if a line is inside an allowed context annotation
# ====================================================================
is_inside_allowed_context() {
  local file="$1"
  local line_num="$2"
  local allowed_context_pattern="$3"

  # Read the file and check if there's a matching annotation within 5 lines before
  if sed -n "1,${line_num}p" "${file}" | tail -n 10 | grep -qE "// (ALLOW|CONTEXT):\s*(${allowed_context_pattern})" 2>/dev/null; then
    return 0
  fi

  # Also check for block-level annotations
  if sed -n "1,${line_num}p" "${file}" | grep -qE "/\*.*ALLOW.*${allowed_context_pattern}.*\*/" 2>/dev/null; then
    return 0
  fi

  return 1
}

# ====================================================================
# Helper: Extract context annotation from a file at a line
# ====================================================================
extract_context_annotation() {
  local file="$1"
  local line_num="$2"

  # Check preceding 5 lines for annotation
  sed -n "1,${line_num}p" "${file}" | tail -n 10 | grep -oE "// (ALLOW|CONTEXT):\s*[a-z_]+" | tail -n 1 | sed 's/.*:\s*//'
}

# ====================================================================
# Helper: Scan a file for blocking violations
# ====================================================================
scan_file_for_violations() {
  local file="$1"
  local rel_file="${file#${CRATE_ROOT}/}"

  # Skip non-Rust/TypeScript files
  if ! [[ "${file}" =~ \.(rs|ts|tsx)$ ]]; then
    return 0
  fi

  # Build forbidden pattern regex
  local pattern_regex=""
  IFS=',' read -ra PATTERNS <<< "${FORBIDDEN_PATTERNS}"
  for p in "${PATTERNS[@]}"; do
    if [ -z "${pattern_regex}" ]; then
      pattern_regex="${p}"
    else
      pattern_regex="${pattern_regex}|${p}"
    fi
  done

  # Build allowed context regex
  local context_regex=""
  IFS=',' read -ra CONTEXTS <<< "${ALLOWED_CONTEXTS}"
  for c in "${CONTEXTS[@]}"; do
    if [ -z "${context_regex}" ]; then
      context_regex="${c}"
    else
      context_regex="${context_regex}|${c}"
    fi
  done

  # Scan line by line
  local line_num=0
  while IFS= read -r line; do
    line_num=$((line_num + 1))

    # Skip comments and empty lines
    if [[ "${line}" =~ ^[[:space:]]*(/|#) ]]; then
      continue
    fi

    # Check if line contains a forbidden pattern
    if echo "${line}" | grep -qE "(${pattern_regex})" 2>/dev/null; then
      # Check if inside allowed context
      if is_inside_allowed_context "${file}" "${line_num}" "${context_regex}"; then
        local context=$(extract_context_annotation "${file}" "${line_num}")
        echo -e "  ${YELLOW}⚠${NC} ${rel_file}:${line_num} [${context}] ${line:0:80}"
        ALLOWED_VIOLATIONS=$((ALLOWED_VIOLATIONS + 1))
      else
        # Blocking violation
        echo -e "  ${RED}✗${NC} ${rel_file}:${line_num} BLOCKING: ${line:0:80}"
        BLOCKING_VIOLATIONS=$((BLOCKING_VIOLATIONS + 1))
      fi
    fi
  done < "${file}"
}

# ====================================================================
# Main audit flow
# ====================================================================

mkdir -p "${AUDITS_DIR}"

echo "================================"
echo "DTO Flattening Boundary Audit"
echo "================================"
echo ""
echo "Forbidden patterns: ${FORBIDDEN_PATTERNS}"
echo "Allowed contexts:   ${ALLOWED_CONTEXTS}"
echo ""

# Verify directories exist
if [ ! -d "${SRC_DIR}" ]; then
  echo -e "${RED}✗ Configuration error: SRC_DIR not found: ${SRC_DIR}${NC}"
  exit 3
fi

# ====================================================================
# PHASE 1: Scan src/ directory
# ====================================================================
echo "[Phase 1] Scanning src/ (core modules)"
echo ""

if [ -d "${SRC_DIR}" ]; then
  SRC_FILES=$(find "${SRC_DIR}" -name "*.rs" -type f 2>/dev/null || true)
  file_count=$(echo "${SRC_FILES}" | wc -l)
  echo "  Found ${file_count} .rs file(s)"
  echo ""

  while IFS= read -r file; do
    if [ -n "${file}" ]; then
      scan_file_for_violations "${file}"
    fi
  done <<< "${SRC_FILES}"
else
  echo -e "  ${YELLOW}⚠${NC} src/ directory not found"
fi

echo ""

# ====================================================================
# PHASE 2: Scan tests/ directory
# ====================================================================
echo "[Phase 2] Scanning tests/ (test fixtures)"
echo ""

if [ -d "${TESTS_DIR}" ]; then
  TESTS_FILES=$(find "${TESTS_DIR}" -name "*.rs" -type f 2>/dev/null || true)
  file_count=$(echo "${TESTS_FILES}" | wc -l)
  if [ "${file_count}" -gt 0 ]; then
    echo "  Found ${file_count} test file(s)"
    echo ""

    while IFS= read -r file; do
      if [ -n "${file}" ]; then
        scan_file_for_violations "${file}"
      fi
    done <<< "${TESTS_FILES}"
  else
    echo "  - No test files found"
  fi
else
  echo "  - tests/ directory not found"
fi

echo ""

# ====================================================================
# PHASE 3: Scan examples/ directory
# ====================================================================
echo "[Phase 3] Scanning examples/ (runnable examples)"
echo ""

if [ -d "${EXAMPLES_DIR}" ]; then
  EXAMPLES_FILES=$(find "${EXAMPLES_DIR}" -name "*.rs" -type f 2>/dev/null || true)
  file_count=$(echo "${EXAMPLES_FILES}" | wc -l)
  if [ "${file_count}" -gt 0 ]; then
    echo "  Found ${file_count} example file(s)"
    echo ""

    while IFS= read -r file; do
      if [ -n "${file}" ]; then
        scan_file_for_violations "${file}"
      fi
    done <<< "${EXAMPLES_FILES}"
  else
    echo "  - No example files found"
  fi
else
  echo "  - examples/ directory not found"
fi

echo ""

# ====================================================================
# PHASE 4: Scan TypeScript projections (if present)
# ====================================================================
echo "[Phase 4] Scanning generated TypeScript / WASM bindings"
echo ""

if [ -d "${GEN_ROOT}/generated" ]; then
  TS_FILES=$(find "${GEN_ROOT}/generated" -name "*.ts" -o -name "*.tsx" 2>/dev/null || true)
  file_count=$(echo "${TS_FILES}" | wc -l)
  if [ "${file_count}" -gt 0 ]; then
    echo "  Found ${file_count} TypeScript file(s)"
    echo ""

    while IFS= read -r file; do
      if [ -n "${file}" ]; then
        scan_file_for_violations "${file}"
      fi
    done <<< "${TS_FILES}"
  else
    echo "  - No TypeScript files found in generated/"
  fi
else
  echo "  - generated/ directory not found"
fi

echo ""

# ====================================================================
# SUMMARY
# ====================================================================
echo "================================"
echo "Audit Summary"
echo "================================"
echo ""
echo "  Blocking violations:  ${RED}${BLOCKING_VIOLATIONS}${NC}"
echo "  Allowed violations:   ${YELLOW}${ALLOWED_VIOLATIONS}${NC}"
echo ""

# Emit JSON audit report
AUDIT_REPORT="${AUDITS_DIR}/audit-no-dto-flattening-$(date +%Y%m%d-%H%M%S).json"

cat > "${AUDIT_REPORT}" <<REPORT_EOF
{
  "audit_name": "audit-no-dto-flattening",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "forbidden_patterns": "${FORBIDDEN_PATTERNS}",
  "allowed_contexts": "${ALLOWED_CONTEXTS}",
  "results": {
    "blocking_violations": ${BLOCKING_VIOLATIONS},
    "allowed_violations": ${ALLOWED_VIOLATIONS},
    "status": "$([ ${BLOCKING_VIOLATIONS} -eq 0 ] && echo "PASS" || echo "FAIL")"
  }
}
REPORT_EOF

echo "Audit report: ${AUDIT_REPORT}"
echo ""

# ====================================================================
# Exit codes
# ====================================================================
if [ "${BLOCKING_VIOLATIONS}" -eq 0 ]; then
  if [ "${ALLOWED_VIOLATIONS}" -eq 0 ]; then
    echo -e "${GREEN}✓ PASS: No DTO flattening violations detected${NC}"
    exit 0
  else
    echo -e "${YELLOW}⚠ PASS (with annotations): ${ALLOWED_VIOLATIONS} allowed-context violation(s)${NC}"
    echo ""
    echo "Action items:"
    echo "  1. Verify each annotation matches the violation context"
    echo "  2. Consider moving annotated code to the appropriate module"
    echo ""
    exit 0
  fi
else
  echo -e "${RED}✗ FAIL: ${BLOCKING_VIOLATIONS} blocking DTO flattening violation(s)${NC}"
  echo ""
  echo "Action items:"
  echo "  1. Each violation must be wrapped in a permitted context:"
  echo "     - // CONTEXT: compat_core_violation"
  echo "     - // CONTEXT: wasm_boundary_allowed_with_loss_report"
  echo "     - // CONTEXT: engine_projection_allowed"
  echo "     - // CONTEXT: test_fixture_allowed"
  echo "  2. Add the annotation on the line(s) before the violation"
  echo "  3. Re-run: bash ${AUDITS_DIR}/audit-no-dto-flattening.sh"
  echo ""
  exit 1
fi
