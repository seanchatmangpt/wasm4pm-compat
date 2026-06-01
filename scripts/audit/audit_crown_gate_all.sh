#!/usr/bin/env bash
set -uo pipefail
cd "$(dirname "$0")/../.."
PASS=0; FAIL=0; WARN=0
for s in scripts/audit/audit_*.sh; do
  name=$(basename "$s" .sh)
  # Skip self to prevent infinite recursion
  [[ "$name" == "audit_crown_gate_all" ]] && continue
  if bash "$s" > /dev/null 2>&1; then
    echo "  PASS  $name"; PASS=$((PASS + 1))
  else
    echo "  FAIL  $name"; FAIL=$((FAIL + 1))
  fi
done
echo "--- Crown Audit Gate: $PASS pass, $FAIL fail, $WARN warn ---"
[ "$FAIL" -eq 0 ]
