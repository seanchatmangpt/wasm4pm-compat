# Doc Coverage Log — Combinatorial Maximalism Loop

## 2026-06-14 — Iteration 1

### State
- Commit: d62a119 (clean tree before this iteration)
- Toolchain: nightly-aarch64-apple-darwin

### Coverage Map

#### Exercised surface (examples/ files, 19 total)
| Example | Primary APIs demonstrated |
|---|---|
| `basic_eventlog` | eventlog (Event, Trace, EventLog, EventStream) |
| `basic_ocel` | ocel (OcelEvent, OcelLog, Object, EventObjectLink, ObjectObjectLink, ObjectChange) |
| `evidence_lifecycle` | evidence (Evidence::raw), admission (Admit, Admission, Refusal) |
| `ocel_to_xes_projection` | formats (FormatExport, FormatKind, ImportFormat), loss (LossPolicy, LossReport) |
| `loss_projection` | loss (LossPolicy, ProjectionName, LossReport), ocel |
| `strict_boundary_claim` | strict (ProcessBoundary, StrictCheck, StrictViolation) |
| `graduation_candidate` | wasm4pm feature (GraduateToWasm4pm, GraduationCandidate, GraduationReason) |
| `witness_authority` | witness (Ocel20, WitnessFamily, WfNetSoundnessPaper) |
| `petri_net_construction` | petri (PetriNet, Place, Transition, Arc, Marking) |
| `powl_process_tree` | powl (Powl, PowlChoiceNode, PowlNode, TypedLoopNode, ProcessTree) |
| `conformance_metrics` | conformance (Metric, FitnessConst, PrecisionConst, QualityProfile) |
| `declare_constraint_model` | declare (DeclareModel, DeclareConstraint, DeclareTemplate, DeclareScope, DeclareRefusal) |
| `causal_net_shape` | causality (CausalNet, CausalBinding, DependencyMeasure) |
| `ocpq_typed_query` | ocpq (CardinalityBoundConst, ChildSetBoundConst, EventPredicate, ObjectScope) |
| `receipt_chain` | receipt (ReceiptChain, ReceiptEnvelope, ReceiptChainConst, GraduationReceipt, Digest) |
| `c8_adversary_gap_demo` | adversary (LogicPlayer, GraphPlayer, GapVerdict) |
| `c8_collider_demo` | internal collider model |
| `c8_event_horizon_demo` | internal event horizon model |
| `c8_market_planck_demo` | internal market planck model |
| `sealing_admit_chain` ★ NEW | admission seam: SealingAdmit, recompute_and_match, ChainProof, RuntimeSeal, SealedAdmission, AffidavitReceiptChain |

#### Documented-but-unexercised (GAP — highest priority)

| Module | Pub items | Gap type |
|---|---|---|
| `prediction` | 12 | NO example: PredictionHorizon, PredictionProblem, PredictionTarget, ComplianceTarget, PredictionRefusal |
| `streaming` | 8 | NO example: StreamingConformance, EventWindow |
| `process_cube` | 8 | NO example: ProcessCubeSlice, ProcessCubeDimension |
| `multiperspective` | 8 | NO example: MultiperspectiveProfile |
| `dfg` | 8 | NO example: DirectlyFollowsGraph, DfgArc |
| `bpmn` | 10 | NO example: BpmnProcess, BpmnActivity, BpmnGateway |
| `interop` | 16 | NO example: interop connectors |
| `workflow` | 9 | NO example: WorkflowModel |
| `object_lifecycle` | 10 | NO example: ObjectLifecycleModel |
| `xes` | 12 | partially via ocel_to_xes_projection; XES import path unexercised |
| `models` | 12 | NO example |
| `import/xes/stream_xes` | 7 | NO example |

#### Exercised-but-undocumented
- `c8_adversary_gap_demo`, `c8_collider_demo`, `c8_event_horizon_demo`, `c8_market_planck_demo` — demonstrate internal market-physics / adversary modules not documented in CLAUDE.md examples table or API_TOUR.

### Triple closed this iteration

**Capability cluster: SealingAdmit chain-sealing seam (v26.6.14)**

| Part | Status |
|---|---|
| Doc | `src/admission.rs` rustdoc for all 6 new pub items; `docs/API_TOUR.md` §"Chain-sealing admission" section added |
| Example | `examples/sealing_admit_chain.rs` |
| Link | CLAUDE.md examples table updated; API_TOUR references example by path |

