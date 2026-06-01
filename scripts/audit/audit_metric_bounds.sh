#!/usr/bin/env bash
# audit_metric_bounds.sh — grep src/law.rs for Between01.
# Exit 1 if not found.
set -euo pipefail
cd "$(dirname "$0")/../.."

LAW="src/law.rs"

if [[ ! -f "$LAW" ]]; then
  echo "  FAIL  $LAW not found"
  exit 1
fi

if grep -q "Between01" "$LAW"; then
  echo "  PASS  Between01 const law bounds found in $LAW"
  exit 0
fi

echo "  FAIL  Between01 not found in $LAW — metric const bounds missing"
exit 1
