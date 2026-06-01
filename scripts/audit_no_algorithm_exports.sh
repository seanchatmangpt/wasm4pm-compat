#!/usr/bin/env bash
# audit_no_algorithm_exports.sh — check src/lib.rs for pub use of
# Miner/Checker/Replayer/Aligner; exit 1 if found.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LIB_RS="$REPO_ROOT/src/lib.rs"

echo "=== Algorithm Export Boundary Audit ==="
echo ""

if [[ ! -f "$LIB_RS" ]]; then
    echo "FAIL: src/lib.rs not found"
    exit 1
fi

fail=0

# Algorithm type names that must NOT be exported from the compat crate
ALGORITHM_TYPES=(
    "Miner"
    "Checker"
    "Replayer"
    "Aligner"
    "Discoverer"
    "ConformanceChecker"
    "AlphaAlgorithm"
    "HeuristicsMiner"
    "InductiveMiner"
    "TokenBasedReplayer"
    "AlignmentBasedReplayer"
    "ProcessDiscovery"
    "AlignmentChecker"
)

echo "Scanning src/lib.rs for algorithm pub use exports..."
echo ""

for algo in "${ALGORITHM_TYPES[@]}"; do
    if grep -qE "pub use .*${algo}|pub.*re-export.*${algo}" "$LIB_RS"; then
        echo "FAIL: Algorithm type '$algo' found in pub use exports:"
        grep -nE "pub use .*${algo}" "$LIB_RS" | while IFS= read -r line; do
            echo "  $line"
        done
        fail=1
    fi
done

if [[ $fail -eq 0 ]]; then
    echo "OK: No algorithm types (Miner/Checker/Replayer/Aligner) exported from lib.rs"
fi

echo ""
echo "Checking pub use exports (full list for review)..."
grep -n "^pub use\|^    pub use" "$LIB_RS" | while IFS= read -r line; do
    echo "  $line"
done

echo ""
if [[ $fail -eq 1 ]]; then
    echo "RESULT: HARD FAIL — algorithm type(s) exported from compat crate."
    echo "        These must graduate to wasm4pm, not be exported here."
    exit 1
else
    echo "RESULT: PASS — no algorithm exports found."
    exit 0
fi
