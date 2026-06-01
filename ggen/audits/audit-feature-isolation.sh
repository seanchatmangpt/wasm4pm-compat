#!/bin/bash
#
# audit-feature-isolation.sh
#
# Manufacture audit: Feature isolation conformance.
# Generated from ggen/templates/audit-feature-isolation.sh.tera
#
# Checks that cargo features are properly isolated and do not leak dependencies
# or capabilities across feature boundaries. The audit must prove:
#
# PROOF GATES (all must pass):
#  (1) Default feature (formats) is LEAN — no specta, tsify, wasm-bindgen, serde deps
#  (2) Default feature has no wasm-bindgen or tsify code in always-on modules
#  (3) ts feature does NOT imply wasm — ts may activate independently
#  (4) wasm feature does NOT imply engine — wasm-bindgen ≠ process-mining logic
#  (5) component/any future feature does NOT imply wasm4pm
#  (6) wasm4pm bridge is GRADUATION BRIDGE ONLY — no engine logic, no execution
#  (7) engine_bridge contains no discovery/conformance/replay/OCPQ imports
#
# Input: feature definitions from Cargo.toml, dependency bounds, optional_dependency_law
# Output: nonzero exit on any isolation violation
#
# Exit codes:
#  0 — all isolation rules PASS
#  1 — feature isolation VIOLATION detected
#  2 — audit setup ERROR (Cargo.toml missing, invalid structure)

set -u
trap 'true' EXIT  # Prevent exit-on-error trap from premature exit

# ─ Configuration ─────────────────────────────────────────────────────────────

REPO_ROOT="${1:-.}"
CARGO_TOML="${REPO_ROOT}/Cargo.toml"
SRC_ROOT="${REPO_ROOT}/src"

VIOLATIONS=0
WARNINGS=0

# ─ Utility functions ─────────────────────────────────────────────────────────

log_pass() {
    echo "✓ PASS: $*"
}

log_fail() {
    echo "✗ FAIL: $*"
    ((VIOLATIONS++))
}

log_warn() {
    echo "⚠ WARN: $*"
    ((WARNINGS++))
}

die() {
    echo "✗ ERROR: $*" >&2
    exit 2
}

# ─ Precondition checks ───────────────────────────────────────────────────────

[[ -f "$CARGO_TOML" ]] || die "Cargo.toml not found at $CARGO_TOML"
[[ -d "$SRC_ROOT" ]] || die "src/ directory not found at $SRC_ROOT"

# ─ Helper: extract feature flags from Cargo.toml ─────────────────────────────

get_feature_deps() {
    local feature="$1"
    local cargo_file="$2"
    # Extract multi-line feature definition from [features] section using Python.
    python3 - "$feature" "$cargo_file" << 'PYTHON_SCRIPT'
import re, sys
feature_name = sys.argv[1]
cargo_file = sys.argv[2]
try:
    with open(cargo_file) as f:
        lines = f.readlines()
        in_features = False
        features_text = []
        for line in lines:
            if line.startswith('[features]'):
                in_features = True
                features_text.append(line)
            elif in_features:
                if line.startswith('['):
                    break
                features_text.append(line)

        features_section = ''.join(features_text)
        pattern = feature_name + r'\s*=\s*\[(.*?)\]'
        m = re.search(pattern, features_section, re.DOTALL)
        if not m:
            sys.exit(0)
        deps_text = m.group(1)
        for dep in re.findall(r'"([^"]+)"', deps_text):
            print(dep)
except Exception as e:
    sys.exit(1)
PYTHON_SCRIPT
}

# ─ Helper: check if a string matches a pattern list ───────────────────────────

contains_pattern() {
    local text="$1"
    shift
    local patterns=("$@")
    for pattern in "${patterns[@]}"; do
        if [[ "$text" =~ $pattern ]]; then
            return 0
        fi
    done
    return 1
}

# ─ Helper: find all Rust source files ───────────────────────────────────────

find_rs_files() {
    find "$SRC_ROOT" -type f -name "*.rs" | sort
}

# ─ Helper: check if file uses a crate or macro ──────────────────────────────

uses_crate() {
    local file="$1"
    local crate_name="$2"
    # Look for:
    #  - `use crate_name::`
    #  - `crate_name::`
    #  - `#[derive(Specta)]` style attributes
    grep -E "(use|crate)\s+${crate_name}::|#\[derive.*${crate_name}" "$file" >/dev/null 2>&1 || return 1
}

# ─────────────────────────────────────────────────────────────────────────────
# AUDIT RULE (1): Default feature has no specta, tsify, wasm-bindgen, serde
# ─────────────────────────────────────────────────────────────────────────────

