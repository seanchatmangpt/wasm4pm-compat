#!/usr/bin/env bash
# audit_no_unsealed_refusal.sh — grep src/admission.rs for InvalidInput or
# GenericError variants. Exit 1 if found.
set -euo pipefail
cd "$(dirname "$0")/../.."

ADMISSION="src/admission.rs"

if [[ ! -f "$ADMISSION" ]]; then
  echo "  FAIL  $ADMISSION not found"
  exit 1
fi

# Only match non-comment, non-doc-comment code lines.
# grep -n output is "lineno:content"; strip to content, then filter out comment lines.
# Disable pipefail here: grep -v exits 1 when all lines are filtered (expected for clean state).
HITS=$(set +o pipefail; grep -nE "(InvalidInput|GenericError)" "$ADMISSION" 2>/dev/null \
  | sed 's/^[0-9]*://' \
  | grep -vE '^[[:space:]]*//' \
  | wc -l | tr -d ' ')

if [[ "$HITS" -gt 0 ]]; then
  echo "  FAIL  unsealed refusal variant (InvalidInput or GenericError) found in non-comment lines of $ADMISSION"
  grep -nE "(InvalidInput|GenericError)" "$ADMISSION" 2>/dev/null \
    | sed 's/^[0-9]*://' \
    | grep -vE '^[[:space:]]*//' \
    | sed 's/^/         /'
  exit 1
fi

echo "  PASS  no unsealed refusal variants in $ADMISSION"
exit 0
