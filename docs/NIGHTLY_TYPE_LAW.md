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
| `declare_response_constraint` — `DeclareTemplate::Response` is a first-class binary template, not a string | `declare::DeclareTemplate::Response` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `declare_precedence_constraint` — `DeclareTemplate::Precedence` is a first-class binary template | `declare::DeclareTemplate::Precedence` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `declare_succession_constraint` — `DeclareTemplate::Succession` = Response ∧ Precedence; enforced as a single typed template | `declare::DeclareTemplate::Succession` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `declare_absence_constraint` — `DeclareTemplate::Absence` is a unary template; passing it as binary is a structural defect | `declare::DeclareTemplate::Absence` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `declare_existence_constraint` — `DeclareTemplate::Existence` is a unary template; arity mismatch is refused at the structural level | `declare::DeclareTemplate::Existence` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `declare_object_scoped_witness` — OC-Declare scope (`DeclareScope`) is a first-class typed scope, not an optional annotation | `declare::DeclareScope` / `witness::DeclareFamily` | `compile_pass/declare_constraint_shape.rs` | `compile_fail/declare_binary_arity_rejected.rs` | `declare_binary_arity_rejected.stderr` |
| `ocpq_object_scope_law` — `ObjectScope` with zero object types is refused as `OcpqRefusal::MissingObjectScope` | `ocpq::ObjectScope` / `ocpq::OcpqRefusal::MissingObjectScope` | `compile_pass/ocpq_scoped_query.rs` | `compile_fail/ocpq_missing_scope_rejected.rs` | `ocpq_missing_scope_rejected.stderr` |
| `ocpq_event_predicate_law` — `Predicate<EventPredicate>` is a first-class typed predicate; `EventPredicate` cannot substitute `ObjectPredicate` | `ocpq::Predicate<EventPredicate>` / `ocpq::EventPredicate` | `compile_pass/ocpq_scoped_query.rs` | `compile_fail/ocpq_object_type_mixing.rs` | `ocpq_object_type_mixing.stderr` |
| `ocel_v1_e2o_required` — OCEL 1.0 event-to-object link is a first-class structural law (not optional annotation) | `ocel::EventObjectLink` / `ocel::OcelEvent` | `compile_pass/ocel_event_object_relation.rs` | `compile_fail/ocel_e2o_missing_link.rs` | `ocel_e2o_missing_link.stderr` |
| `yawl_cancellation_region_rejected` — raw `Vec<String>` not accepted as `CancellationRegion` | `yawl::CancellationRegion` newtype | `compile_pass/yawl_cancellation_region.rs` | `compile_fail/yawl_cancellation_region_rejected.rs` | `yawl_cancellation_region_rejected.stderr` |
| `yawl_multi_instance_bounds_rejected` — `MultipleInstanceSpecConst<MIN, MAX>` enforces MIN ≤ MAX | `yawl::MultipleInstanceSpecConst<MIN, MAX>` | `compile_pass/yawl_multi_instance.rs` | `compile_fail/yawl_multi_instance_bounds_rejected.rs` | `yawl_multi_instance_bounds_rejected.stderr` |
| `yawl_wrong_task_type` — `MultipleInstanceSpecConst` not accepted where `CancellationRegion` required | `yawl::CancellationRegion` / `yawl::MultipleInstanceSpecConst` (distinct) | `compile_pass/yawl_cancellation_region.rs` | `compile_fail/yawl_wrong_task_type.rs` | `yawl_wrong_task_type.stderr` |

---

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
