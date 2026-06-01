#!/usr/bin/env bash
# audit_no_engine_creep.sh — fail if engine function/struct names appear in src/.
#
# Engine names that must not live in this (structure-only) crate:
#   discover     — process discovery algorithm
#   replay_log   — token replay engine
#   align_trace  — alignment computation
#   token_replay — token replay variant
#
# Only checks non-comment lines (lines not starting with optional whitespace + //).

REPO_ROOT="$(dirname "$0")/../.."
FAIL=0

for name in discover replay_log align_trace token_replay; do
    # Grep for the exact name as a whole word (\b boundaries) in non-comment lines.
    # Filter out lines whose first non-whitespace characters are // (inline comments
    # may still appear after code; we only exclude pure comment lines here).
    matches=$(grep -rn "\b${name}\b" "$REPO_ROOT/src/" 2>/dev/null \
        | grep '\.rs:' \
        | grep -Ev '^[^:]+:[0-9]+:[[:space:]]*//' \
        || true)
    if [ -n "$matches" ]; then
        echo "FAIL: engine symbol '$name' found in non-comment source lines:" >&2
        echo "$matches" >&2
        FAIL=1
    fi
done

if [ "$FAIL" -eq 0 ]; then
    echo "PASS: no engine symbols (discover, replay_log, align_trace, token_replay) in src/."
fi

exit "$FAIL"
