#!/usr/bin/env bash
# audit_crosswalk_links.sh — check docs/NIGHTLY_TYPE_LAW.md exists and has rows
# with pipe separators. Warn if fewer than 20 rows. Exit 0.
set -euo pipefail
cd "$(dirname "$0")/../.."

DOC="docs/NIGHTLY_TYPE_LAW.md"

if [[ ! -f "$DOC" ]]; then
  echo "  WARN  $DOC not found"
  exit 0
fi

ROW_COUNT=$(grep -c "|" "$DOC" 2>/dev/null || echo "0")

if [[ "$ROW_COUNT" -lt 20 ]]; then
  echo "  WARN  $DOC has only $ROW_COUNT pipe-separated rows (expected >= 20)"
else
  echo "  PASS  $DOC has $ROW_COUNT pipe-separated rows"
fi

exit 0
