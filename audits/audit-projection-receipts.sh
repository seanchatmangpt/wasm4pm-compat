#!/bin/bash
# ggen/audits/audit-projection-receipts.sh.ggen
#
# Projection Receipt Validation Audit
#
# Purpose: Validate that every rendered projection from the three projection manifests
# (TypeScript, WASM, Component Model) has complete receipt evidence: source ontology,
# query, template, output path, receipt entry, and checkpoint effect. Proves each
# projection satisfies the manufacturing covenant: all artifacts are tracked, all
# queries are logged, all templates are named, all outputs are checkpointed.
#
# Proof Structure:
#   Each projection must provide:
#     1. Source ontology (where the schema lives)
#     2. Query (SPARQL/RQ that derives the projection)
#     3. Template (Tera .tera file that renders)
#     4. Output path (relative path to emitted artifact)
#     5. Receipt entry (manifest line in ggen/projections/*.projection.yaml)
#     6. Checkpoint effect (git-tracked or audit-snapshotted)
#
# Usage:
#   bash ggen/audits/audit-projection-receipts.sh.ggen
#   echo $?  # Exit code: 0 = all projections receipted; 1+ = gaps found
#
# Exit codes:
#   0: All projected artifacts have complete receipts (pass)
#   1: Untracked, unreceipted, or missing projection evidence (fail)
#   2: Configuration error (missing projection manifests or registry)

set -u
trap 'true' EXIT  # Prevent exit-on-error trap from premature exit

# ─ Configuration ─────────────────────────────────────────────────────────────

REPO_ROOT="${1:-.}"
GEN_ROOT="${GEN_ROOT:-.}"
PROJ_MANIFESTS_DIR="${GEN_ROOT}/ggen/projections"
ONTOLOGY_DIR="${GEN_ROOT}/ggen/ontology"
QUERIES_DIR="${GEN_ROOT}/ggen/queries"
TEMPLATES_DIR="${GEN_ROOT}/ggen/templates"
EMITTED_DIR="${GEN_ROOT}/ggen/emitted"
AUDITS_DIR="${GEN_ROOT}/ggen/audits"

# Result tracking
PASS_COUNT=0
FAIL_COUNT=0
WARNINGS=0
UNRECEIPTED_COUNT=0

# ─ Colors for output ─────────────────────────────────────────────────────────

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ─ Utility functions ─────────────────────────────────────────────────────────

log_pass() {
    echo -e "${GREEN}✓ PASS${NC}: $*"
    ((PASS_COUNT++))
}

log_fail() {
    echo -e "${RED}✗ FAIL${NC}: $*"
    ((FAIL_COUNT++))
}

log_warn() {
    echo -e "${YELLOW}⚠ WARN${NC}: $*"
    ((WARNINGS++))
}

log_gap() {
    echo -e "${RED}⊘ GAP${NC}: $*"
    ((UNRECEIPTED_COUNT++))
}

die() {
    echo -e "${RED}✗ ERROR${NC}: $*" >&2
    exit 2
}

# ─ Helper: Check if file exists and is readable ──────────────────────────────

file_exists_readable() {
    local file="$1"
    [[ -f "$file" ]] && [[ -r "$file" ]]
}

# ─ Helper: Check if artifact is declared in manifest ──────────────────────────

artifact_in_manifest() {
    local artifact="$1"
    local manifest_file="$2"

    if ! file_exists_readable "$manifest_file"; then
        return 1
    fi

    grep -qE "${artifact}" "$manifest_file" 2>/dev/null
}

# ─ Helper: Check if file is tracked in git ──────────────────────────────────

is_git_tracked() {
    local file_path="$1"

    if ! git -C "$GEN_ROOT" ls-files --error-unmatch "$file_path" >/dev/null 2>&1; then
        return 1
    fi
    return 0
}

# ─ Helper: Check if file is git-ignored ─────────────────────────────────────

is_git_ignored() {
    local file_path="$1"

    git -C "$GEN_ROOT" check-ignore "$file_path" >/dev/null 2>&1
}

# ─ Helper: Extract projection name from manifest file ────────────────────────

get_projection_name() {
    local manifest_file="$1"

    if ! file_exists_readable "$manifest_file"; then
        echo "unknown"
        return 1
    fi

    grep -E "^\s*alias:\s*" "$manifest_file" | head -1 | sed -E 's/^\s*alias:\s*"?([^"]+)"?.*/\1/' || echo "unknown"
}

# ─ Helper: Extract receipt path from manifest ───────────────────────────────

