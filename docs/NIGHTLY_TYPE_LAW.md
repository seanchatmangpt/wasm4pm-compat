# Nightly Type Law — Law-Packet Notes and Type-Law Crosswalk

This document serves two purposes:

1. **Law-packet notes** for papers classified as `COVERED_BY_GRADUATION_BOUNDARY`
   or `PARTIAL_WITH_REASON` in `PAPER_COVERAGE_LEDGER.md`.
2. **Type-law crosswalk** — an index of every law surface added during sprint
   phases, cross-referencing the enforcing type, pass fixture, fail fixture,
   and expected stderr.

---

## Type-Law Crosswalk

Every row represents a type-law enforcement surface. A law with a compile-fail
fixture **must** have a `.stderr` file — a sealed compiler diagnostic. A law
without `.stderr` is not a valid type-law receipt.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Stderr |
|---|---|---|---|---|
| `bpmn_pool_as_lane` — BPMN Pool cannot substitute a Lane | `bpmn::Pool` / `bpmn::Lane` (distinct newtypes) | `compile_pass/bpmn_pool_lane.rs` | `compile_fail/bpmn_pool_as_lane.rs` | `bpmn_pool_as_lane.stderr` |
| `compliance_not_outcome_label` — conformance metric is not a label | `conformance::Metric<KIND, NUM, DEN>` | `compile_pass/conformance_verdict_complete.rs` | `compile_fail/compliance_not_outcome_label.rs` | `compliance_not_outcome_label.stderr` |
| `declare_binary_arity_rejected` — Declare constraint arity ≥ 2 | `declare::DeclareConstraint` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `dfg_engine_boundary_rejected` — DFG is structure, not an engine | `dfg::DirectlyFollowsGraph` | `compile_pass/dfg_shape.rs` | `compile_fail/dfg_engine_boundary_rejected.rs` | `dfg_engine_boundary_rejected.stderr` |
| `dfg_wrong_edge_type` — DFG edge must carry typed endpoints | `dfg::DfgEdge` | `compile_pass/dfg_shape.rs` | `compile_fail/dfg_wrong_edge_type.rs` | `dfg_wrong_edge_type.stderr` |
| `engine_creep_discovery_absent` — discovery absent from compat surface | graduation boundary | `compile_pass/graduation_candidate_marker.rs` | `compile_fail/engine_creep_discovery_absent.rs` | `engine_creep_discovery_absent.stderr` |
| `metric_out_of_bounds` — `Between01<NUM, DEN>` rejects NUM > DEN | `law::Between01<NUM, DEN>` | `compile_pass/ocpq_cardinality_valid_bounds.rs` | `compile_fail/metric_out_of_bounds.rs` | `metric_out_of_bounds.stderr` |
| `need9_condition_cell` — `ConditionCell<BITS>` rejects BITS ≥ 9 | `law::ConditionCell<BITS>` | `compile_pass/condition_cell_8.rs` | `compile_fail/need9_condition_cell.rs` | `need9_condition_cell.stderr` |
| `object_id_as_event_id` — `ObjectId` and `EventId` are distinct | `ids::ObjectId` / `ids::EventId` | `compile_pass/typed_id_construction.rs` | `compile_fail/object_id_as_event_id.rs` | `object_id_as_event_id.stderr` |
| `ocel_e2o_missing_link` — OCEL event-to-object link must be declared | `ocel::OcelEvent` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | `ocel_e2o_missing_link.stderr` |
| `ocel_o2o_missing_link` — OCEL object-to-object link must be declared | `ocel::OcelObject` | `compile_pass/ocel_object_object_relation.rs` | `compile_fail/ocel_o2o_missing_link.rs` | `ocel_o2o_missing_link.stderr` |
| `ocel_to_xes_no_loss_report` — OCEL→XES export requires `LossReport` | `loss::LossReport` / `formats::LossyFormatExport` | `compile_pass/ocel_to_xes_with_named_projection.rs` | `compile_fail/ocel_to_xes_no_loss_report.rs` | `ocel_to_xes_no_loss_report.stderr` |
| `ocpq_cardinality_overflow` — OCPQ cardinality const exceeds bound | `ocpq::CardinalityBound` | `compile_pass/ocpq_cardinality_valid_bounds.rs` | `compile_fail/ocpq_cardinality_overflow.rs` | `ocpq_cardinality_overflow.stderr` |
| `ocpq_cardinality_rejected` — OCPQ cardinality must be within typed bound | `ocpq::CardinalityBound` | `compile_pass/ocpq_cardinality_valid_bounds.rs` | `compile_fail/ocpq_cardinality_rejected.rs` | `ocpq_cardinality_rejected.stderr` |
| `ocpq_flattening_rejected` — OCPQ flattening query refused | `ocpq::OcpqRefusal` | `compile_pass/ocpq_non_flattening_query.rs` | `compile_fail/ocpq_flattening_rejected.rs` | `ocpq_flattening_rejected.stderr` |
| `ocpq_missing_scope_rejected` — OCPQ Def 6: empty ObjectScope refused | `ocpq::OcpqRefusal::MissingObjectScope` | `compile_pass/ocpq_scoped_query.rs` | `compile_fail/ocpq_missing_scope_rejected.rs` | `ocpq_missing_scope_rejected.stderr` |
| `petri_place_to_place_arc` — P→P arcs are not lawful in Petri nets | `petri::PlaceToTransitionArc` / `petri::TransitionToPlaceArc` | `compile_pass/petri_place_to_transition_arc.rs` | `compile_fail/petri_place_to_place_arc.rs` | `petri_place_to_place_arc.stderr` |
| `petri_transition_to_transition_arc` — T→T arcs are not lawful | `petri::PlaceToTransitionArc` / `petri::TransitionToPlaceArc` | `compile_pass/petri_transition_to_place_arc.rs` | `compile_fail/petri_transition_to_transition_arc.rs` | `petri_transition_to_transition_arc.stderr` |
| `powl_order_edge_choice_confusion` — `OrderEdge` and `ChoiceGraphEdge` are distinct | `powl::OrderEdge` / `powl::ChoiceGraphEdge` | `compile_pass/powl_choice_graph.rs` | `compile_fail/powl_order_edge_choice_confusion.rs` | `powl_order_edge_choice_confusion.stderr` |
| `powl_silent_tree_projection` — `ExceedsProcessTree` cannot project to tree | `powl::TreeProjectable` (sealed) | `compile_pass/powl_process_tree_projectable.rs` | `compile_fail/powl_silent_tree_projection.rs` | `powl_silent_tree_projection.stderr` |
| `process_tree_bad_loop_arity` — loop node arity must be exactly 2 | `process_tree::TypedLoopNode<ARITY>` with `Require<{ ARITY == 2 }>` | `compile_pass/process_tree_loop_arity_2.rs` | `compile_fail/process_tree_bad_loop_arity.rs` | `process_tree_bad_loop_arity.stderr` |
| `raw_export_as_admitted` — `Evidence<T, Raw, W>` cannot substitute `Admitted` | `evidence::Evidence<T, State, W>` typestate | `compile_pass/evidence_admitted_construction.rs` | `compile_fail/raw_export_as_admitted.rs` | `raw_export_as_admitted.stderr` |
| `receipt_missing_witness` — receipt requires a named witness | `receipt::Receipt<W>` | `compile_pass/receipt_shape.rs` | `compile_fail/receipt_missing_witness.rs` | `receipt_missing_witness.stderr` |
| `separable_wfnet_rejected` — plain `WfNetConst` not accepted where `SeparableWfNet` required | `petri::SeparableWfNet` | `compile_pass/separable_wfnet_marker.rs` | `compile_fail/separable_wfnet_rejected.rs` | `separable_wfnet_rejected.stderr` |
| `strict_claim_no_fixture` — strict export boundary claim requires witness | `strict::ExportBoundaryConst<HAS_WITNESS, HAS_ROUND_TRIP>` | `compile_pass/strict_export_boundary_with_fixture.rs` | `compile_fail/strict_claim_no_fixture.rs` | `strict_claim_no_fixture.stderr` |
| `wfnet_forged_soundness` — soundness witness cannot be forged | `petri::WfNetConst<SOUNDNESS>` non-forgeable witness path | `compile_pass/wfnet_with_soundness_witness.rs` | `compile_fail/wfnet_forged_soundness.rs` | `wfnet_forged_soundness.stderr` |
| `wfnet2powl_precondition_rejected` — WF-net→POWL requires separability marker | `petri::SeparableWfNet` precondition | `compile_pass/wfnet2powl_witness.rs` | `compile_fail/wfnet2powl_precondition_rejected.rs` | `wfnet2powl_precondition_rejected.stderr` |
| `wfnet2powl_wrong_source` — plain `PetriNet` cannot enter WF-net→POWL gate | `petri::WfNetConst` required | `compile_pass/wfnet2powl_witness.rs` | `compile_fail/wfnet2powl_wrong_source.rs` | `wfnet2powl_wrong_source.stderr` |
| `workflow_pattern_wrong_kind` — wrong `WorkflowPatternKind` const param rejected | `law::WorkflowPatternKind` (`ConstParamTy`) | `compile_pass/workflow_pattern_const_param.rs` | `compile_fail/workflow_pattern_wrong_kind.rs` | `workflow_pattern_wrong_kind.stderr` |
| `xes_not_object_centric` — XES case-centric log is not OCED | `xes::XesCaseCentricLog` (sealed distinction) | `compile_pass/xes_case_centric_log.rs` | `compile_fail/xes_not_object_centric.rs` | `xes_not_object_centric.stderr` |
| `xes_to_oced_loss_report_rejected` — XES→OCED export requires `LossReport` | `loss::LossReport` / `formats::LossyFormatExport` | `compile_pass/xes_to_oced_named_projection.rs` | `compile_fail/xes_to_oced_loss_report_rejected.rs` | `xes_to_oced_loss_report_rejected.stderr` |
| `xes_undeclared_extension_prefix_rejected` — XES law violations type as `XesRefusal` | `xes::XesRefusal::UndeclaredExtensionPrefix` | `compile_pass/xes_declared_extension_prefix.rs` | `compile_fail/xes_undeclared_extension_prefix_rejected.rs` | `xes_undeclared_extension_prefix_rejected.stderr` |
| `yawl_cancellation_region_rejected` — raw `Vec<String>` not accepted as `CancellationRegion` | `yawl::CancellationRegion` newtype | `compile_pass/yawl_cancellation_region.rs` | `compile_fail/yawl_cancellation_region_rejected.rs` | `yawl_cancellation_region_rejected.stderr` |
| `yawl_multi_instance_bounds_rejected` — `MultipleInstanceSpecConst<MIN, MAX>` enforces MIN ≤ MAX | `yawl::MultipleInstanceSpecConst<MIN, MAX>` | `compile_pass/yawl_multi_instance.rs` | `compile_fail/yawl_multi_instance_bounds_rejected.rs` | `yawl_multi_instance_bounds_rejected.stderr` |
| `yawl_wrong_task_type` — `MultipleInstanceSpecConst` not accepted where `CancellationRegion` required | `yawl::CancellationRegion` / `yawl::MultipleInstanceSpecConst` (distinct) | `compile_pass/yawl_cancellation_region.rs` | `compile_fail/yawl_wrong_task_type.rs` | `yawl_wrong_task_type.stderr` |

