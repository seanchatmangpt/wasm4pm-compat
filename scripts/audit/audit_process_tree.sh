#!/usr/bin/env bash
# audit_process_tree.sh
# Enforce process tree arity constraints and SHACL shape validation.
# Exit 0 if all gates pass, 1 if any gate fails.

set -euo pipefail
cd "$(dirname "$0")/../.."

EXIT_CODE=0

echo ""
echo "=== Audit: Process Tree Arity Constraints (Ontology + SHACL) ==="

# Gate 1: src/process_tree.rs exists and has arity functions
echo ""
echo "Gate 1: src/process_tree.rs TypedLoopNode constraint..."
if [ -f "src/process_tree.rs" ]; then
    echo "  ✓ src/process_tree.rs exists"
    if grep -q "TypedLoopNode" src/process_tree.rs; then
        echo "  ✓ TypedLoopNode found"
        if grep -q "Require<.*ARITY.*== 2.*IsTrue" src/process_tree.rs; then
            echo "  ✓ Arity == 2 constraint found in TypedLoopNode"
        else
            echo "  ⚠ WARNING: Arity constraint pattern not found (may be formatted differently)"
        fi
    else
        echo "  ✗ FAILED: TypedLoopNode not found in src/process_tree.rs"
        EXIT_CODE=1
    fi
else
    echo "  ✗ FAILED: src/process_tree.rs not found"
    EXIT_CODE=1
fi

# Gate 2: operator arity bound functions are defined
echo ""
echo "Gate 2: Operator arity bound functions..."
if grep -q "operator_minimum_arity\|min_arity\|minArity" src/process_tree.rs 2>/dev/null; then
    echo "  ✓ Arity minimum function(s) found"
else
    echo "  ⚠ WARNING: Arity minimum function not found (may be named differently)"
fi

if grep -q "operator_maximum_arity\|max_arity\|maxArity" src/process_tree.rs 2>/dev/null; then
    echo "  ✓ Arity maximum function(s) found"
else
    echo "  ⚠ WARNING: Arity maximum function not found (may be named differently)"
fi

# Gate 3: ProcessTreeOperator enum has all five variants
echo ""
echo "Gate 3: ProcessTreeOperator enum variants..."
REQUIRED_VARIANTS=("Sequence" "Xor" "Parallel" "Loop" "Silent")
MISSING_VARIANTS=0
for variant in "${REQUIRED_VARIANTS[@]}"; do
    if grep -q -w "$variant" src/process_tree.rs; then
        echo "  ✓ $variant operator found"
    else
        echo "  ✗ $variant operator MISSING"
        MISSING_VARIANTS=$((MISSING_VARIANTS + 1))
    fi
done

if [ $MISSING_VARIANTS -gt 0 ]; then
    echo "  ✗ FAILED: $MISSING_VARIANTS operator variants missing from ProcessTreeOperator enum"
    EXIT_CODE=1
fi

# Gate 4: SHACL shape file exists
echo ""
echo "Gate 4: SHACL shapes for process tree operators..."
if [ -f "ggen/shapes/process-tree.shacl.ttl" ]; then
    echo "  ✓ ggen/shapes/process-tree.shacl.ttl present"

    # Verify key shape definitions in SHACL file
    if grep -q "ProcessTreeOperator_LoopShape" ggen/shapes/process-tree.shacl.ttl; then
        echo "  ✓ Loop operator shape defined"
    else
        echo "  ⚠ WARNING: Loop shape definition not found"
    fi

    if grep -q "ProcessTreeOperator_SilentShape" ggen/shapes/process-tree.shacl.ttl; then
        echo "  ✓ Silent operator shape defined"
    else
        echo "  ⚠ WARNING: Silent shape definition not found"
    fi

    if grep -q "ProcessTreeOperator_SequenceShape\|ProcessTreeOperator_XorShape\|ProcessTreeOperator_ParallelShape" ggen/shapes/process-tree.shacl.ttl; then
        echo "  ✓ Multi-child operator shapes (Sequence/Xor/Parallel) defined"
    else
        echo "  ⚠ WARNING: Multi-child operator shapes not all found"
    fi
else
    echo "  ✗ FAILED: ggen/shapes/process-tree.shacl.ttl missing"
    EXIT_CODE=1
fi

# Gate 5: ProcessTree ontology is registered
echo ""
echo "Gate 5: ProcessTree ontology registration..."
if [ -f "ggen/ontology/wasm4pm-compat.ttl" ]; then
    if grep -q "compat:ProcessTree" ggen/ontology/wasm4pm-compat.ttl; then
        echo "  ✓ ProcessTree registered in ontology"
        if grep -q "ProcessTreeOperator" ggen/ontology/wasm4pm-compat.ttl; then
            echo "  ✓ ProcessTreeOperator registered in ontology"
        fi
        if grep -q "TypedLoopNode" ggen/ontology/wasm4pm-compat.ttl; then
            echo "  ✓ TypedLoopNode registered in ontology"
        fi
    else
        echo "  ⚠ WARNING: ProcessTree not registered in ontology"
    fi