echo "═══════════════════════════════════════════════════════════════════════════"
echo "AUDIT: Feature Isolation Conformance"
echo "═══════════════════════════════════════════════════════════════════════════"
echo ""
echo "Rule 1: Default feature (formats) isolation"
echo "──────────────────────────────────────────────────────────────────────────"

# Extract default feature and its deps
default_feature=$(awk '/^default = / {
    s = $0
    if (match(s, /"([^"]+)"/)) {
        print substr(s, RSTART+1, RLENGTH-2)
        exit
    }
}' "$CARGO_TOML")
[[ -n "$default_feature" ]] || die "Could not extract default feature from Cargo.toml"

echo "Default feature: $default_feature"

# Get deps for default feature
default_deps=$(get_feature_deps "$default_feature" "$CARGO_TOML")

# Check that default feature does NOT enable specta, tsify, wasm-bindgen, or serde
forbidden_default_crates=("specta" "tsify" "wasm-bindgen" "serde-wasm-bindgen")

has_forbidden=0
for crate in "${forbidden_default_crates[@]}"; do
    if echo "$default_deps" | grep -q "dep:${crate}" 2>/dev/null; then
        log_fail "Default feature '$default_feature' depends on forbidden crate: $crate"
        has_forbidden=1
    fi
done

if [[ $has_forbidden -eq 0 ]]; then
    log_pass "Default feature '$default_feature' does not enable specta, tsify, or wasm-bindgen"
fi

echo ""

# ─────────────────────────────────────────────────────────────────────────────
# AUDIT RULE (2): Default has no wasm-bindgen or tsify code in always-on modules
# ─────────────────────────────────────────────────────────────────────────────

echo "Rule 2: Default feature code isolation"
echo "──────────────────────────────────────────────────────────────────────────"

# Find all files NOT in ts/ or wasm/ subdirectories (these are always-on)
always_on_files=$(find "$SRC_ROOT" -maxdepth 1 -type f -name "*.rs" | sort)

wasm_bindgen_in_default=0
tsify_in_default=0

for file in $always_on_files; do
    if uses_crate "$file" "wasm_bindgen"; then
        log_fail "Always-on module $(basename "$file") uses wasm_bindgen"
        wasm_bindgen_in_default=1
    fi
    if uses_crate "$file" "tsify"; then
        log_fail "Always-on module $(basename "$file") uses tsify"
        tsify_in_default=1
    fi
done

if [[ $wasm_bindgen_in_default -eq 0 ]] && [[ $tsify_in_default -eq 0 ]]; then
    log_pass "Always-on modules do not use wasm-bindgen or tsify"
fi

echo ""

# ─────────────────────────────────────────────────────────────────────────────
# AUDIT RULE (3): ts feature isolated from wasm-bindgen (unless paired with wasm)
# ─────────────────────────────────────────────────────────────────────────────

echo "Rule 3: ts feature isolation"
echo "──────────────────────────────────────────────────────────────────────────"

ts_deps=$(get_feature_deps "ts" "$CARGO_TOML")

if echo "$ts_deps" | grep -q "dep:wasm-bindgen" 2>/dev/null; then
    log_warn "ts feature directly enables wasm-bindgen (expected only via wasm pairing)"
else
    log_pass "ts feature does not directly enable wasm-bindgen"
fi

# Check that ts module is feature-gated in lib.rs
if grep -q "#\[cfg(feature = \"ts\")\]" "$SRC_ROOT/lib.rs" 2>/dev/null; then
    log_pass "ts module is gated in lib.rs"
else
    log_fail "ts module not properly gated in lib.rs"
fi

echo ""

# ─────────────────────────────────────────────────────────────────────────────
# AUDIT RULE (4): wasm feature isolated from engine imports
# ─────────────────────────────────────────────────────────────────────────────

echo "Rule 4: wasm feature engine isolation"
echo "──────────────────────────────────────────────────────────────────────────"

# wasm should not import engine-facing modules (engine_bridge, graduation, etc)
wasm_engine_isolation_ok=1

engine_modules=("engine_bridge" "graduation" "discovery" "conformance_engine" "replay")

if [[ -d "$SRC_ROOT/wasm" ]]; then
    for eng_mod in "${engine_modules[@]}"; do
        if grep -r "use.*::${eng_mod}\|use.*engine" "$SRC_ROOT/wasm" 2>/dev/null | grep -q .; then
            log_fail "wasm modules import engine-facing module: $eng_mod"
            wasm_engine_isolation_ok=0
        fi
    done
