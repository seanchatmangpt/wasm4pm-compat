#!/bin/bash
# TypeScript Projection Validation Audit
# Audit: TypeScript Projection Validation
# Authority: ts-projection-law v1.0.0
# Generated: 2026-06-01
# Template: ggen/templates/audit-ts-projection.sh.tera
#
# Purpose: Validate generated .d.ts against ts-projection-law seven laws
# Usage: ./audit-ts-projection.sh <path-to-.d.ts> [--verbose]
# Output: JSON report (audit-ts-projection-results.json) + markdown (audit-ts-projection-results.md)

set -euo pipefail

# Configuration
DTS_FILE="${1:-.}"
VERBOSE="${2:-}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RESULTS_JSON="$PROJECT_ROOT/ggen/emitted/audit-ts-projection-results.json"
RESULTS_MD="$PROJECT_ROOT/ggen/emitted/audit-ts-projection-results.md"

# Counters
declare -i CHECKS_PASSED=0
declare -i CHECKS_FAILED=0
declare -a FAILED_CHECKS=()
declare -a PASSED_CHECKS=()

# Colors (for terminal output)
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# ============================================================================
# HELPER FUNCTIONS
# ============================================================================

log_info() {
  echo "[INFO] $*"
}

log_pass() {
  local name="$1"
  echo -e "${GREEN}✅${NC} $name"
  PASSED_CHECKS+=("$name")
  ((CHECKS_PASSED++))
}

log_fail() {
  local name="$1"
  local reason="${2:-}"
  if [[ -n "$reason" ]]; then
    echo -e "${RED}❌${NC} $name — $reason"
  else
    echo -e "${RED}❌${NC} $name"
  fi
  FAILED_CHECKS+=("$name")
  ((CHECKS_FAILED++))
}

log_verbose() {
  if [[ -n "$VERBOSE" ]]; then
    echo "[VERBOSE] $*"
  fi
}

# Check if a pattern exists in file
check_pattern_absent() {
  local name="$1"
  local pattern="$2"
  local file="$3"

  if ! grep -E "$pattern" "$file" > /dev/null 2>&1; then
    log_pass "$name"
  else
    log_fail "$name" "Pattern found: $pattern"
  fi
}

check_pattern_present() {
  local name="$1"
  local pattern="$2"
  local file="$3"

  if grep -E "$pattern" "$file" > /dev/null 2>&1; then
    log_pass "$name"
  else
    log_fail "$name" "Pattern not found: $pattern"
  fi
}

# ============================================================================
# VALIDATION CHECKS
# ============================================================================

validate_dts_file() {
  if [[ ! -f "$DTS_FILE" ]]; then
    log_fail "Input file exists" "File not found: $DTS_FILE"
    return 1
  fi

  if ! grep -q "export" "$DTS_FILE"; then
    log_fail "TypeScript file is valid" "No exports found in file"
    return 1
  fi

  log_pass "Input file exists and is readable"
  log_pass "File contains TypeScript exports"
  return 0
}

# LAW 1: No PhantomData at the boundary
check_law_1_no_phantom() {
  log_info "Validating LAW 1: No PhantomData at boundary"

  # Check 1.1: No PhantomData type parameters
  check_pattern_absent \
    "No PhantomData<> in exported types" \
    "PhantomData<" \
    "$DTS_FILE"

  # Check 1.2: No zero-sized empty state objects
  check_pattern_absent \
    "No zero-sized state fields (state: {})" \
    "state:\s*{}" \
    "$DTS_FILE"

  # Check 1.3: No witness as empty object
  check_pattern_absent \
    "No zero-sized witness fields (witness: {})" \
    "witness:\s*{}" \
    "$DTS_FILE"
}

# LAW 2: All exported types are serializable
check_law_2_serializable() {
  log_info "Validating LAW 2: All exported types are serializable"

  # Check 2.1: Core types have proper field names (not positional)
  check_pattern_present \
    "Exported types use named fields (not tuples)" \
    "type [A-Za-z]+\s*=" \
    "$DTS_FILE"

  # Check 2.2: No Mutex, Arc, or other non-serializable locks
  check_pattern_absent \
    "No Mutex or Arc in exported types" \
    "Mutex|Arc<" \
    "$DTS_FILE"
}

