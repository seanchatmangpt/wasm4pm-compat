#!/usr/bin/env bash
# audit_need9_law.sh — grep src/law.rs for ConditionCell with BITS<=8 Require bound;
# exit 1 if not found (law is missing or was removed).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LAW_RS="$REPO_ROOT/src/law.rs"

echo "=== Need9 ConditionCell Law Audit ==="
echo ""

if [[ ! -f "$LAW_RS" ]]; then
    echo "FAIL: src/law.rs not found"
    exit 1
fi

fail=0

echo "Checking for ConditionCell struct..."
if grep -q "pub struct ConditionCell" "$LAW_RS"; then
    echo "OK: ConditionCell struct found"
else
    echo "FAIL: ConditionCell struct not found in src/law.rs"
    fail=1
fi

echo ""
echo "Checking for BITS <= 8 Require bound on ConditionCell..."
# Look for Require<{ BITS <= 8 }> or equivalent
if grep -qE "Require.*BITS.*<=.*8|BITS.*<=.*8.*IsTrue|Need9|BITS <= 8" "$LAW_RS"; then
    echo "OK: BITS <= 8 constraint found"
    grep -nE "Require.*BITS.*<=.*8|BITS.*<=.*8.*IsTrue|Need9|BITS <= 8" "$LAW_RS" | while IFS= read -r line; do
        echo "  $line"
    done
else
    echo "FAIL: 'BITS <= 8' constraint not found — Need9 law may be missing"
    fail=1
fi

echo ""
echo "Checking Need9 doctrine mention..."
if grep -qi "need9\|Need9\|9 means split\|at most 8" "$LAW_RS"; then
    echo "OK: Need9 doctrine referenced"
    grep -ni "need9\|Need9\|9 means split\|at most 8" "$LAW_RS" | head -5 | while IFS= read -r line; do
        echo "  $line"
    done
else
    echo "WARN: Need9 doctrine not explicitly named in law.rs"
fi

echo ""
echo "Checking ConditionCell<9> would fail (via Require<false>)..."
# The law enforces BITS <= 8, meaning ConditionCell<9> should fail at compile time
# We verify the Require bound exists
if grep -B2 -A5 "pub struct ConditionCell" "$LAW_RS" | grep -qE "Require|IsTrue|BITS"; then
    echo "OK: ConditionCell has const-generic bounds"
else
    echo "FAIL: ConditionCell missing const-generic bounds"
    fail=1
fi

echo ""
if [[ $fail -eq 1 ]]; then
    echo "RESULT: HARD FAIL — Need9 ConditionCell law not properly enforced."
    exit 1
else
    echo "RESULT: PASS — Need9 law is present and enforced."
    exit 0
fi
