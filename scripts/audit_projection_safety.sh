#!/usr/bin/env bash
# audit_projection_safety.sh — check projection safety receipts and manifest consistency
# This script validates:
# 1. Projection-related types are properly distinct (no enum confusion)
# 2. ProjectionName uses &'static str (not dynamic strings)
# 3. LossReport generic parameters are consistently used across boundaries
# 4. LossyFormatExport is mandatory for lossy operations (not optional)
# 5. Compile-fail fixtures cover all identified projection safety gaps
# Exit 1 if any safety gap is detected.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOSS_RS="$REPO_ROOT/src/loss.rs"
FORMATS_RS="$REPO_ROOT/src/formats.rs"
COMPILE_FAIL_DIR="$REPO_ROOT/tests/ui/compile_fail"

echo "=== Projection Safety Audit ==="
echo ""

fail=0

# ==============================================================================
# CHECK 1: Type distinctness — FormatKind vs LossPolicy
# ==============================================================================
echo "CHECK 1: Type distinctness — FormatKind vs LossPolicy"
if [[ ! -f "$COMPILE_FAIL_DIR/format_kind_as_loss_policy.rs" ]]; then
    echo "FAIL: Missing fixture format_kind_as_loss_policy.rs"
    echo "      Risk: FormatKind (format enum) could be confused with LossPolicy (decision enum)"
    fail=1
else
    echo "OK: Fixture format_kind_as_loss_policy.rs exists"
fi
echo ""

# ==============================================================================
# CHECK 2: ProjectionName lifetime binding
# ==============================================================================
echo "CHECK 2: ProjectionName lifetime binding (&'static str required)"
if grep -q "pub struct ProjectionName(pub &'static str)" "$LOSS_RS"; then
    echo "OK: ProjectionName enforces &'static str"
else
    echo "FAIL: ProjectionName does not enforce &'static str"
    fail=1
fi

if [[ ! -f "$COMPILE_FAIL_DIR/projection_name_string_lifetime.rs" ]]; then
    echo "FAIL: Missing fixture projection_name_string_lifetime.rs"
    echo "      Risk: Dynamic strings could be used, breaking auditability"
    fail=1
else
    echo "OK: Fixture projection_name_string_lifetime.rs exists"
fi
echo ""

# ==============================================================================
# CHECK 3: LossReport shape tag consistency
# ==============================================================================
echo "CHECK 3: LossReport generic shape parameters (From/To consistency)"
if grep -q "pub struct LossReport<From, To, Items>" "$LOSS_RS"; then
    echo "OK: LossReport uses From/To shape tags"
else
    echo "FAIL: LossReport shape tags not found"
    fail=1
fi

if [[ ! -f "$COMPILE_FAIL_DIR/loss_report_shape_mismatch_from.rs" ]]; then
    echo "FAIL: Missing fixture loss_report_shape_mismatch_from.rs"
    echo "      Risk: Shape tags could be reversed, breaking projection identity"
    fail=1
else
    echo "OK: Fixture loss_report_shape_mismatch_from.rs exists"
fi
echo ""

# ==============================================================================
# CHECK 4: LossReport items type fidelity
# ==============================================================================
echo "CHECK 4: LossReport items type fidelity (Items consistency)"
if [[ ! -f "$COMPILE_FAIL_DIR/loss_report_items_type_mismatch.rs" ]]; then
    echo "FAIL: Missing fixture loss_report_items_type_mismatch.rs"
    echo "      Risk: Items type mismatch could hide lost evidence"
    fail=1
else
    echo "OK: Fixture loss_report_items_type_mismatch.rs exists"
fi
echo ""

# ==============================================================================
# CHECK 5: LossyFormatExport mandatory vs FormatExport optional
# ==============================================================================
echo "CHECK 5: LossyFormatExport mandatory vs FormatExport optional"
if grep -q "pub struct LossyFormatExport" "$FORMATS_RS"; then
    echo "OK: LossyFormatExport type exists"
else
    echo "FAIL: LossyFormatExport type not found"
    fail=1
