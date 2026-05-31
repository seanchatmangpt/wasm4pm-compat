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
| `place_marker_law` — `Place` is a distinct named token-holding location; not interchangeable with `Transition` | `petri::Place` (newtype over id) | `compile_pass/petri_place_to_transition_arc.rs` | — | — |
| `transition_marker_law` — `Transition` is a distinct firing element; `is_silent()` distinguishes silent (tau) from labeled transitions at the API level | `petri::Transition` (newtype with label) | `compile_pass/petri_transition_to_place_arc.rs` | — | — |
| `marking_shape_law` — `Marking` is the token-count snapshot; WF-net requires a non-empty initial and a non-empty final marking or is refused with named law | `petri::Marking` / `PetriRefusal::MissingInitialMarking` / `PetriRefusal::MissingFinalMarking` | `compile_pass/refusal_missing_final_marking.rs` | — | — |
| `absence_of_dead_transitions_law` — a dead transition (one that can never fire from any reachable marking) is a named soundness defect; `PetriRefusal::DeadTransition` is the typed boundary law | `petri::PetriRefusal::DeadTransition` (named refusal variant) | `compile_pass/wfnet_with_soundness_witness.rs` | — | — |
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
| `bipartite_arc_sealed_trait_law` — only `PlaceToTransitionArc` and `TransitionToPlaceArc` implement `IsValidArc`; the sealed trait prevents external types from forging new arc directions | `petri::IsValidArc` (sealed trait) | `compile_pass/petri_place_to_transition_arc.rs` | — | — |
| `petri_place_to_place_arc` — P→P arcs are not lawful in Petri nets | `petri::PlaceToTransitionArc` / `petri::TransitionToPlaceArc` | `compile_pass/petri_place_to_transition_arc.rs` | `compile_fail/petri_place_to_place_arc.rs` | `petri_place_to_place_arc.stderr` |
| `petri_transition_to_transition_arc` — T→T arcs are not lawful | `petri::PlaceToTransitionArc` / `petri::TransitionToPlaceArc` | `compile_pass/petri_transition_to_place_arc.rs` | `compile_fail/petri_transition_to_transition_arc.rs` | `petri_transition_to_transition_arc.stderr` |
| `powl_order_edge_choice_confusion` — `OrderEdge` and `ChoiceGraphEdge` are distinct | `powl::OrderEdge` / `powl::ChoiceGraphEdge` | `compile_pass/powl_choice_graph.rs` | `compile_fail/powl_order_edge_choice_confusion.rs` | `powl_order_edge_choice_confusion.stderr` |
| `powl_silent_tree_projection` — `ExceedsProcessTree` cannot project to tree | `powl::TreeProjectable` (sealed) | `compile_pass/powl_process_tree_projectable.rs` | `compile_fail/powl_silent_tree_projection.rs` | `powl_silent_tree_projection.stderr` |
| `process_tree_bad_loop_arity` — loop node arity must be exactly 2 | `process_tree::TypedLoopNode<ARITY>` with `Require<{ ARITY == 2 }>` | `compile_pass/process_tree_loop_arity_2.rs` | `compile_fail/process_tree_bad_loop_arity.rs` | `process_tree_bad_loop_arity.stderr` |
| `raw_export_as_admitted` — `Evidence<T, Raw, W>` cannot substitute `Admitted` | `evidence::Evidence<T, State, W>` typestate | `compile_pass/evidence_admitted_construction.rs` | `compile_fail/raw_export_as_admitted.rs` | `raw_export_as_admitted.stderr` |
| `receipt_missing_witness` — receipt requires a named witness | `receipt::Receipt<W>` | `compile_pass/receipt_shape.rs` | `compile_fail/receipt_missing_witness.rs` | `receipt_missing_witness.stderr` |
| `separable_wfnet_rejected` — plain `WfNetConst` not accepted where `SeparableWfNet` required | `petri::SeparableWfNet` | `compile_pass/separable_wfnet_marker.rs` | `compile_fail/separable_wfnet_rejected.rs` | `separable_wfnet_rejected.stderr` |
| `strict_claim_no_fixture` — strict export boundary claim requires witness | `strict::ExportBoundaryConst<HAS_WITNESS, HAS_ROUND_TRIP>` | `compile_pass/strict_export_boundary_with_fixture.rs` | `compile_fail/strict_claim_no_fixture.rs` | `strict_claim_no_fixture.stderr` |
| `soundness_witness_state_law` — the three soundness states (`SoundnessUnknown`, `SoundnessClaimed`, `SoundnessWitnessed`) are uninhabited empty enums used as `PhantomData` tokens; advancing from `Claimed` to `Witnessed` requires a `SoundnessProof` that is only constructible inside the `petri` module | `petri::SoundnessUnknown` / `petri::SoundnessClaimed` / `petri::SoundnessWitnessed` | `compile_pass/wfnet_with_soundness_witness.rs` | `compile_fail/wfnet_forged_soundness.rs` | `wfnet_forged_soundness.stderr` |
| `wfnet_const_generic_state_law` — `WfNetConst<SOUNDNESS>` encodes soundness as a const generic `SoundnessState` parameter; `WfNetConst<{SoundnessState::Witnessed}>` and `WfNetConst<{SoundnessState::Unknown}>` are distinct types that cannot be substituted for each other | `petri::WfNetConst<SOUNDNESS>` (const-generic struct) | `compile_pass/wfnet_with_soundness_witness.rs` | `compile_fail/wfnet_forged_soundness.rs` | `wfnet_forged_soundness.stderr` |
| `wfnet_forged_soundness` — soundness witness cannot be forged | `petri::WfNetConst<SOUNDNESS>` non-forgeable witness path | `compile_pass/wfnet_with_soundness_witness.rs` | `compile_fail/wfnet_forged_soundness.rs` | `wfnet_forged_soundness.stderr` |
| `wfnet2powl_precondition_rejected` — WF-net→POWL requires separability marker | `petri::SeparableWfNet` precondition | `compile_pass/wfnet2powl_witness.rs` | `compile_fail/wfnet2powl_precondition_rejected.rs` | `wfnet2powl_precondition_rejected.stderr` |
| `wfnet2powl_wrong_source` — plain `PetriNet` cannot enter WF-net→POWL gate | `petri::WfNetConst` required | `compile_pass/wfnet2powl_witness.rs` | `compile_fail/wfnet2powl_wrong_source.rs` | `wfnet2powl_wrong_source.stderr` |
| `workflow_pattern_wrong_kind` — wrong `WorkflowPatternKind` const param rejected | `law::WorkflowPatternKind` (`ConstParamTy`) | `compile_pass/workflow_pattern_const_param.rs` | `compile_fail/workflow_pattern_wrong_kind.rs` | `workflow_pattern_wrong_kind.stderr` |
| `xes_not_object_centric` — XES case-centric log is not OCED | `xes::XesCaseCentricLog` (sealed distinction) | `compile_pass/xes_case_centric_log.rs` | `compile_fail/xes_not_object_centric.rs` | `xes_not_object_centric.stderr` |
| `xes_to_oced_loss_report_rejected` — XES→OCED export requires `LossReport` | `loss::LossReport` / `formats::LossyFormatExport` | `compile_pass/xes_to_oced_named_projection.rs` | `compile_fail/xes_to_oced_loss_report_rejected.rs` | `xes_to_oced_loss_report_rejected.stderr` |
| `xes_undeclared_extension_prefix_rejected` — XES law violations type as `XesRefusal` | `xes::XesRefusal::UndeclaredExtensionPrefix` | `compile_pass/xes_declared_extension_prefix.rs` | `compile_fail/xes_undeclared_extension_prefix_rejected.rs` | `xes_undeclared_extension_prefix_rejected.stderr` |
| `declare_response_constraint` — `DeclareTemplate::Response` is a first-class binary template, not a string | `declare::DeclareTemplate::Response` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `declare_precedence_constraint` — `DeclareTemplate::Precedence` is a first-class binary template | `declare::DeclareTemplate::Precedence` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `declare_succession_constraint` — `DeclareTemplate::Succession` = Response ∧ Precedence; enforced as a single typed template | `declare::DeclareTemplate::Succession` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `declare_absence_constraint` — `DeclareTemplate::Absence` is a unary template; passing it as binary is a structural defect | `declare::DeclareTemplate::Absence` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `declare_existence_constraint` — `DeclareTemplate::Existence` is a unary template; arity mismatch is refused at the structural level | `declare::DeclareTemplate::Existence` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `declare_object_scoped_witness` — OC-Declare scope (`DeclareScope`) is a first-class typed scope, not an optional annotation | `declare::DeclareScope` / `witness::DeclareFamily` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `ocpq_object_scope_law` — `ObjectScope` with zero object types is refused as `OcpqRefusal::MissingObjectScope` | `ocpq::ObjectScope` / `ocpq::OcpqRefusal::MissingObjectScope` | `compile_pass/ocpq_scoped_query.rs` | `compile_fail/ocpq_missing_scope_rejected.rs` | `ocpq_missing_scope_rejected.stderr` |
| `ocpq_event_predicate_law` — `Predicate<EventPredicate>` is a first-class typed predicate; `EventPredicate` cannot substitute `ObjectPredicate` | `ocpq::Predicate<EventPredicate>` / `ocpq::EventPredicate` | `compile_pass/ocpq_scoped_query.rs` | `compile_fail/ocpq_object_type_mixing.rs` | `ocpq_object_type_mixing.stderr` |
| `ocpq_relation_predicate_law` — `Predicate<RelationPredicate>` with `E2ORelation` and `O2ORelation` variants enforces typed link-kind distinction at the structural level | `ocpq::Predicate<RelationPredicate>` / `ocpq::PredicateKind::E2ORelation` / `ocpq::PredicateKind::O2ORelation` | `compile_pass/ocpq_typed_relation.rs` | `compile_fail/ocpq_object_type_mixing.rs` | `ocpq_object_type_mixing.stderr` |
| `ocpq_cardinality_bound_law` — `Predicate<CardinalityPredicate>` with `Cardinality { min, max }` requires `min <= max`; violated bound is refused as `OcpqRefusal::InvalidCardinality` | `ocpq::Predicate<CardinalityPredicate>` / `ocpq::OcpqRefusal::InvalidCardinality` | `compile_pass/ocpq_cardinality_valid_bounds.rs` | `compile_fail/ocpq_cardinality_rejected.rs` | `ocpq_cardinality_rejected.stderr` |
| `ocpq_typed_child_set_law` — `PredicateKind::ChildSetBound` requires a non-empty `branch_label` and `min <= max`; violated is refused as `OcpqRefusal::InvalidChildSetBound` | `ocpq::PredicateKind::ChildSetBound` / `ocpq::OcpqRefusal::InvalidChildSetBound` | `compile_pass/ocpq_cbs_predicate.rs` | `compile_fail/ocpq_cardinality_overflow.rs` | `ocpq_cardinality_overflow.stderr` |
| `ocel_v1_e2o_required` — OCEL 1.0 event-to-object link is a first-class structural law (not optional annotation) | `ocel::EventObjectLink` / `ocel::OcelEvent` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | `ocel_e2o_missing_link.stderr` |
| `yawl_cancellation_region_rejected` — raw `Vec<String>` not accepted as `CancellationRegion` | `yawl::CancellationRegion` newtype | `compile_pass/yawl_cancellation_region.rs` | `compile_fail/yawl_cancellation_region_rejected.rs` | `yawl_cancellation_region_rejected.stderr` |
| `yawl_multi_instance_bounds_rejected` — `MultipleInstanceSpecConst<MIN, MAX>` enforces MIN ≤ MAX | `yawl::MultipleInstanceSpecConst<MIN, MAX>` | `compile_pass/yawl_multi_instance.rs` | `compile_fail/yawl_multi_instance_bounds_rejected.rs` | `yawl_multi_instance_bounds_rejected.stderr` |
| `yawl_wrong_task_type` — `MultipleInstanceSpecConst` not accepted where `CancellationRegion` required | `yawl::CancellationRegion` / `yawl::MultipleInstanceSpecConst` (distinct) | `compile_pass/yawl_cancellation_region.rs` | `compile_fail/yawl_wrong_task_type.rs` | `yawl_wrong_task_type.stderr` |
| `xes_to_oced_without_loss_policy` — `FormatExport` (optional loss) does not satisfy the XES→OCED mandatory-report gate | `formats::LossyFormatExport` (mandatory) vs `formats::FormatExport` (optional) | `compile_pass/xes_to_oced_named_projection.rs` | `compile_fail/xes_to_oced_without_loss_policy.rs` | `xes_to_oced_without_loss_policy.stderr` |
| `xes_trace_attribute_shape` — `XesTrace` carries an ordered sequence of `XesEvent`s; attribute lookup is by namespaced key | `xes::XesTrace` / `xes::XesEvent` | `compile_pass/xes_trace_attributes.rs` | — (structural law, no negative path) | — |
| `xes_event_attribute_witness` — `XesEvent` exposes standard keys (`concept:name`, `time:timestamp`, `org:resource`) via typed helpers; arbitrary keys via `attribute()` | `xes::XesEvent` | `compile_pass/xes_trace_attributes.rs` | — (structural law) | — |
| `xes_lifecycle_transition_witness` — `XesRefusal::InvalidLifecycleTransition` is the named refusal for `lifecycle:transition` values outside the declared alphabet | `xes::XesRefusal::InvalidLifecycleTransition` | `compile_pass/xes_declared_extension_prefix.rs` | — (refusal variant; compile-fail fixture TBD) | — |
| `xes_to_ocel_direction_law` — XES→OCEL lifting is distinct from XES→OCED: OCEL adds typed object types and E2O links absent from a flat XES log; the lifting requires a `LossReport` naming inferred object-type assumptions | `xes::XesLog` → `ocel::OcelLog` via `formats::LossyFormatExport` | `compile_pass/xes_case_centric_log.rs` | `compile_fail/xes_not_object_centric.rs` | `xes_not_object_centric.stderr` |
| `xes_missing_concept_name` — an `XesEvent` lacking `concept:name` is refused as `XesRefusal::MissingConceptName`; a structural exchange law | `xes::XesRefusal::MissingConceptName` | `compile_pass/xes_case_centric_log.rs` | — (runtime refusal; no compile-fail for stringly-typed attribute bag) | — |
| `xes_missing_trace_name` — a `XesTrace` lacking `concept:name` (case id) is refused as `XesRefusal::MissingTraceName` | `xes::XesRefusal::MissingTraceName` | `compile_pass/xes_trace_attributes.rs` | — (runtime refusal path) | — |
| `xes_empty_trace` — a `XesTrace` with zero events is refused as `XesRefusal::EmptyTrace` | `xes::XesRefusal::EmptyTrace` | `compile_pass/xes_trace_attributes.rs` | — (runtime refusal path) | — |
| `xes_invalid_extension` — an `XesExtension` with an empty prefix is refused as `XesRefusal::InvalidExtension` | `xes::XesRefusal::InvalidExtension` | `compile_pass/xes_declared_extension_prefix.rs` | — (runtime refusal path) | — |
| `xes_no_traces` — a `XesLog` with no traces is refused as `XesRefusal::NoTraces` | `xes::XesRefusal::NoTraces` | `compile_pass/xes_case_centric_log.rs` | — (runtime refusal path) | — |
| `ocel_missing_object_law` — `OcelLog` with no declared objects is refused as `OcelRefusal::MissingObject` | `ocel::OcelLog` / `OcelRefusal::MissingObject` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | `ocel_e2o_missing_link.stderr` |
| `ocel_missing_event_law` — `OcelLog` with no declared events is refused as `OcelRefusal::MissingEvent` | `ocel::OcelLog` / `OcelRefusal::MissingEvent` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | `ocel_e2o_missing_link.stderr` |
| `ocel_duplicate_object_id_law` — two objects with the same id are refused as `OcelRefusal::DuplicateObjectId` | `ocel::OcelLog` / `OcelRefusal::DuplicateObjectId` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | `ocel_e2o_missing_link.stderr` |
| `ocel_duplicate_event_id_law` — two events with the same id are refused as `OcelRefusal::DuplicateEventId` | `ocel::OcelLog` / `OcelRefusal::DuplicateEventId` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | `ocel_e2o_missing_link.stderr` |
| `ocel_object_change_invalidated_law` — `ObjectChange` referencing undeclared object or empty attribute is refused as `OcelRefusal::InvalidObjectChange` | `ocel::ObjectChange` / `OcelRefusal::InvalidObjectChange` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | `ocel_e2o_missing_link.stderr` |
| `ocel_missing_object_type_law` — object with empty type string is refused as `OcelRefusal::MissingObjectType` | `ocel::OcelObject` / `OcelRefusal::MissingObjectType` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | `ocel_e2o_missing_link.stderr` |
| `ocel_flattening_loss_refusal` — OCEL flattening to single case notion is named `OcelRefusal::FlatteningLoss`; requires `LossReport` | `ocel::OcelRefusal::FlatteningLoss` / `formats::LossyFormatExport` | `compile_pass/ocel_to_xes_with_named_projection.rs` | `compile_fail/ocel_to_xes_no_loss_report.rs` | `ocel_to_xes_no_loss_report.stderr` |
| `ocel_shape_marker_law` — `OcelShape` and `XesShape` are distinct uninhabited markers; `LossReport<OcelShape, XesShape, _>` cannot be assembled in the wrong direction | `interop::OcelShape` / `interop::XesShape` | `compile_pass/ocel_to_xes_with_named_projection.rs` | — (law enforced by uninhabited type parameter direction) | — |
| `ocel_dims_vocabulary_law` — `OcelDims` is a first-class named vocabulary surface, distinct from `OcelLog`; not a free `Vec<String>` | `ocel::OcelDims` (distinct type from `OcelLog`) | `compile_pass/ocel_event_object_relation.rs` | — (structural type distinction) | — |
| `ocel_attribute_typed_value_law` — `OcelAttributeValue` is a named enum; attribute values are not free strings | `ocel::OcelAttributeValue` (enum) | `compile_pass/ocel_event_object_relation.rs` | — (law enforced by enum exhaustiveness) | — |
| `partial-order-law` — `OrderEdge` and `PowlNodeKind::PartialOrder` are first-class structural types; a partial-order node carries a DAG of typed precedence edges, never a free list | `powl::OrderEdge` / `powl::PowlNodeKind::PartialOrder` | `compile_pass/powl_choice_graph.rs` | — | — |
| `acyclicity-marker-law` — a partial-order node containing a cycle is refused as `PowlRefusal::CyclicPartialOrder`; acyclicity is a named structural law, not a runtime assertion | `powl::PowlRefusal::CyclicPartialOrder` | `compile_pass/powl_choice_graph.rs` | — | — |
| `choice-minimum-branch-law` — a `Choice` node requires at least two branches; `TypedNode<{PowlKind::Xor}>::min_branches()` returns 2 as a compile-time constant | `powl::PowlNodeKind::Choice` / `nightly_foundry::powl_law::TypedNode<{PowlKind::Xor}>` | `compile_pass/powl_choice_graph.rs` | `compile_fail/powl_order_edge_choice_confusion.rs` | `powl_order_edge_choice_confusion.stderr` |
| `loop-marker-law` — a POWL loop node (`PowlNodeKind::Loop`) is a first-class structural kind; `PowlRefusal::InvalidLoop` is the named refusal when the loop body is absent | `powl::PowlNodeKind::Loop` / `powl::PowlRefusal::InvalidLoop` | `compile_pass/process_tree_loop_arity_2.rs` | `compile_fail/process_tree_bad_loop_arity.rs` | `process_tree_bad_loop_arity.stderr` |
| `irreducible-state-law` — `Irreducible` and `ExceedsProcessTree` are first-class witness markers; a node tagged `ExceedsProcessTree` cannot pass through a `TreeProjectable`-gated function | `powl::Irreducible` / `powl::ExceedsProcessTree` | `compile_pass/powl_process_tree_projectable.rs` | `compile_fail/powl_silent_tree_projection.rs` | `powl_silent_tree_projection.stderr` |
| `projection-refusal-law` — `PowlRefusal::IrreducibleProjection` is the named law when an `Irreducible` partial order is asked to project to a process tree; `ProcessTreeRefusal::UnsupportedProjection` is the mirror law on the tree side | `powl::PowlRefusal::IrreducibleProjection` / `process_tree::ProcessTreeRefusal::UnsupportedProjection` | `compile_pass/powl_process_tree_projectable.rs` | `compile_fail/powl_silent_tree_projection.rs` | `powl_silent_tree_projection.stderr` |
| `powl-to-process-tree-boundary` — `WfNet2PowlWitness` is non-forgeable; `TreeProjectable` is sealed to `ProcessTreeProjectable` only; the POWL-to-process-tree boundary requires both markers | `powl::WfNet2PowlWitness` / `powl::TreeProjectable` (sealed) | `compile_pass/wfnet2powl_witness.rs` | `compile_fail/powl_silent_tree_projection.rs` | `powl_silent_tree_projection.stderr` |
| `silent-node-law` — `SilentTransition` / `PowlNodeKind::Silent` is a first-class POWL node kind distinct from an activity atom; `TypedNode<{PowlKind::Silent}>::is_observable()` returns `false` at compile time | `powl::PowlNodeKind::Silent` / `nightly_foundry::powl_law::TypedNode<{PowlKind::Silent}>` | `compile_pass/powl_process_tree_projectable.rs` | `compile_fail/powl_silent_tree_projection.rs` | `powl_silent_tree_projection.stderr` |
| `process-tree-loop-arity-law` — `TypedLoopNode<Children, ARITY>` enforces `ARITY == 2` via `Require<{ ARITY == 2 }>: IsTrue`; a loop node with arity 3 does not compile | `process_tree::TypedLoopNode<Children, ARITY>` | `compile_pass/process_tree_loop_arity_2.rs` | `compile_fail/process_tree_bad_loop_arity.rs` | `process_tree_bad_loop_arity.stderr` |
| `process-tree-operator-law` — `ProcessTreeOperator` is a closed enum of five structural operators; an operator node without a declared kind is structurally ill-formed | `process_tree::ProcessTreeOperator` | `compile_pass/process_tree_operator_node_shape.rs` | — | — |
| `powl-choice-graph-connectivity-law` — a `ChoiceGraph` node whose edges leave any node disconnected from the path start-to-end is refused as `PowlRefusal::ChoiceGraphDisconnected`; the connectivity law is named, not a bare runtime error | `powl::PowlNodeKind::ChoiceGraph` / `powl::PowlRefusal::ChoiceGraphDisconnected` | `compile_pass/powl_choice_graph.rs` | — | — |

