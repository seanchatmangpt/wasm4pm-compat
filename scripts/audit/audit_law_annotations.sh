#!/usr/bin/env bash
# audit_law_annotations.sh — count compile_fail/*.rs files whose first 5 lines
# contain 'Law:'. Report unannotated count. Exit 0.
set -euo pipefail
cd "$(dirname "$0")/../.."

FAIL_DIR="tests/ui/compile_fail"
ANNOTATED=0
UNANNOTATED=0
TOTAL=0

for rs_file in "$FAIL_DIR"/*.rs; do
  [[ -f "$rs_file" ]] || continue
  ((TOTAL++))
  if head -5 "$rs_file" | grep -q "Law:"; then
    ((ANNOTATED++))
  else
    echo "  WARN  no Law: annotation in first 5 lines: $(basename "$rs_file")"
    ((UNANNOTATED++))
  fi
done

echo "--- law-annotation audit: $ANNOTATED annotated, $UNANNOTATED unannotated out of $TOTAL fixtures ---"
exit 0