**Run output (real exit code captured):**
```
=== All assertions passed — SealingAdmit seam is witnessed ===
  Claims: recompute_and_match + ChainProof + RuntimeSeal + SealingAdmit + SealedAdmission
  Witness: every assertion above; breaks if any claim regresses.
EXIT:0
```

**Why this example is not doc-laundering:** it asserts `proof.is_ok()`, the tampered proof `.is_err()`, `seal.digest() == &claimed`, `sealed.value == good_payload`, and both named refusal patterns via `matches!`. A broken `recompute_and_match`, `RuntimeSeal`, or `SealingAdmit` impl makes at least one assert fail and the example exits non-zero.

---

## 2026-06-14 — Iteration 2

**Cluster:** `prediction` module (12 pub items, no standalone example)

**Triple:**
- **Doc:** `src/prediction.rs` rustdoc (already complete with per-fn doctests)
- **Example:** `examples/prediction_problem_shape.rs` — exercises `PredictionHorizon` × 3, `PredictionTarget` × 6, `PredictionProblem<T>` builder chain, `ComplianceKind` × 3, `PredictionRefusal` × 6 named laws, all phantom witness markers
- **Link:** README.md and CLAUDE.md example table updated to include this example

**Run output (real exit code):**
```
=== All assertions passed — prediction module surface is witnessed ===
  Documented: PredictionHorizon, PredictionTarget, PredictionProblem<T>,
              ComplianceKind, PredictionRefusal (6 named laws)
  Witness: Display strings + field values asserted; breaks on rename or removal.
EXIT: 0
```

**Covered ✅:** `prediction` module — documented-but-unexercised gap CLOSED.

**Gap map update:**
- `prediction` → COVERED ✅ (example runs, all 6 refusal laws asserted)
- Remaining documented-but-unexercised: `streaming`, `process_cube`, `multiperspective`, `dfg`, `bpmn`, `interop`, `workflow`, `object_lifecycle`, `models`

### Queued (next iterations)

Priority 1 — `dfg` module (DirectlyFollowsGraph is a foundation for most discovery algorithms, no example)
Priority 2 — `bpmn` module (BpmnProcess/BpmnGateway/BpmnPool + named BpmnRefusal)
Priority 3 — `models` module (PetriNet structural metrics: explain(), structural_unsoundness_score())
Priority 4 — `streaming` module (StreamingConformance, EventWindow)
Priority 5 — Cross-product example: OcelLog → admission → named projection → receipt chain (the canonical three-module pipeline, no composition example exists)

### Hard stops
None this iteration. Disk: not checked (no ENOSPC encountered).

---

## 2026-06-14 — Iteration 3

**Cluster:** `dfg` module (DFG is the most foundational undocumented surface — used as input to all process discovery algorithms)

**Triple:**
- **Doc:** `src/dfg.rs` rustdoc (already complete with IS/IS-NOT/graduation structure)
- **Example:** `examples/dfg_shape.rs` — exercises `DfgNode`, `DfgEdge`, `DfgWeight`, `Dfg::validate()`, both `DfgRefusal` named laws (`EmptyGraph`, `DanglingEdge`), `DfgEdgeFull` with/without duration, `ObjectCentricDfg` per-type DFG map
- **Link:** README.md and CLAUDE.md example table updated

**Run output (real exit code):**
```
=== All assertions passed — dfg module is witnessed ===
  Covered: DfgNode, DfgEdge, DfgWeight, Dfg::validate, DfgRefusal × 2,
           DfgEdgeFull (with/without duration), ObjectCentricDfg.
  Witness: validate() called on both valid and invalid graphs; named laws asserted.
EXIT: 0
```

**Covered ✅:** `dfg` module — documented-but-unexercised gap CLOSED.

**Gap map update:**
- `dfg` → COVERED ✅
- Remaining documented-but-unexercised: `bpmn`, `models`, `streaming`, `process_cube`, `multiperspective`, `workflow`, `object_lifecycle`

### Queued (next iterations)

Priority 1 — `bpmn` module (BpmnProcess/BpmnGateway/BpmnPool + named BpmnRefusal — rich refusal surface)
Priority 2 — `models` module (PetriNet structural metrics: `explain()`, `structural_unsoundness_score()`)
Priority 3 — Cross-product example: OcelLog → Dfg (shape) → conformance verdict — the canonical pipeline composition
Priority 4 — `streaming` module (StreamingConformance, EventWindow)
Priority 5 — `process_cube` / `multiperspective` (check if these are significant API surfaces or thin wrappers)

