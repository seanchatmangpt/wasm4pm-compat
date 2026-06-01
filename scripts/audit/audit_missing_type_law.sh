#!/usr/bin/env bash
# audit_missing_type_law.sh — check docs/PAPER_COVERAGE_LEDGER.md for
# MISSING_TYPE_LAW entries with a non-zero count. Exit 1 if any active gaps
# remain. (Gate 2 — complements audit_paper_law_ledger.sh.)
set -euo pipefail
cd "$(dirname "$0")/../.."

LEDGER="docs/PAPER_COVERAGE_LEDGER.md"

if [[ ! -f "$LEDGER" ]]; then
  echo "  FAIL  $LEDGER not found"
  exit 1
fi

# The summary row looks like: | `MISSING_TYPE_LAW` | <count> | ...
# The legend/definition row does not have a numeric second column.
count_line=$(grep "^| \`MISSING_TYPE_LAW\` |" "$LEDGER" | grep -v "Canon family present" || true)

if [[ -z "$count_line" ]]; then
  echo "  FAIL  MISSING_TYPE_LAW summary row not found in $LEDGER"
  exit 1
fi

count=$(echo "$count_line" | awk -F'|' '{print $3}' | tr -d ' ')

if [[ "$count" != "0" ]]; then
  echo "  FAIL  MISSING_TYPE_LAW count is ${count} (expected 0) in $LEDGER"
  exit 1
fi

echo "  PASS  MISSING_TYPE_LAW count = 0 in $LEDGER"
exit 0