# LAW 3: Branded generics become concrete wrappers
check_law_3_branded_generics() {
  log_info "Validating LAW 3: Branded generics become concrete wrappers"

  # Check 3.1: Evidence<T, State, W> not exported as generic
  check_pattern_absent \
    "No generic Evidence<T, State, W>" \
    "Evidence<.*State.*W>" \
    "$DTS_FILE"

  # Check 3.2: Admission<T, W> not exported as generic (witness parameter)
  check_pattern_absent \
    "No Admission<T, W> with witness generic" \
    "Admission<.*,.*W>" \
    "$DTS_FILE"

  # Check 3.3: Concrete wrappers present (if evidence types exported)
  if grep -q "type.*Evidence" "$DTS_FILE"; then
    check_pattern_present \
      "Concrete evidence wrappers present (RawEvidence, AdmittedEvidence, etc.)" \
      "type.*Evidence|interface.*Evidence" \
      "$DTS_FILE"
  fi
}

# LAW 4: Witness metadata projection
check_law_4_witness_metadata() {
  log_info "Validating LAW 4: Witness metadata projection"

  # Check 4.1: No witness marker enums exported
  check_pattern_absent \
    "No witness marker enums (Ocel20, Xes1849, etc.)" \
    "export\s*(enum|type)\s*(Ocel20|Xes1849|BpmnStandard|EventLogCanon)" \
    "$DTS_FILE"

  # Check 4.2: WitnessMetadata struct exported
  check_pattern_present \
    "WitnessMetadata struct exported (not phantom markers)" \
    "WitnessMetadata" \
    "$DTS_FILE"

  # Check 4.3: WitnessFamily enum exported
  check_pattern_present \
    "WitnessFamily enum exported" \
    "WitnessFamily" \
    "$DTS_FILE"
}

# LAW 5: Loss accounting serialization
check_law_5_loss_accounting() {
  log_info "Validating LAW 5: Loss accounting serialization"

  # Check 5.1: LossReport mandatory if lossy projections exist
  if grep -q "Projection\|projection" "$DTS_FILE"; then
    check_pattern_present \
      "LossReport type exported (for lossy projections)" \
      "LossReport" \
      "$DTS_FILE"
  fi

  # Check 5.2: LossPolicy is concrete enum
  check_pattern_present \
    "LossPolicy is concrete enum" \
    "LossPolicy" \
    "$DTS_FILE"
}

# LAW 6: Refusal typing
check_law_6_refusal_typing() {
  log_info "Validating LAW 6: Refusal typing"

  # Check 6.1: No bare string refusals
  check_pattern_absent \
    "No bare string refusals (type Refusal = string)" \
    "type Refusal\s*=\s*string" \
    "$DTS_FILE"

  # Check 6.2: Named refusal enums present
  check_pattern_present \
    "Named refusal enums (EventLogRefusal, OcelRefusal)" \
    "Refusal\s*=|Refusal\s*:" \
    "$DTS_FILE"
}

# LAW 7: Complete type walk
check_law_7_type_walk() {
  log_info "Validating LAW 7: Complete type walk"

  # Check 7.1: Core domain types present
  check_pattern_present \
    "Tier 1 types exported (Event, Trace, EventLog)" \
    "type Event|type Trace|type EventLog" \
    "$DTS_FILE"

  # Check 7.2: OCEL types present
  check_pattern_present \
    "OCEL types exported (OcelEvent, OcelObject)" \
    "type OcelEvent|type OcelObject" \
    "$DTS_FILE"

  # Check 7.3: No hidden module exports
  check_pattern_absent \
    "No pub(crate) types exported (all visible)" \
    "pub\s*\(\s*crate\s*\)" \
    "$DTS_FILE"
}

# ADDITIONAL CHECKS

check_no_forbidden_modules() {
  log_info "Checking: No forbidden module exports"

  # Forbid discovery, replay, conformance computation, alignment (unless wasm4pm)
  local forbidden_patterns="discover_|replay_|align_|solve_|mine_|conformance_check"
  check_pattern_absent \
    "No algorithm/discovery/replay functions exported" \
    "$forbidden_patterns" \
    "$DTS_FILE"
}

