#!/usr/bin/env bash
# audit_no_stable_language.sh — fail if affirmative stable-language claims appear in src/ or docs/.
#
# Forbidden phrases (affirmative use):
#   "MSRV"               — only allowed when negated ("no MSRV") or as a future milestone label
#   "stable Rust builds" — this crate is nightly-only; no stable build target exists
#   "wasm4pm_compat_nightly" — this identifier implies a 'nightly' feature gate; forbidden
#   "minimum stable"     — no minimum stable Rust version
#   "stable fallback"    — no stable fallback
#
# MSRV appears legitimately in:
#   - "no MSRV" (negating context)
#   - "documented MSRV" in a future-milestone table row (forward-looking label)
# Those are excluded by filtering out the negating/future patterns.

REPO_ROOT="$(dirname "$0")/../.."
FAIL=0

# ── 1. Exact forbidden phrases (no legitimate use anywhere) ─────────────────
#
# Excluded patterns (negating/meta-doc contexts that are correct usage):
#   "No instances of:"     — FINAL_ALIVE_REPORT.md reporting absence
#   "no affirmative"       — meta-audit language
#   "no stable fallback"   — NIGHTLY_ONLY_COVENANT.md affirming absence (correct)
#   "no stable fallback,"  — same, with trailing comma
#   "Phrases such as:"     — ANTI_REGRESSION_LAWS.md listing examples of forbidden phrases
#   "stable fallback\","   — ANTI_REGRESSION_LAWS.md listing forbidden phrase in enumeration

for phrase in "stable Rust builds" "wasm4pm_compat_nightly" "minimum stable" "stable fallback"; do
    matches=$(grep -rn "$phrase" "$REPO_ROOT/src/" "$REPO_ROOT/docs/" 2>/dev/null \
        | grep -v "No instances of:" \
        | grep -v "no affirmative" \
        | grep -v -i "no stable fallback" \
        | grep -v "### No stable fallback" \
        | grep -v "Phrases such as:" \
        || true)
    if [ -n "$matches" ]; then
        echo "FAIL: forbidden phrase '$phrase' found:" >&2
        echo "$matches" >&2
        FAIL=1
    fi
done

# ── 2. MSRV — allowed only when negated or as a future-milestone label ───────

# Collect all MSRV occurrences, then filter out the allowed patterns:
#   "no MSRV"              → affirmative negation, correct
#   "documented MSRV"      → future milestone table row, forward-looking, correct
#   "No stable/MSRV"       → FINAL_ALIVE_REPORT.md audit table row, meta-report
#   "Instances of"         → FINAL_ALIVE_REPORT.md contextual description
#   "Phrases such as:"     → ANTI_REGRESSION_LAWS.md listing forbidden phrases as meta-doc
msrv_hits=$(grep -rn "MSRV" "$REPO_ROOT/src/" "$REPO_ROOT/docs/" 2>/dev/null \
    | grep -v "no MSRV" \
    | grep -v "documented MSRV" \
    | grep -v "no-MSRV" \
    | grep -v "No stable/MSRV" \
    | grep -v "Instances of" \
    | grep -v "Phrases such as:" \
    | grep -v "do not add MSRV" \
    | grep -v "MSRV badges" \
    || true)

if [ -n "$msrv_hits" ]; then
    echo "FAIL: affirmative MSRV claim found (not in negating/future context):" >&2
    echo "$msrv_hits" >&2
    FAIL=1
fi

if [ "$FAIL" -eq 0 ]; then
    echo "PASS: no forbidden stable-language claims found in src/ or docs/."
fi

exit "$FAIL"
