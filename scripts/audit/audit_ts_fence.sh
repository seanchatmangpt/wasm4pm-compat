#!/usr/bin/env bash
# audit_ts_fence.sh — machine-check the four TS fence gates over SHIPPED files only.
#
# Shipped set is derived from the "files" arrays in:
#   wasm4pm-compat-ts/package.json
#   wasm4pm-compat-wasm/package.json
#
# Gates:
#   1. engine-verb gate — no engine verbs (discover/replay/align/mine/predict)
#      as identifiers in non-comment lines of shipped files. Kebab-case tokens
#      (e.g. 'claims-replay', 'process-mining') are exempt: the verb must not
#      be preceded by a hyphen.
#   2. compat-ts export-shape gate — in @wasm4pm/compat-ts shipped files every
#      `export const` must be a `*Schema` const and every `export type` must be
#      a `z.infer<...>` alias. compat-wasm is EXEMPT (its index.ts may declare
#      OcelV2Surface and re-export the jco-generated bindings).
#   3. imperative-construct gate — no `class`, `function`, `let`, `var`, `new`,
#      `await`, brace-bodied arrows (`=> {`), or for/while loops in shipped
#      files of compat-ts, nor in compat-wasm/index.ts. Exception: z.lazy
#      thunks of the form z.lazy(() => ...) are allowed (expression arrows).
#   4. files-field guard — `npm pack --dry-run` packed set matches the "files"
#      array (plus the npm-always-included package.json/README/LICENSE), and no
#      test/ or node_modules path would ship.
#
# Exit 1 on any failure. Prints PASS/FAIL with offending file:line.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TS_DIR="$REPO_ROOT/wasm4pm-compat-ts"
WASM_DIR="$REPO_ROOT/wasm4pm-compat-wasm"
FAIL=0

fail() {
    echo "FAIL: $*" >&2
    FAIL=1
}

# Strip pure comment lines: // lines, /* openers, * continuations, */ closers.
# (Inline trailing comments after code are NOT stripped; only whole-line comments.)
noncomment_lines() {
    grep -Ev '^[[:space:]]*(//|/\*|\*|\*/)' "$1" || true
}

# ---- derive the shipped TS file set from package.json "files" fields ----

shipped_files() {
    local dir="$1"
    node -e '
const fs = require("fs");
const dir = process.argv[1];
const pkg = JSON.parse(fs.readFileSync(dir + "/package.json", "utf8"));
for (const f of pkg.files || []) {
    if (/\.(ts|js)$/.test(f) || f.endsWith("/")) {
        const p = dir + "/" + f;
        if (f.endsWith("/")) {
            if (fs.existsSync(p)) {
                for (const e of walk(p)) console.log(e);
            }
        } else if (fs.existsSync(p)) {
            console.log(p);
        }
    }
}
function walk(d) {
    const out = [];
    for (const e of fs.readdirSync(d, { withFileTypes: true })) {
        const full = d + "/" + e.name;
        if (e.isDirectory()) out.push(...walk(full));
        else out.push(full);
    }
    return out;
}
' "$dir"
}

# ---- Gate 1: engine-verb gate (all shipped files, both packages) ----