---

## 2026-06-14 — Iteration 4

**Cluster:** `bpmn` module — richest undocumented refusal surface (8 named laws)

**Triple:**
- **Doc:** `src/bpmn.rs` rustdoc (complete with IS/IS-NOT/graduation + per-fn doctests)
- **Example:** `examples/bpmn_process_shape.rs` — exercises `BpmnTask`, `BpmnGateway` ×5, `BpmnEvent` ×4, `BpmnNode` ×3 constructors, `BpmnEdge`, `BpmnProcess::validate()` (valid + 5 named refusals triggered directly), all 8 `BpmnRefusal` Display names, `BpmnLane`, `BpmnPool::validate()`, `LaneNodeNotDeclared`
- **Link:** README.md and CLAUDE.md example tables updated

**Run output (real exit code):**
```
=== All assertions passed — bpmn module is witnessed ===
  Covered: BpmnTask, BpmnGateway × 5, BpmnEvent × 4, BpmnNode × 3 constructors,
           BpmnEdge, BpmnProcess::validate, BpmnRefusal × 8 named laws,
           BpmnLane, BpmnPool::validate, LaneNodeNotDeclared.
  Witness: validate() returns exact named law per bad input; breaks on any rename.
EXIT: 0
```

**Covered ✅:** `bpmn` module — documented-but-unexercised gap CLOSED.

**Gap map update:**
- `bpmn` → COVERED ✅
- Remaining documented-but-unexercised: `models`, `streaming`, `process_cube`, `multiperspective`, `workflow`, `object_lifecycle`

### Queued (next iterations)

Priority 1 — `models` module: `PetriNet` structural metrics (`explain()`, `structural_unsoundness_score()`, `is_structural_workflow_net()`) — these are the only structural metrics in the crate that derive from the graph itself
Priority 2 — Cross-product example: `OcelLog` → `Dfg` (shape) + `ConformanceResult` — compose OCEL admission, DFG construction, and conformance verdict into a single pipeline witness
Priority 3 — `streaming` module (StreamingConformance, EventWindow)
Priority 4 — `process_cube` / `multiperspective`

---

## 2026-06-14 — Iteration 5

**Cluster:** `models::PetriNet` structural metrics (the only metric surface in the crate that derives from graph structure, not from log replay)

**Triple:**
- **Doc:** `src/models.rs` rustdoc (complete with per-fn doctests and IS/IS-NOT framing)
- **Example:** `examples/petri_net_metrics.rs` — exercises `is_structural_workflow_net()` (true/false/empty), `structural_unsoundness_score()` (0.0 sound, >0 defective, 10.0 sentinel), `mdl_score()`, `mdl_score_with_ontology(4)`, `explain()` (self-derived, checked for node counts not static string), `canonical_hash()` (deterministic, distinct), `incidence_matrix()` with `FlatIncidenceMatrix::get()`, `PetriNetRefusal::EmptyNet`
- **Link:** README.md and CLAUDE.md example tables updated

**Run output (real exit code):**
```
=== All assertions passed — models::PetriNet structural metrics witnessed ===
  explain() = "Structural summary: 2 places, 1 transitions, 2 arcs. Structural workflow-net: true. Structural unsoundness score: 0.0."
  canonical_hash same structure = 0x4a6682df2b990e0a
EXIT: 0
```

**Covered ✅:** `models::PetriNet` structural metrics — documented-but-unexercised gap CLOSED.

**Gap map update:**
- `models` (PetriNet metrics) → COVERED ✅
- Remaining documented-but-unexercised: `streaming`, `process_cube`, `multiperspective`, `workflow`, `object_lifecycle`

### Queued (next iterations)

Priority 1 — Cross-product example: compose OcelLog admission + Dfg shape + ConformanceResult into single pipeline (this is the highest-value composition — no cross-module example shows the handoff between OCEL, DFG, and conformance verdict)
Priority 2 — `streaming` module (StreamingConformance, EventWindow — check src/streaming.rs for pub API)
Priority 3 — `process_cube` module (ProcessCubeSlice, ProcessCubeDimension)
Priority 4 — `workflow` / `object_lifecycle` modules (check pub API size)

---

## 2026-06-14 — Iteration 6

**Cluster:** Cross-product composition — OcelLog → ObjectCentricDfg → ConformanceResult → ReceiptEnvelope

