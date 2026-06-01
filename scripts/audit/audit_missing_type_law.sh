#!/usr/bin/env bash
# audit_missing_type_law.sh — grep docs/PAPER_COVERAGE_LEDGER.md for MISSING_TYPE_LAW.
# Exit 1 if any found. (Gate 2.)
set -euo pipefail
cd "$(dirname "$0")/../.."

LEDGER="docs/PAPER_COVERAGE_LEDGER.md"

if [[ ! -f "$LEDGER" ]]; then
  echo "  FAIL  $LEDGER not found"
  exit 1
fi

COUNT=$(grep -c "MISSING_TYPE_LAW" "$LEDGER" 2>/dev/null || echo "0")

if [[ "$COUNT" -gt 0 ]]; then
  echo "  FAIL  $COUNT MISSING_TYPE_LAW entries found in $LEDGER"
  grep -n "MISSING_TYPE_LAW" "$LEDGER" | sed 's/^/         /'
  exit 1
fi

echo "  PASS  no MISSING_TYPE_LAW entries in $LEDGER"
exit 0