---

---

## Petri-Law Family Law Packet

The petri-law family covers all structural laws derived from Petri net, WF-net, and OC-Petri-net theory. The following table groups the petri-law concepts with their enforcing types, fixtures, and paper sources.

| Law Concept | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| bipartite-arc-law | `petri::PlaceToTransitionArc` / `petri::TransitionToPlaceArc` | `petri_place_to_transition_arc.rs` | `petri_place_to_place_arc.rs` | Murata (1989) §2 |
| place-marker-law | `petri::Place` | `petri_place_to_transition_arc.rs` | — | Murata (1989) §2 |
| transition-marker-law | `petri::Transition` | `petri_transition_to_place_arc.rs` | — | Murata (1989) §2 |
| marking-shape-law | `petri::Marking` / `PetriRefusal::MissingInitialMarking` / `MissingFinalMarking` | `refusal_missing_final_marking.rs` | — | van der Aalst (1998) §2 |
| soundness-witness-state | `petri::SoundnessUnknown` / `SoundnessClaimed` / `SoundnessWitnessed` | `wfnet_with_soundness_witness.rs` | `wfnet_forged_soundness.rs` | van der Aalst (1998) §3 |
| separable-wfnet-law | `petri::SeparableWfNet` | `separable_wfnet_marker.rs` | `separable_wfnet_rejected.rs` | Kourani et al. (2026) Def 4.1 |
| WfNet-to-POWL-witness | `petri::SeparableWfNet` precondition | `wfnet2powl_witness.rs` | `wfnet2powl_precondition_rejected.rs` / `wfnet2powl_wrong_source.rs` | Kourani et al. (2026) Thm 4.3 |
| absence-of-dead-transitions-law | `petri::PetriRefusal::DeadTransition` | `wfnet_with_soundness_witness.rs` | — | van der Aalst (1998) §3 |

---

## #33 — Petri Nets: Properties, Analysis and Applications (Murata, 1989)

**Paper:** Petri Nets: Properties, Analysis and Applications  
**Canon family:** `PETRI_NETS`  
**Verdict:** `COVERED_BY_TYPE`

**Law-packet notes:**

Murata (1989) §2 defines the foundational Petri net laws: the bipartite arc
structure (no P→P or T→T arcs), the incidence matrix formulation (W-/W+
pre/post condition matrices), and the enabling condition (all input places
marked before firing).

| Paper formal object | Rust surface | Enforcing law |
|---|---|---|
| Place-to-Transition arc | `src/petri.rs::PlaceToTransitionArc` | `petri_place_to_place_arc` compile-fail |
| Transition-to-Place arc | `src/petri.rs::TransitionToPlaceArc` | `petri_transition_to_transition_arc` compile-fail |
| Incidence matrix W- (pre) | `src/petri.rs::IncidenceMatrix` W- | structure-only type |
| Incidence matrix W+ (post) | `src/petri.rs::IncidenceMatrix` W+ | structure-only type |
| Enabling condition | `src/petri.rs` (const-generic predicate) | — |
| Petri net law surface | `src/nightly_foundry.rs::petri_law` | cites Murata (1989) §2 |
| Token law surface | `src/nightly_foundry.rs::token_law` | cites Murata (1989) §2 |

**Structural laws this crate enforces:**

- A P→P arc is not lawful. The `petri_place_to_place_arc` compile-fail
  fixture seals this from Murata (1989) §2.
- A T→T arc is not lawful. The `petri_transition_to_transition_arc`
  compile-fail fixture seals this from Murata (1989) §2.
- The `petri_law` and `token_law` surfaces in `src/nightly_foundry.rs`
  directly cite Murata (1989) §2 as the authoritative formal grounding
  for all Petri net type laws in this crate.

**What must NOT live in this crate:**

- Reachability analysis (state space enumeration)
- Liveness and boundedness checking algorithms
- Coverability graph computation
- Simulation or firing sequence generation

---

## #34 — The Application of Petri Nets to Workflow Management (van der Aalst, 1998)

**Paper:** The Application of Petri Nets to Workflow Management  
**Canon family:** `WF_NET_SOUNDNESS`  
**Verdict:** `COVERED_BY_TYPE`

**Law-packet notes:**

Van der Aalst (1998) defines the WF-net soundness criterion: a WF-net is
sound if and only if (1) option completeness — every reachable marking can
reach the final marking, (2) proper completion — the final marking is the
unique terminal state, and (3) no dead transitions — every transition is
reachable from the initial marking.

| Paper formal object | Rust surface | Enforcing law |
|---|---|---|
| WF-net (source/sink place) | `src/petri.rs::WfNetConst<SOUNDNESS>` | `wfnet_forged_soundness` compile-fail |
| Soundness witness (non-forgeable) | `src/petri.rs::WfNetSoundnessWitness` | constructor is `pub(crate)` |
| Soundness paper witness marker | `src/witness.rs::WfNetSoundnessPaper` | — |
| Soundness state | `src/petri.rs::SoundnessState` | — |

**Structural laws this crate enforces:**

- `WfNetConst<true>` (sound) cannot be forged — the `WfNetSoundnessWitness`
  constructor is `pub(crate)`. The `wfnet_forged_soundness` compile-fail
  fixture seals this: it is impossible to construct a sound WF-net
  without going through the lawful admission path.
- `WfNetSoundnessPaper` in `src/witness.rs` is the named receipt that
  a given `WfNetConst<SOUNDNESS>` derives its soundness claim from the
  van der Aalst (1998) criterion, not from an ad-hoc boolean flag.
- `WfNetConst<false>` (unsound) is a distinct type — passing an unsound
  net where a sound net is required is a compile error, not a runtime panic.

**What must NOT live in this crate:**

- WF-net soundness verification algorithm execution (WOFLAN reduction)
- WF-net to free-choice net transformation (structural reduction algorithm)
- Coverability graph construction for soundness analysis
- Dead-transition detection execution

---

## #35 — OCEL: A Standard for Object-Centric Event Logs (van der Aalst, Berti, 2020)

**Paper:** OCEL: A Standard for Object-Centric Event Logs  
**Canon family:** `OCEL_OBJECT_CENTRIC`  
**Verdict:** `COVERED_BY_TYPE`

**Law-packet notes:**

