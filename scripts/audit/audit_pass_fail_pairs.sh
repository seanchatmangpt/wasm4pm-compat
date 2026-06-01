#!/usr/bin/env bash
# audit_pass_fail_pairs.sh — for each compile_fail/*.rs fixture, check that a
# compile_pass/*.rs exists with a matching law surface keyword.
# Exit 0 (soft warn only).
set -euo pipefail
cd "$(dirname "$0")/../.."

FAIL_DIR="tests/ui/compile_fail"
PASS_DIR="tests/ui/compile_pass"
UNMATCHED=0

for fail_rs in "$FAIL_DIR"/*.rs; do
  [[ -f "$fail_rs" ]] || continue
  base=$(basename "$fail_rs" .rs)
  # Extract the law keyword: first token of the filename (up to first underscore group)
  # Strategy: look for any compile_pass fixture sharing the first meaningful word
  first_word=$(echo "$base" | cut -d'_' -f1)
  matches=$(ls "$PASS_DIR"/"${first_word}"*.rs 2>/dev/null | wc -l)
  if [[ "$matches" -eq 0 ]]; then
    echo "  WARN  no compile_pass pair for: $base (keyword: $first_word)"
    ((UNMATCHED++))
  fi
done

echo "--- pass-fail pairs audit: $UNMATCHED unmatched compile_fail fixtures ---"
exit 0
