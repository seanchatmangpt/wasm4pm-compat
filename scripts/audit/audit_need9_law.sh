#!/usr/bin/env bash
# audit_need9_law.sh — grep src/law.rs for ConditionCell.
# Exit 1 if not found.
set -euo pipefail
cd "$(dirname "$0")/../.."

LAW="src/law.rs"

if [[ ! -f "$LAW" ]]; then
  echo "  FAIL  $LAW not found"
  exit 1
fi

if grep -q "ConditionCell" "$LAW"; then
  echo "  PASS  ConditionCell (Need9 law) found in $LAW"
  exit 0
fi

echo "  FAIL  ConditionCell not found in $LAW — Need9 law surface missing"
exit 1