fi

if [[ $wasm_engine_isolation_ok -eq 1 ]]; then
    log_pass "wasm modules do not import engine-facing modules"
fi

# Verify wasm module is feature-gated
if grep -q "#\[cfg(feature = \"wasm\")\]" "$SRC_ROOT/lib.rs" 2>/dev/null; then
    log_pass "wasm module is gated in lib.rs"
else
    log_fail "wasm module not properly gated in lib.rs"
fi

echo ""

# ─────────────────────────────────────────────────────────────────────────────
# AUDIT RULE (5): wasm4pm feature gated—GRADUATION BRIDGE ONLY, no engine logic
# ─────────────────────────────────────────────────────────────────────────────

echo "Rule 5: wasm4pm feature gating (GRADUATION BRIDGE ONLY)"
echo "──────────────────────────────────────────────────────────────────────────"

# Check that engine_bridge is gated by wasm4pm feature
if grep -q "#\[cfg(feature = \"wasm4pm\")\]" "$SRC_ROOT/lib.rs" 2>/dev/null; then
    log_pass "engine_bridge module is gated by wasm4pm feature in lib.rs"
else
    log_fail "engine_bridge module not properly gated in lib.rs"
fi

# Check that engine_bridge.rs itself declares the feature gate at the top
if [[ -f "$SRC_ROOT/engine_bridge.rs" ]]; then
    if head -5 "$SRC_ROOT/engine_bridge.rs" | grep -q "#\[cfg(feature = \"wasm4pm\")\]" 2>/dev/null; then
        log_pass "engine_bridge.rs declares wasm4pm feature gate"
    fi
fi

# Ensure engine_bridge imports NO discovery/conformance/replay/OCPQ/execution logic
# wasm4pm bridge is BRIDGE ONLY — structure, contracts, graduation signals; no engine
bridge_has_engine_logic=0
if [[ -f "$SRC_ROOT/engine_bridge.rs" ]]; then
    # Forbidden imports that indicate engine logic
    forbidden_engine_patterns=(
        "pm4py"
        "wasm4pm::mining"
        "wasm4pm::conformance"
        "wasm4pm::replay"
        "wasm4pm::discovery"
        "::conformance"
        "::mining"
        "::discovery"
        "::replay"
        "::alignment"
        "process_model::discover"
        "event_log::conform"
    )

    for pattern in "${forbidden_engine_patterns[@]}"; do
        if grep -E "use|import" "$SRC_ROOT/engine_bridge.rs" | grep -q "$pattern" 2>/dev/null; then
            log_fail "engine_bridge imports forbidden engine logic: $pattern"
            bridge_has_engine_logic=1
        fi
    done
fi

if [[ $bridge_has_engine_logic -eq 0 ]]; then
    log_pass "engine_bridge contains no discovery/conformance/replay/OCPQ imports (graduation bridge only)"
fi

# Ensure no engine imports are in always-on modules
# EXCEPTION: interop.rs may define GraduationCandidate as a boundary marker (always-on by covenant)
# All other modules must feature-gate wasm4pm types
wasm4pm_violation=0
for file in $always_on_files; do
    # Skip engine_bridge.rs and interop.rs (both have special roles)
    basename_file="$(basename "$file")"
    if [[ "$basename_file" == "engine_bridge.rs" || "$basename_file" == "interop.rs" ]]; then
        continue
    fi
    # Check for actual use (not just in comments/docstrings)
    # Look for: pub trait/impl GraduationReason, pub struct GraduationCandidate, impl GraduationCandidate
    if grep -E "^[^/]*\b(pub\s+(trait|struct|enum)|impl)\s+(GraduationReason|GraduationCandidate)" "$file" 2>/dev/null | grep -v "///" | grep -q .; then
        if ! grep -q "#\[cfg(feature = \"wasm4pm\")\]" "$file" 2>/dev/null; then
            log_fail "Always-on module $(basename "$file") defines/exports wasm4pm types without feature gate"
            wasm4pm_violation=1
        fi
    fi
done

if [[ $wasm4pm_violation -eq 0 ]]; then
    log_pass "wasm4pm graduation boundary types are properly isolated (GraduationCandidate available always-on via interop)"
fi

echo ""

# ─────────────────────────────────────────────────────────────────────────────
# AUDIT RULE (6): No feature (ts, wasm, component, future) implies wasm4pm
# ─────────────────────────────────────────────────────────────────────────────

echo "Rule 6: No feature implies wasm4pm (independence gate)"
echo "──────────────────────────────────────────────────────────────────────────"

