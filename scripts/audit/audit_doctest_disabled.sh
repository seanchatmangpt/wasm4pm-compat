#!/usr/bin/env bash
# audit_doctest_disabled.sh — grep Cargo.toml for 'doctest = false' under [lib]
# section. Exit 1 if missing.
set -euo pipefail
cd "$(dirname "$0")/../.."

CARGO="Cargo.toml"

if [[ ! -f "$CARGO" ]]; then
  echo "  FAIL  $CARGO not found"
  exit 1
fi

# Check that doctest = false appears after [lib] and before the next section header
IN_LIB=0
FOUND=0
while IFS= read -r line; do
  if [[ "$line" =~ ^\[lib\] ]]; then
    IN_LIB=1
    continue
  fi
  if [[ "$IN_LIB" -eq 1 && "$line" =~ ^\[ ]]; then
    IN_LIB=0
  fi
  if [[ "$IN_LIB" -eq 1 && "$line" =~ doctest[[:space:]]*=[[:space:]]*false ]]; then
    FOUND=1
    break
  fi
done < "$CARGO"

if [[ "$FOUND" -eq 1 ]]; then
  echo "  PASS  doctest = false found under [lib] in $CARGO"
  exit 0
fi

echo "  FAIL  doctest = false not found under [lib] in $CARGO — doctest storm prevention missing"
exit 1