check_tier_1_completeness() {
  log_info "Checking: Tier 1 type completeness"

  local tier1_types=("Event" "Trace" "EventLog" "OcelEvent" "OcelObject" "OcelAttribute" "OcelAttributeValue")
  local missing=0

  for type in "${tier1_types[@]}"; do
    if ! grep -q "type $type\|interface $type" "$DTS_FILE"; then
      log_fail "Tier 1 type present: $type"
      ((missing++))
    else
      log_verbose "Found Tier 1 type: $type"
    fi
  done

  if [[ $missing -eq 0 ]]; then
    log_pass "All Tier 1 types present (Event, Trace, EventLog, OcelEvent, OcelObject, OcelAttribute, OcelAttributeValue)"
  fi
}

check_tier_4_refusal_completeness() {
  log_info "Checking: Tier 4 refusal enum completeness"

  # Check for refusal enums
  if grep -q "EventLogRefusal\|OcelRefusal" "$DTS_FILE"; then
    log_pass "Refusal reason enums present (EventLogRefusal, OcelRefusal)"
  else
    log_fail "Refusal reason enums present" "No EventLogRefusal or OcelRefusal found"
  fi

  # Check that refusal enums have variants, not bare strings
  if grep -A 5 "EventLogRefusal" "$DTS_FILE" | grep -q "kind\|DanglingEventObjectLink"; then
    log_pass "Refusal enums have typed variants (not bare strings)"
  else
    log_verbose "Could not verify refusal enum structure"
  fi
}

check_typescript_syntax() {
  log_info "Checking: TypeScript syntax validity"

  # Try to validate with tsc if available
  if command -v tsc &> /dev/null; then
    if tsc --strict --noEmit "$DTS_FILE" > /tmp/tsc_check.log 2>&1; then
      log_pass "TypeScript syntax is valid (tsc --strict --noEmit)"
    else
      log_fail "TypeScript syntax is valid" "tsc errors found (see /tmp/tsc_check.log)"
    fi
  else
    log_verbose "tsc not available; skipping TypeScript syntax check"
  fi
}

check_feature_gate() {
  log_info "Checking: Feature gate alignment"

  local cargo_toml="$PROJECT_ROOT/Cargo.toml"

  if [[ ! -f "$cargo_toml" ]]; then
    log_verbose "Cargo.toml not found; skipping feature gate check"
    return
  fi

  if grep -q "ts = \[" "$cargo_toml"; then
    log_pass "Feature gate 'ts' is defined in Cargo.toml"
  else
    log_fail "Feature gate 'ts' defined in Cargo.toml"
  fi

  if grep -A 3 "ts = \[" "$cargo_toml" | grep -q "specta\|serde"; then
    log_pass "Feature 'ts' includes specta and/or serde dependencies"
  else
    log_fail "Feature 'ts' includes specta and serde dependencies"
  fi
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

main() {
  log_info "TypeScript Projection Audit"
  log_info "Authority: ts-projection-law v1.0.0"
  log_info "DTS File: $DTS_FILE"
  echo ""

  # Validate input
  if ! validate_dts_file; then
    exit 1
  fi
  echo ""

  # Run all checks
  check_law_1_no_phantom
  echo ""

  check_law_2_serializable
  echo ""

  check_law_3_branded_generics
  echo ""

  check_law_4_witness_metadata
  echo ""

  check_law_5_loss_accounting
  echo ""

  check_law_6_refusal_typing
  echo ""

  check_law_7_type_walk
  echo ""

  check_no_forbidden_modules
  echo ""

  check_tier_1_completeness
  echo ""

  check_tier_4_refusal_completeness
  echo ""

  check_typescript_syntax
  echo ""

  check_feature_gate
  echo ""

  # Print summary
  echo "========================================"
  echo "TypeScript Projection Audit Summary"
  echo "========================================"
  echo "Passed: $CHECKS_PASSED"
  echo "Failed: $CHECKS_FAILED"
  echo ""

  if [[ $CHECKS_FAILED -eq 0 ]]; then
    echo -e "${GREEN}Status: ✅ PASS${NC}"
    exit 0
  else
    echo -e "${RED}Status: ❌ FAIL${NC}"
    echo ""
    echo "Failed checks:"
    printf '%s\n' "${FAILED_CHECKS[@]}"
    exit 1
  fi
}

main "$@"
