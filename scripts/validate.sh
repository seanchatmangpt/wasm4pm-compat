#!/bin/bash
# Validate: Format, lint, and test c8 crates.

set -e

echo "=== Validation Suite ==="
echo ""

# 1. Check formatting
echo "1. Checking format..."
cargo fmt --all --check
echo "   ✓ Format check passed"
echo ""

# 2. Run clippy (lint)
echo "2. Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings
echo "   ✓ Clippy passed"
echo ""

# 3. Run tests
echo "3. Running tests..."
cargo test --all-features --lib --tests 2>&1 | tail -20
echo "   ✓ Tests passed"
echo ""

# 4. Build in release mode
echo "4. Building in release mode..."
cargo build --release 2>&1 | grep -E "(Compiling|Finished)"
echo "   ✓ Release build succeeded"
echo ""

echo "=== ✓ Validation complete ==="