**Triple:**
- **Doc:** Header in example file + cross-references to `src/ocel.rs`, `src/dfg.rs`, `src/conformance.rs`, `src/receipt.rs`
- **Example:** `examples/ocel_to_conformance_pipeline.rs` — exercises the 4-module pipeline: OCEL with E2O links + validate(), ObjectCentricDfg per-type, ConformanceResult held verdict + NaN coercion, ReceiptEnvelope well-shaped check + MissingSubject refusal
- **Link:** README.md and CLAUDE.md example tables updated

**Run output (real exit code):**
```
=== Pipeline complete — all module handoffs witnessed ===
  Stage 1: OcelLog.validate() → Ok  (E2O link admission)
  Stage 2: Dfg.validate() × 2 → Ok  (per-type DFG shapes)
  Stage 3: ConformanceResult.conformance_rate() = 1.00  (held verdict)
  Stage 4: ReceiptEnvelope.is_well_shaped() → true  (provenance stamp)
EXIT: 0
```

**Covered ✅:** Cross-product composition — the canonical four-module pipeline.

**Hard stop:** 3 triples this iteration (iterations 4 + 5 + 6). Queue below.

**Gap map update (remaining documented-but-unexercised):**
- `streaming` module — check pub API
- `process_cube` module — check pub API
- `multiperspective` module — check pub API
- `workflow` module — check pub API
- `object_lifecycle` module — check pub API

### Queued (next iterations)

Priority 1 — `streaming`, `object_lifecycle`, `workflow` modules (found untracked in working tree, now confirmed EXIT 0)
Priority 2 — `process_cube`, `multiperspective` modules (found already created in working tree, confirmed EXIT 0)

---

## 2026-06-14 — Iteration 7

**Cluster:** `streaming` + `object_lifecycle` + `workflow` modules — all three untracked in working tree, EXIT 0

**Triples:**

### streaming_context.rs
- **Doc:** `src/streaming.rs` — `ContextualEvidence`, `EventWindow<T,SIZE>`, `StreamingSource<WINDOW_SIZE>`, `TemporalOrderConfusion`, `OnlineEvidence`/`OfflineEvidence` type aliases
- **Example:** `examples/streaming_context.rs` — ring-buffer eviction asserted (`push(40)` returns `Some(10)`), online/offline wrappers, `TemporalOrderConfusion` (direct construct, named law not bare string)
- **Link:** README.md and CLAUDE.md updated

**Run output (real exit code):**
```
  Witness: inner values + ring-buffer eviction asserted; breaks on API change.
  Structure only — no event ingestion, no sliding windows, no monitoring.
  Graduate to wasm4pm for: stream ingestion, online conformance, drift detection.
EXIT:0
```

### object_lifecycle_phases.rs — PARTIAL WITNESS (nightly E0391)
- **Doc:** `src/object_lifecycle.rs` — `ObjectLifecyclePhase` ×5, `LifecycledObject<T,PHASE>` const-generic typestate, 5 type aliases, `ObjectLifecycleWitness`
- **Example:** `examples/object_lifecycle_phases.rs` — Display for all 5 phases, `LifecycledObject::new`, 5 type alias inner values asserted. Transition methods (`.activate/.modify/.archive/.delete`) trigger nightly E0391 cycle bug from examples/ context — honestly documented
- **Link:** README.md and CLAUDE.md updated

**Run output (real exit code):**
```
  trigger nightly E0391 (adt_const_params cycle) from example context.
  Transitions are covered by crate-internal unit tests but cannot be witnessed from examples/ until nightly cycle is resolved.
EXIT:0
```

### workflow_typestate.rs
- **Doc:** `src/workflow.rs` — `BranchToken<T,S>`, `Pending`/`Running`/`Completed`/`Canceled` markers, `ParallelWorkflow<A,B,SA,SB>`, `JoinPoint`, `CompletedWorkflow`
- **Example:** `examples/workflow_typestate.rs` — full Pending→Running→Completed chain, split→complete_a→complete_b→join_success, cancel_b_from_a→join_canceled_b, zero-size proof (all 4 markers + ParallelWorkflow = 0 bytes)
- **Link:** README.md and CLAUDE.md updated

**Run output (real exit code):**
```
           JoinPoint::join_success, JoinPoint::join_canceled_b,
           cancel_b_from_a, complete_a/complete_b, zero-size verification.
  Witness: typestate enforced at compile time — only valid chains compile.
EXIT:0
```

