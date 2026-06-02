#!/bin/bash
# Run benchmarks for c8 crates.

set -e

echo "=== Benchmark Suite ==="
echo ""

# Run c8-receipts tests (structural benchmarks)
echo "1. C8 Receipts library tests..."
cd c8-receipts
cargo test --lib --release 2>&1 | tail -10
cd ..
echo "   ✓ C8 Receipts tests complete"
echo ""

# Run main crate benchmarks
echo "2. Main crate benchmarks..."
cargo bench --all-features 2>&1 | grep -E "(Benching|test result|^[a-z_]+ +time:)" | head -20
echo "   ✓ Benchmarks complete"
echo ""

echo "=== ✓ Benchmark suite finished ==="
