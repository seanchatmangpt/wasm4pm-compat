#!/usr/bin/env bash
# audit_projection_loss.sh — check src/loss.rs has LossPolicy enum.
# Exit 1 if missing.
set -euo pipefail
cd "$(dirname "$0")/../.."

LOSS="src/loss.rs"

if [[ ! -f "$LOSS" ]]; then
  echo "  FAIL  $LOSS not found"
  exit 1
fi

if grep -q "LossPolicy" "$LOSS"; then
  echo "  PASS  LossPolicy found in $LOSS"
  exit 0
fi

echo "  FAIL  LossPolicy not found in $LOSS"
exit 1
