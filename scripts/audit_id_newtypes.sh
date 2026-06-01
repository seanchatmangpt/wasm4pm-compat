#!/usr/bin/env bash
# audit_id_newtypes.sh — check src/ids.rs for struct EventId pattern vs type aliases;
# warn if type aliases used instead of newtypes. Exit 0 (soft warn).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
IDS_RS="$REPO_ROOT/src/ids.rs"

echo "=== Typed ID Newtype Audit ==="
echo ""

if [[ ! -f "$IDS_RS" ]]; then
    echo "FAIL: src/ids.rs not found"
    exit 0
fi

warn=0

echo "Scanning $IDS_RS for ID type definitions..."
echo ""

# Collect pub type aliases (weak — just a rename, not a newtype)
alias_count=0
echo "--- Type aliases (weak — warn if used for IDs) ---"
while IFS= read -r line; do
    # Match lines like: pub type FooId = ...;
    if echo "$line" | grep -qE "^pub type [A-Z][A-Za-z0-9]*Id[^;]* ="; then
        echo "WARN: type alias for ID: $line"
        alias_count=$((alias_count + 1))
        warn=1
    fi
done < "$IDS_RS"

if [[ $alias_count -eq 0 ]]; then
    echo "OK: No bare type aliases for IDs found."
fi

echo ""
echo "--- Newtype structs (strong — correct pattern) ---"
newtype_count=0
while IFS= read -r line; do
    if echo "$line" | grep -qE "^pub struct [A-Z][A-Za-z0-9]*Id"; then
        echo "OK: newtype struct: $line"
        newtype_count=$((newtype_count + 1))
    fi
done < "$IDS_RS"

echo ""
echo "Newtype structs found : $newtype_count"
echo "Type aliases found    : $alias_count"

echo ""
if [[ $warn -eq 1 ]]; then
    echo "RESULT: WARN — $alias_count ID type alias(es) found. Newtypes are stronger."
    echo "        This is a soft warning — exit 0."
else
    echo "RESULT: PASS — all ID types use newtype structs."
fi

exit 0
