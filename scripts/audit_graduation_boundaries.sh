#!/usr/bin/env bash
# audit_graduation_boundaries.sh — check GRADUATION_BOUNDARIES.md exists and
# src/lib.rs has no execution engine APIs exported. Exit 1 if engine API found.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BOUNDARIES_DOC="$REPO_ROOT/docs/GRADUATION_BOUNDARIES.md"
LIB_RS="$REPO_ROOT/src/lib.rs"

echo "=== Graduation Boundary Audit ==="
echo ""

fail=0

# Check doc exists
if [[ -f "$BOUNDARIES_DOC" ]]; then
    echo "OK: GRADUATION_BOUNDARIES.md found at docs/GRADUATION_BOUNDARIES.md"
else
    echo "WARN: GRADUATION_BOUNDARIES.md not found at docs/GRADUATION_BOUNDARIES.md"
    # Soft warn — check root too
    if [[ -f "$REPO_ROOT/GRADUATION_BOUNDARIES.md" ]]; then
        echo "     Found at repo root instead."
    fi
fi

# Check lib.rs for engine API exports
# Engine APIs are: types named Miner, Checker, Replayer, Aligner, Discoverer, ConformanceChecker
ENGINE_PATTERNS="Miner|Checker|Replayer|Aligner|Discoverer|ConformanceChecker|AlphaAlgorithm|HeuristicsMiner|InductiveMiner|TokenBasedReplayer|AlignmentBasedReplayer"

echo ""
echo "Scanning $LIB_RS for execution engine exports..."

if grep -E "pub use .*(${ENGINE_PATTERNS})" "$LIB_RS" > /dev/null 2>&1; then
    echo ""
    echo "FAIL: Execution engine APIs found in pub use exports:"
    grep -E "pub use .*(${ENGINE_PATTERNS})" "$LIB_RS" | while IFS= read -r line; do
        echo "  $line"
    done
    fail=1
else
    echo "OK: No execution engine APIs exported from lib.rs"
fi

# Also check for pub fn with engine-like names at top level
if grep -E "^pub fn (mine|check_conformance|replay|align|discover)" "$LIB_RS" > /dev/null 2>&1; then
    echo ""
    echo "FAIL: Top-level engine function(s) found in lib.rs:"
    grep -E "^pub fn (mine|check_conformance|replay|align|discover)" "$LIB_RS" | while IFS= read -r line; do
        echo "  $line"
    done
    fail=1
fi

echo ""
if [[ $fail -eq 1 ]]; then
    echo "RESULT: HARD FAIL — graduation boundary violated."
    exit 1
else
    echo "RESULT: PASS — graduation boundaries are clean."
    exit 0
fi
