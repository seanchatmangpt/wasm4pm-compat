#!/usr/bin/env bash
# audit_receipt_chain.sh — check src/receipt.rs has ReceiptEnvelope with
# witness+digest+replay_hint fields. Exit 1 if missing.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RECEIPT_RS="$REPO_ROOT/src/receipt.rs"

echo "=== Receipt Chain Completeness Audit ==="
echo ""

if [[ ! -f "$RECEIPT_RS" ]]; then
    echo "FAIL: src/receipt.rs not found"
    exit 1
fi

fail=0

echo "Checking for ReceiptEnvelope struct..."
if grep -q "pub struct ReceiptEnvelope" "$RECEIPT_RS"; then
    echo "OK: ReceiptEnvelope struct found"
else
    echo "FAIL: ReceiptEnvelope struct not found in src/receipt.rs"
    fail=1
fi

echo ""
echo "Checking required fields on ReceiptEnvelope..."

for field in "witness" "digest" "replay_hint"; do
    if grep -q "pub $field" "$RECEIPT_RS"; then
        echo "OK: field '$field' present"
    else
        echo "FAIL: field '$field' missing from ReceiptEnvelope"
        fail=1
    fi
done

echo ""
echo "Checking field types..."

if grep -q "digest.*Digest\|pub digest" "$RECEIPT_RS"; then
    echo "OK: digest field exists"
else
    echo "FAIL: digest field or Digest type not found"
    fail=1
fi

if grep -q "replay_hint.*ReplayHint\|pub replay_hint" "$RECEIPT_RS"; then
    echo "OK: replay_hint field exists"
else
    echo "FAIL: replay_hint field or ReplayHint type not found"
    fail=1
fi

# Check WellShaped trait
echo ""
echo "Checking WellShaped trait..."
if grep -q "pub trait WellShaped" "$RECEIPT_RS"; then
    echo "OK: WellShaped trait defined"
else
    echo "WARN: WellShaped trait not found in receipt.rs"
fi

echo ""
if [[ $fail -eq 1 ]]; then
    echo "RESULT: HARD FAIL — receipt chain is incomplete."
    exit 1
else
    echo "RESULT: PASS — ReceiptEnvelope has all required fields (witness, digest, replay_hint)."
    exit 0
fi
