#!/usr/bin/env bash
# audit_no_algorithm_exports.sh — grep src/lib.rs pub use for Miner/Checker/Replayer.
# Exit 1 if found.
set -euo pipefail
cd "$(dirname "$0")/../.."

LIB="src/lib.rs"

if [[ ! -f "$LIB" ]]; then
  echo "  FAIL  $LIB not found"
  exit 1
fi

HITS=$(grep -cE "pub use.*(Miner|Checker|Replayer)" "$LIB" 2>/dev/null || echo "0")

if [[ "$HITS" -gt 0 ]]; then
  echo "  FAIL  algorithm exports (Miner/Checker/Replayer) found in $LIB"
  grep -nE "pub use.*(Miner|Checker|Replayer)" "$LIB" | sed 's/^/         /'
  exit 1
fi

echo "  PASS  no algorithm exports in $LIB"
exit 0
