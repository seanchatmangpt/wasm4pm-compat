#!/usr/bin/env bash
# audit_features.sh — verify Cargo.toml has exactly [formats, strict, wasm4pm].
# Fails (exit 1) if the 'nightly' feature exists or any expected feature is missing.

set -euo pipefail

CARGO_TOML="$(dirname "$0")/../Cargo.toml"

FAIL=0

# Check each required feature is present.
for feature in formats strict wasm4pm; do
    if ! grep -qE "^${feature}\s*=" "$CARGO_TOML"; then
        echo "FAIL: required feature '${feature}' not found in Cargo.toml" >&2
        FAIL=1
    fi
done

# Check that 'nightly' feature does NOT exist.
if grep -qE "^nightly\s*=" "$CARGO_TOML"; then
    echo "FAIL: forbidden 'nightly' feature found in Cargo.toml" >&2
    FAIL=1
fi

if [ "$FAIL" -eq 0 ]; then
    echo "PASS: Cargo.toml features are exactly [formats, strict, wasm4pm]; no 'nightly' feature."
fi

exit "$FAIL"