OCEL 1.0 (van der Aalst, Berti, 2020) defines the original object-centric
event log standard. The event-to-object link (E2O) is the foundational
structural novelty — unlike flat XES logs, OCEL events belong to multiple
objects simultaneously. OCEL 1.0 is the structural ancestor of OCEL 2.0
(which adds object-to-object links and a richer attribute model; see #25).

| Paper formal object | Rust surface | Enforcing law |
|---|---|---|
| Object-centric event log | `src/ocel.rs::OcelLog` | — |
| Event (belongs to multiple objects) | `src/ocel.rs::OcelEvent` | `ocel_e2o_missing_link` compile-fail |
| Object | `src/ocel.rs::OcelObject` | — |
| Event-to-object link | `src/ocel.rs::EventObjectLink` | `ocel_e2o_missing_link` compile-fail |
| Object type, event type | `src/ocel.rs` (typed fields) | — |
| Witness covering both versions | `src/witness.rs::Ocel20` | subsumes OCEL 1.0 and 2.0 |

**Structural laws this crate enforces:**

- An `OcelEvent` without a declared `EventObjectLink` is a type error.
  The `ocel_e2o_missing_link` compile-fail fixture seals this for both
  OCEL 1.0 and OCEL 2.0.
- `Ocel20` in `src/witness.rs` is the named witness for both OCEL versions;
  OCEL 1.0 structures are a strict subset of OCEL 2.0.
- OCEL 1.0 does not flatten to XES without a `LossReport` — object
  multiplicity is lost in the flattening. The `ocel_to_xes_no_loss_report`
  compile-fail fixture seals this.

**What must NOT live in this crate:**

- OCEL JSON/XML/SQLite wire format parsing
- Object-type inference from raw data
- OCEL discovery algorithm execution

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

---

## #25 — OCEL 2.0 Specification (van der Aalst et al., 2023)

**Paper:** OCEL 2.0 Specification  
**Canon family:** `OCEL_OBJECT_CENTRIC`  
**Verdict:** `COVERED_BY_TYPE`

**Law-packet notes:**

The OCEL 2.0 formal data model defines object-centric event logs with two
first-class link types: event-to-object (E2O) and object-to-object (O2O).
Both are structural laws, not optional annotations.

| Spec formal object | Rust surface | Enforcing law |
|---|---|---|
| `OcelLog` | `src/ocel.rs::OcelLog` | — |
| `OcelEvent` | `src/ocel.rs::OcelEvent` | `ocel_e2o_missing_link` compile-fail |
| `OcelObject` | `src/ocel.rs::OcelObject` | `ocel_o2o_missing_link` compile-fail |
| `EventObjectLink` | `src/ocel.rs::EventObjectLink` | `ocel_e2o_missing_link` compile-fail |
| `ObjectObjectLink` | `src/ocel.rs::ObjectObjectLink` | `ocel_o2o_missing_link` compile-fail |
| `OcelDims` | `src/ocel.rs::OcelDims` | — |
| `OcelAttribute` | `src/ocel.rs::OcelAttribute` | — |

**Structural laws this crate enforces:**

- An `OcelEvent` without a declared `EventObjectLink` is a type error, not a
  runtime warning. The `ocel_e2o_missing_link` compile-fail fixture seals this.
- An `OcelObject` participating in an O2O relationship must declare the link
  via `ObjectObjectLink`. The `ocel_o2o_missing_link` compile-fail fixture
  seals this.
- OCEL 2.0 does not flatten to XES without a `LossReport`. The
  `ocel_to_xes_no_loss_report` compile-fail fixture seals this.

**What must NOT live in this crate:**

- OCEL parsing from JSON/XML wire formats (graduates to wasm4pm I/O layer)
- OCEL discovery algorithm execution (Alpha/Inductive over OCEL)
- Object-type inference from raw data

---

## #26 — XES IEEE Standard 1849-2023

**Paper:** XES IEEE Standard 1849-2023  
**Canon family:** `XES_EVENT_LOG`  
**Verdict:** `COVERED_BY_TYPE`

**Law-packet notes:**

XES is an IEEE standard (not a discovery algorithm) defining the formal
schema for classic flat event logs. The case-centric structure and extension
declaration requirement are first-class structural laws.

| Standard formal object | Rust surface | Enforcing law |
|---|---|---|
| `XesLog` | `src/xes.rs::XesLog` | — |
| `XesTrace` | `src/xes.rs::XesTrace` | — |
| `XesEvent` | `src/xes.rs::XesEvent` | — |
| Case-centric marker | `src/xes.rs::CaseCentricMarker` | `xes_not_object_centric` compile-fail |
| Extension declaration | `src/xes.rs::XesExtension` | `xes_undeclared_extension_prefix_rejected` compile-fail |

**Structural laws this crate enforces:**

- A `XesCaseCentricLog` cannot substitute an OCED/OCEL structure. The
  `xes_not_object_centric` compile-fail fixture seals this distinction.
- An XES attribute using an extension prefix that has not been declared
  in the log header is refused as `XesRefusal::UndeclaredExtensionPrefix`.
  The `xes_undeclared_extension_prefix_rejected` compile-fail fixture seals
  this.
- XES→OCED conversion requires a `LossReport` — object-to-object links
  present in OCED are structurally absent in XES. The
  `xes_to_oced_loss_report_rejected` compile-fail fixture seals this.

**What must NOT live in this crate:**

- XES file parsing (`.xes` / `.xes.gz` I/O graduates to wasm4pm)
- XES validator execution (checks beyond structure are runtime)
- XES extension semantic evaluation

---

## #28 — Declare/LTL Constraint Mining (Pesic, van der Aalst, 2006)

**Paper:** Declare: Full Support for Loosely-Structured Processes  
**Canon family:** `DECLARE_CONSTRAINTS`  
**Verdict:** `COVERED_BY_TYPE`

**Law-packet notes:**

The Declare constraint model defines named templates as first-class
structural laws. Each template (Existence, Absence, Response, Precedence,
etc.) is a distinct type — not a free string — and binary constraints
require at least 2 activity arguments.

| Paper formal object | Rust surface | Enforcing law |
|---|---|---|
| Constraint template | `src/declare.rs::DeclareTemplate` (`ConstParamTy`) | `declare_binary_arity_rejected` compile-fail |
| Constraint instance | `src/declare.rs::DeclareConstraint` | `declare_binary_arity_rejected` compile-fail |
| Constraint provenance | `src/declare.rs::DeclareWitness` | — |

**Structural laws this crate enforces:**

- A `DeclareConstraint` with arity < 2 is a compile error. The
  `declare_binary_arity_rejected` compile-fail fixture seals this.
- A `DeclareTemplate` is a `ConstParamTy` const-generic parameter — a
  constraint parameterized with one template cannot be silently substituted
  for one with a different template at the type level.
- `DeclareWitness` is a `PhantomData` proof that a constraint instance
  came from a named template, not from a free-form string.

**What must NOT live in this crate:**

- Declare constraint checking execution (LTL automaton evaluation)
- Constraint mining from event logs (log-driven template discovery)
- RuleML/Declare XML serialization I/O

---

## #31 — Object-Centric Petri Nets (van der Aalst, 2019)

**Paper:** Object-Centric Behavioral Constraints  
**Canon family:** `OBJECT_CENTRIC_PETRI`  
**Verdict:** `COVERED_BY_TYPE`

**Law-packet notes:**

OC-Petri nets extend standard Petri nets with object-centric arc
inscriptions (each arc carries an object type, enabling multi-instance
token semantics). The typed bipartite arc law is the foundational
structural compliance surface.

| Paper formal object | Rust surface | Enforcing law |
|---|---|---|
| Place-to-Transition arc | `src/petri.rs::PlaceToTransitionArc` | `petri_place_to_place_arc` compile-fail |
| Transition-to-Place arc | `src/petri.rs::TransitionToPlaceArc` | `petri_transition_to_transition_arc` compile-fail |
| WF-net soundness | `src/petri.rs::WfNetConst<SOUNDNESS>` | `wfnet_forged_soundness` compile-fail |
| Soundness state | `src/petri.rs::SoundnessState` | — |

**Structural laws this crate enforces:**

- A P→P arc is not lawful in a Petri net. The `petri_place_to_place_arc`
  compile-fail fixture seals this.
- A T→T arc is not lawful in a Petri net. The
  `petri_transition_to_transition_arc` compile-fail fixture seals this.
- `WfNetConst<true>` (sound) cannot be forged — the `WfNetSoundnessWitness`
  constructor is `pub(crate)`. The `wfnet_forged_soundness` compile-fail
  fixture seals this.
- OC-Petri net arc inscriptions (object-type markers) are structural —
  an arc without a declared object type is a structural gap, not a runtime
  default.

**What must NOT live in this crate:**

- OC-Petri net execution semantics (binding element evaluation)
- Object-centric token replay
- OC-Petri net discovery from OCEL logs
- Variable arc vs. regular arc execution distinction (runtime semantics)

---

## #43 — POWL: Partially Ordered Workflow Language (Kourani, van der Aalst, 2023)

**Paper:** POWL: Partially Ordered Workflow Language  
**Canon family:** `POWL`  
**Verdict:** `COVERED_BY_TYPE`

**Law-packet notes:**

POWL (Kourani & van der Aalst, 2023) defines four node kinds as first-class
structural laws: `StrictPartialOrder`, `OperatorNode`, `Transition`, and
`SilentTransition`. Each is a distinct type — not a free string and not an
annotation on a generic node. The choice graph edge (`ChoiceGraphEdge`) and
the partial-order edge (`OrderEdge`) are distinct newtypes; confusing them
is a compile error.

| Paper formal object | Rust surface | Enforcing law |
|---|---|---|
| `StrictPartialOrder` node kind | `src/powl.rs::PowlNodeKind::StrictPartialOrder` | — |
| `OperatorNode` (loop/choice/parallel) | `src/powl.rs::PowlNodeKind::OperatorNode` | — |
| `Transition` | `src/powl.rs::PowlNodeKind::Transition` | — |
| `SilentTransition` | `src/powl.rs::PowlNodeKind::SilentTransition` | `powl_silent_tree_projection` compile-fail |
| `ChoiceGraphEdge` | `src/powl.rs::ChoiceGraphEdge` | `powl_order_edge_choice_confusion` compile-fail |
| `OrderEdge` | `src/powl.rs::OrderEdge` | `powl_order_edge_choice_confusion` compile-fail |
| Tree-projectable POWL subclass | `src/powl.rs::TreeProjectable` (sealed) | `powl_silent_tree_projection` compile-fail |
| POWL paper witness | `src/witness.rs::PowlPaper` | — |
| POWL law surface | `src/nightly_foundry.rs::powl_law` | cites POWL paper |

**Structural laws this crate enforces:**

- `ChoiceGraphEdge` and `OrderEdge` are distinct newtypes — substituting one
  for the other is a compile error. The `powl_order_edge_choice_confusion`
  compile-fail fixture seals this.
- A POWL node that carries an `ExceedsProcessTree` marker (i.e., uses a
  `ChoiceGraph` sub-model with cycles or non-block structure) cannot project
  to a process tree. The `powl_silent_tree_projection` compile-fail fixture
  seals this via the `TreeProjectable` sealed trait.
- `SilentTransition` is a first-class POWL node kind — it is not an
  annotation on a `Transition`. The `PowlNodeKind` enum distinguishes them
  at the type level.
- `PowlPaper` in `src/witness.rs` is the non-forgeable receipt that a POWL
  structure derives its node-kind laws from the Kourani & van der Aalst (2023)
  definition, not from an ad-hoc enum.
- The `powl_law` surface in `src/nightly_foundry.rs` is the compile-time law
  kernel that directly cites the POWL paper as authoritative grounding.

**What must NOT live in this crate:**

- POWL discovery algorithm execution (inductive miner over POWL output shape)
- POWL → WF-net translation execution (structural reduction)
- POWL conformance checking (replay over partial-order models)
- POWL serialization/deserialization (PTML wire format I/O)

---

## #47 — BPMN 2.0 — Business Process Model and Notation (OMG Specification, 2011)

**Paper:** BPMN 2.0 — Business Process Model and Notation (OMG Specification)
**Canon family:** `WORKFLOW_PATTERNS_BPMN`
**Verdict:** `COVERED_BY_TYPE`

**Law-packet notes:**

The OMG BPMN 2.0 specification is the normative metamodel behind the
practical BPMN reference (#11 Real-Life BPMN). Each element class in the
OMG metamodel is a distinct structural type — not a free string annotation.
Gateway kinds (XOR/AND/OR) and event kinds (Start/Intermediate/End) are
first-class structural laws.

| Spec formal object | Rust surface | Enforcing law |
|---|---|---|
| `BpmnElement` (task, gateway, event, subprocess) | `src/bpmn.rs::BpmnElement` | — |
| `GatewayKind` (XOR/AND/OR) | `src/bpmn.rs::GatewayKind` | structural type distinction |
| `BpmnSubprocess` | `src/bpmn.rs::BpmnSubprocess` | — |
| `EventKind` (Start/Intermediate/End) | `src/bpmn.rs::EventKind` | structural type distinction |

**Structural laws this crate enforces:**

- `GatewayKind` is a typed enum — XOR, AND, and OR gateways are distinct
  structural types, not configuration strings. A function requiring an
  AND-join cannot silently accept an XOR-join.
- `EventKind` distinguishes Start, Intermediate, and End events at the
  structural level; connecting an End event as a source is a structural
  defect.
- The OMG BPMN 2.0 specification is the normative grounding for all
  gateway and event type distinctions in `src/bpmn.rs`.

**What must NOT live in this crate:**

- BPMN formal operational semantics (token-passing execution)
- BPMN 2.0 XML serialization/deserialization (`.bpmn` wire format I/O)
- BPMN simulation or process execution engine

---

## #48 — Multi-Perspective Process Mining (van der Aalst, 2011)

**Paper:** Multi-Perspective Process Mining
**Canon family:** `XES_EVENT_LOG`
**Verdict:** `PARTIAL_WITH_REASON`
**Active obligation:** `ResourcePerspective` and `DataPerspective` typed extension namespaces in `src/xes.rs`

**Law-packet notes:**

Van der Aalst (2011) multi-perspective process mining extends case-centric
XES logs with four named perspectives: control-flow (activity sequence),
resource (who performed the activity), data (attribute values), and time
(timestamps). Each perspective is a first-class XES extension, not a
stringly-typed attribute.

| Paper formal object | Rust surface | Enforcing law |
|---|---|---|
| Time perspective (timestamp) | `src/xes.rs::XesEvent` (timestamp field) | covered |
| Control-flow perspective (concept:name) | `src/xes.rs::XesEvent` (activity attribute) | covered |
| Resource perspective (org:resource) | `src/xes.rs` — not yet a typed namespace | **gap** |
| Data perspective (named attribute map) | `src/xes.rs` — attribute map exists, not perspective-scoped | **gap** |
| Perspective-specific extension declaration | `src/xes.rs::XesExtension` — generic, not per-perspective typed | **gap** |

**Structural laws this crate partially enforces:**

- `XesEvent` carries a timestamp (time perspective) and an attribute map
  (data perspective substrate) in `src/xes.rs`.
- `XesExtension` covers the generic extension declaration law
  (`xes_undeclared_extension_prefix_rejected` compile-fail fixture).
- Resource perspective (`org:resource`, `org:role`, `org:group`) is NOT
  yet typed as a distinct `ResourcePerspective` namespace; an org:resource
  attribute is structurally indistinguishable from any other string
  attribute — this is the gap.
- Data perspective attributes are not yet typed as a distinct
  `DataPerspective` namespace scoped to a named extension declaration.

**Gap requiring future type surface:**

- `ResourcePerspective` as a `PhantomData` extension marker on `XesEvent`
- `DataPerspective` as a typed attribute namespace with declared extension
- Perspective-scoped attribute typed surface that prevents mixing
  resource attributes with data attributes silently

**What must NOT live in this crate:**

- Multi-perspective conformance checking (resource, data, and time
  constraint evaluation — graduates to wasm4pm)
- Social network mining from resource perspective (graduates to wasm4pm)
- Decision mining from data perspective (graduates to wasm4pm)

---

## #49 — Object-Centric Process Mining: Dealing with Divergence and Convergence (van der Aalst, Berti, 2020)

**Paper:** Object-Centric Process Mining: Dealing with Divergence and Convergence in Event Data
**Canon family:** `OCEL_OBJECT_CENTRIC`
**Verdict:** `COVERED_BY_TYPE`
**Pending witness types:** `DivergenceWitness` and `ConvergenceWitness` in `src/witness.rs` (structural support already present)

**Law-packet notes:**

Van der Aalst & Berti (2020) name two structural defects in flattened event
logs: divergence (one case-id maps to many objects — events are duplicated,
inflating frequencies) and convergence (many case-ids share one object —
events are merged, deflating frequencies). OCEL resolves both by making
event-to-object links first-class structural elements.

| Paper formal object | Rust surface | Enforcing law |
|---|---|---|
| Divergence (one-to-many case-to-object) | `src/ocel.rs::EventObjectLink` | `ocel_e2o_missing_link` compile-fail |
| Convergence (many-to-one case-to-object) | `src/ocel.rs::EventObjectLink` | `ocel_e2o_missing_link` compile-fail |
| Object-centric event log (structural fix) | `src/ocel.rs::OcelLog` | — |
| Divergence structural law witness | `src/witness.rs` — `DivergenceWitness` not yet typed | **gap** |
| Convergence structural law witness | `src/witness.rs` — `ConvergenceWitness` not yet typed | **gap** |

**Structural laws this crate enforces:**

- `OcelLog` with `EventObjectLink` resolves both divergence and convergence
  by construction — each event explicitly names the objects it relates to,
  eliminating the ambiguity that causes duplication or merging in flat logs.
- An `OcelEvent` without a declared `EventObjectLink` is a compile error
  (sealed by `ocel_e2o_missing_link` compile-fail fixture), preventing the
  divergence/convergence defect from entering the system.
- `DivergenceWitness` and `ConvergenceWitness` as named unit-struct witness
  types in `src/witness.rs` would complete the named law receipts — each
  certifies that the carrying structure has been assessed for the respective
  defect and found structurally sound.

**What must NOT live in this crate:**

- Divergence/convergence detection algorithms (case-id frequency analysis)
- Flattening from OCEL to XES (this has loss; requires LossReport — handled
  by `ocel_to_xes_no_loss_report` compile-fail fixture)
- Object-centric process discovery execution

---

## #51 — Process Querying Methods (Polyvyanyy, Ouyang, Barros, van der Aalst, 2017)

**Paper:** Process Querying Methods
**Canon family:** `OCPQ_QUERYING`
**Verdict:** `PARTIAL_WITH_REASON`
**Active obligation:** `ProcessQueryWitness` marker in `src/ocpq.rs` linking the OCPQ surface to the Polyvyanyy et al. 2017 process querying framework; `TemporalPredicate` coverage of temporal ordering axioms

**Law-packet notes:**

Polyvyanyy et al. (2017) define a typed process querying framework: a query
is issued against a named process model class (Petri net, process tree,
POWL), predicates are structural or behavioral, and results carry typed
evidence of what was matched. OCPQ (#6 in ledger) extends this framework to
the object-centric domain — `OcpqQuery` and `OcpqResult` in `src/ocpq.rs`
are the OCPQ-extended surface.

| Paper formal object | Rust surface | Enforcing law |
|---|---|---|
| Process query (typed over model class) | `src/ocpq.rs::OcpqQuery` | — |
| Query result (typed evidence of match) | `src/ocpq.rs::OcpqResult` | — |
| Event predicate witness | `src/ocpq.rs::EventPredicate` | — |
| Object predicate witness | `src/ocpq.rs::ObjectPredicate` | — |
| Temporal predicate witness | `src/ocpq.rs::TemporalPredicate` | — |
| Framework provenance witness | `src/ocpq.rs` — `ProcessQueryWitness` not yet typed | **gap** |

**Structural laws this crate partially enforces:**

- `OcpqQuery` with typed predicate witnesses (`EventPredicate`, `ObjectPredicate`,
  `TemporalPredicate`) implements the query framework surface from Polyvyanyy
  et al. (2017) extended to the object-centric domain.
- `OcpqResult` carries typed evidence — a query result is not a free string,
  it is typed over the query's predicate witnesses.
- `TemporalPredicate` covers temporal ordering axioms from the framework
  (before, after, during, concurrent); the full temporal predicate coverage
  from the Polyvyanyy et al. framework needs verification.
- `ProcessQueryWitness` is NOT yet typed — without it there is no
  non-forgeable receipt linking the `OcpqQuery` surface to the Polyvyanyy
  et al. (2017) process querying framework definition.

**Gap requiring future type surface:**

- `ProcessQueryWitness` as a named witness type in `src/ocpq.rs` (or `src/witness.rs`)
  that non-forgeably links `OcpqQuery` to the Polyvyanyy et al. 2017 framework
- Verification that `TemporalPredicate` covers the full temporal ordering
  axiom set (before, after, during, concurrent, overlap) from the framework

**What must NOT live in this crate:**

- Query execution over process model shapes (graph traversal, automaton
  evaluation — graduates to wasm4pm via `NeedsObjectCentricQueryExecution`)
- Query language parsing (string → typed query construction)
- Query result aggregation or scoring

---

## #57 — Stochastic Conformance Checking with Stochastic Petri Nets (Leemans, Syring, van der Aalst, 2019)

**Paper:** Stochastic Conformance Checking with Stochastic Petri Nets
**Canon family:** `PETRI_NETS`
**Verdict:** `PARTIAL_WITH_REASON`
**Active obligation:** `StochasticArcWeight<NUM, DEN>`, `ImmediateTransition`, and `TimedTransition` structural annotations in `src/petri.rs`

**Law-packet notes:**

A stochastic Petri net (SPN) extends a standard WF-net by annotating each
transition with a firing rate: immediate transitions fire at rate ∞ (zero
delay), timed transitions fire at a given positive rate. These are
structurally distinct node kinds — not a boolean flag on a generic transition
type. `StochasticArcWeight` is a probability annotation on an arc (the
proportion of time the arc is chosen), distinct from the plain bipartite arc
topology enforced by `WfNetConst`.

| Paper formal object | Rust surface | Enforcing law |
|---|---|---|
| WF-net base structure | `src/petri.rs::WfNetConst<SOUNDNESS>` | `wfnet_forged_soundness` compile-fail |
| `ImmediateTransition` (zero-delay) | `src/petri.rs` — not yet typed | **gap** |
| `TimedTransition` (rate-annotated) | `src/petri.rs` — not yet typed | **gap** |
| `StochasticArcWeight<NUM, DEN>` | `src/petri.rs` — not yet typed | **gap** |
| Earth mover distance metric | `src/conformance.rs` — would extend `Metric<KIND, NUM, DEN>` | graduates to wasm4pm |

**Structural laws this crate partially enforces:**

- `WfNetConst<SOUNDNESS>` in `src/petri.rs` provides the underlying net shape;
  the bipartite arc law (`petri_place_to_place_arc`, `petri_transition_to_transition_arc`
  compile-fail fixtures) and the non-forgeable soundness witness are the
  foundational structural receipts that stochastic conformance builds upon.
- `ImmediateTransition` and `TimedTransition` must be distinct unit-struct
  markers on `WfNetConst` — passing a `TimedTransition` where an
  `ImmediateTransition` is required is a type error, not a runtime check.
- `StochasticArcWeight<NUM, DEN>` must be a `Between01`-bounded fraction
  (from `src/law.rs::Between01<NUM, DEN>`) — a stochastic weight that
  exceeds the unit interval is a type error.

**Gap requiring future type surface:**

- `ImmediateTransition` and `TimedTransition` as distinct unit-struct
  marker types in `src/petri.rs`; a function accepting only immediate
  transitions cannot silently accept a timed one
- `StochasticArcWeight<NUM, DEN>` as a newtype bounded by `Between01<NUM, DEN>`
  in `src/petri.rs`; a weight that exceeds the unit interval is a compile
  error, not a runtime assertion

**What must NOT live in this crate:**

- Earth mover distance computation (stochastic language distance metric,
  graduates to wasm4pm via `NeedsConformanceExecution`)
- Stochastic language derivation from event logs
- Firing rate estimation algorithms

---

## #64 — Temporal Profile Conformance Checking (Stertz et al., 2020)

**Paper:** Temporal Profile Conformance Checking
**Canon family:** `CONFORMANCE_ALIGNMENT`
**Verdict:** `PARTIAL_WITH_REASON`
**Active obligation:** `TemporalProfile<ActivityPair>`, `TimeDelta`, and `ZScore` structural types in `src/conformance.rs`

**Law-packet notes:**

Stertz et al. (2020) define a temporal profile `TP(A,B) = (AVG, STD)` for
each pair of activities (A, B) in an event log: the mean and standard
deviation of the observed time distance between them. Conformance checking
then measures, for each case, how many standard deviations the actual
time distance deviates from the profile — the zeta-value. A case is
conforming if all its activity-pair zeta-values fall within a user-supplied
tolerance.

| Paper formal object | Rust surface | Enforcing law |
|---|---|---|
| Metric substrate (zeta-value bound) | `src/conformance.rs::Metric<KIND, NUM, DEN>` with `Between01<NUM, DEN>` | `metric_out_of_bounds` compile-fail |
| `TemporalProfile<ActivityPair>` (AVG/STD per pair) | `src/conformance.rs` — not yet typed | **gap** |
| `TimeDelta` (typed time-distance newtype) | `src/conformance.rs` — not yet typed | **gap** |
| `ZScore` (typed deviation measure) | `src/conformance.rs` — not yet typed | **gap** |
| Zeta-value conformance execution | graduates to wasm4pm (`NeedsConformanceExecution`) | — |

**Structural laws this crate partially enforces:**

- `Metric<KIND, NUM, DEN>` with `Between01<NUM, DEN>` in `src/conformance.rs`
  provides the metric shape substrate that any normalized zeta-value score
  must obey; a deviation score escaping the unit interval is a compile error
  (sealed by `metric_out_of_bounds` compile-fail fixture).
- `TemporalProfile` must be a distinct structural type from `Metric` — it
  maps an activity-pair key to an `(AVG, STD)` shape, not to a
  `Between01`-bounded fraction.
- `TimeDelta` must be a typed duration newtype — not a bare `u64` or
  `f64`; mixing duration units is a structural defect.
- `ZScore` is a typed deviation newtype over a rational fraction; it is
  not identical to `Between01<NUM, DEN>` because the tolerance threshold
  is user-supplied, not a fixed [0,1] bound.

**Gap requiring future type surface:**

- `TemporalProfile<ActivityPair>` struct in `src/conformance.rs` carrying
  AVG (mean time-distance) and STD (standard deviation) for a typed
  activity-pair key
- `TimeDelta` typed duration newtype (prevents unit confusion)
- `ZScore` typed deviation newtype (distinct from `Between01` metric)

**What must NOT live in this crate:**

- Temporal profile construction from event logs (AVG/STD computation
  over activity-pair time distances — graduates to wasm4pm
  via `NeedsConformanceExecution`)
- Zeta-value threshold evaluation and case classification
- Temporal profile conformance checking execution

---

## #67 — Event Logs and Their Metadata in Process Mining (Verbeek et al., 2011)

**Paper:** Event Logs and Their Metadata in Process Mining
**Canon family:** `XES_EVENT_LOG`
**Verdict:** `COVERED_BY_TYPE`

**Law-packet notes:**

Verbeek et al. (2011) define the operational XES/OpenXES metadata model:
a lawful XES log must declare its extensions (so that attribute prefixes
are not dangling references), may define classifiers (named event-class
definitions that determine event identity for discovery), and may carry
global attributes (log-wide defaults for trace and event attributes).
These are structural laws — not runtime validation rules — and they are
already reified in `src/xes.rs`.

| Paper formal object | Rust surface | Enforcing law |
|---|---|---|
| XES log structure | `src/xes.rs::XesLog` | — |
| XES trace structure | `src/xes.rs::XesTrace` | — |
| XES event structure | `src/xes.rs::XesEvent` | — |
| Extension declaration (metadata law) | `src/xes.rs::XesExtension` | `xes_undeclared_extension_prefix_rejected` compile-fail |
| Case-centric distinction | `src/xes.rs::CaseCentricMarker` | `xes_not_object_centric` compile-fail |
| Standard provenance | `src/witness.rs::Xes1849` | subsumes XES/OpenXES metadata model |
| Base event log | `src/eventlog.rs::EventLog` | — |

**Structural laws this crate enforces:**

- An XES attribute using an extension prefix that has not been declared
  in the log header is refused as `XesRefusal::UndeclaredExtensionPrefix`.
  The `xes_undeclared_extension_prefix_rejected` compile-fail fixture
  seals this from Verbeek et al. (2011) §3 extension declaration law.
- A `XesCaseCentricLog` cannot substitute an OCED/OCEL structure — the
  `xes_not_object_centric` compile-fail fixture seals the flat vs.
  object-centric structural distinction.
- The `Xes1849` witness in `src/witness.rs` is the named provenance
  receipt that ties these structural laws to the IEEE XES standard;
  the OpenXES metadata model (classifiers, global attributes, extension
  declarations) is the operational counterpart captured by `XesExtension`.

**What must NOT live in this crate:**

- XES file I/O (`.xes` / `.xes.gz` parsing and serialization)
- XES classifier evaluation (computing event identity at runtime)
- XES validator execution (checks beyond structural extension declaration)
- OpenXES library API (Java implementation concerns)

---

## OCEL Law Packet — Object-Change and Relation Laws

**Paper family:** `OCEL_OBJECT_CENTRIC`
**Sources:** OCEL 2.0 Specification (van der Aalst et al., 2023); OCEL 1.0 (van der Aalst, Berti, 2020)

The OCEL law family covers structural laws governing object evolution, event-to-object and object-to-object relations, attribute typing, dimension vocabulary, and the OCEL-to-XES boundary. Each law is a distinct named type surface — not a runtime validation flag.

### object-change-law

An `ObjectChange` records which object's which attribute changed to which value, optionally when. A change naming an undeclared object or an empty attribute name is refused as `OcelRefusal::InvalidObjectChange`. The law prevents silent attribute mutation without a named object and a non-empty attribute key.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocel_object_change_law` — `ObjectChange` must name a declared object and non-empty attribute | `ocel::ObjectChange` / `OcelRefusal::InvalidObjectChange` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | OCEL 2.0 §4.2 object evolution |

### event-relation-law

The E2O link (`EventObjectLink`) is the structural law that makes OCEL different from XES. Every `OcelLog` must have at least one `EventObjectLink`. A log with no E2O links is refused as `OcelRefusal::EmptyEventObjectLinks`. A link pointing at an undeclared event or object is refused as `OcelRefusal::DanglingEventObjectLink`.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocel_event_relation_law` — E2O link must reference declared event and object | `ocel::EventObjectLink` / `OcelRefusal::DanglingEventObjectLink` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | OCEL 2.0 §3 formal model |
| `ocel_empty_e2o_law` — log without E2O links is structurally empty | `ocel::OcelLog` / `OcelRefusal::EmptyEventObjectLinks` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | OCEL 2.0 §3 formal model |

### object-relation-law

The O2O link (`ObjectObjectLink`) is OCEL 2.0's second link type (absent in OCEL 1.0). A link referencing an undeclared object is refused as `OcelRefusal::DanglingObjectObjectLink`. The law prevents ghost object relationships.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocel_object_relation_law` — O2O link must reference declared objects | `ocel::ObjectObjectLink` / `OcelRefusal::DanglingObjectObjectLink` | `compile_pass/ocel_object_object_relation.rs` | `compile_fail/ocel_o2o_missing_link.rs` | OCEL 2.0 §3.2 object-to-object links |

### attribute-witness-law

OCEL 2.0 attributes are typed at the data model level: `OcelAttributeValue` is an enum distinguishing String, Integer, Float, Boolean, and TimestampNs variants. A raw stringly-typed attribute map is structurally weaker than a typed one — conflating attribute types without a named value variant is a structural defect. The `OcelAttribute` struct enforces that every attribute carries both a key (`String`) and a typed value (`OcelAttributeValue`), preventing the silent promotion of untyped key-value pairs to OCEL attributes.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocel_attribute_typed_value_law` — OCEL attribute value must be a named typed variant | `ocel::OcelAttributeValue` (enum, not free string) | `compile_pass/ocel_event_object_relation.rs` | n/a (law enforced by enum exhaustiveness) | OCEL 2.0 §4.1 attribute types |
| `ocel_attribute_key_law` — `OcelAttribute` carries both key and typed value | `ocel::OcelAttribute` (struct, not `HashMap<String, String>`) | `compile_pass/ocel_event_object_relation.rs` | n/a (structural by construction) | OCEL 2.0 §4.1 attribute types |

### dimensions-law

`OcelDims` captures the dimension vocabulary of an `OcelLog` — the distinct object types and activity names — as a structural surface that can be inspected without materializing the full relational tables. This prevents dimension-sensitive code from operating on an `OcelLog` without explicitly naming its dimension vocabulary. An `OcelDims` derived from an empty log is distinct from one derived from a populated log.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocel_dims_law` — log dimensions are a named structural vocabulary, not free sets | `ocel::OcelDims` (distinct from `OcelLog`) | `compile_pass/ocel_event_object_relation.rs` | n/a (law enforced by type distinction) | OCEL 2.0 §2 data model vocabulary |

### object-type-witness-law

Every `OcelObject` must declare a non-empty object type. An object with an empty type string is refused as `OcelRefusal::MissingObjectType`. In OCEL the object type is not an annotation — it is the structural identity of the object. A typeless object cannot participate lawfully in E2O or O2O links because the type determines which process perspective the object belongs to.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocel_object_type_witness_law` — every object must have a non-empty type | `ocel::OcelObject` / `OcelRefusal::MissingObjectType` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | OCEL 2.0 §3.1 object types |

### event-type-witness-law

Every `OcelEvent` carries an activity name (the event type in OCEL terminology). The activity name is the process-centric label of the event and must be a non-empty string. A nameless activity collapses all event types into an indistinguishable set, which defeats process mining. The `OcelEvent::new` constructor accepts only a non-empty activity name; a validation that checks for empty activity names must be added to `OcelLog::validate` to complete this law.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocel_event_type_witness_law` — every event must carry a non-empty activity name | `ocel::OcelEvent` (activity field, non-empty) | `compile_pass/ocel_event_object_relation.rs` | n/a (gap: validation pending in `OcelLog::validate`) | OCEL 2.0 §3.3 activity (event type) |

### relation-qualifier-law

Both `EventObjectLink` (E2O) and `ObjectObjectLink` (O2O) carry an optional `qualifier` that names the role of the object in the event or the role of the relationship between two objects (e.g., `"places"`, `"contains"`, `"belongs_to"`). A qualifier is not a free annotation — in OCEL 2.0, qualifiers are part of the formal data model and distinguish multiple links between the same pair. A missing qualifier is lawful (the link is unqualified); a qualifier that is present must be a non-empty string. Assigning a qualified link with an empty qualifier string is a structural defect because it produces an indistinguishable qualifier from the absence of a qualifier.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocel_e2o_qualifier_law` — E2O qualifier, when present, names a role | `ocel::EventObjectLink::qualified()` (builder) | `compile_pass/ocel_event_object_relation.rs` | n/a (gap: empty-qualifier validation pending) | OCEL 2.0 §3.4 relation qualifier |
| `ocel_o2o_qualifier_law` — O2O qualifier, when present, names a relationship type | `ocel::ObjectObjectLink::qualified()` (builder) | `compile_pass/ocel_object_object_relation.rs` | n/a (gap: empty-qualifier validation pending) | OCEL 2.0 §3.4 relation qualifier |

### cardinality-projection-law

OCPQ (Object-Centric Process Querying) introduces cardinality constraints over OCEL object sets: a query result that requests "at most N objects of type T" is a `CardinalityBound<N>` surface, not a free integer. The cardinality law connects the OCEL object-type vocabulary (`OcelDims::object_types`) with the OCPQ cardinality bound: only object types that appear in the log's dimension vocabulary are lawful targets for cardinality constraints. A `CardinalityBound` that references an unknown object type is a structural defect — it cannot be grounded by the log.

The `Between01<NUM, DEN>` metric bound from `src/law.rs` applies to cardinality projections when the bound is expressed as a fraction of the total object count. An `ocpq_cardinality_overflow` compile-fail fixture already seals the arithmetic bound; the cardinality-projection law adds the semantic grounding law connecting the OCEL dimension vocabulary to the OCPQ query scope.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocel_cardinality_projection_law` — OCPQ cardinality bound must be grounded by OCEL object type | `ocpq::CardinalityBound` + `ocel::OcelDims` (vocabulary grounding) | `compile_pass/ocpq_cardinality_valid_bounds.rs` | `compile_fail/ocpq_cardinality_overflow.rs` | OCEL 2.0 §2 + OCPQ Def 6 |
| `ocel_dims_scope_law` — OCPQ object scope must intersect OCEL dimension vocabulary | `ocpq::OcpqRefusal::MissingObjectScope` + `ocel::OcelDims` | `compile_pass/ocpq_scoped_query.rs` | `compile_fail/ocpq_missing_scope_rejected.rs` | OCEL 2.0 §2 + OCPQ Def 6 |

### ocel-to-xes-boundary-law

Flattening OCEL to XES is the most consequential structural transition in process mining: it converts a multi-object-notion log into a single-case-notion log, losing all object-to-object relationships and the event-to-multiple-objects multiplicity. This is modeled in three complementary surfaces:

1. `OcelToXesProjection` — the named projection descriptor that makes the choice of case notion (`case_type`) explicit. The `PROJECTION_NAME` constant (`"ocel-flatten-to-xes:by-case-type"`) is the stable receipt that the adopter can cite in a `LossReport`.
2. `OcelShape` / `XesShape` — uninhabited zero-sized types used as `From` and `To` in `LossReport<OcelShape, XesShape, Vec<String>>`. These prevent the loss report from being assembled with wrong-direction markers.
3. `LossyFormatExport` — the mandatory-loss export that requires a `LossReport`, preventing silent structural loss.

The `ocel_to_xes_no_loss_report` compile-fail fixture already seals the law that `LossyFormatExport` (not `FormatExport`) must be used for this boundary. The law packet below cross-references that fixture with its projection surface.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocel_to_xes_boundary_law` — OCEL-to-XES flattening requires named projection + loss report | `interop::OcelToXesProjection` + `formats::LossyFormatExport` | `compile_pass/ocel_to_xes_with_named_projection.rs` | `compile_fail/ocel_to_xes_no_loss_report.rs` | OCEL 1.0 §3 (convergence/divergence) |
| `ocel_shape_direction_law` — OCEL→XES `LossReport` must use `OcelShape` as `From` and `XesShape` as `To` | `interop::OcelShape` / `interop::XesShape` (distinct uninhabited markers) | `compile_pass/ocel_to_xes_with_named_projection.rs` | n/a (law enforced by type parameter direction) | OCEL 2.0 §5.1 format boundary |
| `ocel_flattening_loss_law` — `OcelRefusal::FlatteningLoss` names the convergence/divergence loss explicitly | `ocel::OcelRefusal::FlatteningLoss` | `compile_pass/ocel_to_xes_with_named_projection.rs` | `compile_fail/ocel_to_xes_no_loss_report.rs` | OCEL 1.0 (van der Aalst & Berti, 2020) convergence/divergence |

---

## Conformance-Prediction Law Packet

**Paper family:** `CONFORMANCE_ALIGNMENT` / `PREDICTIVE_MONITORING`
**Sources:** Rozinat & van der Aalst (2008); van der Aalst et al. alignments (2012); Tax et al. (2017); De Santis et al. (2026)

The conformance-prediction family covers structural laws for alignment cost,
token-replay boundary, four quality metrics (fitness, precision, F1,
generalization, simplicity), prediction target witnesses, compliance-target law,
and risk-score witness. Every law is structure-only: this crate carries and
refuses shapes but never computes alignment, replays tokens, or trains models.

### alignment-cost-law

Alignment-based conformance represents each log-model comparison step as a
named move. Three move kinds are structurally distinct: `SyncMove` (log and
model agree), `LogOnlyMove` (log had a step the model could not match),
`ModelOnlyMove` (model required a step the log did not show). Confusing move
kinds is a structural defect. `Deviation<M>` is parametric over the move
witness `M` — `Deviation<SyncMove>` and `Deviation<LogOnlyMove>` are different
types, not aliases and not runtime-tagged variants.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `alignment-cost-law` — `SyncMove`, `LogOnlyMove`, `ModelOnlyMove` are distinct move witnesses on `Deviation<M>` | `conformance::SyncMove` / `conformance::LogOnlyMove` / `conformance::ModelOnlyMove` + `conformance::Deviation<M>` | `compile_pass/conformance_deviation_shape.rs` | — (structure-only; no cross-move confusion fixture) | van der Aalst et al., Alignments in Process Mining (2012) |
| `alignment-move-type-distinction` — `Deviation<SyncMove>` and `Deviation<LogOnlyMove>` are not interchangeable types | `conformance::Deviation<M>` (phantom move witness `M`) | `compile_pass/conformance_deviation_shape.rs` | — (distinct phantom params prevent conflation at compile time) | van der Aalst et al., Alignments in Process Mining (2012) |

### token-replay-boundary

Token replay is a conformance engine concern — not a structure concern. This
crate defines `ConformanceVerdict` as the carrier of replay results (fitness,
precision, deviations), but it never executes replay. A `ConformanceVerdict`
with `fitness = None` means no replay was run, not that fitness is zero. The
boundary between structure (this crate) and execution (wasm4pm) is encoded in
the module doc and enforced by the module architecture: `ConformanceVerdict`
is not a replay engine.

The `ConformanceRefusal` surface names exactly why a verdict cannot be
admitted: `MissingLog`, `MissingModel`, `MissingDeviationPath`,
`FitnessUnavailable`, `PrecisionUnavailable`, `F1Unavailable`. Each variant
names a specific structural law — no variant catches all failures with a bare
`InvalidInput`.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `token-replay-boundary` — `ConformanceVerdict` carries replay output, never executes token replay | graduation boundary: `conformance::ConformanceVerdict` — module doc, no engine logic present | `compile_pass/conformance_verdict_complete.rs` | — (graduation boundary; enforced by module-level doc and architecture) | van der Aalst, Replay in Process Mining (2016) |
| `conformance-refusal-named-law` — every `ConformanceRefusal` variant names a specific structural law | `conformance::ConformanceRefusal` (no bare `InvalidInput`) | `compile_pass/conformance_verdict_complete.rs` | — (law enforced by enum shape: each variant is a named law) | Rozinat & van der Aalst, Conformance Checking (2008) |

### fitness-precision-f1-metric-law

The four quality dimensions of process mining (Rozinat & van der Aalst, 2008)
are fitness, precision, F1, and generalization/simplicity. This crate reifies
the first three as distinct const-generic types: `FitnessConst<N,D>`,
`PrecisionConst<N,D>`, `F1Const<N,D>`. Each is a type alias for
`Metric<KIND, NUM, DEN>` where `KIND` distinguishes the metric family. The
`Between01<NUM, DEN>` bound in `src/law.rs` ensures `NUM/DEN ∈ [0,1]` at the
type level — a score outside the unit interval is a compile error.

`FitnessConst<3,4>` (0.75 fitness) and `PrecisionConst<3,4>` (0.75 precision)
are different types even though NUM and DEN match — the `KIND` const param
carries the distinction. A function requiring `FitnessConst<3,4>` cannot
silently accept `PrecisionConst<3,4>`.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `fitness-precision-f1-metric-law` — `FitnessConst`, `PrecisionConst`, `F1Const` are distinct types; `N/D ∈ [0,1]` at the type level | `conformance::Metric<KIND, NUM, DEN>` + `law::QualityMetricKind::{Fitness, Precision, F1}` | `compile_pass/conformance_verdict_complete.rs`, `compile_pass/conformance_precision_f1_aliases.rs` | `compile_fail/metric_out_of_bounds.rs` | Rozinat & van der Aalst, Conformance Checking (2008) |
| `fitness-kind-distinctness` — `FitnessConst<N,D>` ≠ `PrecisionConst<N,D>` even when NUM/DEN match | `conformance::Metric<KIND, NUM, DEN>` — KIND const param distinguishes metric families | `compile_pass/conformance_precision_f1_aliases.rs` | — (type-level: KIND param enforces distinction) | Rozinat & van der Aalst, Conformance Checking (2008) |
| `metric-unit-interval-law` — `Metric<KIND, N, D>` with `N > D` does not compile | `law::Between01<NUM, DEN>` where-bound on `Metric` | `compile_pass/conformance_verdict_complete.rs` | `compile_fail/metric_out_of_bounds.rs` | Rozinat & van der Aalst, Conformance Checking (2008) |

---

## XES Law Family — IEEE 1849 Case-Centric Exchange Structure

**Standard:** IEEE 1849-2023 (XES — eXtensible Event Stream)
**Canon family:** `XES_EVENT_LOG`
**Sources:** Verbeek et al. (2011), IEEE 1849-2023, van der Aalst (2011 multi-perspective)

XES is a case-centric, flat event-log interchange format. It is emphatically NOT
object-centric. Every structural law in this family enforces the distinction between
XES's flat-trace world and OCED/OCEL's object-graph world. The type surfaces live in
`src/xes.rs`, `src/witness.rs`, and `src/formats.rs`. No XES law in this crate
performs parsing, validation execution, or mining — those graduate to `wasm4pm`.

### case-centric-marker

The foundational XES structural distinction: a `XesLog` is case-centric, not
object-centric. `CaseCentricMarker` is a zero-sized `PhantomData` tag that makes it
impossible at the type level to confuse a flat case-centric log with an
`OcelLog` (object-centric). The distinction is enforced by the type system, not by
a runtime `is_object_centric()` check.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `case-centric-marker` — `XesCaseCentricLog` is not OCED; substituting one for the other is a compile error | `xes::CaseCentricMarker` (zero-sized `PhantomData` tag) | `compile_pass/xes_case_centric_log.rs` | `compile_fail/xes_not_object_centric.rs` | IEEE 1849-2023 §4 (case notion); van der Aalst & Berti (2020) divergence/convergence |
| `xes_not_object_centric` — sealed compile-fail: `XesCaseCentricLog` rejected where `OcelLog` required | `xes::CaseCentricMarker` | `compile_pass/xes_case_centric_log.rs` | `compile_fail/xes_not_object_centric.rs` | `xes_not_object_centric.stderr` |

### generalization-simplicity-law

Process quality is four-dimensional: fitness, precision, generalization,
simplicity (Buijs, van Dongen & van der Aalst, 2014). Generalization measures
whether the model covers unseen traces (not just those in the log).
Simplicity measures structural parsimony — a model should not be more complex
than needed. Both are reified as distinct `Metric<KIND, N, D>` variants:
`QualityMetricKind::Generalization` and `QualityMetricKind::Simplicity`.

Unlike fitness/precision/F1, no type aliases (`GeneralizationConst`,
`SimplicityConst`) exist yet — callers use the generic `Metric<{KIND}, N, D>`
form directly. The `Between01` bound applies uniformly: a generalization score
outside `[0,1]` is a compile error just as for fitness.

`Metric<{Generalization}, 1, 2>` and `Metric<{Simplicity}, 1, 2>` are distinct
types — passing a simplicity metric where a generalization metric is required
is a compile error.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `generalization-metric-law` — `Metric<{Generalization}, N, D>` enforces `N/D ∈ [0,1]` at type level | `conformance::Metric<{QualityMetricKind::Generalization}, N, D>` | `compile_pass/conformance_generalization_metric.rs` | `compile_fail/metric_out_of_bounds.rs` | Buijs, van Dongen & van der Aalst (2014) |
| `simplicity-metric-law` — `Metric<{Simplicity}, N, D>` enforces `N/D ∈ [0,1]` at type level | `conformance::Metric<{QualityMetricKind::Simplicity}, N, D>` | `compile_pass/conformance_simplicity_metric.rs` | `compile_fail/metric_out_of_bounds.rs` | Buijs, van Dongen & van der Aalst (2014) |
| `generalization-simplicity-kind-distinctness` — `Metric<{Generalization}, 1, 2>` ≠ `Metric<{Simplicity}, 1, 2>` even when NUM/DEN match | `conformance::Metric<KIND, N, D>` — KIND const param distinguishes generalization from simplicity | `compile_pass/conformance_simplicity_metric.rs` | — (type-level: KIND param enforces distinction) | Buijs, van Dongen & van der Aalst (2014) |

### prediction-target-witness

Predictive process monitoring (PPM) addresses five prediction problem families
(Maggi et al., 2014; Tax et al., 2017): next activity, outcome label, remaining
time, drift signal, and risk score. Each is a structurally distinct prediction
target. The `PredictionProblem<T>` type carries the target kind at the type
level via a phantom witness `T`. A function that accepts
`PredictionProblem<NextActivity>` cannot silently accept
`PredictionProblem<RemainingTime>` — they are different types.

The `PrefixTrace` witness certifies that the problem's input is a lawful
prefix (a case observed so far). Without a prefix, no prediction problem can
be admitted: `PredictionRefusal::EmptyPrefix` and `MissingPrefix` name the
specific structural violations.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `prediction-target-witness` — `PrefixTrace`, `OutcomeLabel`, `RemainingTime`, `NextActivity`, `DriftSignal` are distinct phantom witnesses on `PredictionProblem<T>` | `prediction::PredictionProblem<T>` target witness markers | `compile_pass/prediction_problem_shape.rs`, `compile_pass/prediction_next_activity_target.rs`, `compile_pass/prediction_remaining_time_target.rs` | — (cross-witness confusion covered by compliance-target-law fixtures) | Maggi et al. (2014); Tax et al. (2017) |
| `prediction-problem-refusal-named-law` — every `PredictionRefusal` variant names a specific structural law | `prediction::PredictionRefusal` (no bare `InvalidInput`) | `compile_pass/prediction_problem_shape.rs` | — (law enforced by enum shape: each variant is a named law) | Tax et al., RNN-based PPM (2017) |
| `prediction-prefix-required` — a `PredictionProblem` without a prefix trace is `PredictionRefusal::EmptyPrefix` | `prediction::PredictionRefusal::EmptyPrefix` / `MissingPrefix` | `compile_pass/prediction_problem_shape.rs` | — (refusal law enforced by named variant) | Tax et al., RNN-based PPM (2017) |


---

## Declare/OCPQ Law Packet — Binary Constraint Templates

**Paper family:** `DECLARE_CONSTRAINTS`
**Sources:** Pesic & van der Aalst (2006); OC-Declare (van der Aalst, 2019)

The Response, Precedence, and Succession Declare templates are binary constraint templates: each requires exactly two activity arguments (activation + target). These are the foundational temporal ordering laws of the Declare framework. Their arity is enforced at the structural level — a binary template with zero or one activity argument is a structural defect, not a runtime error.

### response-constraint

`DeclareTemplate::Response` names the constraint "every occurrence of activity A is eventually followed by activity B." It is a typed enum variant, not a string. A constraint using `Response` without a target activity is refused as `DeclareRefusal::MissingTarget`.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `declare_response_constraint` — `DeclareTemplate::Response` is a first-class binary template, not a string | `declare::DeclareTemplate::Response` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | Pesic & van der Aalst (2006) §3 |

### precedence-constraint

`DeclareTemplate::Precedence` names the constraint "every occurrence of B is preceded by A." Like `Response`, it is a typed enum variant requiring both activation and target.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `declare_precedence_constraint` — `DeclareTemplate::Precedence` is a first-class binary template | `declare::DeclareTemplate::Precedence` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | Pesic & van der Aalst (2006) §3 |

### succession-constraint

`DeclareTemplate::Succession` is the conjunction of `Response` and `Precedence` — it is a single typed template, not a pair of constraints. A succession constraint without a target is refused as `DeclareRefusal::MissingTarget`.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `declare_succession_constraint` — `DeclareTemplate::Succession` = Response ∧ Precedence; enforced as a single typed template | `declare::DeclareTemplate::Succession` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | Pesic & van der Aalst (2006) §3 |

### trace-attribute-witness

A `XesTrace` carries an ordered sequence of `XesEvent`s. The trace's case
identifier is its `concept:name` attribute. The structural law requires:
- Traces have a non-empty name (refused as `XesRefusal::MissingTraceName`)
- Traces have at least one event (refused as `XesRefusal::EmptyTrace`)
- Event order is preserved verbatim — no reordering occurs at the structure layer

The `xes_trace_attributes.rs` compile-pass fixture seals the positive path: a
lawfully constructed `XesTrace` exposes its events, its name, and its length
via typed accessors. The `XesTrace::is_empty()` / `XesTrace::len()` accessors
are the lawful surface for structural shape checks.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `trace-attribute-witness` — `XesTrace` preserves event order and exposes named trace identifier | `xes::XesTrace` (ordered `Vec<XesEvent>` + `concept:name`) | `compile_pass/xes_trace_attributes.rs` | — (structural law; failure is runtime refusal `MissingTraceName`/`EmptyTrace`) | IEEE 1849-2023 §5.1 trace element; Verbeek et al. (2011) §3 |
| `xes_missing_trace_name` — a `XesTrace` lacking `concept:name` is refused as `XesRefusal::MissingTraceName` | `xes::XesRefusal::MissingTraceName` | `compile_pass/xes_trace_attributes.rs` | — (runtime refusal path; attribute bag does not expose missing fields at compile time) | IEEE 1849-2023 §5.1 |
| `xes_empty_trace` — a `XesTrace` with zero events is refused as `XesRefusal::EmptyTrace` | `xes::XesRefusal::EmptyTrace` | `compile_pass/xes_trace_attributes.rs` | — (runtime refusal path) | IEEE 1849-2023 §5.1 |

### compliance-target-law

De Santis et al. (2026) introduce compliance-aware predictive process
monitoring (PPM): the prediction target is not an outcome label but a named
compliance rule — "does this prefix comply with constraint C?". A
`PredictionProblem<ComplianceTarget>` is structurally distinct from a
`PredictionProblem<OutcomeLabel>`.

The structural law is twofold:

1. `ComplianceTarget ≠ OutcomeLabel` as phantom witnesses on
   `PredictionProblem<T>`. A compliance-constrained monitor slot
   (`PredictionProblem<ComplianceTarget>`) rejects an outcome-labelled
   problem (`PredictionProblem<OutcomeLabel>`) at compile time.
2. A `PredictionTarget::ComplianceConstraint` problem without a named
   constraint reference is `PredictionRefusal::ConstraintNotNamed`. An
   anonymous compliance check is structurally inadmissible — the constraint
   must be named so the prediction can be grounded against a specific rule.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `compliance-target-law` — `PredictionProblem<ComplianceTarget>` ≠ `PredictionProblem<OutcomeLabel>` | `prediction::ComplianceTarget` / `prediction::OutcomeLabel` (distinct phantom witnesses) | `compile_pass/compliance_prediction_target.rs` | `compile_fail/compliance_not_outcome_label.rs` | De Santis et al. (2026) |
| `compliance-witness-wrong-target-law` — compliance slot rejects non-compliance witness | `prediction::ComplianceTarget` phantom param enforcement | `compile_pass/compliance_prediction_target.rs` | `compile_fail/compliance_witness_wrong_target.rs` | De Santis et al. (2026) |
| `compliance-constraint-must-be-named` — `PredictionTarget::ComplianceConstraint` without named rule is `ConstraintNotNamed` | `prediction::PredictionRefusal::ConstraintNotNamed` | `compile_pass/compliance_prediction_target.rs` | — (law enforced by named refusal variant) | De Santis et al. (2026) |

### event-attribute-witness

A `XesEvent` is a bag of namespaced key/value attributes. The structural law
requires that standard extension keys (`concept:name`, `time:timestamp`,
`org:resource`) are accessible via typed helper methods, while arbitrary
namespaced keys are accessible via the generic `attribute(key)` accessor. The
event is structure-only: it holds attributes verbatim; it does not interpret
timestamps, parse lifecycle transitions, or validate resource identifiers.

The `xes_trace_attributes.rs` compile-pass fixture demonstrates the full
positive law surface: standard key helpers, generic accessor, attribute count.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `event-attribute-witness` — `XesEvent` exposes `concept:name`, `time:timestamp`, `org:resource` via typed helpers | `xes::XesEvent::concept_name()`, `timestamp()`, `resource()` | `compile_pass/xes_trace_attributes.rs` | — (structural law; helper returns `Option<&str>`, none required at type level) | IEEE 1849-2023 §5.2 event element; van der Aalst (2011) multi-perspective |
| `xes_missing_concept_name` — an `XesEvent` lacking `concept:name` is refused as `XesRefusal::MissingConceptName`; structural exchange law | `xes::XesRefusal::MissingConceptName` | `compile_pass/xes_case_centric_log.rs` | — (runtime refusal; stringly-typed attribute bag cannot enforce presence at compile time) | IEEE 1849-2023 §5.2; Verbeek et al. (2011) §3 |

---

## powl-law family — POWL/Process-Tree Law Packets

The following sections document the law packets for the POWL (Partially Ordered
Workflow Language) and process-tree structural surfaces. Each subsection names
one law concept, its enforcing Rust type surface, its fixture references, and
the paper source.

---

### partial-order-law

**Law concept:** A partial order over POWL child nodes is a DAG of typed
precedence edges (`OrderEdge`), not a free list. The partial-order node kind
(`PowlNodeKind::PartialOrder`) and its edges are first-class structural types.

**Paper:** Kourani & van der Aalst (2023) — POWL §3: a `StrictPartialOrder` is
a pair `(nodes, ≺)` where `≺` is a strict partial order (irreflexive, asymmetric,
transitive). The precedence relation is defined over named POWL sub-models, not
over opaque identifiers.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `partial-order-dag-law` — precedence edges form a DAG; a cycle is refused as `PowlRefusal::CyclicPartialOrder` | `powl::OrderEdge` / `powl::PowlNodeKind::PartialOrder` | `compile_pass/powl_choice_graph.rs` | — | Kourani & van der Aalst (2023) §3 |
| `order-edge-typed-distinctness` — `OrderEdge` and `ChoiceGraphEdge` are distinct newtypes with the same field layout; substituting one for the other is a compile error | `powl::OrderEdge` vs `powl::ChoiceGraphEdge` | `compile_pass/powl_choice_graph.rs` | `compile_fail/powl_order_edge_choice_confusion.rs` | Kourani & van der Aalst (2023) §3 |
| `partial-order-concurrency-law` — two nodes lacking an edge in either direction are concurrent; `TypedNode<{PowlKind::Partial}>::are_concurrent()` formalises this at the type level | `nightly_foundry::powl_law::TypedNode<{PowlKind::Partial}>` | `compile_pass/powl_choice_graph.rs` | — | Kourani & van der Aalst (2023) §3 |

**What must NOT live in this crate:**

- Topological sort of partial-order nodes (execution scheduling — graduates to wasm4pm)
- Transitive closure computation for the precedence relation
- Partial-order replay or interleaving semantics

### risk-score-witness

Risk score prediction is a distinct PPM target family (van der Aalst, No AI
Without PI, 2025): the prediction target is a threat or hazard probability
estimate, not a categorical outcome label or a compliance check.
`PredictionProblem<RiskScore>` is structurally distinct from
`PredictionProblem<OutcomeLabel>`, `PredictionProblem<ComplianceTarget>`,
and `PredictionProblem<NextActivity>`.

The `RiskScore` witness marker certifies at the type level that a prediction
problem is asking about risk quantification. This prevents risk-oriented
monitor slots from silently accepting non-risk prediction problems — the KIND
distinction is enforced at compile time through the phantom witness on
`PredictionProblem<T>`.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `risk-score-witness` — `PredictionProblem<RiskScore>` is distinct from outcome, compliance, next-activity, and remaining-time families | `prediction::RiskScore` witness marker on `PredictionProblem<RiskScore>` | `compile_pass/prediction_risk_target.rs` | — (cross-witness confusion covered by compliance-target-law fixtures; no dedicated risk confusion fixture) | van der Aalst, No AI Without PI (arXiv:2508.00116, 2025) |
| `risk-target-variant` — `PredictionTarget::Risk` is a first-class enum variant, not a subcase of `OutcomeLabel` | `prediction::PredictionTarget::Risk` (distinct enum variant) | `compile_pass/prediction_risk_target.rs` | — (enum exhaustiveness prevents conflation) | van der Aalst, No AI Without PI (arXiv:2508.00116, 2025) |

### extension-prefix-law

Every XES attribute key is namespaced by an extension prefix (e.g. `concept:name`,
`time:timestamp`, `lifecycle:transition`). An extension must be declared in the
`XesLog` header before its prefix can appear in any event attribute. An attribute key
referencing an undeclared prefix is refused as `XesRefusal::UndeclaredExtensionPrefix`.
This is the core XES interchange law: dangling extension references are structural
defects, not warnings.

The `xes_undeclared_extension_prefix_rejected.rs` compile-fail fixture seals the
negative path. The `xes_declared_extension_prefix.rs` compile-pass fixture seals
the positive path.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `extension-prefix-law` — every namespaced attribute key must reference a declared extension prefix | `xes::XesRefusal::UndeclaredExtensionPrefix` | `compile_pass/xes_declared_extension_prefix.rs` | `compile_fail/xes_undeclared_extension_prefix_rejected.rs` | IEEE 1849-2023 §4 extension declaration; Verbeek et al. (2011) §3 |
| `xes_invalid_extension` — an `XesExtension` with an empty prefix is refused as `XesRefusal::InvalidExtension` | `xes::XesRefusal::InvalidExtension` | `compile_pass/xes_declared_extension_prefix.rs` | — (runtime refusal; empty-prefix check is in `XesLog::validate()`) | IEEE 1849-2023 §4; Verbeek et al. (2011) §3 extension declaration law |
| `xes_no_traces` — a `XesLog` with no traces is refused as `XesRefusal::NoTraces` | `xes::XesRefusal::NoTraces` | `compile_pass/xes_case_centric_log.rs` | — (runtime refusal; shape check in `XesLog::validate()`) | IEEE 1849-2023 §5 log element |

---

### acyclicity-marker-law

**Law concept:** A POWL partial-order node (`PowlNodeKind::PartialOrder`) must
contain no directed cycle in its precedence edges. When a cycle is detected,
the shape is refused as `PowlRefusal::CyclicPartialOrder`. Acyclicity is a
named structural law, not a silent runtime assertion.

**Paper:** Kourani & van der Aalst (2023) POWL §3: the precedence relation `≺`
is a *strict partial order* — irreflexive, asymmetric, and transitive. These
three properties jointly exclude all cycles. A structure that violates any of
them is not a lawful POWL partial-order node.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `cyclic-partial-order-refused` — a cycle in precedence edges is refused as `PowlRefusal::CyclicPartialOrder`, not a silent no-op | `powl::PowlRefusal::CyclicPartialOrder` | `compile_pass/powl_choice_graph.rs` | — (runtime refusal; cycle detection graduates to wasm4pm) | Kourani & van der Aalst (2023) §3 |
| `partial-order-irreflexivity` — a node cannot precede itself; self-loops are excluded by the strict partial order definition | `powl::PowlRefusal::CyclicPartialOrder` (subsumes self-loop case) | `compile_pass/powl_choice_graph.rs` | — | Kourani & van der Aalst (2023) §3 |
| `partial-order-asymmetry` — if `a ≺ b` then `b ⊀ a`; the presence of both edges is a cycle and must be refused | `powl::PowlRefusal::CyclicPartialOrder` (subsumes anti-symmetry case) | `compile_pass/powl_choice_graph.rs` | — | Kourani & van der Aalst (2023) §3 |

**What must NOT live in this crate:**

- Cycle detection algorithm execution (DFS / Kahn's algorithm — graduates to wasm4pm)
- Transitive reduction of the precedence relation
- Partial-order canonicalization or normalization

### lifecycle-transition-witness

XES defines a standard `lifecycle` extension with a `lifecycle:transition` attribute.
The transition value must come from a declared alphabet: `start`, `complete`,
`assign`, `ate_abort`, `withdraw`, `suspend`, `resume`, `pi_abort`, `schedule`,
`unknown`, `autoskip`, `manualskip`. A value outside this alphabet is refused as
`XesRefusal::InvalidLifecycleTransition`. This is a structural interchange law:
lifecycle semantics (what transitions mean for conformance) are an engine concern
that graduates to `wasm4pm`.

The `lifecycle:transition` extension must itself be declared in the log header via
`XesExtension::new("Lifecycle", "lifecycle", ...)` — an undeclared `lifecycle:`
prefix is refused first as `XesRefusal::UndeclaredExtensionPrefix` (the more
fundamental law). `InvalidLifecycleTransition` applies only when the extension is
declared but the value is outside the alphabet.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `lifecycle-transition-witness` — `XesRefusal::InvalidLifecycleTransition` is the named refusal for `lifecycle:transition` values outside the declared alphabet | `xes::XesRefusal::InvalidLifecycleTransition` (named variant, not bare `InvalidInput`) | `compile_pass/xes_declared_extension_prefix.rs` | — (compile-fail fixture for invalid lifecycle transition value TBD) | IEEE 1849-2023 §5.3 lifecycle extension; Verbeek et al. (2011) §3 |
| `lifecycle-extension-must-be-declared` — `lifecycle:transition` key requires a declared `lifecycle` extension prefix | `xes::XesRefusal::UndeclaredExtensionPrefix` (fired first if extension not declared) | `compile_pass/xes_declared_extension_prefix.rs` | `compile_fail/xes_undeclared_extension_prefix_rejected.rs` | IEEE 1849-2023 §4 + §5.3 |

### XES-to-OCED projection boundary

The XES→OCED direction is a *lifting* projection: a flat, case-centric log is
elevated into an object-centric event data (OCED) structure. This is lossy in
the upward direction — not because data is discarded, but because structural
assumptions must be inferred (e.g. which case notion maps to which object type).
The result must carry a `LossReport` naming exactly what structural assumptions
were made. A bare `FormatExport` with an optional `LossReport` does not enforce
this — only `LossyFormatExport` with a mandatory report does.

Two compile-fail fixtures seal this boundary:
- `xes_to_oced_loss_report_rejected.rs` — the direct case: a caller passes `FormatExport`
  to `accept_lossy_xes_to_oced`, which requires `LossyFormatExport`. Rejected.
- `xes_to_oced_without_loss_policy.rs` — the indirect case: even a `FormatExport::lossy`
  carrying a `LossReport` is rejected because the Optional wrapper means the
  LossPolicy is not named at the type level.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `xes-to-oced-projection-boundary` — XES→OCED lifting requires `LossyFormatExport` (mandatory report), not `FormatExport` (optional) | `formats::LossyFormatExport` / `formats::accept_lossy_xes_to_oced` | `compile_pass/xes_to_oced_named_projection.rs` | `compile_fail/xes_to_oced_loss_report_rejected.rs` | van der Aalst & Berti (2020) OCED §3; IEEE 1849-2023 |
| `xes-to-oced-without-loss-policy` — `FormatExport::lossy` (optional wrapper) rejected where mandatory `LossyFormatExport` required | `formats::LossyFormatExport` mandatory type gate | `compile_pass/xes_to_oced_named_projection.rs` | `compile_fail/xes_to_oced_without_loss_policy.rs` | `xes_to_oced_without_loss_policy.stderr` |

### XES-to-OCEL direction law

XES→OCEL is structurally distinct from XES→OCED. OCEL 2.0 adds two first-class
link types absent from flat XES: event-to-object (E2O) links and object-to-object
(O2O) links. Lifting a XES log to OCEL requires not only inferring object types
(as in XES→OCED) but also manufacturing E2O links from the flat case notion.
This is a higher-loss, higher-assumption projection.

The structural distinction:
- XES→OCED: case notion → object type, single-case assumption inferred
- XES→OCEL: case notion → object type, E2O links manufactured, O2O links absent (gap)

Both directions require `LossyFormatExport`. The `xes_not_object_centric.rs`
compile-fail fixture seals the fundamental structural direction: a `XesCaseCentricLog`
cannot substitute an `OcelLog` at the type level, regardless of direction.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `xes-to-ocel-direction-law` — XES→OCEL lifting requires E2O link manufacturing and a `LossReport`; distinct from XES→OCED | `xes::XesLog` → `ocel::OcelLog` via `formats::LossyFormatExport` | `compile_pass/xes_case_centric_log.rs` | `compile_fail/xes_not_object_centric.rs` | OCEL 2.0 §3 E2O / O2O link model; van der Aalst & Berti (2020) |
| `xes-ocel-structural-gap` — a flat XES log has no O2O links; lifting to OCEL leaves O2O structurally absent | graduation boundary: `ocel::ObjectObjectLink` absent in XES; gap named in `LossReport` | `compile_pass/xes_case_centric_log.rs` | — (gap documented in LossReport, not a compile error) | OCEL 2.0 §3 O2O link model |

---

## Declare/OCPQ Law Packet — Unary Constraint Templates

**Paper family:** `DECLARE_CONSTRAINTS`
**Sources:** Pesic & van der Aalst (2006)

The Absence and Existence Declare templates are unary constraint templates: each requires exactly one activity argument (activation only, no target). Passing a unary template with a target activity, or a binary template with only one argument, is refused as `DeclareRefusal::InvalidTemplateArity`.

### absence-constraint

`DeclareTemplate::Absence` names the constraint "activity A does not occur." A constraint using `Absence` is constructed via `DeclareConstraint::unary`; passing it through `DeclareConstraint::binary` is a structural defect.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `declare_absence_constraint` — `DeclareTemplate::Absence` is a unary template; passing it as binary is a structural defect | `declare::DeclareTemplate::Absence` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | Pesic & van der Aalst (2006) §3 |

### existence-constraint

`DeclareTemplate::Existence` names the constraint "activity A occurs at least once." Like `Absence`, it is a unary template enforced structurally via `DeclareTemplate::arity()`.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `declare_existence_constraint` — `DeclareTemplate::Existence` is a unary template; arity mismatch is refused at the structural level | `declare::DeclareTemplate::Existence` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | Pesic & van der Aalst (2006) §3 |

### XES refusal law — structural exchange validation

`XesRefusal` is the complete enumeration of named structural laws under which a
XES interchange shape is refused. Each variant is a distinct named law — never a
bare `InvalidInput`. The `#[non_exhaustive]` attribute ensures the law set can be
extended in future standards revisions without breaking pattern matches.

The five core refusal variants cover the complete structural validation surface of
`XesLog::validate()`:
1. `MissingLogName` — log lacks a `concept:name`
2. `NoTraces` — log has no traces
3. `MissingTraceName` — trace lacks a `concept:name` (case id)
4. `EmptyTrace` — trace has no events
5. `MissingConceptName` — event lacks `concept:name`
6. `InvalidExtension` — extension has empty prefix
7. `UndeclaredExtensionPrefix` — attribute references undeclared extension
8. `InvalidTimestamp` — timestamp value is malformed
9. `InvalidLifecycleTransition` — lifecycle:transition value outside alphabet

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `xes-refusal-named-law` — every `XesRefusal` variant names a specific structural law, never bare `InvalidInput` | `xes::XesRefusal` (`#[non_exhaustive]` enum, 9 named variants) | `compile_pass/xes_case_centric_log.rs` | — (law enforced by enum shape: no catch-all variant) | IEEE 1849-2023 §4 validation; Verbeek et al. (2011) §3 |
| `xes-missing-log-name` — `XesRefusal::MissingLogName` is the named refusal for a log without `concept:name` | `xes::XesRefusal::MissingLogName` | `compile_pass/xes_case_centric_log.rs` | — (runtime refusal; shape check in `XesLog::validate()`) | IEEE 1849-2023 §5 log element |

### XES witness marker law

`Xes1849` in `src/witness.rs` is the named, non-forgeable receipt that ties a
structure admitted under the XES law surface to the IEEE 1849 standard. An
`Admission<T, Xes1849>` is distinguishable at the type level from
`Admission<T, Ocel20>` — confusing them is a compile error, not a runtime check.

The `witness_xes1849_marker.rs` compile-pass fixture seals the positive path:
`Xes1849::KEY == "xes-1849-2016"`, `Xes1849::FAMILY == WitnessFamily::Standard`,
`Xes1849::YEAR == Some(2016)`. These constants are the human-facing metadata layer
that lets diagnostics explain which authority a value was admitted against.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `xes-witness-marker-law` — `Xes1849` is the named authority label for all XES-admitted structures; distinct from `Ocel20` at the type level | `witness::Xes1849` (empty enum, implements `Witness`) | `compile_pass/witness_xes1849_marker.rs` | — (law enforced by distinct empty-enum identity; mixing requires explicit coercion) | IEEE 1849-2023; IEEE 1849-2016 |
| `xes-witness-family-standard` — `Xes1849::FAMILY == WitnessFamily::Standard`; XES is a published interchange standard, not a paper or API grammar | `witness::WitnessFamily::Standard` | `compile_pass/witness_xes1849_marker.rs` | — | IEEE 1849-2023 (standard, not paper) |

---

### choice-marker-law

**Law concept:** A `Choice` node (exclusive-choice / XOR operator) in POWL is a
first-class structural kind requiring at least two branches. The `Choice` witness
and `PowlNodeKind::Choice` enum variant are distinct types; confusing them with
`PowlNodeKind::Loop` or `ChoiceGraph` is a compile error. The minimum-branch
constant is fixed at 2 as a compile-time fact.

**Paper:** Kourani & van der Aalst (2023) POWL §3: the choice operator `×(M₁, M₂)`
takes two or more sub-models as operands. Fewer than two branches yields an
ill-formed model. POWL 2.0 (Kourani et al., 2026) replaces the flat `×` operator
with the `ChoiceGraph` variant for non-block-structured decisions, but the flat
`Choice` kind remains valid for simple binary or n-ary block-structured choices.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `choice-minimum-branch-law` — a `Choice` node requires at least two branches; `TypedNode<{PowlKind::Xor}>::min_branches()` returns 2 as a compile-time constant | `nightly_foundry::powl_law::TypedNode<{PowlKind::Xor}>` | `compile_pass/powl_choice_graph.rs` | `compile_fail/powl_order_edge_choice_confusion.rs` | Kourani & van der Aalst (2023) §3 |
| `choice-node-kind-distinctness` — `PowlNodeKind::Choice` is not `PowlNodeKind::ChoiceGraph`; the former is a flat list of branches, the latter is a directed graph; they are different enum variants | `powl::PowlNodeKind::Choice` vs `powl::PowlNodeKind::ChoiceGraph` | `compile_pass/powl_choice_graph.rs` | — (structural: enum variants are always distinct) | Kourani & van der Aalst (2023) §3; Kourani et al. (2026) Def. 3.6 |
| `choice-refusal-invalid-choice` — a `Choice` node with fewer than two branches is refused as `PowlRefusal::InvalidChoice` | `powl::PowlRefusal::InvalidChoice` | `compile_pass/powl_choice_graph.rs` | — (runtime refusal path) | Kourani & van der Aalst (2023) §3 |

**What must NOT live in this crate:**

- Choice resolution semantics (which branch fires at runtime — graduates to wasm4pm)
- Stochastic choice probability annotation (graduates to wasm4pm)
- Inductive miner choice node discovery (graduates to wasm4pm)

### XES multi-perspective law (gap surface)

Van der Aalst (2011) multi-perspective process mining defines four named
perspectives over XES logs: control-flow (activity sequence), resource
(`org:resource`, `org:role`, `org:group`), data (attribute values), and time
(timestamps). Each perspective is a first-class XES extension, not a stringly-typed
annotation.

This crate covers the time perspective (`time:timestamp` helper on `XesEvent`)
and the control-flow perspective (`concept:name` helper on `XesEvent` and
`XesTrace`). The resource perspective and data perspective are NOT yet typed as
distinct namespace markers — they are accessible only via the generic
`attribute(key)` accessor. This is a documented structural gap.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `xes-time-perspective` — `XesEvent::timestamp()` is the typed accessor for the time perspective; `time:timestamp` key is a standard extension attribute | `xes::XesEvent::timestamp()` | `compile_pass/xes_trace_attributes.rs` | — (structural law; key is stringly-typed, accessor returns `Option<&str>`) | van der Aalst (2011) multi-perspective; IEEE 1849-2023 §5.3 time extension |
| `xes-resource-perspective-gap` — `org:resource`, `org:role`, `org:group` are NOT yet typed as a distinct `ResourcePerspective` namespace; they are accessible only via `attribute()` | `xes::XesEvent::resource()` helper exists; typed `ResourcePerspective` namespace is the gap | `compile_pass/xes_trace_attributes.rs` | — (gap; no typed enforcement yet) | van der Aalst (2011) multi-perspective §3 resource dimension |
| `xes-control-flow-perspective` — `concept:name` on events and traces is the typed control-flow perspective accessor | `xes::XesEvent::concept_name()` / `xes::XesTrace::name()` | `compile_pass/xes_trace_attributes.rs` | `compile_fail/xes_undeclared_extension_prefix_rejected.rs` | van der Aalst (2011) multi-perspective; IEEE 1849-2023 §5.2 |

---

## Declare/OCPQ Law Packet — Object-Scoped Witness

**Paper family:** `DECLARE_CONSTRAINTS`
**Sources:** OC-Declare (van der Aalst, 2019); Pesic & van der Aalst (2006)

### object-scoped-witness

The `DeclareScope` enum (`SingleObjectScope`, `MultiObjectScope`, `SynchronizedObjectScope`) is the OC-Declare extension of classical Declare. Each scope variant is a first-class typed value, not an optional annotation or a stringly-typed configuration. A `DeclareConstraint` without a scope cannot exist — `DeclareScope` is a required field. An OC-Declare scope with zero object types is refused as `DeclareRefusal::EmptyObjectScope`.

The `DeclareFamily` witness in `src/witness.rs` is the non-forgeable named authority receipt that links any `Admission<T, DeclareFamily>` to the Declare constraint-template family, ensuring classical Declare constraints are not silently confused with OC-Declare constraints at the type level.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `declare_object_scoped_witness` — OC-Declare scope (`DeclareScope`) is a first-class typed scope, not an optional annotation | `declare::DeclareScope` / `witness::DeclareFamily` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | van der Aalst (2019) OC-Declare §2 |
| `declare_empty_scope_refused` — `DeclareScope` with zero object types is refused as `DeclareRefusal::EmptyObjectScope` | `declare::DeclareRefusal::EmptyObjectScope` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | van der Aalst (2019) OC-Declare §2 |
| `declare_synchronization_scope` — `SynchronizedObjectScope` requires all listed types share a joint lifecycle | `declare::DeclareScope::SynchronizedObjectScope` / `DeclareRefusal::SynchronizationViolation` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | van der Aalst (2019) OC-Declare §3 |

---

### loop-marker-law

**Law concept:** A loop node in POWL (`PowlNodeKind::Loop`) and in the process
tree (`TypedLoopNode<ARITY>`) is a first-class structural kind with a mandatory
`do` body. The POWL loop carries an optional `redo` body (absent means "execute
once with no rework"). The process-tree loop enforces exactly 2 children
(`do` body + `redo` branch) via a `Require<{ ARITY == 2 }>: IsTrue` where-bound.
A loop node with arity ≠ 2 does not compile.

**Paper:** Kourani & van der Aalst (2023) POWL §3: `↺(M₁, M₂)` is the loop
operator — `M₁` is the `do` body (always executes at least once), `M₂` is the
`redo` body (optional rework). Leemans (2013) inductive miner defines the
process-tree loop operator as requiring exactly two children: the `do` subtree
and the `redo` subtree.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `loop-marker-law` — `PowlNodeKind::Loop` is a first-class kind requiring a declared `do` body; absence is refused as `PowlRefusal::InvalidLoop` | `powl::PowlNodeKind::Loop` / `powl::PowlRefusal::InvalidLoop` | `compile_pass/process_tree_loop_arity_2.rs` | `compile_fail/process_tree_bad_loop_arity.rs` | Kourani & van der Aalst (2023) §3 |
| `process-tree-loop-arity-2` — `TypedLoopNode<Children, ARITY>` enforces `ARITY == 2`; `TypedLoopNode<_, 3>` does not compile | `process_tree::TypedLoopNode<Children, ARITY>` with `Require<{ ARITY == 2 }>: IsTrue` | `compile_pass/process_tree_loop_arity_2.rs` | `compile_fail/process_tree_bad_loop_arity.rs` | Leemans (2013) inductive miner; process-tree loop definition |
| `loop-do-body-required` — the `do` body of a loop node must be declared; a loop with missing `do` is `PowlRefusal::InvalidLoop` | `powl::PowlRefusal::InvalidLoop` | `compile_pass/process_tree_loop_arity_2.rs` | — (runtime refusal path; compile-fail covers arity) | Kourani & van der Aalst (2023) §3 |
| `loop-redo-optional` — the `redo` body is `Option<PowlNodeId>`; `None` means execute once with no rework; this is not a structural defect | `powl::PowlNodeKind::Loop::redo: Option<PowlNodeId>` | `compile_pass/process_tree_loop_arity_2.rs` | — (lawful path; no negative fixture needed) | Kourani & van der Aalst (2023) §3 |

**What must NOT live in this crate:**

- Loop unrolling or loop execution (execute-do-redo semantics — graduates to wasm4pm)
- Loop fitness measurement (how often the redo branch is taken — graduates to wasm4pm)
- Loop detection from event logs (inductive miner loop discovery — graduates to wasm4pm)

### XES law family — graduation boundary

This section enumerates what must NOT live in this crate for the XES law family,
consistent with the crate's structure-only, graduation-boundary doctrine.

**Engine concerns that graduate to `wasm4pm`:**
- XES file I/O (`.xes` / `.xes.gz` parsing and serialization)
- XES validator execution beyond structural shape (semantic extension evaluation)
- XES classifier evaluation (event-class identity at runtime)
- XES discovery algorithm execution (Alpha, Inductive, Heuristic miners)
- XES conformance checking execution (token replay, alignment over XES logs)
- XES→OCEL lifting algorithm (object-type inference from flat case notion)
- XES→OCED lifting algorithm (object-centric assumption inference)
- Multi-perspective conformance (resource/data constraint evaluation)
- Social network mining from `org:resource` perspective
- Decision mining from data perspective attributes

**What stays in this crate:**
- `XesLog`, `XesTrace`, `XesEvent` shape definitions
- `XesExtension` declaration shape
- `CaseCentricMarker` type-level distinction from OCEL/OCED
- `XesRefusal` named-law enumeration
- `Xes1849` witness authority label
- `accept_lossy_xes_to_oced` / `accept_lossy_ocel_to_xes` type gates
- `LossyFormatExport` mandatory-report type for all XES projection boundaries

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `xes-graduation-boundary` — XES engine logic (discovery, conformance, file I/O) is absent from this crate; graduation to `wasm4pm` is required | graduation boundary: `xes::XesLog` is structure-only; module doc enforces no engine logic | `compile_pass/xes_case_centric_log.rs` | `compile_fail/engine_creep_discovery_absent.rs` | IEEE 1849-2023 §1 scope; van der Aalst (2016) process mining graduation |

### XES round-trip claim law

The XES format has a round-trip claim: `import(export(x)) ~ x` for a named fixture.
`RoundTripClaim` in `src/formats.rs` is the structure-only promissory note. It does
not perform the round trip; it names the fixture under which the equivalence is
asserted so a test (in `tests/format_contracts.rs`) can discharge it.

For XES specifically: round-trip is `lossy_tolerant` (whitespace and attribute
ordering normalization are expected). A round-trip claim that does not name a fixture
is not a real claim (`is_named()` returns false for empty fixture strings).

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `xes-round-trip-claim` — XES import/export round trip is a named claim, not a guarantee; `RoundTripClaim::lossy_tolerant` names the fixture | `formats::RoundTripClaim` | `compile_pass/formats_round_trip_claim.rs` | — (law enforced by `is_named()` check; unnamed claims are rejected by tests) | IEEE 1849-2023 §4 interchange fidelity |
| `xes-format-kind-tag` — `FormatKind::XesXml` is a distinct tag from `FormatKind::OcelJson`; a XES envelope cannot be silently treated as OCEL | `formats::FormatKind::XesXml` | `compile_pass/formats_envelope_shape.rs` | — (distinct enum variant; no implicit coercion) | IEEE 1849-2023 §3 XES XML serialization |
| `xes-format-not-object-centric` — `FormatKind::XesXml.is_object_centric()` returns `false`; XES is structurally distinct from OCEL formats | `formats::FormatKind::is_object_centric()` | `compile_pass/formats_envelope_shape.rs` | — (structural law; method returns const bool) | IEEE 1849-2023; OCEL 2.0 §1 (object-centric vs. case-centric) |

---

## Declare/OCPQ Law Packet — OCPQ Object Scope and Event Predicate

**Paper family:** `OCPQ_QUERYING`
**Sources:** OCPQ (Ghahfarokhi et al., 2024); van der Aalst (2019)

### OCPQ-object-scope

`ObjectScope` is the declaration of which object types a query ranges over. An `OcpqQuery` without an `ObjectScope` (or with an empty scope) is refused as `OcpqRefusal::MissingObjectScope`. This is the structural grounding of OCPQ Definition 6 (Def 6): a query that does not name any object type is not a query over an object-centric log — it is a structural defect.

The scope check is the first gate in OCPQ admission: before any predicate is evaluated, the scope must be non-empty. A scope referencing an object type not present in the admitted log is refused as `OcpqRefusal::UnknownObjectType`.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocpq_object_scope_law` — `ObjectScope` with zero object types is refused as `OcpqRefusal::MissingObjectScope` | `ocpq::ObjectScope` / `ocpq::OcpqRefusal::MissingObjectScope` | `compile_pass/ocpq_scoped_query.rs` | `compile_fail/ocpq_missing_scope_rejected.rs` | OCPQ §3 Def 6 |
| `ocpq_unknown_object_type_refused` — scope referencing undeclared object type is refused as `OcpqRefusal::UnknownObjectType` | `ocpq::OcpqRefusal::UnknownObjectType` | `compile_pass/ocpq_scoped_query.rs` | `compile_fail/ocpq_missing_scope_rejected.rs` | OCPQ §3 Def 6 |

### event-predicate

`Predicate<EventPredicate>` is the typed marker for predicates over single events. The witness `EventPredicate` (a zero-sized `struct`) prevents an event predicate from being substituted for an object predicate at the type level — `Predicate<EventPredicate>` and `Predicate<ObjectPredicate>` are distinct types.

`PredicateKind::Event(String)` is the opaque event condition (retained for compatibility). For OCPQ Section 4 typed predicates, use `PredicateKind::E2ORelation` (event-to-object link) instead of a free-form string.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocpq_event_predicate_law` — `Predicate<EventPredicate>` is a first-class typed predicate; `EventPredicate` cannot substitute `ObjectPredicate` | `ocpq::Predicate<EventPredicate>` / `ocpq::EventPredicate` | `compile_pass/ocpq_scoped_query.rs` | `compile_fail/ocpq_object_type_mixing.rs` | OCPQ §4 BASIC_L |

---

### irreducible-state-law

**Law concept:** A POWL partial-order node is `Irreducible` when its structure
cannot be represented by any block-structured process tree without language loss.
The `Irreducible` marker and `ExceedsProcessTree` marker are first-class witness
types. A node tagged `ExceedsProcessTree` cannot pass through a `TreeProjectable`-
gated function — this is a sealed-trait law enforced at compile time.

**Paper:** Kourani & van der Aalst (2023) POWL §3 and §4: POWL can express
partial orders that no block-structured tree can represent. The paper defines the
`ProcessTreeProjectable` sub-class of POWL models as those whose language is
expressible as a block-structured process tree. Models outside this sub-class
carry the `ExceedsProcessTree` marker — projection would silently lose language.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `irreducible-state-law` — `Irreducible` and `ExceedsProcessTree` are first-class witness markers on `PowlNode<W>` | `powl::Irreducible` / `powl::ExceedsProcessTree` | `compile_pass/powl_process_tree_projectable.rs` | `compile_fail/powl_silent_tree_projection.rs` | Kourani & van der Aalst (2023) §3–4 |
| `exceeds-process-tree-sealed` — `ExceedsProcessTree` does NOT implement `TreeProjectable`; passing it to a `TreeProjectable`-gated function is a compile error | `powl::TreeProjectable` (sealed trait; only `ProcessTreeProjectable` implements it) | `compile_pass/powl_process_tree_projectable.rs` | `compile_fail/powl_silent_tree_projection.rs` | Kourani & van der Aalst (2023) §4 |
| `process-tree-projectable-sealed` — `ProcessTreeProjectable` is the only implementor of `TreeProjectable`; the sealed trait prevents external forgery | `powl::TreeProjectable` sealed via `tree_projectable_seal::Sealed` | `compile_pass/powl_process_tree_projectable.rs` | `compile_fail/powl_silent_tree_projection.rs` | Kourani & van der Aalst (2023) §4 |

**What must NOT live in this crate:**

- Irreducibility detection algorithm (determining whether a POWL model exceeds
  any process tree — graduates to wasm4pm)
- Process-tree induction from an irreducible partial order
- Language equivalence checking between POWL and process tree shapes

---

## Declare/OCPQ Law Packet — Relation Predicate

**Paper family:** `OCPQ_QUERYING`
**Sources:** OCPQ (Ghahfarokhi et al., 2024)

### relation-predicate

`Predicate<RelationPredicate>` is the typed marker for predicates over event-to-object (E2O) or object-to-object (O2O) links. OCPQ Section 4 (BASIC_L) introduces two typed relation predicate variants:

- `PredicateKind::E2ORelation { event_var, object_var, qualifier? }` — asserts that a named event is related to a named object, optionally via a qualifier.
- `PredicateKind::O2ORelation { object_var1, object_var2, qualifier? }` — asserts that two named objects are related, optionally via a qualifier.

These replace the opaque `Relation(String)` variant for call sites where the link type is known. The `RelationPredicate` witness prevents E2O predicates from being silently substituted for O2O predicates — they are structurally distinct because `E2ORelation` carries an `event_var` while `O2ORelation` carries two `object_var` fields.

A `TimeBetweenEvents` predicate (TBE) is also in `RelationPredicate` family: it asserts that the duration between two named events lies in `[t_min, t_max]`. Structure-only; temporal evaluation graduates to `wasm4pm`.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocpq_relation_predicate_law` — `Predicate<RelationPredicate>` with `E2ORelation` and `O2ORelation` variants enforces typed link-kind distinction at the structural level | `ocpq::Predicate<RelationPredicate>` / `ocpq::PredicateKind::E2ORelation` / `ocpq::PredicateKind::O2ORelation` | `compile_pass/ocpq_typed_relation.rs` | `compile_fail/ocpq_object_type_mixing.rs` | OCPQ §4 BASIC_L |
| `ocpq_tbe_predicate_law` — `PredicateKind::TimeBetweenEvents` names time-between-events constraint with `[t_min, t_max]` bounds | `ocpq::PredicateKind::TimeBetweenEvents` | `compile_pass/ocpq_typed_relation.rs` | — (structure-only, evaluation graduates to wasm4pm) | OCPQ §4 BASIC_L TBE |

**What must NOT live in this crate:**

- E2O/O2O link resolution against a log (variable binding, traversal)
- TBE evaluation (timestamp difference computation)
- Relation predicate cardinality aggregation

---

### projection-refusal-law

**Law concept:** When a POWL model with an `Irreducible` partial order is asked
to project to a process tree, the refusal is named `PowlRefusal::IrreducibleProjection`.
On the process-tree side, the mirror law is `ProcessTreeRefusal::UnsupportedProjection`.
Both names are specific structural laws — neither is a bare `InvalidInput` or
untyped error string. The `assert_tree_projectable` gate enforces this at the
type level: a caller must pass a `ProcessTreeProjectable`-witnessed marker, not an
`ExceedsProcessTree`-witnessed one.

**Paper:** Kourani & van der Aalst (2023) POWL §4: conversion POWL → process tree
is only defined for the `ProcessTreeProjectable` sub-class. Attempting conversion
outside this sub-class must be an explicit, named refusal — not a silent failure.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `projection-refusal-law` — `PowlRefusal::IrreducibleProjection` is the named law for POWL-to-tree projection failure | `powl::PowlRefusal::IrreducibleProjection` | `compile_pass/powl_process_tree_projectable.rs` | `compile_fail/powl_silent_tree_projection.rs` | Kourani & van der Aalst (2023) §4 |
| `process-tree-unsupported-projection` — `ProcessTreeRefusal::UnsupportedProjection` is the mirror law on the tree side when a foreign shape (e.g. a POWL irreducible) cannot be represented | `process_tree::ProcessTreeRefusal::UnsupportedProjection` | `compile_pass/powl_process_tree_projectable.rs` | — (same gate as IrreducibleProjection; mirror refusal) | Kourani & van der Aalst (2023) §4 |
| `assert-tree-projectable-gate` — `assert_tree_projectable<P: TreeProjectable>` is the type-law gate; it is not a runtime assertion, it is a compiler proof obligation | `powl::assert_tree_projectable` (sealed function) | `compile_pass/powl_process_tree_projectable.rs` | `compile_fail/powl_silent_tree_projection.rs` | Kourani & van der Aalst (2023) §4 |

**What must NOT live in this crate:**

- POWL-to-process-tree language-preserving reduction algorithm
- Tau-loop insertion for block-structuredness recovery
- Language loss measurement during projection

---

## Declare/OCPQ Law Packet — Cardinality Bound Law

**Paper family:** `OCPQ_QUERYING`
**Sources:** OCPQ (Ghahfarokhi et al., 2024)

### cardinality-bound-law

`Predicate<CardinalityPredicate>` with `PredicateKind::Cardinality { min, max }` asserts a count bound: between `min` and `max` bindings (inclusive) must satisfy the surrounding query node. The structural law requires `min <= max`. A cardinality predicate with `min > max` is refused as `OcpqRefusal::InvalidCardinality`.

The anonymous `Cardinality` variant is for general count bounds. When the bound is over a named child branch, use `PredicateKind::ChildSetBound` instead (see typed-child-set-law below).

The `Between01<NUM, DEN>` law from `src/law.rs` governs cardinality bounds when expressed as a fraction of total object count. `ocpq_cardinality_overflow` and `ocpq_cardinality_rejected` compile-fail fixtures seal the arithmetic overflow and inversion laws.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocpq_cardinality_bound_law` — `Predicate<CardinalityPredicate>` with `Cardinality { min, max }` requires `min <= max`; violated bound is refused as `OcpqRefusal::InvalidCardinality` | `ocpq::Predicate<CardinalityPredicate>` / `ocpq::OcpqRefusal::InvalidCardinality` | `compile_pass/ocpq_cardinality_valid_bounds.rs` | `compile_fail/ocpq_cardinality_rejected.rs` | OCPQ §4 cardinality constraints |
| `ocpq_cardinality_overflow` — OCPQ `CardinalityBound` const-generic overflow at type level | `ocpq::CardinalityBound` / `law::Between01<NUM, DEN>` | `compile_pass/ocpq_cardinality_valid_bounds.rs` | `compile_fail/ocpq_cardinality_overflow.rs` | OCPQ §4 + `law::Between01` |

**What must NOT live in this crate:**

- Cardinality predicate evaluation (counting bindings at runtime)
- Adaptive bound derivation from log frequency statistics

---

### POWL-to-process-tree boundary

**Law concept:** The conversion from a WF-net to a POWL model (via Kourani et al.,
2026 Theorem 4.3) requires the `SeparableWfNet` precondition. The resulting POWL
model is certified by a `WfNet2PowlWitness` — a non-forgeable struct with a
private seal. The subsequent projection from POWL to a process tree requires
`TreeProjectable`. Together these form a two-step boundary: `WfNet → POWL` requires
separability; `POWL → ProcessTree` requires projectability.

**Paper:** Kourani, Park & van der Aalst (2026) Theorem 4.3: a separable WF-net
can be converted to a POWL 2.0 model while preserving the process language. The
`WfNet2PowlWitness` records that the conversion happened under the separability
precondition; it cannot be forged externally.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `wfnet2powl-separability-required` — WF-net→POWL conversion requires `SeparableWfNet` marker; plain `WfNetConst` is rejected | `petri::SeparableWfNet` precondition | `compile_pass/wfnet2powl_witness.rs` | `compile_fail/wfnet2powl_precondition_rejected.rs` | Kourani et al. (2026) Thm 4.3 |
| `wfnet2powl-wrong-source` — plain `PetriNet` cannot enter the WF-net→POWL gate; `WfNetConst` is required | `petri::WfNetConst` required | `compile_pass/wfnet2powl_witness.rs` | `compile_fail/wfnet2powl_wrong_source.rs` | Kourani et al. (2026) Thm 4.3 |
| `wfnet2powl-witness-non-forgeable` — `WfNet2PowlWitness` has a private seal; it cannot be constructed outside the `powl` module or the `wasm4pm` bridge | `powl::WfNet2PowlWitness` private `_seal` field | `compile_pass/wfnet2powl_witness.rs` | — (forgery is a compiler error: private field) | Kourani et al. (2026) Thm 4.3 |
| `powl-to-tree-projectable-required` — POWL→ProcessTree projection requires `TreeProjectable`; `ExceedsProcessTree` cannot pass the gate | `powl::TreeProjectable` (sealed) | `compile_pass/powl_process_tree_projectable.rs` | `compile_fail/powl_silent_tree_projection.rs` | Kourani & van der Aalst (2023) §4 |

**What must NOT live in this crate:**

- WF-net separability check execution (separability detection algorithm)
- WF-net→POWL decomposition algorithm execution (graduates to wasm4pm)
- POWL language equivalence verification after conversion

---

## Declare/OCPQ Law Packet — Typed Child Set Law

**Paper family:** `OCPQ_QUERYING`
**Sources:** OCPQ (Ghahfarokhi et al., 2024)

### typed-child-set-law

`PredicateKind::ChildSetBound` is the OCPQ Section 4 Child Set (CBS) predicate: it asserts that a parent node has between `min` and `max` child bindings satisfying the branch named `branch_label`. Unlike `PredicateKind::Cardinality` (an anonymous count bound), `ChildSetBound` requires a non-empty `branch_label` — the branch name is structurally required.

Two structural laws apply:
1. `branch_label` must be non-empty — a CBS predicate without a branch name cannot be grounded to a specific child branch.
2. `min <= max` — a CBS predicate with an inverted bound is refused as `OcpqRefusal::InvalidChildSetBound`.

`OcpqRefusal::InvalidChildSetBound` is the named first-class refusal law for CBS violations — it is a specific structural law, not a generic `InvalidInput`.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `ocpq_typed_child_set_law` — `PredicateKind::ChildSetBound` requires a non-empty `branch_label` and `min <= max`; violated is refused as `OcpqRefusal::InvalidChildSetBound` | `ocpq::PredicateKind::ChildSetBound` / `ocpq::OcpqRefusal::InvalidChildSetBound` | `compile_pass/ocpq_cbs_predicate.rs` | `compile_fail/ocpq_cardinality_overflow.rs` | OCPQ §4 CBS |
| `ocpq_cbs_empty_label_refused` — CBS predicate with empty `branch_label` is refused as `OcpqRefusal::InvalidChildSetBound` | `ocpq::PredicateKind::ChildSetBound` (`branch_label` non-empty invariant) | `compile_pass/ocpq_cbs_predicate.rs` | — (gap: empty-label compile-fail fixture TBD) | OCPQ §4 CBS |

**What must NOT live in this crate:**

- CBS evaluation (child-binding counting against a log)
- CBS branch-label resolution against query variables
- CBS predicate optimization (query planning)

---

### silent-node-law

**Law concept:** `SilentTransition` / `PowlNodeKind::Silent` is a first-class
POWL node kind. It is not an annotation on an activity atom — it is a distinct
structural kind with zero observable activity. The `TypedNode<{PowlKind::Silent}>`
in `nightly_foundry::powl_law` formalises this: `is_observable()` returns `false`
as a compile-time constant. Assigning an atom to a silent-typed binding, or vice
versa, is a compile error.

**Paper:** Kourani & van der Aalst (2023) POWL §3: `τ` (silent transition / tau step)
is a first-class element of the POWL grammar — `POWL ::= A | ×(M₁, M₂) | ↺(M₁, M₂)
| P(M⁺, ≺) | τ`. Silent transitions are used to model administrative steps, routing
decisions, or empty paths that carry no observable label. They are never collapsed
to atoms silently.

| Law | Enforcing Type | Pass Fixture | Fail Fixture | Paper Source |
|---|---|---|---|---|
| `silent-node-kind-law` — `PowlNodeKind::Silent` is a distinct variant; it is not a `Transition` with an empty label | `powl::PowlNodeKind::Silent` (distinct enum variant) | `compile_pass/powl_process_tree_projectable.rs` | `compile_fail/powl_silent_tree_projection.rs` | Kourani & van der Aalst (2023) §3 |
| `silent-node-not-observable` — `TypedNode<{PowlKind::Silent}>::is_observable()` returns `false`; `TypedNode<{PowlKind::Atom}>::is_observable()` returns `true` | `nightly_foundry::powl_law::TypedNode<{PowlKind::Silent}>` | `compile_pass/powl_process_tree_projectable.rs` | — (compile-fail fixture: atom assigned to silent binding — `powl_law` module doctest) | Kourani & van der Aalst (2023) §3 |
| `silent-atom-type-distinctness` — `TypedNode<{PowlKind::Atom}>` and `TypedNode<{PowlKind::Silent}>` are different types; assignment or substitution is a compile error | `nightly_foundry::powl_law::TypedNode` const-generic type distinction | — (law enforced by const generic: distinct const param values → distinct types) | `compile_fail` doctest in `nightly_foundry::powl_law` | Kourani & van der Aalst (2023) §3 |

**What must NOT live in this crate:**

- Silent transition elimination (tau-loop reduction algorithm — graduates to wasm4pm)
- Observability checking over POWL models at runtime
- Silent-step language projection (trace-level vs. event-level projection)
