#!/usr/bin/env bash
# audit_pass_fail_pairs.sh — for each compile-fail fixture, check if a
# corresponding compile-pass exists. Reports unmatched pairs. Exit 0 (soft warn).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FAIL_DIR="$REPO_ROOT/tests/ui/compile_fail"
PASS_DIR="$REPO_ROOT/tests/ui/compile_pass"

unmatched=0
total=0

echo "=== Pass/Fail Pair Coverage Audit ==="
echo ""

while IFS= read -r fail_file; do
    base="$(basename "$fail_file" .rs)"
    total=$((total + 1))
    pass_file="$PASS_DIR/${base}.rs"
    if [[ ! -f "$pass_file" ]]; then
        echo "UNMATCHED: $base (no compile-pass counterpart)"
        unmatched=$((unmatched + 1))
    fi
done < <(find "$FAIL_DIR" -maxdepth 1 -name "*.rs" | sort)

echo ""
echo "Total compile-fail fixtures : $total"
echo "Unmatched (no pass pair)    : $unmatched"
echo "Matched                     : $((total - unmatched))"

if [[ $unmatched -gt 0 ]]; then
    echo ""
    echo "WARN: $unmatched compile-fail fixtures have no matching compile-pass fixture."
    echo "      This is a soft warning — exit 0."
fi

exit 0
