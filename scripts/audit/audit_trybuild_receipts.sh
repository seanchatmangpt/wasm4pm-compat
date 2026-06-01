#!/usr/bin/env bash
# audit_trybuild_receipts.sh — every compile_fail .rs must have a matching .stderr receipt.
#
# For every .rs file in tests/ui/compile_fail/, checks that a .stderr file with
# the same stem exists. Lists any missing receipts and exits 1 if any are found.

REPO_ROOT="$(dirname "$0")/../.."
COMPILE_FAIL_DIR="$REPO_ROOT/tests/ui/compile_fail"
FAIL=0
MISSING=()

for rs_file in "$COMPILE_FAIL_DIR"/*.rs; do
    [ -f "$rs_file" ] || continue
    stem="${rs_file%.rs}"
    stderr_file="${stem}.stderr"
    if [ ! -f "$stderr_file" ]; then
        MISSING+=("$(basename "$rs_file") — missing $(basename "$stderr_file")")
        FAIL=1
    fi
done

if [ "$FAIL" -ne 0 ]; then
    echo "FAIL: the following compile_fail fixtures are missing .stderr receipts:" >&2
    for m in "${MISSING[@]}"; do
        echo "  $m" >&2
    done
else
    total=$(ls "$COMPILE_FAIL_DIR"/*.rs 2>/dev/null | wc -l | tr -d ' ')
    echo "PASS: all ${total} compile_fail fixtures have matching .stderr receipts."
fi

exit "$FAIL"