**Covered ✅:** `streaming`, `object_lifecycle` (partial), `workflow` — all documented-but-unexercised gaps CLOSED.

**Hard stop:** 3 triples this iteration.

**Gap map update (remaining):** `process_cube`, `multiperspective`

---

## 2026-06-14 — Iteration 8

**Cluster:** `process_cube` + `multiperspective` + cross-product composition (found pre-existing in working tree, all EXIT 0)

**Triples:**

### process_cube_shape.rs
- **Doc:** `src/process_cube.rs` — `CubeDimension<NAME>`, `CubeDimensionKind` (6 kinds), `CubeSlice<D,V>`, `CubeCell<DIMS>`, `CubeProjectionWitness<FROM,TO>`, `ProcessCube<Log,DIMS>`, `CellComparison<DIM_COUNT>`, `ProcessCubeLaw`
- **Example:** `examples/process_cube_shape.rs` — all 6 `CubeDimensionKind` Display names asserted, realistic slice composition, `ProcessCube::dimension_count()`, `CubeProjectionWitness` instantiation, `CellComparison` variant structure
- **Link:** README.md and CLAUDE.md updated

**Run output:**
```
=== All assertions passed — process_cube module surface is witnessed ===
  Covered: CubeDimension<N> (const-param axis), CubeDimensionKind (6 kinds),
           CubeSlice, CubeCell, CubeProjectionWitness, ProcessCube, CellComparison.
EXIT:0
```

### multiperspective_evidence.rs
- **Doc:** `src/multiperspective.rs` — `ProcessPerspective`, `ControlFlowPerspective`, `DataPerspective`, `ResourcePerspective`, `TimePerspective`, `MultiPerspectiveEvidence<T,Perspectives>`, `PerspectiveCombination<A,B>`, `ParityComparer`
- **Example:** `examples/multiperspective_evidence.rs` — all 4 `ProcessPerspective` Display names asserted, `MultiPerspectiveEvidence` single + combined, `PerspectiveCombination` 2/3/4-way nesting, `ParityComparer::assert_epsilon_close`
- **Link:** README.md and CLAUDE.md updated

**Run output:**
```
=== All assertions passed — multiperspective module surface is witnessed ===
  Covered: ProcessPerspective (4 kinds + Display), ControlFlowPerspective,
           DataPerspective, ResourcePerspective, TimePerspective,
           MultiPerspectiveEvidence (single + combined), PerspectiveCombination (2, 3, 4-way), ParityComparer.
EXIT:0
```

### process_pipeline_composition.rs (bonus cross-product)
- **Doc:** 7-module pipeline spanning ocel → dfg → interop → conformance → prediction → multiperspective → process_cube
- **Example:** `examples/process_pipeline_composition.rs` — found pre-existing, all 6 pipeline stages assert expected values, EXIT 0
- **Link:** README.md and CLAUDE.md already have entries for prior sub-modules

**Run output:**
```
=== Pipeline coherence verified across 7 modules ===
  ocel → dfg → interop+conformance → prediction → multiperspective → process_cube
EXIT:0
```

**Covered ✅:** `process_cube`, `multiperspective` — all remaining documented-but-unexercised gaps CLOSED.

## BIJECTIVE COVERAGE STATUS: COMPLETE ✅

All documented modules in `src/` now have a running example in `examples/`:
- `prediction` ✅ `streaming` ✅ `workflow` ✅ `object_lifecycle` ✅ (partial — nightly E0391)
- `process_cube` ✅ `multiperspective` ✅ `dfg` ✅ `bpmn` ✅ `models` ✅
- All prior modules (eventlog, ocel, evidence, admission, loss, formats, strict, wasm4pm, witness, petri, powl, conformance, declare, causality, ocpq, receipt) ✅

Every example in `examples/` is referenced in README.md and CLAUDE.md. Every documented module has at least one running example. Bijective coverage achieved.

---

## 2026-06-14 — Iteration 9

**Gap audit result:** The DOC_COVERAGE_LOG.md previously declared BIJECTIVE COVERAGE STATUS: COMPLETE, but a module-by-module audit of `src/*.rs` vs `examples/*.rs` revealed additional uncovered canon modules. Re-opened the loop.

