#!/usr/bin/env bash
# audit_stderr_quality.sh — for each .stderr, check it contains "error[E" (real
# Rust error code); warn if empty or missing error code. Exit 0 (soft warn).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FAIL_DIR="$REPO_ROOT/tests/ui/compile_fail"

echo "=== Stderr Receipt Quality Audit ==="
echo ""

total=0
empty=0
no_code=0
good=0

while IFS= read -r stderr_file; do
    base="$(basename "$stderr_file")"
    total=$((total + 1))

    size=$(wc -c < "$stderr_file" | xargs)
    if [[ "$size" -eq 0 ]]; then
        echo "EMPTY: $base (0 bytes — no receipt)"
        empty=$((empty + 1))
        continue
    fi

    if grep -q "error\[E" "$stderr_file"; then
        good=$((good + 1))
    elif grep -q "^error" "$stderr_file"; then
        echo "WARN: $base has 'error' but no error code (e.g. error[E0308])"
        no_code=$((no_code + 1))
    else
        echo "WARN: $base has no 'error' line at all — may be empty or malformed"
        no_code=$((no_code + 1))
    fi

done < <(find "$FAIL_DIR" -maxdepth 1 -name "*.stderr" | sort)

echo ""
echo "Total .stderr files    : $total"
echo "Good (has error[E...]) : $good"
echo "No error code          : $no_code"
echo "Empty                  : $empty"

warn_count=$((no_code + empty))

if [[ $warn_count -gt 0 ]]; then
    echo ""
    echo "WARN: $warn_count .stderr files may be incomplete receipts."
    echo "      Run: cargo test --test ui_tests -- --ignored"
    echo "      to regenerate them."
    echo "      This is a soft warning — exit 0."
fi

echo ""
echo "RESULT: $([ $warn_count -eq 0 ] && echo 'PASS' || echo 'WARN') — $good/$total stderr files have valid Rust error codes."
exit 0
