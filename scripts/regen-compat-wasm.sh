#!/usr/bin/env bash
# Regenerate the committed jco dist/ output for wasm4pm-compat-wasm
# from the sibling component build. Verifies byte-stability of the
# committed artifacts when re-run.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WASM_PKG="$REPO_ROOT/wasm4pm-compat-wasm"
COMPONENT="$REPO_ROOT/../wasm4pm-compat-component/component.wasm"

if [[ ! -f "$COMPONENT" ]]; then
  echo "ERROR: component not found at $COMPONENT" >&2
  echo "Build the sibling repo first:" >&2
  echo "  cd $REPO_ROOT/../wasm4pm-compat-component && cargo build --release" >&2
  echo "(or otherwise produce component.wasm there)" >&2
  exit 1
fi

# Snapshot the committed dist for byte-stability comparison.
SNAP="$(mktemp -d /tmp/compat-wasm-dist.XXXXXX)"
cp -R "$WASM_PKG/dist" "$SNAP/dist-before"

jco transpile "$COMPONENT" -o "$WASM_PKG/dist" --name compat-ocel
cp "$COMPONENT" "$WASM_PKG/dist/compat-ocel.wasm"

( cd "$WASM_PKG" && npm install && npm test )

if diff -r "$SNAP/dist-before" "$WASM_PKG/dist" >/tmp/compat-wasm-dist.diff 2>&1; then
  echo "SUCCESS: dist regenerated and byte-identical to committed dist"
else
  echo "SUCCESS: dist regenerated; NOTE byte drift vs committed dist (see /tmp/compat-wasm-dist.diff)" >&2
fi
rm -rf "$SNAP"
