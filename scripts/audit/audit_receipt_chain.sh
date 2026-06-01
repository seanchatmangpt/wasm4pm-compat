#!/usr/bin/env bash
# audit_receipt_chain.sh — check src/receipt.rs has ReceiptEnvelope struct.
# Exit 1 if missing.
set -euo pipefail
cd "$(dirname "$0")/../.."

RECEIPT="src/receipt.rs"

if [[ ! -f "$RECEIPT" ]]; then
  echo "  FAIL  $RECEIPT not found"
  exit 1
fi

if grep -q "ReceiptEnvelope" "$RECEIPT"; then
  echo "  PASS  ReceiptEnvelope found in $RECEIPT"
  exit 0
fi

echo "  FAIL  ReceiptEnvelope not found in $RECEIPT"
exit 1
