#!/usr/bin/env bash
# Checks that every src/*.rs module has at least one //! doc comment line.
set -euo pipefail
cd "$(dirname "$0")/../.."
MISSING=0
for f in src/*.rs; do
    if ! grep -q "^//!" "$f"; then
        echo "  WARN: $f has no //! module documentation"
        ((MISSING++)) || true
    fi
done
if [ "$MISSING" -gt 0 ]; then
    echo "WARN: $MISSING module(s) lack //! documentation (soft warning)"
fi
echo "PASS: module doc audit complete ($MISSING undocumented)"