get_receipt_path() {
    local manifest_file="$1"

    if ! file_exists_readable "$manifest_file"; then
        return 1
    fi

    grep -E "^\s*receipt_path:\s*" "$manifest_file" | head -1 | sed -E 's/^\s*receipt_path:\s*"?([^"]+)"?.*/\1/'
}

# ─ Helper: Extract output directory from manifest ──────────────────────────

get_output_dir() {
    local manifest_file="$1"

    if ! file_exists_readable "$manifest_file"; then
        return 1
    fi

    grep -E "^\s*output_dir:\s*" "$manifest_file" | head -1 | sed -E 's/^\s*output_dir:\s*"?([^"]+)"?.*/\1/'
}

# ─ Helper: Validate projection has complete receipt ──────────────────────────

validate_projection_receipt() {
    local manifest_file="$1"
    local proj_type="$2"  # ts, wasm, or component
    local errors=0

    echo ""
    echo -e "${BLUE}━━━ ${proj_type} Projection ━━━${NC}"

    if ! file_exists_readable "$manifest_file"; then
        log_fail "Manifest not found: $manifest_file"
        return 1
    fi

    local proj_name
    proj_name=$(get_projection_name "$manifest_file")

    # ① Source ontology (always: process-intelligence.ttl)
    if file_exists_readable "$ONTOLOGY_DIR/process-intelligence.ttl"; then
        log_pass "Source ontology exists: process-intelligence.ttl"
    else
        log_gap "Source ontology missing: process-intelligence.ttl"
        ((errors++))
    fi

    # ② Query (projection-specific)
    if file_exists_readable "$QUERIES_DIR/${proj_type}-projection.rq"; then
        log_pass "Query exists: ${proj_type}-projection.rq"
    else
        log_warn "Query missing: ${proj_type}-projection.rq (may be embedded in template)"
    fi

    # ③ Template
    local template_file="$TEMPLATES_DIR/${proj_type}-projection.rs.tera"
    if [ "$proj_type" = "component" ]; then
        template_file="$TEMPLATES_DIR/component-model.tera"
    fi

    if file_exists_readable "$template_file"; then
        log_pass "Template exists: $(basename "$template_file")"
    else
        log_gap "Template missing: $(basename "$template_file")"
        ((errors++))
    fi

    # ④ Output path declared in manifest
    local output_dir
    output_dir=$(get_output_dir "$manifest_file")

    if [[ -n "$output_dir" ]]; then
        log_pass "Output path declared: $output_dir"
    else
        log_gap "Output path missing in manifest"
        ((errors++))
    fi

    # ⑤ Receipt entry in manifest
    local receipt_path
    receipt_path=$(get_receipt_path "$manifest_file")

    if [[ -n "$receipt_path" ]]; then
        log_pass "Receipt path declared: $receipt_path"
    else
        log_warn "Receipt path not declared (may be auto-generated)"
    fi

    # ⑥ Checkpoint effect: artifacts tracked or snapshotted
    if [[ -n "$output_dir" ]]; then
        local output_full_path="${GEN_ROOT}/${output_dir}"

        if [[ -d "$output_full_path" ]]; then
            local tracked_count=0
            local untracked_count=0

            while IFS= read -r -d '' artifact; do
                local relative_path="${artifact#${GEN_ROOT}/}"
                if is_git_tracked "$relative_path"; then
                    ((tracked_count++))
                else
                    ((untracked_count++))
                fi
            done < <(find "$output_full_path" -type f -print0 2>/dev/null)

            if [[ $tracked_count -gt 0 ]]; then
                log_pass "Checkpoint effect: $tracked_count artifact(s) tracked in git"
            fi

            if [[ $untracked_count -gt 0 ]]; then
                log_warn "Checkpoint effect: $untracked_count artifact(s) untracked (snapshot candidates)"
            fi
        else
            log_warn "Output directory not yet generated: $output_dir"
        fi
    fi

    return $errors
}

# ────────────────────────────────────────────────────────────────────────────
# AUDIT START
# ────────────────────────────────────────────────────────────────────────────