**New gaps found:**
- `interop` — 17 pub items; partially exercised in `process_pipeline_composition` but no dedicated example
- `process_tree` — 12 pub items; used in `powl_process_tree` for `TypedLoopNode` only; `ProcessTree::admit_shape()`, all refusals, all 5 typed operator nodes unexercised
- `ids`, `temporal`, `diagnostic`, `law`, `nightly_foundry` — 2–25 pub items; no examples

**Cluster:** `interop` + `process_tree` (2 triples this iteration — hard stop)

### interop_boundary_grammar.rs

- **Doc:** `src/interop.rs` — 17 pub items covering PM4Py adoption grammar
- **Example:** `examples/interop_boundary_grammar.rs` — Pm4pyShape (7 tag+oc assertions), FilterShape×5, SummaryShape×5, ConformanceTriple (claimed_count + is_grounded), ArtifactGrounding (lawful + UngroundedArtifact + FlatClaimOverObjectCentric), InteropRefusal (5 named laws + Display contains law name), check_filter_shape (ok + DimensionShapeMismatch), OcelToXesProjection::project() ok, XesToOcedProjection RefuseLoss→err/AllowLoss→ok, OcelShape/XesShape/OcedShape zero-sized, FilterShapeConst<true> passes RequiresObjectCentric, GraduationCandidate sealed marker
- **Link:** README.md and CLAUDE.md updated

**Run output (real exit code):**
```
=== All assertions passed — interop module surface is witnessed ===
  Covered: Pm4pyShape (7 tags), FilterShape, SummaryShape,
           ConformanceTriple (is_grounded + claimed_count),
           ArtifactGrounding (lawful + 2 refusals),
           InteropRefusal (5 named laws + Display),
           check_filter_shape (ok + DimensionShapeMismatch),
           OcelToXesProjection + XesToOcedProjection (Project impls),
           OcelShape/XesShape/OcedShape (zero-sized markers),
           FilterShapeConst<IS_OC> + assert_filter_oc_compatible,
           GraduationCandidate sealed trait.
EXIT:0
```

**Covered ✅:** `interop` — documented-but-unexercised gap CLOSED.

---

### process_tree_shape.rs

- **Doc:** `src/process_tree.rs` — 12 pub items
- **Example:** `examples/process_tree_shape.rs` — operator_minimum/maximum_arity (6 kinds, const fn), TypedLoopNode<ARITY=2>/TypedXorNode<3>/TypedAndNode/TypedSeqNode/TypedOrNode (arity law compile-time), ProcessTreeNodeId zero-cost + ordered, ProcessTree::admit_shape() (valid Sequence(a,b)), 5 named refusals (MissingRoot, DanglingNodeReference, TauLeafWithChildren, BelowMinimumArity, InvalidArity), all 9 Display strings asserted, ProcessTreeOperator×6
- **Link:** README.md and CLAUDE.md updated

**Run output (real exit code):**
```
=== All assertions passed — process_tree module surface is witnessed ===
  Covered: operator_minimum/maximum_arity (6 operator kinds),
           TypedLoopNode/XorNode/AndNode/SeqNode/OrNode (arity law),
           ProcessTreeNodeId (zero-cost, ordered), ProcessTreeOperator (6),
           ProcessTree::admit_shape() (ok + 5 named refusal laws),
           ProcessTreeRefusal (9 laws with Display).
EXIT:0
```

**Covered ✅:** `process_tree` — documented-but-unexercised gap CLOSED.

**Hard stop: 2 triples reached in this iteration.** (queue below)

**Gap map update (remaining documented canon modules without dedicated examples):**
- `ids` (5 pub items) — `ObjectTypeName<K>`, `EventTypeName<K>`, `id_of<T>`, `TypedId` trait, `NewFromRaw` trait
- `temporal` (5 pub items) — check src/temporal.rs for actual API
- `diagnostic` (2 pub items) — check src/diagnostic.rs for actual API
- `nightly_foundry` (5 pub items) — petri_law, powl_law, evidence_law, token_law surfaces
- `xes` (12 pub items) — import path only partially covered by ocel_to_xes_projection

### Queued (next iterations)

Priority 1 — `ids` module: zero-cost typed identifier newtypes — foundation for all event/object referencing
Priority 2 — `temporal` module: temporal ordering vocabulary
Priority 3 — `diagnostic` module: minimal pub surface
Priority 4 — `nightly_foundry` module: always-on law surfaces derived from 4 papers
Priority 5 — `xes` module: XES import path (XesEvent, XesTrace, XesLog, XesRefusal) — unexercised outside ocel_to_xes_projection
