#!/usr/bin/env bash
# audit_id_newtypes.sh — check src/ids.rs for struct.*Id patterns.
# Warn if type aliases found. Exit 0.
set -euo pipefail
cd "$(dirname "$0")/../.."

IDS="src/ids.rs"

if [[ ! -f "$IDS" ]]; then
  echo "  WARN  $IDS not found"
  exit 0
fi

STRUCT_COUNT=$(grep -cE "struct.*Id" "$IDS" 2>/dev/null || echo "0")
ALIAS_COUNT=$(grep -cE "^type .*Id\s*=" "$IDS" 2>/dev/null || echo "0")

if [[ "$ALIAS_COUNT" -gt 0 ]]; then
  echo "  WARN  $ALIAS_COUNT type alias Id(s) found in $IDS — prefer newtype structs"
  grep -nE "^type .*Id\s*=" "$IDS" | sed 's/^/         /'
fi

echo "  PASS  $STRUCT_COUNT struct-based Id types in $IDS (aliases: $ALIAS_COUNT)"
exit 0
