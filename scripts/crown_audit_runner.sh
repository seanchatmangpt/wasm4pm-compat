#!/usr/bin/env bash
# crown_audit_runner.sh — runs ALL audit scripts, outputs a summary table,
# exits 1 if any hard audit failed (exit 1 scripts).
set -euo pipefail

SCRIPTS_DIR="$(cd "$(dirname "$0")" && pwd)"

# Make all scripts executable
chmod +x "$SCRIPTS_DIR"/*.sh

echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║           CROWN AUDIT RUNNER — wasm4pm-compat audit mesh        ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo ""

# Ordered list of all audit scripts (excluding crown runner itself)
AUDIT_SCRIPTS=(
    "audit_pass_fail_pairs.sh"
    "audit_crosswalk_links.sh"
    "audit_graduation_boundaries.sh"
    "audit_no_unsealed_refusal.sh"
    "audit_receipt_chain.sh"
    "audit_projection_loss.sh"
    "audit_witness_markers.sh"
    "audit_id_newtypes.sh"
    "audit_need9_law.sh"
    "audit_metric_bounds.sh"
    "audit_no_algorithm_exports.sh"
    "audit_stderr_quality.sh"
    "audit_doctest_disabled.sh"
    "audit_features.sh"
    "audit_no_engine_creep.sh"
    "audit_no_stable_language.sh"
    "audit_paper_law_ledger.sh"
    "audit_trybuild_receipts.sh"
)

# Results tracking
declare -a results_name
declare -a results_status
declare -a results_exit
hard_failures=0
total=0

run_audit() {
    local script="$1"
    local script_path="$SCRIPTS_DIR/$script"

    if [[ ! -f "$script_path" ]]; then
        results_name+=("$script")
        results_status+=("SKIP")
        results_exit+=(0)
        return
    fi

    total=$((total + 1))
    echo "──────────────────────────────────────────────────────────────────"
    echo "▶ $script"
    echo "──────────────────────────────────────────────────────────────────"

    set +e
    output=$("$script_path" 2>&1)
    exit_code=$?
    set -e

    echo "$output"
    echo ""

    results_name+=("$script")
    results_exit+=($exit_code)

    if [[ $exit_code -eq 0 ]]; then
        # Check if output contains WARN
        if echo "$output" | grep -q "^WARN:"; then
            results_status+=("WARN")
        else
            results_status+=("PASS")
        fi
    else
        results_status+=("FAIL")
        hard_failures=$((hard_failures + 1))
    fi
}

for script in "${AUDIT_SCRIPTS[@]}"; do
    run_audit "$script"
done

echo "╔══════════════════════════════════════════════════════════════════╗"
echo "║                    CROWN AUDIT SUMMARY TABLE                    ║"
echo "╚══════════════════════════════════════════════════════════════════╝"
echo ""
printf "%-45s %-8s %s\n" "AUDIT SCRIPT" "STATUS" "EXIT"
printf "%-45s %-8s %s\n" "─────────────────────────────────────────────" "────────" "────"

pass_count=0
warn_count=0
fail_count=0
skip_count=0

for i in "${!results_name[@]}"; do
    name="${results_name[$i]}"
    status="${results_status[$i]}"
    exit_val="${results_exit[$i]}"

    case "$status" in
        PASS) marker="✓" ; pass_count=$((pass_count + 1)) ;;
        WARN) marker="⚠" ; warn_count=$((warn_count + 1)) ;;
        FAIL) marker="✗" ; fail_count=$((fail_count + 1)) ;;
        SKIP) marker="-" ; skip_count=$((skip_count + 1)) ;;
        *)    marker="?" ;;
    esac

    printf "%-45s %-8s %s\n" "$name" "$marker $status" "$exit_val"
done

echo ""
printf "%-45s %-8s %s\n" "─────────────────────────────────────────────" "────────" "────"
echo ""
echo "PASS  : $pass_count"
echo "WARN  : $warn_count  (soft — exit 0)"
echo "FAIL  : $fail_count  (hard — exit 1)"
echo "SKIP  : $skip_count  (script not found)"
echo ""

if [[ $hard_failures -gt 0 ]]; then
    echo "╔══════════════════════════════════════════════════════════════════╗"
    echo "║  CROWN RESULT: HARD FAIL — $hard_failures audit(s) returned exit 1          ║"
    echo "╚══════════════════════════════════════════════════════════════════╝"
    exit 1
else
    echo "╔══════════════════════════════════════════════════════════════════╗"
    echo "║  CROWN RESULT: PASS — all hard audits clean                     ║"
    echo "╚══════════════════════════════════════════════════════════════════╝"
    exit 0
fi