while IFS= read -r f; do
    hits=$(awk -v file="$f" '
        {
            line = $0
            # lower-case copy for matching
            low = tolower(line)
            # find verb tokens not preceded by a hyphen
            n = 0
            s = low
            while (match(s, /(^|[^a-z0-9_-])(discover|replay|align|mine|predict)(s|ed|ing)?([^a-z0-9_-]|$)/)) {
                pre = substr(s, RSTART, RLENGTH)
                c = substr(s, RSTART, 1)
                if (c != "-") { print file ":" FNR ":" line; break }
                s = substr(s, RSTART + RLENGTH - 1)
            }
        }')
    if [ -n "$hits" ]; then
        fail "engine verb in shipped file (gate 1)"
        echo "$hits" >&2
    fi
done < <(shipped_files "$TS_DIR"; shipped_files "$WASM_DIR")

# ---- Gate 2: compat-ts export-shape gate ----

while IFS= read -r f; do
    bad=$(grep -HnE '^[[:space:]]*export ' "$f" \
        | grep -vE '^[^:]+:[0-9]+:export const [A-Za-z0-9_]+Schema\b' \
        | grep -vE '^[^:]+:[0-9]+:export type [A-Za-z0-9_]+ = z\.infer<typeof [A-Za-z0-9_]+Schema>' \
        | grep -vE '^[^:]+:[0-9]+:export \*' \
        || true)
    if [ -n "$bad" ]; then
        fail "non-Schema export in $f (gate 2)"
        echo "$bad" >&2
    fi
done < <(shipped_files "$TS_DIR")

# ---- Gate 3: imperative-construct gate (compat-ts shipped + compat-wasm/index.ts) ----

while IFS= read -r f; do
    bad=$(awk -v file="$f" '
        !/^[[:space:]]*(\/\/|\/\*|\*|\*\/)/ {
            if ($0 ~ /\b(class |function |let |var |new |await |while[[:space:]]*\(|for ?\(|=>[[:space:]]*\{)/) {
                print file ":" FNR ": " $0
            }
        }' "$f")
    if [ -n "$bad" ]; then
        fail "imperative construct in shipped file (gate 3)"
        echo "$bad" >&2
    fi
done < <(shipped_files "$TS_DIR"; echo "$WASM_DIR/index.ts")

# ---- Gate 4: files-field guard (npm pack --dry-run vs files array) ----

for dir in "$TS_DIR" "$WASM_DIR"; do
    pkg_json="$dir/package.json"
    # Packed set from npm itself (real tarball simulation).
    packed=$(npm pack --dry-run --json "$dir" 2>/dev/null \
        | node -e 'let d="";process.stdin.on("data",c=>d+=c).on("end",()=>{const j=JSON.parse(d);const e=j[0]||j;console.log(e.files.map(f=>f.path).join("\n"))})')
    # Declared set: files array (dirs expanded) + npm always-included files.
    declared=$(node -e '
const fs = require("fs");
const dir = process.argv[1];
const pkg = JSON.parse(fs.readFileSync(dir + "/package.json", "utf8"));
const out = new Set(["package.json", "README.md", "LICENSE-MIT"]);
function walk(d) {
    const acc = [];
    for (const e of fs.readdirSync(d, { withFileTypes: true })) {
        const full = d + "/" + e.name;
        if (e.isDirectory()) acc.push(...walk(full));
        else acc.push(full);
    }
    return acc;
}
for (const f of pkg.files || []) {
    const p = (dir + "/" + f).replace(/\/+$/, "") || dir;
    if (f.endsWith("/")) { if (fs.existsSync(p)) for (const w of walk(p)) out.add(w); }
    else out.add(p);
}
const norm = [...out].map(o => o.replace(dir + "/", ""));
for (const o of new Set(norm)) console.log(o);
' "$dir" | sort)
    actual=$(echo "$packed" | sort)
    if ! diff -u <(echo "$declared") <(echo "$actual") >/tmp/ts_fence_pack_diff.$$ 2>&1; then
        fail "packed set does not match files array for $pkg_json (gate 4)"
        sed "s|^|  |" /tmp/ts_fence_pack_diff.$$ >&2
    fi
    rm -f /tmp/ts_fence_pack_diff.$$
    # No test/node_modules path may ship.
    if echo "$packed" | grep -Eq '(^|/)(test|node_modules)(/|$)|\.test\.'; then
        fail "test/node_modules path would ship in $(basename "$dir") (gate 4)"
        echo "$packed" | grep -E '(^|/)(test|node_modules)(/|$)|\.test\.' >&2
    fi
done

# ---- Summary ----

if [ "$FAIL" -eq 0 ]; then
    echo "PASS: TS fence gates 1-4 hold over the shipped files of @wasm4pm/compat-ts and @wasm4pm/compat-wasm."
fi

exit "$FAIL"
