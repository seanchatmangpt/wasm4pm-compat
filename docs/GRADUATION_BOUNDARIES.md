# Graduation Boundaries

**Last Updated:** 2026-06-14

> Structure-only. Nightly-always. No engines here.

This document is the authoritative list of what **must graduate** to `wasm4pm`
and what **must stay** in `wasm4pm-compat`. The division is non-negotiable. Any
code in `wasm4pm-compat` that crosses the graduation boundary is a defect, not
a feature.

See also: `docs/GRADUATION.md` for the `GraduationCandidate` / `GraduateToWasm4pm`
bridge machinery.

---

## The Invariant

`wasm4pm-compat` carries evidence shapes and enforces structural laws.
`wasm4pm` adjudicates evidence and runs engines. Neither borrows the other's
mandate.

---

## What Graduates to wasm4pm

These capabilities are **structurally represented** in this crate (types exist)
but **executed nowhere in this crate**. Execution belongs to `wasm4pm`.

### Process Discovery

Algorithms that produce a process model from an event log. This crate provides
the output shapes (typed Petri nets, POWL graphs, process trees, DFGs); it does
not run any miner.

| What stays here (shape) | What graduates (execution) |
|---|---|
| `petri::WfNetConst<SOUNDNESS>` — typed WF-net structure | Inductive Miner, Split Miner, Heuristic Miner |
| `powl::ChoiceGraph`, `powl::PartialOrder`, `powl::PowlComposition<Inner, DEPTH>` — POWL shapes | POWL discovery from OCEL logs |
| `process_tree::TypedLoopNode<ARITY>` — process tree structure | Process tree induction |
| `dfg::DirectlyFollowsGraph` — DFG structure | DFG frequency/time annotation and mining |

### Conformance Checking Execution

Algorithms that compare a process model against an event log. This crate provides
the verdict shapes; it does not run any checker.

| What stays here (shape) | What graduates (execution) |
|---|---|
| `conformance::Metric<KIND, NUM, DEN>` — unit-interval metric | Fitness/precision/generalization computation |
| `conformance::ConformanceVerdict` — verdict structure | Alignment-based conformance checking |
| `diagnostic::DiagnosticReport` — diagnostic surface | Model-vs-log diagnostic engine |

### Token Replay

Execution of a token game against a Petri net. This crate provides the net
structure and soundness witness; it does not replay tokens.

| What stays here (shape) | What graduates (execution) |
|---|---|
| `petri::WfNetConst<SOUNDNESS>` — net with soundness marker | Token replay engine |
| `petri::SeparableWfNet` — separability-typed WF-net | Replay against separable nets |
| `witness::WfNetSoundnessPaper` — soundness witness type | Replay correctness guarantee |

### Prediction Engines

Machine learning models that operate on process prefixes. This crate provides
the typed prefix and target shapes; it does not train or infer.

| What stays here (shape) | What graduates (execution) |
|---|---|
| `prediction::PredictionTarget` — typed prediction target | Next-event prediction models |
| `evidence::Evidence<T, Admitted, W>` — lawful prefix carrier | Prefix-based ML training/inference |
| `ocel::OcelLog` — OCED event data structure | Predictive monitoring pipelines |

### Query Execution

OCPQ and OCEL query evaluation. This crate provides typed query shapes and
cardinality bounds; it does not evaluate queries.

| What stays here (shape) | What graduates (execution) |
|---|---|
| `ocpq::OcpqQuery` — typed OCPQ query structure | Object-centric query evaluation engine |
| `ocpq::CardinalityBound` — const-bounded cardinality | Query planning and execution |
| `ocpq::OcpqRefusal` — query refusal surface | Query validation and rejection |

### Alignment Computation

Cost-based alignment algorithms between a log trace and a process model. This
crate provides the model and verdict structures; it does not compute alignments.

| What stays here (shape) | What graduates (execution) |
|---|---|
| `conformance::Metric<KIND, NUM, DEN>` — alignment score shape | Alignment computation (A*, Dijkstra-based) |
| `petri::WfNetConst<SOUNDNESS>` — model for alignment | Synchronous product construction |
| `receipt::Receipt<W>` — receipt surface | Alignment receipt minting and chaining |

---

## What Must NOT Graduate (Stays Here)

The following must remain in `wasm4pm-compat` regardless of how the engine evolves:

- All type definitions: state tokens, witness markers, evidence carrier
- All admission and refusal surfaces: `Admit`, `Refusal<R, W>`, named law reasons
- All loss policy machinery: `LossPolicy`, `LossReport`, `Project`
- All format contracts: `LossyFormatExport`, round-trip claims
- All const-generic type-law machinery: `Between01`, `ConditionCell`, `Assert`
- All graduation candidate machinery: `GraduationCandidate`, `GraduateToWasm4pm`
- All diagnostic shapes: `DiagnosticReport` structure (not evaluation)
- All YAWL shapes: `CancellationRegion`, `MultipleInstanceSpecConst`

---

## Hard Signals (Immediate Graduation Required)

When any of these appear inside `wasm4pm-compat`, graduation is mandatory:

| Signal | Reason |
|---|---|
| A function that discovers a model from a log | Discovery belongs to `wasm4pm` |
| A function that computes fitness, precision, or generalization | Conformance execution belongs to `wasm4pm` |
| A function that replays a token against a net | Replay belongs to `wasm4pm` |
| A function that trains or scores a prediction model | ML execution belongs to `wasm4pm` |
| A function that evaluates an OCPQ or OCEL query | Query execution belongs to `wasm4pm` |
| A function that computes an alignment | Alignment computation belongs to `wasm4pm` |
| Any import of an optimization or search crate | Engine-level dependency; graduate the feature |

Use `GraduationReason::RebuildingProcessMiningLocally` to signal the loudest
of these — when the compat layer starts re-implementing process mining.
