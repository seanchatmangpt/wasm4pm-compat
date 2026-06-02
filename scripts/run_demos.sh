#!/bin/bash
# Run all c8 example demos.

set -e

echo "=== Running C8 Demos ==="
echo ""

DEMOS=(
    "c8_market_planck_demo"
    "c8_event_horizon_demo"
    "c8_collider_demo"
    "c8_adversary_gap_demo"
)

for demo in "${DEMOS[@]}"; do
    echo "Running: $demo"
    cargo run --example "$demo" 2>&1 | tail -15
    echo ""
    echo "---"
    echo ""
done

echo "=== ✓ All demos completed ==="
