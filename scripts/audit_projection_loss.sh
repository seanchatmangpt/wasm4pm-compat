#!/usr/bin/env bash
# audit_projection_loss.sh — check src/loss.rs has LossPolicy+ProjectionName+LossReport.
# Exit 1 if any are missing.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOSS_RS="$REPO_ROOT/src/loss.rs"

echo "=== Projection Loss Chain Audit ==="
echo ""

if [[ ! -f "$LOSS_RS" ]]; then
    echo "FAIL: src/loss.rs not found"
    exit 1
fi

fail=0

echo "Checking required types in src/loss.rs..."
echo ""

for type_name in "LossPolicy" "ProjectionName" "LossReport"; do
    if grep -qE "pub (struct|enum|type) $type_name" "$LOSS_RS"; then
        echo "OK: $type_name defined"
    else
        echo "FAIL: $type_name not found in src/loss.rs"
        fail=1
    fi
done

echo ""
echo "Checking LossPolicy variants (RefuseLoss, AllowNamedProjection, AllowLossWithReport)..."
for variant in "RefuseLoss" "AllowNamedProjection" "AllowLossWithReport"; do
    if grep -q "$variant" "$LOSS_RS"; then
        echo "OK: variant $variant present"
    else
        echo "FAIL: LossPolicy variant '$variant' missing"
        fail=1
    fi
done

echo ""
echo "Checking LossReport carries ProjectionName..."
if grep -A20 "pub struct LossReport" "$LOSS_RS" | grep -q "ProjectionName\|projection_name\|name:"; then
    echo "OK: LossReport references ProjectionName"
else
    echo "WARN: LossReport may not carry ProjectionName — verify manually"
fi

echo ""
echo "Checking NamedLoss type..."
if grep -qE "pub (struct|type) NamedLoss" "$LOSS_RS"; then
    echo "OK: NamedLoss defined"
else
    echo "WARN: NamedLoss not found (optional but expected)"
fi

echo ""
if [[ $fail -eq 1 ]]; then
    echo "RESULT: HARD FAIL — loss chain is incomplete."
    exit 1
else
    echo "RESULT: PASS — LossPolicy, ProjectionName, and LossReport all present."
    exit 0
fi
