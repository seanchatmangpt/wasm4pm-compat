#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
PASS=0; FAIL=0; WARN=0
for s in scripts/audit/audit_*.sh; do
  name=$(basename "$s" .sh)
  if bash "$s" > /dev/null 2>&1; then
    echo "  PASS  $name"; ((PASS++))
  else
    echo "  FAIL  $name"; ((FAIL++))
  fi
done
echo "--- Crown Audit Gate: $PASS pass, $FAIL fail, $WARN warn ---"
[ "$FAIL" -eq 0 ]