fi

if grep -q "pub struct FormatExport" "$FORMATS_RS"; then
    echo "OK: FormatExport type exists"
else
    echo "FAIL: FormatExport type not found"
    fail=1
fi

if [[ ! -f "$COMPILE_FAIL_DIR/lossy_format_export_required_not_optional.rs" ]]; then
    echo "FAIL: Missing fixture lossy_format_export_required_not_optional.rs"
    echo "      Risk: Lossy operations might accept optional LossReport"
    fail=1
else
    echo "OK: Fixture lossy_format_export_required_not_optional.rs exists"
fi
echo ""

# ==============================================================================
# CHECK 6: NamedLoss category static binding
# ==============================================================================
echo "CHECK 6: NamedLoss category static binding (&'static str required)"
if grep -q "category: &'static str" "$LOSS_RS"; then
    echo "OK: NamedLoss enforces &'static str for category"
else
    echo "FAIL: NamedLoss category does not enforce &'static str"
    fail=1
fi

if [[ ! -f "$COMPILE_FAIL_DIR/named_loss_category_missing.rs" ]]; then
    echo "FAIL: Missing fixture named_loss_category_missing.rs"
    echo "      Risk: Dynamic loss categories could break auditability"
    fail=1
else
    echo "OK: Fixture named_loss_category_missing.rs exists"
fi
echo ""

# ==============================================================================
# CHECK 7: RefuseLoss policy semantic consistency
# ==============================================================================
echo "CHECK 7: RefuseLoss policy semantic consistency"
if grep -q "RefuseLoss" "$LOSS_RS"; then
    echo "OK: RefuseLoss policy variant exists"
else
    echo "FAIL: RefuseLoss policy variant not found"
    fail=1
fi

if [[ ! -f "$COMPILE_FAIL_DIR/loss_policy_refuse_with_lossy_export.rs" ]]; then
    echo "WARN: Missing fixture loss_policy_refuse_with_lossy_export.rs"
    echo "      Risk: RefuseLoss could be used in lossy export contexts (semantic error)"
else
    echo "OK: Fixture loss_policy_refuse_with_lossy_export.rs exists"
fi
echo ""

# ==============================================================================
# CHECK 8: Compile-fail fixture count validation
# ==============================================================================
echo "CHECK 8: Projection safety fixture inventory"
fixture_count=$(find "$COMPILE_FAIL_DIR" -name "*loss*.rs" -o -name "*projection*.rs" 2>/dev/null | wc -l)
echo "Found $fixture_count loss/projection-related compile_fail fixtures"

expected_new_fixtures=7
if [[ $fixture_count -lt 17 ]]; then
    echo "WARN: Expected at least 17 total fixtures (17 existing + 7 new from this audit)"
    echo "      Current count: $fixture_count"
fi
echo ""

# ==============================================================================
# CHECK 9: Projection names across the codebase
# ==============================================================================
echo "CHECK 9: Audit projection names for consistency"
echo "Scanning for ProjectionName construction across codebase..."
proj_names=$(grep -r "ProjectionName(" "$REPO_ROOT/src" --include="*.rs" | grep -v "test" | grep -v "//!" | wc -l)
if [[ $proj_names -gt 0 ]]; then
    echo "OK: Found $proj_names ProjectionName instances"
    echo "    Verify these are &'static str constants, not dynamic constructions"
else
    echo "INFO: No ProjectionName instances found in source (expected in boundary fns)"
fi
echo ""

# ==============================================================================
# SUMMARY
# ==============================================================================
echo "=== Projection Safety Audit Results ==="
if [[ $fail -eq 0 ]]; then
    echo "RESULT: PASS — all projection safety receipts present."
    exit 0
else
    echo "RESULT: FAIL — projection safety gaps detected."
    echo ""
    echo "To close gaps:"
    echo "  1. Ensure all compile-fail fixtures are present"
    echo "  2. Run: cargo test --test ui_tests -- --ignored"
    echo "  3. Verify each fixture fails for the intended law, not accidentally"
    exit 1
fi
