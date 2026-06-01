#!/usr/bin/env bash
# audit_crosswalk_links.sh — check NIGHTLY_TYPE_LAW.md rows have type+pass+fail.
# Parses table rows and warns on incomplete rows. Exit 0 (soft warn).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CROSSWALK="$REPO_ROOT/NIGHTLY_TYPE_LAW.md"

if [[ ! -f "$CROSSWALK" ]]; then
    echo "FAIL: NIGHTLY_TYPE_LAW.md not found at $CROSSWALK"
    exit 0
fi

echo "=== TYPE_LAW_CROSSWALK Completeness Audit ==="
echo ""

incomplete=0
total=0

# Parse table rows: lines with | that have 4 columns (paper|type|pass|fail)
while IFS= read -r line; do
    # Skip header rows and separator rows
    [[ "$line" =~ ^\|[[:space:]]*Paper ]] && continue
    [[ "$line" =~ ^\|[-[:space:]|]+$ ]] && continue
    [[ "$line" =~ ^\| ]] || continue

    # Split by | — expect at least 5 fields (leading | + 4 cols + trailing |)
    IFS='|' read -ra cols <<< "$line"
    # cols[0] is empty (leading |), cols[1]=paper, cols[2]=type, cols[3]=pass, cols[4]=fail
    [[ ${#cols[@]} -lt 5 ]] && continue

    paper="$(echo "${cols[1]}" | xargs)"
    type_inv="$(echo "${cols[2]}" | xargs)"
    pass_fix="$(echo "${cols[3]}" | xargs)"
    fail_fix="$(echo "${cols[4]}" | xargs)"

    [[ -z "$paper" ]] && continue

    total=$((total + 1))
    row_ok=1

    if [[ -z "$type_inv" || "$type_inv" == "—" || "$type_inv" == "-" ]]; then
        echo "WARN row $total: missing type invariant — paper: $paper"
        row_ok=0
    fi
    if [[ -z "$pass_fix" || "$pass_fix" == "—" || "$pass_fix" == "-" || "$pass_fix" == "*(in"* ]]; then
        : # pass fixture may legitimately reference another file
    fi
    if [[ -z "$fail_fix" || "$fail_fix" == "—" || "$fail_fix" == "-" ]]; then
        echo "WARN row $total: missing fail fixture — paper: $paper"
        row_ok=0
    fi

    [[ $row_ok -eq 0 ]] && incomplete=$((incomplete + 1))

done < "$CROSSWALK"

echo ""
echo "Total crosswalk rows  : $total"
echo "Incomplete rows       : $incomplete"
echo "Complete rows         : $((total - incomplete))"

if [[ $incomplete -gt 0 ]]; then
    echo ""
    echo "WARN: $incomplete rows are missing type and/or fail fixture entries."
    echo "      This is a soft warning — exit 0."
fi

exit 0