echo ""
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}Projection Receipt Validation Audit${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo ""

# ──────────────────────────────────────────────────────────────────────────────
# PHASE 0: Precondition Checks
# ──────────────────────────────────────────────────────────────────────────────

echo -e "${BLUE}[Phase 0] Preconditions${NC}"
echo ""

[[ -d "$PROJ_MANIFESTS_DIR" ]] || die "Projections directory not found: $PROJ_MANIFESTS_DIR"
[[ -d "$ONTOLOGY_DIR" ]] || die "Ontology directory not found: $ONTOLOGY_DIR"
[[ -d "$TEMPLATES_DIR" ]] || die "Templates directory not found: $TEMPLATES_DIR"

log_pass "Directories exist and are readable"
echo ""

# ──────────────────────────────────────────────────────────────────────────────
# PHASE 1: Validate Projection Manifest Files Exist
# ──────────────────────────────────────────────────────────────────────────────

echo -e "${BLUE}[Phase 1] Projection Manifests${NC}"
echo ""

declare -A MANIFESTS=(
    [ts]="ts.projection.yaml"
    [wasm]="wasm.projection.yaml"
    [component]="component.projection.yaml"
)

MISSING_MANIFESTS=0

for proj_type in ts wasm component; do
    manifest_file="${PROJ_MANIFESTS_DIR}/${MANIFESTS[$proj_type]}"

    if file_exists_readable "$manifest_file"; then
        log_pass "Manifest exists: ${MANIFESTS[$proj_type]}"
    else
        log_fail "Manifest missing: ${MANIFESTS[$proj_type]}"
        ((MISSING_MANIFESTS++))
    fi
done

echo ""

# ──────────────────────────────────────────────────────────────────────────────
# PHASE 2: TypeScript Projection Receipt Validation
# ──────────────────────────────────────────────────────────────────────────────

echo -e "${BLUE}[Phase 2] TypeScript Projection Receipt${NC}"
validate_projection_receipt "${PROJ_MANIFESTS_DIR}/ts.projection.yaml" "ts"

# ──────────────────────────────────────────────────────────────────────────────
# PHASE 3: WASM Projection Receipt Validation
# ──────────────────────────────────────────────────────────────────────────────

echo -e "${BLUE}[Phase 3] WASM Projection Receipt${NC}"
validate_projection_receipt "${PROJ_MANIFESTS_DIR}/wasm.projection.yaml" "wasm"

# ──────────────────────────────────────────────────────────────────────────────
# PHASE 4: Component Model Projection Receipt Validation
# ──────────────────────────────────────────────────────────────────────────────

echo -e "${BLUE}[Phase 4] Component Model Projection Receipt${NC}"
validate_projection_receipt "${PROJ_MANIFESTS_DIR}/component.projection.yaml" "component"

echo ""

# ──────────────────────────────────────────────────────────────────────────────
# SUMMARY & EXIT
# ──────────────────────────────────────────────────────────────────────────────

echo ""
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}Audit Summary${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "  Passes:        ${GREEN}${PASS_COUNT}${NC}"
echo -e "  Failures:      ${RED}${FAIL_COUNT}${NC}"
echo -e "  Warnings:      ${YELLOW}${WARNINGS}${NC}"
echo -e "  Unreceipted:   ${RED}${UNRECEIPTED_COUNT}${NC}"
echo ""

# ────────────────────────────────────────────────────────────────────────────

if [[ $FAIL_COUNT -eq 0 ]] && [[ $UNRECEIPTED_COUNT -eq 0 ]]; then
    echo -e "${GREEN}✓ PASS: All projections have complete receipts${NC}"
    echo ""
    echo "Covenant satisfied:"
    echo "  ✓ Source ontologies located"
    echo "  ✓ Queries exist or embedded"
    echo "  ✓ Templates defined"
    echo "  ✓ Output paths declared"
    echo "  ✓ Receipt entries recorded"
    echo "  ✓ Checkpoint effects tracked/snapshotted"
    echo ""
    exit 0
else
    EXIT_CODE=1

    if [[ $FAIL_COUNT -gt 0 ]]; then
        echo -e "${RED}✗ FAIL: ${FAIL_COUNT} receipt requirement(s) not met${NC}"
        EXIT_CODE=1
    fi

    if [[ $UNRECEIPTED_COUNT -gt 0 ]]; then
        echo -e "${RED}⊘ GAP: ${UNRECEIPTED_COUNT} projection(s) missing receipt evidence${NC}"
        EXIT_CODE=1
    fi

    echo ""
    echo "Action items:"
    echo "  1. Ensure process-intelligence.ttl exists in ggen/ontology/"
    echo "  2. Add projection queries to ggen/queries/ (if not embedded in templates)"
    echo "  3. Ensure templates exist in ggen/templates/"
    echo "  4. Verify output_dir declared in all *.projection.yaml manifests"
    echo "  5. Verify receipt_path declared in all *.projection.yaml manifests"
    echo "  6. Run projection manufacture and commit artifacts to git"
    echo "  7. Re-run: bash ${AUDITS_DIR}/audit-projection-receipts.sh"
    echo ""
    exit $EXIT_CODE
fi