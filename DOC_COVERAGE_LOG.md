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
