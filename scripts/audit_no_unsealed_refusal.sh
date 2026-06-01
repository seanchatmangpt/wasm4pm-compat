#!/usr/bin/env bash
# audit_no_unsealed_refusal.sh — grep src/admission.rs for InvalidInput or
# GenericError variants. Exit 1 if found as actual variant definitions.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ADMISSION_RS="$REPO_ROOT/src/admission.rs"

echo "=== Unsealed Refusal Detection Audit ==="
echo ""

if [[ ! -f "$ADMISSION_RS" ]]; then
    echo "FAIL: src/admission.rs not found"
    exit 1
fi

fail=0

# Look for actual enum variant definitions named InvalidInput or GenericError
# (not comments or doc strings that mention them as forbidden)
echo "Scanning $ADMISSION_RS for unsealed refusal variants..."

# Match lines that define these as enum variants (not in comments/doc strings)
if grep -En "^[[:space:]]*(InvalidInput|GenericError)[[:space:]]*[,{(]" "$ADMISSION_RS" > /dev/null 2>&1; then
    echo ""
    echo "FAIL: Unsealed refusal variant(s) found in src/admission.rs:"
    grep -En "^[[:space:]]*(InvalidInput|GenericError)[[:space:]]*[,{(]" "$ADMISSION_RS" | while IFS= read -r line; do
        echo "  $line"
    done
    fail=1
fi

# Also check for struct definitions with these names
if grep -En "^pub struct (InvalidInput|GenericError)" "$ADMISSION_RS" > /dev/null 2>&1; then
    echo ""
    echo "FAIL: Unsealed refusal struct(s) found in src/admission.rs:"
    grep -En "^pub struct (InvalidInput|GenericError)" "$ADMISSION_RS" | while IFS= read -r line; do
        echo "  $line"
    done
    fail=1
fi

# Check all src/ files for GenericError/InvalidInput as reason types
echo "Scanning all src/ files for generic refusal reason usage..."
found_generic=0
while IFS= read -r src_file; do
    if grep -En "Refusal<[^>]*InvalidInput|Refusal<[^>]*GenericError|reason.*InvalidInput|reason.*GenericError" "$src_file" > /dev/null 2>&1; then
        if ! grep -q "never.*InvalidInput\|forbidden.*InvalidInput\|not.*InvalidInput\|avoid.*InvalidInput" "$src_file"; then
            echo "WARN: Possible generic refusal reason in $src_file"
            grep -En "Refusal<[^>]*InvalidInput|Refusal<[^>]*GenericError" "$src_file" | head -3 | while IFS= read -r line; do
                echo "  $line"
            done
            found_generic=1
        fi
    fi
done < <(find "$REPO_ROOT/src" -name "*.rs" | sort)

echo ""
if [[ $fail -eq 1 ]]; then
    echo "RESULT: HARD FAIL — unsealed refusal variant(s) detected."
    exit 1
else
    if [[ $found_generic -eq 1 ]]; then
        echo "RESULT: PASS with warnings — no unsealed variants, but check WARN lines above."
    else
        echo "RESULT: PASS — no unsealed refusal variants found."
    fi
    exit 0
fi
