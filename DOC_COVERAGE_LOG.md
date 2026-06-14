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

### Queued (next iterations)

Priority 1 — prediction module (12 pub items, no examples)
Priority 2 — dfg module (DirectlyFollowsGraph is a foundation for most discovery algorithms, no example)
Priority 3 — process_cube slice (8 pub items)
Priority 4 — c8_* examples added to docs table (exercised-but-undocumented gap)
Priority 5 — cross-product example: full pipeline from OcelLog → admission → dfg discovery → conformance check

### Hard stops
None this iteration. Disk: not checked (no ENOSPC encountered).
