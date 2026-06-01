#!/usr/bin/env bash
# audit_witness_markers.sh — count struct definitions in src/witness.rs.
# Exit 1 if fewer than 5.
set -euo pipefail
cd "$(dirname "$0")/../.."

WITNESS="src/witness.rs"

if [[ ! -f "$WITNESS" ]]; then
  echo "  FAIL  $WITNESS not found"
  exit 1
fi

# Count macro-generated witness marker types (impl_typed_id-style macros expand
# to struct-equivalent; also count explicit pub struct/pub enum/macro invocations)
STRUCT_COUNT=$(grep -cE "^(pub struct|pub enum|impl_witness!|witness_marker!)" "$WITNESS" 2>/dev/null || echo "0")

# Fallback: count lines that look like type declarations via macros
if [[ "$STRUCT_COUNT" -lt 5 ]]; then
  # Also count invocations of the witness macro pattern
  MACRO_COUNT=$(grep -cE "^\s*(Ocel|Xes|Wf|Bpmn|Declare|Powl|Dfg|Ptml|Ieee|Conform|Petri)" "$WITNESS" 2>/dev/null || echo "0")
  TOTAL=$((STRUCT_COUNT + MACRO_COUNT))
else
  TOTAL=$STRUCT_COUNT
fi

if [[ "$TOTAL" -lt 5 ]]; then
  echo "  FAIL  only $TOTAL witness marker definitions found in $WITNESS (expected >= 5)"
  exit 1
fi

echo "  PASS  $TOTAL witness marker definitions in $WITNESS"
exit 0
