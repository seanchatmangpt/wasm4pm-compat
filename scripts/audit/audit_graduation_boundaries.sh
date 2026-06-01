#!/usr/bin/env bash
# audit_graduation_boundaries.sh — check docs/GRADUATION_BOUNDARIES.md exists.
# grep src/lib.rs for pub use of Miner/Checker/Replayer/Aligner.
# Exit 1 if engine API found.
set -euo pipefail
cd "$(dirname "$0")/../.."

DOC="docs/GRADUATION_BOUNDARIES.md"
LIB="src/lib.rs"

if [[ ! -f "$DOC" ]]; then
  echo "  FAIL  $DOC not found"
  exit 1
fi

ENGINE_HITS=$(grep -c "pub use.*\(Miner\|Checker\|Replayer\|Aligner\)" "$LIB" 2>/dev/null || echo "0")

if [[ "$ENGINE_HITS" -gt 0 ]]; then
  echo "  FAIL  engine API (Miner/Checker/Replayer/Aligner) found in $LIB pub use"
  grep -n "pub use.*\(Miner\|Checker\|Replayer\|Aligner\)" "$LIB" | sed 's/^/         /'
  exit 1
fi

echo "  PASS  $DOC exists and no engine API leaked into $LIB"
exit 0
