#!/usr/bin/env bash
# audit_stderr_quality.sh — for each .stderr in tests/ui/compile_fail/, check
# it contains 'error[E'. Count and report files without error codes. Exit 0.
set -euo pipefail
cd "$(dirname "$0")/../.."

FAIL_DIR="tests/ui/compile_fail"
MISSING=0
TOTAL=0

for stderr_file in "$FAIL_DIR"/*.stderr; do
  [[ -f "$stderr_file" ]] || continue
  TOTAL=$((TOTAL + 1))
  if ! grep -q "error\[E" "$stderr_file"; then
    echo "  WARN  no error[E code in: $(basename "$stderr_file")"
    MISSING=$((MISSING + 1))
  fi
done

echo "--- stderr quality audit: $MISSING/$TOTAL .stderr files missing error codes ---"
exit 0
