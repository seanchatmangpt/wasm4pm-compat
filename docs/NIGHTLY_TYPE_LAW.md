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
