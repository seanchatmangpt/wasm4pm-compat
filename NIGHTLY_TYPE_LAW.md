# NIGHTLY_TYPE_LAW.md

Paper-to-type-invariant-to-fixture crosswalk for `wasm4pm-compat`.

Every row says: **which paper law** → **which type invariant** → **which compile-pass fixture proves it open** → **which compile-fail fixture seals it**.

---

## Core machinery (`src/law.rs`)

| Paper / Doctrine | Type invariant | Pass fixture | Fail fixture |
|---|---|---|---|
| Blue River Dam: "Need9 means split" | `ConditionCell<9>` does not compile (`BITS ≤ 8` violated) | `condition_cell_8.rs` | `need9_condition_cell.rs` |
| Quality metrics ∈ [0, 1] | `Between01<2, 1>` does not compile (`NUM ≤ DEN` violated) | `conformance_verdict_complete.rs` | `metric_out_of_bounds.rs` |
| Strict boundary must carry round-trip fixture | `ExportBoundaryConst<true, false>` does not satisfy `HasRoundTripFixture` | *(in conformance_verdict_complete.rs)* | `strict_claim_no_fixture.rs` |

---

## Petri net — bipartite arc law (`src/petri.rs`)

| Paper | Type invariant | Pass fixture | Fail fixture |
|---|---|---|---|
| Murata (1989) §2: arcs ∈ (P×T) ∪ (T×P), no P→P or T→T | `PlaceToTransitionArc` requires `TransitionId` for `to`; `TransitionToPlaceArc` requires `PlaceId` for `to` | `petri_place_to_transition_arc.rs` | `petri_place_to_place_arc.rs` |
| Murata (1989) §2: same | as above | `petri_transition_to_place_arc.rs` | `petri_transition_to_transition_arc.rs` |

---

## WF-net — non-forgeable soundness (`src/petri.rs`)

| Paper | Type invariant | Pass fixture | Fail fixture |
|---|---|---|---|
| van der Aalst: WF-net soundness is a non-trivial verified property | `WfNetConst<{SoundnessState::Witnessed}>` requires `SoundnessProof`; `SoundnessProof` has a private seal field | `wfnet_with_soundness_witness.rs` | `wfnet_forged_soundness.rs` |

---

## Process tree — loop arity law (`src/process_tree.rs`)

| Paper | Type invariant | Pass fixture | Fail fixture |
|---|---|---|---|
| Leemans (2013) inductive miner: loop has exactly 2 children | `TypedLoopNode<_, 3>` does not compile (`ARITY == 2` violated) | `process_tree_loop_arity_2.rs` | `process_tree_bad_loop_arity.rs` |

---

## POWL — projection state law (`src/powl.rs`)

| Paper | Type invariant | Pass fixture | Fail fixture |
|---|---|---|---|
| Kourani (2505.07052) §3: partial orders may exceed block structure | `ExceedsProcessTree` does not implement `TreeProjectable`; only `ProcessTreeProjectable` does | `powl_process_tree_projectable.rs` | `powl_silent_tree_projection.rs` |

---

## Evidence lifecycle — one-way door (`src/evidence.rs`)

| Doctrine | Type invariant | Pass fixture | Fail fixture |
|---|---|---|---|
| Blue River Dam: raw evidence may not be used as admitted | `Evidence<T, Raw, W>` has no `into_inner()` method; only `Evidence<T, Admitted, W>` does | `ocel_event_object_relation.rs` | `raw_export_as_admitted.rs` |

---

## OCEL loss law — no silent flattening (`src/formats.rs`)

| Doctrine | Type invariant | Pass fixture | Fail fixture |
|---|---|---|---|
| Loss covenant: OCEL→XES projection must carry a `LossReport` | `accept_lossy_ocel_to_xes` requires `LossyFormatExport` (mandatory loss report); `FormatExport::lossless` is a different type | `ocel_to_xes_with_named_projection.rs` | `ocel_to_xes_no_loss_report.rs` |

---

## OCEL shape — E2O and O2O relations (`src/ocel.rs`)

| Standard | Type invariant | Pass fixture | Fail fixture |
|---|---|---|---|
| OCEL 2.0: event-to-object and object-to-object relations are first-class | `OcelLog` carries `EventObjectLink` and `ObjectObjectLink` fields as distinct, non-optional collections | `ocel_event_object_relation.rs` | `ocel_e2o_missing_link.rs` — proves E2O and O2O slices are non-interchangeable |
| OCEL 2.0: O2O relations preserved | as above | `ocel_object_object_relation.rs` | `ocel_o2o_missing_link.rs` — proves O2O and E2O slices are non-interchangeable |

---

## XES shape — case-centric is distinct from OCEL (`src/xes.rs`)

| Standard | Type invariant | Pass fixture | Fail fixture |
|---|---|---|---|
| IEEE 1849-2023: XES is case-centric; incompatible with object-centric OCEL | `XesLog` and `OcelLog` are different types; functions accepting one reject the other | `xes_case_centric_log.rs` | `xes_not_object_centric.rs` — proves XesLog cannot substitute for OcelLog |

---

## Conformance quality metrics (`src/conformance.rs`)

| Doctrine | Type invariant | Pass fixture | Fail fixture |
|---|---|---|---|
| Fitness, precision, F1 ∈ [0, 1] at the type level | `FitnessConst<3, 4>` compiles; `FitnessConst<2, 1>` does not | `conformance_verdict_complete.rs` | `metric_out_of_bounds.rs` |

---

## Workflow Patterns — named pattern as const-generic (`src/law.rs`)

| Paper | Type invariant | Pass fixture | Fail fixture |
|---|---|---|---|
| Russell, van der Aalst & ter Hofstede (2016) WCP catalogue | `WorkflowPattern` as `ConstParamTy` enum: `PatternNet<{ParallelSplit}>` ≠ `PatternNet<{ExclusiveChoice}>` at the type level | `workflow_pattern_const_param.rs` | `workflow_pattern_wrong_kind.rs` |

---

## POWL 2.0 — separable WF-net marker (`src/petri.rs`)

| Paper | Type invariant | Pass fixture | Fail fixture |
|---|---|---|---|
| Kourani, Park & van der Aalst (2026) Definition 4.1 | `SeparableWfNet<S>` wraps `WfNetConst<S>` with a private seal; only constructible via `declare_separable()`; expresses separability precondition for POWL 2.0 conversion | `separable_wfnet_marker.rs` | **PARTIAL** — no compile-fail fixture yet; law is expressed structurally by the private seal |

---

## How to add a new type-law receipt

1. Identify the paper law (which paper, which section, which invariant).
2. Add a type in the appropriate public module that encodes the invariant at the type level.
3. Write a `compile_pass/` fixture proving the lawful path compiles.
4. Write a `compile_fail/` fixture proving the unlawful path fails.
5. Run `cargo test --test ui_tests` — trybuild generates the `.stderr` file.
6. Verify the `.stderr` file contains the **intended law failure**, not an accidental import error or feature-flag miss.
7. Add a row to this table.

ALIVE requires intended failures, not just failing files.
