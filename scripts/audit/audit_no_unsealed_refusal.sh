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

HITS=$(grep -cE "(InvalidInput|GenericError)" "$ADMISSION" 2>/dev/null || echo "0")

if [[ "$HITS" -gt 0 ]]; then
  echo "  FAIL  unsealed refusal variant (InvalidInput or GenericError) found in $ADMISSION"
  grep -nE "(InvalidInput|GenericError)" "$ADMISSION" | sed 's/^/         /'
  exit 1
fi

echo "  PASS  no unsealed refusal variants in $ADMISSION"
exit 0