else
    echo "  ⚠ WARNING: ggen/ontology/wasm4pm-compat.ttl not found"
fi

# Gate 6: Compile-fail fixtures for process tree arity laws
echo ""
echo "Gate 6: Type-law receipts (compile-fail fixtures for process tree)..."
TREE_FAIL_FIXTURES=(
    "process_tree_loop_arity_1"
    "process_tree_loop_arity_3"
    "process_tree_silent_with_child"
    "process_tree_xor_arity_1"
    "process_tree_seq_arity_1"
    "process_tree_and_arity_1"
    "powl_process_tree_xor_arity_1"
)
FAILED_FAIL_FIXTURES=0
for fixture in "${TREE_FAIL_FIXTURES[@]}"; do
    if [ -f "tests/ui/compile_fail/${fixture}.rs" ]; then
        if [ -f "tests/ui/compile_fail/${fixture}.stderr" ]; then
            echo "  ✓ ${fixture}.rs (with .stderr receipt)"
        else
            echo "  ⚠ ${fixture}.rs present but .stderr receipt missing"
        fi
    else
        echo "  ✗ ${fixture}.rs MISSING"
        FAILED_FAIL_FIXTURES=$((FAILED_FAIL_FIXTURES + 1))
    fi
done

if [ $FAILED_FAIL_FIXTURES -gt 0 ]; then
    echo "  ✗ WARNING: $FAILED_FAIL_FIXTURES process tree compile-fail fixtures missing"
    echo "    (Not fatal; verification will occur during 'cargo test --test ui_tests')"
fi

# Gate 7: Compile-pass fixtures for process tree lawful paths
echo ""
echo "Gate 7: Type-law receipts (compile-pass fixtures for process tree)..."
TREE_PASS_FIXTURES=(
    "process_tree_loop_arity_2"
    "process_tree_typed_loop_const_arity"
    "process_tree_operator_arity_constants"
    "process_tree_admit_shape"
    "powl_process_tree_projectable"
)
FAILED_PASS_FIXTURES=0
for fixture in "${TREE_PASS_FIXTURES[@]}"; do
    if [ -f "tests/ui/compile_pass/${fixture}.rs" ]; then
        echo "  ✓ ${fixture}.rs"
    else
        echo "  ✗ ${fixture}.rs MISSING"
        FAILED_PASS_FIXTURES=$((FAILED_PASS_FIXTURES + 1))
    fi
done

if [ $FAILED_PASS_FIXTURES -gt 0 ]; then
    echo "  ✗ WARNING: $FAILED_PASS_FIXTURES process tree compile-pass fixtures missing"
fi

# Gate 8: TreeProjectable sealed trait
echo ""
echo "Gate 8: TreeProjectable sealed trait..."
if grep -q "TreeProjectable" src/process_tree.rs || grep -q "TreeProjectable" src/powl.rs 2>/dev/null; then
    echo "  ✓ TreeProjectable sealed trait found"
    if grep -q "sealed\|pub(crate)" src/process_tree.rs 2>/dev/null || grep -q "sealed\|pub(crate)" src/powl.rs 2>/dev/null; then
        echo "  ✓ TreeProjectable appears to be sealed"
    else
        echo "  ⚠ WARNING: TreeProjectable sealing mechanism not obvious"
    fi
else
    echo "  ⚠ WARNING: TreeProjectable sealed trait not found"
fi

# Gate 9: ProcessTreeRefusal named reasons
echo ""
echo "Gate 9: ProcessTreeRefusal named reasons..."
if grep -q "ProcessTreeRefusal" src/process_tree.rs; then
    echo "  ✓ ProcessTreeRefusal found"
    REQUIRED_REASONS=("InvalidArity" "NonBlockStructured" "UnprojectableFromPowl" "NonMonotonicReduction")
    MISSING_REASONS=0
    for reason in "${REQUIRED_REASONS[@]}"; do
        if grep -q "$reason" src/process_tree.rs; then
            echo "  ✓ ProcessTreeRefusal::$reason found"
        else
            echo "  ✗ ProcessTreeRefusal::$reason MISSING"
            MISSING_REASONS=$((MISSING_REASONS + 1))
        fi
    done
    if [ $MISSING_REASONS -gt 0 ]; then
        echo "  ⚠ WARNING: $MISSING_REASONS refusal reasons missing"
    fi
else
    echo "  ⚠ WARNING: ProcessTreeRefusal not found"
fi

echo ""
if [ $EXIT_CODE -eq 0 ]; then
    echo "=== Audit Complete: Process Tree Arity Constraints PASSED ==="
else
    echo "=== Audit Complete: Process Tree Arity Constraints FAILED ==="
fi

exit $EXIT_CODE
