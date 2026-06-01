#!/usr/bin/env bash
# audit_doctest_disabled.sh — check Cargo.toml [lib] section has doctest = false;
# exit 1 if missing (doctest storm prevention).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CARGO_TOML="$REPO_ROOT/Cargo.toml"

echo "=== Doctest Storm Prevention Audit ==="
echo ""

if [[ ! -f "$CARGO_TOML" ]]; then
    echo "FAIL: Cargo.toml not found"
    exit 1
fi

fail=0

echo "Checking Cargo.toml for [lib] section..."
if grep -q "^\[lib\]" "$CARGO_TOML"; then
    echo "OK: [lib] section found"
else
    echo "FAIL: [lib] section not found in Cargo.toml"
    fail=1
fi

echo ""
echo "Checking for doctest = false in [lib] section..."

# Extract [lib] section and check for doctest = false
# Read from [lib] until the next [section]
in_lib=0
found_doctest=0
while IFS= read -r line; do
    if [[ "$line" =~ ^\[lib\] ]]; then
        in_lib=1
        continue
    fi
    if [[ $in_lib -eq 1 && "$line" =~ ^\[ ]]; then
        break
    fi
    if [[ $in_lib -eq 1 ]]; then
        if echo "$line" | grep -qE "^doctest[[:space:]]*=[[:space:]]*false"; then
            found_doctest=1
        fi
    fi
done < "$CARGO_TOML"

if [[ $found_doctest -eq 1 ]]; then
    echo "OK: doctest = false found in [lib] section"
    echo ""
    echo "[lib] section content:"
    awk '/^\[lib\]/{p=1} p && /^\[/ && !/^\[lib\]/{p=0} p{print "  " $0}' "$CARGO_TOML"
else
    echo "FAIL: doctest = false NOT found in [lib] section"
    echo ""
    echo "      This crate has 200+ nightly-feature doctests."
    echo "      Without doctest = false, 'cargo test' triggers 200+ separate rustc"
    echo "      invocations on a nightly compiler, taking 4+ minutes."
    echo "      Add to Cargo.toml [lib]: doctest = false"
    fail=1
fi

echo ""
if [[ $fail -eq 1 ]]; then
    echo "RESULT: HARD FAIL — doctest = false is missing from [lib] section."
    exit 1
else
    echo "RESULT: PASS — doctest storm prevention is in place."
    exit 0
fi
