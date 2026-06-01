#!/usr/bin/env bash
# audit_metric_bounds.sh — grep src/law.rs for Between01 with NUM<=DEN and DEN>0;
# exit 1 if either bound is missing.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LAW_RS="$REPO_ROOT/src/law.rs"

echo "=== Metric Const Law Bounds Audit ==="
echo ""

if [[ ! -f "$LAW_RS" ]]; then
    echo "FAIL: src/law.rs not found"
    exit 1
fi

fail=0

echo "Checking for Between01 struct definition..."
if grep -q "pub struct Between01" "$LAW_RS"; then
    echo "OK: Between01 struct found"
else
    echo "FAIL: Between01 struct not found in src/law.rs"
    fail=1
fi

echo ""
echo "Checking for DEN > 0 bound..."
if grep -qE "DEN[[:space:]]*>[[:space:]]*0|DEN > 0" "$LAW_RS"; then
    echo "OK: DEN > 0 bound found"
    grep -nE "DEN[[:space:]]*>[[:space:]]*0|DEN > 0" "$LAW_RS" | head -3 | while IFS= read -r line; do
        echo "  $line"
    done
else
    echo "FAIL: DEN > 0 bound not found — zero-denominator protection missing"
    fail=1
fi

echo ""
echo "Checking for NUM <= DEN bound..."
if grep -qE "NUM[[:space:]]*<=[[:space:]]*DEN|NUM <= DEN" "$LAW_RS"; then
    echo "OK: NUM <= DEN bound found"
    grep -nE "NUM[[:space:]]*<=[[:space:]]*DEN|NUM <= DEN" "$LAW_RS" | head -3 | while IFS= read -r line; do
        echo "  $line"
    done
else
    echo "FAIL: NUM <= DEN bound not found — out-of-range metric protection missing"
    fail=1
fi

echo ""
echo "Checking const-generic parameter names on Between01..."
if grep -qE "const NUM.*u64|const DEN.*u64" "$LAW_RS"; then
    echo "OK: Between01 uses u64 const-generic parameters"
elif grep -qE "const NUM|const DEN" "$LAW_RS"; then
    echo "OK: Between01 uses const-generic parameters"
else
    echo "WARN: Could not verify Between01 parameter types"
fi

echo ""
echo "Showing Between01 definition context..."
grep -n -A10 "pub struct Between01" "$LAW_RS" | head -15 | while IFS= read -r line; do
    echo "  $line"
done

echo ""
if [[ $fail -eq 1 ]]; then
    echo "RESULT: HARD FAIL — Between01 metric bounds are incomplete."
    exit 1
else
    echo "RESULT: PASS — Between01 has both required bounds (NUM <= DEN and DEN > 0)."
    exit 0
fi