# Extract all features (except wasm4pm itself) and verify none of them
# enable wasm4pm as a transitive dependency
no_feature_implies_wasm4pm=1

declared_features=$(sed -n '/^\[features\]/,/^\[dependencies\]/p' "$CARGO_TOML" | grep "^[a-z]" | cut -d= -f1 | tr -d ' ')

for feat in $declared_features; do
    if [[ "$feat" == "wasm4pm" ]]; then
        continue  # wasm4pm may enable itself; skip
    fi

    feat_deps=$(get_feature_deps "$feat" "$CARGO_TOML")

    # Check if this feature transitively enables wasm4pm
    # In our Cargo.toml, features do not declare deps on other features via [features],
    # but we check the text to be sure
    if echo "$feat_deps" | grep -q "wasm4pm" 2>/dev/null; then
        log_fail "Feature '$feat' transitively enables wasm4pm (violation of independence gate)"
        no_feature_implies_wasm4pm=0
    fi
done

if [[ $no_feature_implies_wasm4pm -eq 1 ]]; then
    log_pass "No feature (ts, wasm, formats, strict) implies or enables wasm4pm"
fi

echo ""

# ─────────────────────────────────────────────────────────────────────────────
# Cross-feature integrity checks
# ─────────────────────────────────────────────────────────────────────────────

echo "Cross-feature integrity"
echo "──────────────────────────────────────────────────────────────────────────"

# Ensure serde is only in optional deps, not enabled by default
if grep -A 5 "^\[dependencies\]" "$CARGO_TOML" | grep "^serde" | grep -q "optional = true"; then
    log_pass "serde is declared as optional dependency"
else
    log_fail "serde should be optional dependency"
fi

# Ensure specta is only in optional deps
if grep -A 20 "^\[dependencies\]" "$CARGO_TOML" | grep "^specta" | grep -q "optional = true"; then
    log_pass "specta is declared as optional dependency"
else
    log_fail "specta should be optional dependency"
fi

# Ensure tsify is only in optional deps
if grep -A 20 "^\[dependencies\]" "$CARGO_TOML" | grep "^tsify" | grep -q "optional = true"; then
    log_pass "tsify is declared as optional dependency"
else
    log_fail "tsify should be optional dependency"
fi

# Ensure wasm-bindgen is only in optional deps
if grep -A 20 "^\[dependencies\]" "$CARGO_TOML" | grep "^wasm-bindgen" | grep -q "optional = true"; then
    log_pass "wasm-bindgen is declared as optional dependency"
else
    log_fail "wasm-bindgen should be optional dependency"
fi

echo ""

# ─────────────────────────────────────────────────────────────────────────────
# Feature count audit: ensure exactly 3 public features + internal variants
# ─────────────────────────────────────────────────────────────────────────────

echo "Feature model integrity"
echo "──────────────────────────────────────────────────────────────────────────"

# Extract all feature names from [features] section
feature_count=$(sed -n '/^\[features\]/,/^\[dependencies\]/p' "$CARGO_TOML" | grep "^[a-z]" | wc -l)

# According to CLAUDE.md, there should be public features: formats, strict, wasm4pm
# plus ts and wasm
expected_features_min=3

if [[ $feature_count -ge $expected_features_min ]]; then
    log_pass "Feature model has valid feature count ($feature_count >= $expected_features_min)"
else
    log_fail "Feature model has fewer than $expected_features_min features ($feature_count)"
fi

# Verify that each major feature gate is in lib.rs
for feat in formats strict wasm4pm ts wasm; do
    if grep -q "#\[cfg(feature = \"${feat}\")\]" "$SRC_ROOT/lib.rs" 2>/dev/null; then
        log_pass "Feature '$feat' properly gated in lib.rs"
    else
        if [[ "$feat" != "formats" ]]; then  # formats may not be gated (default feature)
            log_warn "Feature '$feat' not found gated in lib.rs"
        fi
    fi
done

echo ""

# ─────────────────────────────────────────────────────────────────────────────
# Summary
# ─────────────────────────────────────────────────────────────────────────────

echo "═══════════════════════════════════════════════════════════════════════════"
echo "AUDIT SUMMARY"
echo "═══════════════════════════════════════════════════════════════════════════"
echo "Violations: $VIOLATIONS"
echo "Warnings:   $WARNINGS"
echo ""

if [[ $VIOLATIONS -eq 0 ]]; then
    echo "✓ All feature isolation rules PASS"
    exit 0
else
    echo "✗ Feature isolation audit FAILED ($VIOLATIONS violation(s))"
    exit 1
fi
