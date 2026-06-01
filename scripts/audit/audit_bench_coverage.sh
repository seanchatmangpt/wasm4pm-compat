#!/usr/bin/env bash
# Checks that benches/ directory exists and has at least one benchmark.
set -euo pipefail
cd "$(dirname "$0")/../.."
BENCH_COUNT=$(ls benches/*.rs 2>/dev/null | wc -l | tr -d ' ')
if [ "$BENCH_COUNT" -eq 0 ]; then
    echo "FAIL: no benchmark files found in benches/"
    exit 1
fi
echo "PASS: $BENCH_COUNT benchmark file(s) in benches/"
