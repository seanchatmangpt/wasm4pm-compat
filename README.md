# wasm4pm-compat v26.6.13 — Process Intelligence Compatibility Core

![nightly-only](https://img.shields.io/badge/toolchain-nightly--only-orange)
![no-unsafe](https://img.shields.io/badge/unsafe-forbid%28unsafe__code%29-red)
![structure-only](https://img.shields.io/badge/scope-structure--only-blue)

> **Nightly Rust required. Applications conform upward to future type law.**

---

## Version Alignment

The logical system version, target specification, and documented release standard for this codebase is `26.6.13`. All API behaviors, validation logic, and diagnostic receipts in this repository are designed to conform to the **`wasm4pm-compat v26.6.13`** standard. The root crate, workspace subcrates, and derived manifests are all natively configured to `26.6.13`.

---

## Toolchain & Runtime Constraints

This crate provides no Minimum Supported Rust Version (MSRV) guarantees and contains no stable Rust fallback mechanisms. It is designed and implemented exclusively for the nightly compiler toolchain.

Applications using this library **must run under nightly Rust (refer to docs/explanation/why-nightly.md)**.

The toolchain is pinned via `rust-toolchain.toml` to a specific nightly release. The crate root declares `#![feature(generic_const_exprs, adt_const_params, const_trait_impl, min_specialization, portable_simd)]` without conditional gates. This design ensures that the compiler's monomorphization and const-evaluation engines enforce domain-specific type laws before runtime code generation occurs.

---

## Architectural Mandate: What this Crate IS and IS NOT

### What this Crate IS

`wasm4pm-compat` is a zero-cost structural boundary and verification interface for process mining artifacts. It is defined by the following characteristics:

*   **Process-Evidence Focused**: It specializes in modeling and verifying process artifacts (event logs, Petri nets, process trees) as formal, cryptographic evidence.
*   **Structure-Only**: It defines the data schemas, type parameters, and conversion laws for process-evidence structures, but performs no execution or calculation.
*   **Paper-Complete**: It implements structures representing the entire theoretical canon of process mining and process query formalisms from literature (including Petri nets, WF-nets, BPMN, OCEL 2.0, IEEE XES 1849, POWL, process trees, Declare, OCPQ, DFGs, and conformance alignments).
*   **Feature-Capped**: The crate API is strictly limited to structural validation, import/export contracts, and graduation prep. It does not grow to incorporate runtime solvers.
*   **Refusal-First**: It models boundary rejections as first-class, strongly-typed values (`Refusal`) carrying named structural laws rather than generic runtime strings or raw parse errors.
*   **Loss-Aware**: Any lossy projection is explicitly tracked, governed by a caller-supplied `LossPolicy`, and documented using a detailed, typed `LossReport` containing a static `ProjectionName`.
*   **Receipt-Shaped**: It models the structural envelope, witness metadata, and cryptographic digest shapes for provenance receipts.
*   **Graduation-Ready**: It implements the bridge traits and candidates needed to safely graduate static evidence to the execution engine.

### What this Crate IS NOT

To maintain a clean architectural boundary, `wasm4pm-compat` is:

*   **Not a lite wasm4pm**: It is not a subset or stripped-down version of the execution engine.
*   **Not an engine**: It contains no execution environment, solver, or simulation runtime.
*   **Not a conformance checker**: It does not compute fitness, precision, generalization, or trace alignment scores. It only models their static verdict structures.
*   **Not a replay/discovery engine**: It does not execute discovery algorithms (such as Alpha, Inductive, or Heuristics miners) or replay logs against models.
*   **Not a TypeScript/Zod generator**: It does not generate serialization wrappers or frontend interface schemas. TypeScript bindings live in the `wasm4pm-compat-ts` sidecar crate.
*   **Not a WASM ABI crate**: It does not define low-level WASM linear memory layouts or foreign function interfaces (FFIs).
*   **Not a format laundromat**: It forbids direct, unmonitored format-to-format conversion. Translating data requires admitting the input into a typed compat value under a witness, resolving any data loss under an explicit policy, and then exporting or graduating the result.

---

## Evidence Lifecycle

The central invariant of `wasm4pm-compat` is a typed, one-way lifecycle tracked at compile time using zero-cost typestate markers.

```text
  Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ {Projected | Exportable | Receipted}
    │                                  ▲
    └────────────── refuse ────────────┴──▶ Refused (terminal; carries a named law)
```

The universal carrier struct `Evidence<T, State: EvidenceState, W>` wraps the process payload `T` with two phantom type parameters: `State` representing the lifecycle stage and `W` representing the governing witness. Because `Evidence<T, Raw, W>` and `Evidence<T, Admitted, W>` are distinct types at compile time, functions demanding admitted evidence cannot compile if passed raw inputs.

1.  **Raw**: Untrusted data directly from the boundary.
2.  **Parsed**: Structurally decoded into memory, but not yet evaluated against type law.
3.  **Admitted**: Formally validated against a specific witness standard. This state cannot be constructed directly from outside the crate; it is only reachable by resolving the `Admit` trait.
4.  **Projected**: The result of a named, accounted lossy transformation.
5.  **Exportable**: Approved for conversion back into external serialization formats.
6.  **Receipted**: Sealed inside a provenance-bearing cryptographic receipt envelope.
7.  **Graduation Candidate**: Prepared to exit the compat boundary and pass to the execution engine.

Transitions between states consume the carrier struct by-value (`self`), preventing use-after-move defects at compile time. `Evidence::inner()` borrows the payload without consuming the carrier; `Evidence::from_boundary()` is an alias for `Evidence::raw()` that signals intent at call sites where the value originates at a process boundary.

### Builder Ergonomics

Structure construction stays terse without weakening the type law. `PowlBuilder` is a fluent arena builder for POWL models — `PowlBuilder::new().atom("a").silent("τ").partial_order(…).choice(…).loop_node(…).choice_graph(…)` — terminated by a checked `build()` that returns a named `PowlRefusal` on malformed input (or `build_unchecked()` when the shape is already known lawful). The same pattern appears across the canon (`Event::new(…)`, `Trace::from_events(…)`, `EventLog::from_traces(…)`), so building evidence reads naturally while every illegal shape remains a typed refusal rather than a panic.

---

## Witness Markers

Witnesses are zero-sized empty enums implementing the `Witness` trait (e.g., `Ocel20` for OCEL 2.0, `Xes1849` for IEEE XES 1849, `WfNetSoundnessPaper` for Workflow Net soundness). They serve as type-level markers indicating which authority standard or academic publication governs the validation and formatting laws of a piece of evidence.

Because witnesses are part of the type signature, `Evidence<T, Admitted, Ocel20>` and `Evidence<T, Admitted, Xes1849>` are incompatible types. This prevents the silent mixing of standards. The library tracks witness validation status monotonically using a Join-Semilattice representation (`WitnessState<W: Witness>` with states `Unknown`, `Satisfied`, `Violated`, and `Contradiction`).

### Bibliography Coverage

`src/witnesses.rs` (ggen-rendered) carries **271 paper witness markers** spanning every major process mining sub-discipline — discovery algorithms, conformance checking, object-centric formalisms, POWL, temporal logic, and predictive monitoring. Each marker exposes four `const` metadata fields: `KEY` (a short bibliographic slug), `TITLE`, `YEAR`, and `FAMILY`.

A compile-time uniqueness proof (`witness_law` module) asserts that no two witnesses share a `KEY`. Adding a duplicate key fails the build with a named law violation rather than silently shadowing the existing marker.

### Family Authority Diagnostics

The `Witness` trait uses `#[rustc_on_unimplemented]` annotations to produce teaching compiler diagnostics when a caller passes evidence governed by the wrong witness family. Instead of a bare type-mismatch error, the compiler emits a message identifying which family the value belongs to and which family the call site requires.

---

## Boundary Laws

### The Admission/Refusal Law

Boundary validation is governed by the `Admit` trait, which evaluates raw evidence against a witness and returns `Result<Admission<T, W>, Refusal<R, W>>`. A `Refusal` cannot contain generic error messages or raw strings. Its `R` parameter must be a domain-specific enum variant representing the exact structural law that was violated.

Current named refusal laws by domain:

| Domain | Named refusal variants |
|---|---|
| **OCEL** | `DanglingEventObjectLink`, `DuplicateObjectId`, `UnqualifiedObjectRelation` |
| **WF-net** | `MissingFinalMarking`, `UnsoundWfNet`, `DeadTransition` |
| **XES** | `MissingConceptName`, `NonMonotonicTimestamps` |
| **POWL** | `CyclicPartialOrder`, `DanglingOperatorChild`, `InvalidChoiceArity { declared, required_min }`, `CycleDetected` |
| **Declare** | `MissingTarget`, `InvalidTemplateArity`, `EmptyObjectScope`, `SynchronizationViolation`, `MissingActivation` |
| **Causal Net** | `InvalidBinding`, `MissingInputPlace`, `MissingOutputPlace`, `CycleDetected` |
| **Process Tree** | `InvalidArity`, `MissingDoBody` |
| **OCPQ** | `FlatteningRequired`, `EmptyObjectTypeList`, `SynchronizationRequiresMultipleTypes`, `ScopeMismatch` |
| **Conformance** | `FitnessUnavailable`, `GeneralizationUnavailable`, `SimplicityUnavailable` |
| **Receipt** | `MissingDigest`, `BrokenChainLink(u32)`, `EmptyChain`, `MissingWitness` |

All refusal variants are `PartialEq + Eq`, enabling structural equality assertions in tests (`assert_eq!(r, DeclareRefusal::MissingTarget)`) rather than Display-string substring searches.

### The Loss Law

Transformations that discard evidence (such as flattening multi-perspective OCEL logs into single-perspective XES logs) must implement the `Project` trait. Projection enforces a three-type lock:

$$\text{LossPolicy} \longrightarrow \text{ProjectionName} \longrightarrow \text{LossReport}$$

1.  **LossPolicy**: The caller must explicitly select the loss policy before projection:
    *   `RefuseLoss`: The projection fails and returns a named refusal if any evidence would be dropped.
    *   `AllowNamedProjection`: The projection is permitted under a static `ProjectionName`.
    *   `AllowLossWithReport`: The projection is permitted and produces a `LossReport` itemizing the discarded items.
2.  **ProjectionName**: A newtype wrapper of a `&'static str` representing a static, hardcoded transformation name.
3.  **LossReport**: A structured record containing the projection name, policy, and the itemized collection of lost items, parameterized by the source and target shape tags.

---

## OCEL 2.0 — LinkedOcel Admitter

The `LinkedOcel` type provides a concrete OCEL 2.0 admitter with authority enforcement rather than just structural labeling. Admission validates:

- No dangling event-object links (every linked object ID must exist in the object registry)
- No duplicate object IDs within a type namespace
- Qualified object-object relations carry a non-empty qualifier

The admitter returns `Admission<LinkedOcel, Ocel20>` on success and `Refusal<OcelRefusal, Ocel20>` on any structural violation, keeping the OCEL 2.0 boundary law enforced at the type level.

---

## Receipt-Shaped Evidence & The Graduation Path

When a host needs to perform active computation (such as model discovery or conformance checking), the evidence must graduate:

*   **Receipt-Shaped Evidence**: Modeled via `ReceiptShape` and `ReceiptEnvelope`. These structures represent the cryptographic metadata, digests, and replay hints, but perform no actual hashing or signing. Every receipt envelope requires three CROWN fields: `run_id`, `output_hash`, and `replay_pointer`.
*   **Graduation Bridge**: Decoupled from the execution engine, the `GraduateToWasm4pm` trait (enabled under the `wasm4pm` feature) allows structural shapes to compile a `GraduationCandidate`.
*   **GraduationCandidate**: A structural wrapper containing a `GraduationReason`, the subject name, and a hash reference to the grounding evidence. The external engine consumes these candidates to perform the actual process mining calculations.

Current `GraduationReason` variants:

| Reason | Hard signal | Meaning |
|---|:---:|---|
| `NeedsDiscovery` | yes | No process model exists yet |
| `NeedsConformanceExecution` | yes | A model exists but fitness has not been measured |
| `NeedsBenchmarkGate` | yes | Alignment cost is untested against baselines |
| `NeedsObjectCentricQueryExecution` | yes | OCPQ queries have not been executed against the log |
| `NeedsReplay` | no | Replay is stale or missing |
| `RebuildingProcessMiningLocally` | no | Local rebuild required before graduation |
| `NeedsReceipts` | no | Provenance receipts are absent or incomplete |

---

## Feature Contract

The public feature surface of `wasm4pm-compat` is **exactly three** features. Features control boundary capability stages rather than core domain knowledge — the base profile (compiled with `--no-default-features`) always knows the complete canon of process-evidence structures.

| Feature | Default | Capability Added |
|---|:---:|---|
| `formats` | **Yes** | Enables import/export traits, format covenants, and loss-policy interfaces. |
| `strict` | No | Enables strict boundary checks, `ProcessBoundary`, and `StrictViolation` diagnostic markers. |
| `wasm4pm` | No | Enables the graduation bridge, `GraduateToWasm4pm`, and `GraduationCandidate` types. |

There are no per-format features (e.g., no `ocel` or `xes` flags). The entire canon is always compiled. Nightly is not a cargo feature; it is the toolchain requirement.

---

## TypeScript Sidecar

TypeScript bindings and Zod schemas live in the `wasm4pm-compat-ts` sidecar crate rather than in this crate. The separation enforces the "not a TypeScript/Zod generator" invariant: the compat core defines type law; the sidecar exposes it to TypeScript consumers. Refer to the sidecar crate for schema generation, round-trip covenants, and `bindings/zod_schemas.ts`.

---

## ggen Ecosystem Projection

`ggen` (the generative provision compiler) operates as a provision instrument that translates ontologies (e.g., `wasm4pm-compat.ttl` defining the 271 canonical paper witnesses across all process mining families) and manifests into Rust source definitions, witness registries (`src/witnesses.rs`), and negative verification fixtures.

`wasm4pm-compat` serves as the target type-law court; it does not depend on `ggen` code or runtimes.

```
wasm4pm-compat  — defines the Rust process-evidence court
ggen            — projects into that court from ontology
wasm4pm         — executes judgment after graduation
```

The `graduation-boundary-map` ggen rule maps each `GraduationReason` variant to its target wasm4pm intake handler. The `extract-graduation-candidates` SPARQL query binds both `?candidate` and `?reason` to produce fully-typed graduation manifests.

---

## Anti-Cheat Infrastructure

### Compile-Fail Fixtures

The ALIVE gate (`cargo make alive`) runs 217 compile-fail fixtures and 406 compile-pass fixtures. Compile-fail fixtures use the **function-parameter pattern** to create typed values without calling `pub(crate)` constructors:

```rust
// Law: Evidence<T, Admitted, Xes1849> cannot be passed where Evidence<T, Admitted, Ocel20> is required.
fn _test(xes_ev: Evidence<String, Admitted, Xes1849>) {
    requires_ocel_evidence(xes_ev); // E0308 — the law violation, proven at compile time
}
```

Fixtures that test private-field non-forgeability omit the private field entirely:

```rust
let _forged: WfNetConst<{ SoundnessState::Witnessed }> = WfNetConst {}; // E0063 + E0451
```

Neither pattern uses `todo!()`, `unimplemented!()`, or any stub macro. The type error is the proof; no runtime code is needed.

### Anti-Cheat Gate

`just anti-cheat-gate` runs the full verification pipeline and then passes the source tree through `anti-llm-cheat-lsp` to detect fabricated evidence patterns. The gate covers `src/` law modules, `tests/` (excluding structural-check paths and trybuild fixtures), and `wasm4pm-compat-lsp/src/`. Per-repo suppressions for domain vocabulary (e.g., "fully admitted" as a typestate term) live in `anti-llm.toml`.

---

## Verification Commands

**Always use `cargo make`.** Direct `cargo` invocations are only used when running a single test by name.

```bash
# Fast type check
cargo make check

# Type check — all features
cargo make check-all

# Unit + integration tests
cargo make test-all

# Lint
cargo make clippy

# Format check / apply
cargo make fmt
cargo make fmt-fix

# Type-law receipt gate (ALIVE) — compile-fail + compile-pass fixtures
cargo make alive

# Full CI pipeline
cargo make ci

# Anti-cheat gate
just anti-cheat-gate

# ggen provision (render src/witnesses.rs from ontology)
cargo make ggen-witnesses
cargo make ggen-witnesses-dry   # preview without writing

# Run a single test by name
cargo test --all-features <test_name>
```

---

## Running the Examples

All examples compile and run with the nightly toolchain (`rust-toolchain.toml` handles this automatically). Every example is a real program that exits non-zero if the demonstrated capability regresses.

```bash
cargo run --example <name>                      # default features
cargo run --example <name> --features strict    # strict-feature examples
cargo run --example <name> --features wasm4pm   # wasm4pm-feature examples
```

| Example | Feature | What it demonstrates |
|---|---|---|
| `basic_eventlog` | (none) | `Event`/`Trace`/`EventLog` builder chain, `validate()`, `EventStream` append-only buffer |
| `basic_ocel` | (none) | `OcelLog` with E2O/O2O links and object changes, structural `validate()` |
| `evidence_lifecycle` | (none) | `Evidence<T, State, W>` one-way typestate: `Raw → Parsed → Admitted → Receipted`; illegal transitions rejected at compile time |
| `witness_authority` | (none) | Witness markers as zero-cost distinct types; `Admission<T, Ocel20>` vs `Admission<T, Xes1849>` are incompatible types |
| `loss_projection` | (none) | `LossPolicy` / `LossReport` / `LossChain` — named, auditable, impossible-to-hide structural loss |
| `ocel_to_xes_projection` | `formats` | OCEL → XES under the format covenant: `ProjectionName`, `LossPolicy`, named `XesExportRefusal` |
| `petri_net_construction` | (none) | WF-net typed arcs, `WfNetConst` soundness typestate (`Unknown → Claimed → Witnessed`), non-forgeable `SoundnessProof` |
| `conformance_metrics` | (none) | Fitness/precision/generalization/simplicity as compile-time rational `[0,1]` values; out-of-range values rejected by compiler |
| `declare_constraint_model` | (none) | Declare binary constraints (`Response`, `Precedence`), unary existence, and the OC-Declare object-type scoping extension |
| `ocpq_typed_query` | (none) | OCPQ typed query shapes: scope strategies, predicate families, const-generic cardinality bounds |
| `powl_process_tree` | (none) | POWL partial orders, typed `TypedLoopNode<ARITY>` (arity-2 enforced), `TreeProjectable` sealed gate |
| `causal_net_shape` | (none) | `CausalNet` / `CausalBinding` / `InputBinding` / `OutputBinding` — Heuristics Miner output shapes |
| `receipt_chain` | (none) | `ReceiptEnvelope`, `ReceiptChain` (dynamic) and `ReceiptChainConst<N>` (stack-arity-enforced), `GraduationReceipt` |
| `sealing_admit_chain` | (none) | `SealingAdmit` seam: BLAKE3 fold → `ChainProof` → `RuntimeSeal` → `SealedAdmission` → `Admitted` evidence; tamper witness |
| `strict_boundary_claim` | `strict` | `ProcessBoundary` declaration, `StrictCheck`, named violations: `MissingLossPolicy`, `MissingRefusalPath`, `HiddenProcessMiningGrowth` |
| `graduation_candidate` | `wasm4pm` | `GraduateToWasm4pm` bridge, `GraduationCandidate` grounded vs ungrounded |
| `prediction_problem_shape` | (none) | `PredictionProblem<T>` builder, all 6 `PredictionTarget` variants, 3 `PredictionHorizon` variants, 6 named `PredictionRefusal` laws |
| `dfg_shape` | (none) | `Dfg` + `validate()`, `DfgRefusal::EmptyGraph` / `DanglingEdge`, `DfgEdgeFull` with duration, `ObjectCentricDfg` per-type DFG map |
| `bpmn_process_shape` | (none) | `BpmnTask`, `BpmnGateway` x5, `BpmnProcess::validate`, `BpmnRefusal` x8 named laws, `BpmnLane`, `BpmnPool::validate` |
| `petri_net_metrics` | (none) | `PetriNet` structural metrics: `is_structural_workflow_net`, `structural_unsoundness_score`, `mdl_score`, `explain` (self-derived), `canonical_hash`, `incidence_matrix` |
| `workflow_typestate` | (none) | `BranchToken<T,S>` Pending→Running→Completed, `ParallelWorkflow::split`, `JoinPoint::join_success` / `join_canceled_b`, zero-size verification (all state markers = 0 bytes) |
| `object_lifecycle_phases` | (none) | `ObjectLifecyclePhase` ×5 Display, `LifecycledObject::new`, all 5 type aliases; transition methods partially blocked by nightly E0391 cycle bug |
| `streaming_context` | (none) | `ContextualEvidence::online/offline`, `EventWindow<T,SIZE>` ring-buffer eviction (`push` returns evicted), `StreamingSource<WINDOW_SIZE>`, `TemporalOrderConfusion` |
| `interop_boundary_grammar` | (none) | `Pm4pyShape` (7 tags), `FilterShape`, `SummaryShape`, `ConformanceTriple`, `ArtifactGrounding`, `InteropRefusal` (5 named laws), `check_filter_shape`, `OcelToXesProjection` + `XesToOcedProjection`, `FilterShapeConst<IS_OC>`, `GraduationCandidate` |
| `process_tree_shape` | (none) | `operator_minimum/maximum_arity` (6 kinds), `TypedLoopNode/XorNode/AndNode/SeqNode/OrNode` (arity law), `ProcessTree::admit_shape()`, `ProcessTreeRefusal` (9 named laws) |
| `ids_typed_identifiers` | (none) | `TypedId` sealed trait (`is_zero`, `raw_value`, generic dispatch), `ObjectTypeName<K>` + `EventTypeName<K>` string-backed names (from_static/from_owned/Display/Ord), `id_of::<T>(raw)` phantom-typed constructor, `NewFromRaw` sealed companion, `From`/`Into`/`FromStr` round-trips for all 8 integer-backed id kinds |
| `temporal_order_shapes` | (none) | `TemporalOrder` (4 variants, Display, Copy, Hash), `TemporalProfile<Trace>` zero-cost shape marker (new/default/direct), `TemporalOrderWitness` + `SojournTimeWitness` zero-cost markers, `TimeAwareEvidence<T,Order>` wrapper (new/into_inner, zero-cost PhantomData, distinct types per Order context) |
| `diagnostic_surface` | (none) | `CompatDiagnostic` (9 named law violations, Display → `[Error]`/`[Info]`, Copy, Hash), `DiagnosticSeverity` (Error/Warning/Info, Display) |
| `ocel_to_conformance_pipeline` | (none) | **Cross-product:** `OcelLog` → `ObjectCentricDfg` → `ConformanceResult` → `ReceiptEnvelope` — the four-module pipeline composition; all handoffs explicit |
| `c8_adversary_gap_demo` | (none) | Two-strategy divergence proof (LogicPlayer vs GraphPlayer on same stream) |
| `c8_collider_demo` | (none) | Collider topology mutation: hidden-body manifestation + collision proof emission |
| `c8_event_horizon_demo` | (none) | Event-horizon boundary detection in liquidity-collapse scenarios; boundary proof receipts |
| `c8_market_planck_demo` | (none) | MarketPlanck cell state transitions with receipt generation |

---

## Documentation Structure

The documentation for `wasm4pm-compat` is organized according to the [Diátaxis](https://diataxis.fr) framework:

*   **Explanations (Process Theory and Design)**:
    *   [Rust Typestate and Process Theory](docs/explanation/rust-typestate-and-process-theory.md) - Deep dive into typestates, affine types, and token conservation.
    *   [Genesis Thursday: Day Five Conceptual Framing](docs/explanation/genesis-thursday.md) - Conceptual framing of compile-time structures vs runtime execution.
    *   [Process-Evidence Domain Glossary](docs/explanation/glossary.md) - Mathematical and crate definitions of key terms.
*   **Reference**:
    *   [Public API for ggen](docs/reference/public-api-for-ggen.md) - Target surface for `ggen` integration.
    *   [Module Map & Layout](docs/reference/module-map.md) - Mapping of Rust modules to physical files.
    *   [Evidence Lifecycle States](docs/reference/lifecycle-states.md) - Detail on lifecycle state transitions.
    *   [Feature Model](docs/reference/feature-model.md) - Details of the strict three-feature limit.
    *   [Publish Readiness Checklist](docs/reference/publish-checklist.md) - Release checklist before publishing.
*   **How-To Guides**:
    *   [Preparing for a Crates.io Release](docs/how-to/prepare-crates-io-publish.md) - Release preparation steps.
*   **Research & Reports**:
    *   [Process Theory Alignment](research/process-theory-alignment.md) - Mathematical alignment with literature.
    *   [Verification Report](docs/reports/v26.6.13-verification-report.md) - Status of mandatory verification gates.

---

## License

This project is licensed under either of:

*   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
*   MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
