---
gap_id: GAP_PROCESS_TREE
gap_name: Typed Process Tree Formalization
status: CLOSED
closed_date: 2026-06-02
---

# Gap Closure Receipt: GAP_PROCESS_TREE

## Summary

`GAP_PROCESS_TREE` is closed by the manufacture of `src/process_tree.rs`, which encodes process tree operator arity constraints directly in the type system via `TypedLoopNode<ARITY>` with `Require<{ ARITY == 2 }>: IsTrue` and the corresponding `TypedXorNode`, `TypedAndNode`, `TypedSeqNode`, and `TypedOrNode` variants with their respective minimum-arity bounds. The type-law constraint prevents construction of structurally illegal process tree nodes at compile time, with no runtime check required. This law is verified by trybuild compile-fail and compile-pass receipts in `tests/ui/`.

## Evidence

Files created or modified to close this gap:

- `src/process_tree.rs` — defines `TypedLoopNode<Children, const ARITY: usize>` with `Require<{ ARITY == 2 }>: IsTrue` where-bound; also defines `TypedXorNode`, `TypedAndNode`, `TypedSeqNode`, `TypedOrNode` with `Require<{ ARITY >= 2 }>: IsTrue` bounds; exports `ProcessTreeOperatorKind` variants and arity constants
- `src/law.rs` — provides `Require<const B: bool>`, `IsTrue`, `Assert` machinery that backs all const-generic law bounds in `process_tree.rs`

Compile-fail receipts (each must fail for the named law):

- `tests/ui/compile_fail/process_tree_bad_loop_arity.rs` / `.stderr`
- `tests/ui/compile_fail/process_tree_bad_xor_arity.rs` / `.stderr`
- `tests/ui/compile_fail/process_tree_bad_and_arity.rs` / `.stderr`
- `tests/ui/compile_fail/process_tree_bad_seq_arity.rs` / `.stderr`
- `tests/ui/compile_fail/process_tree_loop_arity_1.rs` / `.stderr`
- `tests/ui/compile_fail/process_tree_and_arity_1.rs` / `.stderr`
- `tests/ui/compile_fail/powl_loop_arity_3.rs` / `.stderr`
- `tests/ui/compile_fail/powl_process_tree_xor_arity_1.rs` / `.stderr`

Compile-pass receipts (each must compile successfully, proving the lawful path is open):

- `tests/ui/compile_pass/process_tree_loop_arity_2.rs`
- `tests/ui/compile_pass/process_tree_loop_admit_shape.rs`
- `tests/ui/compile_pass/process_tree_and_admit_shape.rs`
- `tests/ui/compile_pass/process_tree_seq_admit_shape.rs`
- `tests/ui/compile_pass/process_tree_or_admit_shape.rs`
- `tests/ui/compile_pass/process_tree_admit_shape.rs`
- `tests/ui/compile_pass/process_tree_operator_arity_constants.rs`
- `tests/ui/compile_pass/process_tree_operator_variants_all.rs`
- `tests/ui/compile_pass/process_tree_operator_node_shape.rs`
- `tests/ui/compile_pass/process_tree_node_id_ordering.rs`
- `tests/ui/compile_pass/process_tree_typed_and_nary.rs`
- `tests/ui/compile_pass/process_tree_refusal_all_variants.rs`
- `tests/ui/compile_pass/process_tree_refusal_below_min_arity.rs`
- `tests/ui/compile_pass/process_tree_refusal_invalid_arity_loop.rs`
- `tests/ui/compile_pass/process_tree_refusal_missing_root.rs`
- `tests/ui/compile_pass/powl_typed_loop_node_arity_2.rs`
- `tests/ui/compile_pass/powl_loop_node_kind_construction.rs`
- `tests/ui/compile_pass/powl_process_tree_projectable.rs`
- `tests/ui/compile_pass/powl_exceeds_process_tree_marker.rs`

## Audit Gate

`cargo test --test ui_tests -- --ignored` must pass with all `process_tree_*` and relevant `powl_*` fixtures green: compile-fail fixtures reject illegal arity at the `Require<{ ARITY == 2 }>: IsTrue` bound site, and compile-pass fixtures confirm that `TypedLoopNode<_, 2>` and sibling typed operator nodes are constructible on the lawful path. A fixture that fails for any reason other than the named arity law (e.g. missing import, toolchain drift) is not a valid receipt and must be corrected before this gap may remain closed.

verified: 2026-06-02