---

---

## #21 — No AI Without PI! (van der Aalst, 2025)

**Paper:** No AI Without PI! Object-Centric Process Mining as the Enabler
for Generative, Predictive, and Prescriptive Artificial Intelligence
(arXiv:2508.00116)

**Canon family:** `OBJECT_CENTRIC_PETRI`

**Graduation boundary:**

This crate provides the zero-cost structural surface that grounds all three
AI forms described in the paper:

| Paper concept | This crate's surface | Graduates to wasm4pm |
|---|---|---|
| Object-centric event data (OCED) | `src/ocel.rs` — `OcelLog`, `OcelObject`, `OcelEvent` | — (structure stays here) |
| Process discovery output | `src/petri.rs`, `src/powl.rs`, `src/dfg.rs` — typed net/graph surfaces | Discovery algorithm |
| Compliance analysis | `src/conformance.rs` — `Metric<KIND, NUM, DEN>` with `Between01` bounds | Alignment/replay engine |
| Performance analysis | `src/dfg.rs` — OC-DFG structure | Frequency/time annotation engine |
| Predictive AI input | `src/prediction.rs` — `PredictionTarget` prefix structure | ML model training/inference |
| Prescriptive AI | `src/prediction.rs` + `src/conformance.rs` | Recommendation/intervention engine |

**Structural law this crate enforces:**

- OCED tuple `(E, O, eaval, oaval)` is the only valid evidence carrier for
  all three AI forms. Evidence that cannot be traced to a lawful OCED
  structure is not process intelligence — it is a defect.
- Compliance score is a `Between01<NUM, DEN>` metric, not a free float.
  A compliance "score" that escapes the unit interval is a type error.
- Predictive AI prefix is typed over `Evidence<T, Admitted, W>` — a prefix
  that has not passed admission is not a lawful prediction input.

**What must NOT live in this crate:**

- Discovery algorithms (inductive miner, split miner, etc.)
- Alignment computation or token replay
- Performance frequency/time annotation
- ML model training or inference
- Recommendation generation

These graduate to wasm4pm via the `wasm4pm` feature bridge.
