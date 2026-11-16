#!/usr/bin/env bash
# audit_no_engine_creep.sh — fail if engine logic creeps into src/.
#
# Fence: this crate is structure-only (CLAUDE.md invariant). Discovery,
# mining, replay, and alignment ALGORITHMS graduate to wasm4pm. This audit
# flags engine verbs in fn/const/static/type DECLARATIONS — not prose, not
# routing labels, not named shape types in the canon structure modules.
#
# Patterns (snake_case, closing the discover_ocel_dfg-class compound gap):
#   (fn|const|static) [A-Za-z_]*(discover|mine|replay|align)[a-z_]*
#   (fn|struct|enum|type) [A-Za-z_]*(Discover|Mine|Replay|Align)[A-Za-z_]*
#
# Allowlist (lawful non-algorithmic uses):
#   - src/diagnostic/doctor.rs:745,764  Intent::Discover as_str/parse labels
#   - src/bin/wasm4pm-compat.rs:193     CLI usage "INTENTS:" help text
#   - src/engine_bridge.rs:66           GraduationReason::NeedsDiscovery label
#     (a typed reason routing a consumer TOWARD wasm4pm graduation)
#   - "ocpm-route-discoverer" witness-corpus string literals (paper titles)
#   - Structure-name hits in canon SHAPE modules (src/receipt.rs ReplayHint,
#     src/conformance.rs TokenReplayResult, src/alignment.rs AlignmentClaim):
#     these are structure surfaces (a hint/claim/result CARRIER), not engines.
#     A new algorithm fn in those modules is still caught by the legacy
#     exact-name checks below.
#
# Only checks non-comment lines (lines not starting with optional whitespace + //).

REPO_ROOT="$(dirname "$0")/.."
FAIL=0

# Non-comment, non-test .rs lines under src/. Test modules (#[cfg(test)] to
# EOF of the file) are excluded — they verify the fence, they are not shipped
# engine surface.
src_lines() {
    grep -rln '.' "$REPO_ROOT/src/" --include='*.rs' 2>/dev/null | while read -r f; do
        awk '/#\[cfg\(test\)\]/{exit} {print FILENAME":"FNR":"$0}' "$f"
    done | grep -Ev '^[^:]+:[0-9]+:[[:space:]]*//'
}

# Allowlist filter: lawful routing labels, witness literals, shape carriers.
allowlisted() {
    grep -Ev 'doctor\.rs:(745|764):|wasm4pm-compat\.rs:193:|engine_bridge\.rs:66:' \
        | grep -Ev '(ocpm-route-discoverer|Intent::Discover|NeedsDiscovery)' \
        | grep -Ev 'src/(receipt|conformance|alignment)\.rs:'
}

# 1. snake_case declaration compounds: fn/const/static whose name contains a
#    discovery/mining/replay/alignment verb stem.
snake_hits=$(src_lines \
    | grep -E '(fn|const|static)[[:space:]]+[A-Za-z_]*(discover|mine|replay|align)[a-z_]*' \
    | allowlisted || true)
if [ -n "$snake_hits" ]; then
    echo "FAIL: engine declaration(s) found in src/ (snake_case compounds):" >&2
    echo "$snake_hits" >&2
    FAIL=1
fi

# 2. CamelCase type/variant declarations of engine constructs.
camel_hits=$(src_lines \
    | grep -E '(fn|struct|enum|type)[[:space:]]+[A-Za-z_]*(Discover|Mine|Replay|Align)[A-Za-z_]*' \
    | allowlisted || true)
if [ -n "$camel_hits" ]; then
    echo "FAIL: engine declaration(s) found in src/ (CamelCase):" >&2
    echo "$camel_hits" >&2
    FAIL=1
fi

# 3. Legacy exact-name checks retained for regression clarity.
for name in discover replay_log align_trace token_replay; do
    matches=$(grep -rn "\b${name}\b" "$REPO_ROOT/src/" --include='*.rs' 2>/dev/null \
        | grep -Ev '^[^:]+:[0-9]+:[[:space:]]*//' \
        | allowlisted || true)
    if [ -n "$matches" ]; then
        echo "FAIL: engine symbol '$name' found in non-comment source lines:" >&2
        echo "$matches" >&2
        FAIL=1
    fi
done

if [ "$FAIL" -eq 0 ]; then
    echo "PASS: no engine symbols (discover/mine/replay/align declarations) in src/."
fi

exit "$FAIL"
