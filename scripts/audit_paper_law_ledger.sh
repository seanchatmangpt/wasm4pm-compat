#!/usr/bin/env bash
# audit_paper_law_ledger.sh — fail if any outstanding MISSING_TYPE_LAW entries remain.
#
# The ledger (docs/PAPER_COVERAGE_LEDGER.md) contains MISSING_TYPE_LAW in two
# legitimate locations:
#   1. The classification key table (legend/definition row — expected, not a defect)
#   2. The summary count row — must show count 0
#
# This script checks that the summary count for MISSING_TYPE_LAW is 0.
# Any non-zero count means active type-law gaps remain.

REPO_ROOT="$(dirname "$0")/.."
LEDGER="$REPO_ROOT/docs/PAPER_COVERAGE_LEDGER.md"

if [ ! -f "$LEDGER" ]; then
    echo "FAIL: PAPER_COVERAGE_LEDGER.md not found at $LEDGER" >&2
    exit 1
fi

# Extract the summary row: | MISSING_TYPE_LAW | <count> | ...
# The count is in the second pipe-delimited column.
count_line=$(grep "^| \`MISSING_TYPE_LAW\` |" "$LEDGER" | grep -v "Canon family present" || true)

if [ -z "$count_line" ]; then
    echo "FAIL: could not find MISSING_TYPE_LAW summary row in $LEDGER" >&2
    exit 1
fi

# Extract the count value (second column, strip whitespace).
count=$(echo "$count_line" | awk -F'|' '{print $3}' | tr -d ' ')

if [ "$count" != "0" ]; then
    echo "FAIL: MISSING_TYPE_LAW count is ${count} (expected 0) in $LEDGER" >&2
    echo "  Row: $count_line" >&2
    exit 1
fi

echo "PASS: MISSING_TYPE_LAW count = 0 in PAPER_COVERAGE_LEDGER.md."
exit 0
