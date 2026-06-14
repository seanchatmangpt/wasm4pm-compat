# wasm4pm-compat: Detailed API Witness & Typestate Report

This report analyzes the structural composition of the `wasm4pm-compat` process-evidence compatibility library. It identifies all public structs, enums, typestate markers, `PhantomData` definitions, and state transitions, linking directly to their definitions in the codebase.

---

## 1. Central Evidence Lifecycle State Machine

The central invariant of `wasm4pm-compat` is a typed, one-way lifecycle tracked at compile time using zero-cost typestate tokens. 

```
  Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ {Projected | Exportable | Receipted}
    │                                  ▲
    └────────────── refuse ────────────┴──▶ Refused (terminal)
```

### Typestate Tokens
The following empty enums act as `PhantomData` tags inside the `Evidence` carrier to track the current stage in the lifecycle. They are defined in [state.rs](file:///Users/sac/wasm4pm-compat/src/state.rs):

* [Raw](file:///Users/sac/wasm4pm-compat/src/state.rs#L67) — Untrusted input from the outside world.
* [Parsed](file:///Users/sac/wasm4pm-compat/src/state.rs#L75) — Structurally parsed/decoded, but not yet judged.
* [Admitted](file:///Users/sac/wasm4pm-compat/src/state.rs#L83) — Checked against a named authority/witness. This state cannot be constructed directly from outside the crate; it requires going through the `Admit` trait.
* [Refused](file:///Users/sac/wasm4pm-compat/src/state.rs#L92) — Terminal refusal carrying a specific named law violation.
* [Projected](file:///Users/sac/wasm4pm-compat/src/state.rs#L101) — The result of a named, accounted lossy projection.
* [Exportable](file:///Users/sac/wasm4pm-compat/src/state.rs#L108) — Cleared to leave the boundary ("exit visa").
* [Receipted](file:///Users/sac/wasm4pm-compat/src/state.rs#L118) — Sealed inside a provenance-bearing receipt.

### State Transition Markers
These zero-sized structs are type-level markers for representing the transition currently taking place. They are defined in [state.rs](file:///Users/sac/wasm4pm-compat/src/state.rs):

* [RawToParsed](file:///Users/sac/wasm4pm-compat/src/state.rs#L130)
* [ParsedToAdmitted](file:///Users/sac/wasm4pm-compat/src/state.rs#L137)
* [ParsedToRefused](file:///Users/sac/wasm4pm-compat/src/state.rs#L144)
* [AdmittedToProjected](file:///Users/sac/wasm4pm-compat/src/state.rs#L151)
* [AdmittedToExportable](file:///Users/sac/wasm4pm-compat/src/state.rs#L158)
* [AdmittedToReceipted](file:///Users/sac/wasm4pm-compat/src/state.rs#L166)
* [ProjectedToExportable](file:///Users/sac/wasm4pm-compat/src/state.rs#L173)
* [ProjectedToReceipted](file:///Users/sac/wasm4pm-compat/src/state.rs#L180)
* [ExportableToReceipted](file:///Users/sac/wasm4pm-compat/src/state.rs#L188)

### Central Lifecycle Traits
* [EvidenceState](file:///Users/sac/wasm4pm-compat/src/state.rs#L55) — Sealed marker trait implemented only by the seven lifecycle stage tokens.
* [Projectible](file:///Users/sac/wasm4pm-compat/src/state.rs#L208) — Sealed marker trait indicating that a lifecycle stage is eligible for projection (implemented only for `Admitted` and `Projected`).

---

## 2. Core Carriers, PhantomData & Transition Methods

### The Universal Carrier
The central data carrier is defined in [evidence.rs](file:///Users/sac/wasm4pm-compat/src/evidence.rs):
* [Evidence<T, State: EvidenceState, W>](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L96-L103)
  * **PhantomData Definitions**:
    * `state: PhantomData<State>` at [evidence.rs#L100](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L100)
    * `witness: PhantomData<W>` at [evidence.rs#L102](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L102)

### Carrier State Transition Methods
The following methods on `Evidence` implement the allowed state transitions:
* [Evidence::raw(value: T)](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L121) — Mints `Raw` evidence (entry gate).
* [Evidence::refuse(self)](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L157) — `Raw -> Refused` direct rejection.
* [Evidence::into_parsed(self)](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L181) — `Raw -> Parsed` transition.
* [Evidence::into_refused(self)](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L219) — `Parsed -> Refused` transition.
* [Evidence::sealed(value: T)](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L236) — `pub(crate)` constructor to produce `Admitted` evidence.
* [Evidence::into_exportable(self) (on Admitted)](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L300) — `Admitted -> Exportable` transition.
* [Evidence::into_receipted(self) (on Admitted)](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L327) — `Admitted -> Receipted` transition.
* [Evidence::into_projected(self)](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L352) — `Admitted -> Projected` transition.
* [Evidence::into_exportable(self) (on Projected)](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L381) — `Projected -> Exportable` transition.
* [Evidence::into_receipted(self) (on Projected)](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L408) — `Projected -> Receipted` transition.
* [Evidence::into_receipted(self) (on Exportable)](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L532) — `Exportable -> Receipted` transition.

---

## 3. Boundary Verdicts & The Admit Trait

Verdicts are modeled as first-class structs representing successful admission or structured refusal. They are defined in [admission.rs](file:///Users/sac/wasm4pm-compat/src/admission.rs):

* [Admission<T, W>](file:///Users/sac/wasm4pm-compat/src/admission.rs#L37) — Proof that an admission law was satisfied.
  * **PhantomData Definition**: `witness: PhantomData<W>` at [admission.rs#L40](file:///Users/sac/wasm4pm-compat/src/admission.rs#L40).
* [Refusal<R, W>](file:///Users/sac/wasm4pm-compat/src/admission.rs#L105) — Audit record for a structural boundary refusal.
  * **PhantomData Definition**: `witness: PhantomData<W>` at [admission.rs#L108](file:///Users/sac/wasm4pm-compat/src/admission.rs#L108).
* [Admit](file:///Users/sac/wasm4pm-compat/src/admission.rs#L221) — The boundary verdict trait. Defines `admit()` at [admission.rs#L238](file:///Users/sac/wasm4pm-compat/src/admission.rs#L238), which maps `Evidence<Raw>` to `Result<Admission, Refusal>`.

---

## 4. Witnesses & Witness Semilattices

Witnesses are zero-sized marker types naming the authority (standard, paper, API grammar, or internal bridge) that a piece of evidence answers to. They are defined in [witness.rs](file:///Users/sac/wasm4pm-compat/src/witness.rs) and [witnesses.rs](file:///Users/sac/wasm4pm-compat/src/witnesses.rs):

* [Witness](file:///Users/sac/wasm4pm-compat/src/witness.rs#L76-L85) — The witness marker trait.
* [WitnessFamily](file:///Users/sac/wasm4pm-compat/src/witness.rs#L39-L53) — Enum defining the provenance category of a witness (Standard, Paper, ApiGrammar, RustLaw, InternalBridge).
* [witness_marker!](file:///Users/sac/wasm4pm-compat/src/witness.rs#L88-L104) — Core macro to declare empty-enum witness markers and implement the `Witness` trait.

### Witness Satisfaction Lattice
For tracking satisfaction/violation of a witness monotonically, the library provides a Join-Semilattice implementation:
* [WitnessState<W: Witness>](file:///Users/sac/wasm4pm-compat/src/witness.rs#L723) — Enum representing the lattice node (Unknown, Satisfied, Violated, Contradiction).
* [Join](file:///Users/sac/wasm4pm-compat/src/witness.rs#L713) — Trait for finding the join (least upper bound) of two states.
* [WithTop](file:///Users/sac/wasm4pm-compat/src/witness.rs#L718) — Trait to check if a state has reached contradiction (top).

---

## 5. Submodule Typestates, PhantomData & State Transitions

Several domain modules define localized typestates and PhantomData parameters to enforce domain-specific type laws.

### Causality (`causality.rs`)
Tracks causal consistency constraints:
* [CausalOrderWitness](file:///Users/sac/wasm4pm-compat/src/causality.rs#L48) — Marker struct.
* [CausalLink<From, To>](file:///Users/sac/wasm4pm-compat/src/causality.rs#L58) — Zero-cost causal link.
  * **PhantomData Definitions**: `_from: PhantomData<From>` (line 59), `_to: PhantomData<To>` (line 60).
* [CausallyOrderedEvidence<T>](file:///Users/sac/wasm4pm-compat/src/causality.rs#L189) — Zero-cost carrier for causally-ordered evidence.
  * **PhantomData Definition**: `_witness: PhantomData<CausalOrderWitness>` (line 192).
* [CausalConsistency](file:///Users/sac/wasm4pm-compat/src/causality.rs#L156) — Enum representing structural consistency verdicts (Consistent, HasCycles, HasContradictions, Unknown).
* [ConsistencyProof](file:///Users/sac/wasm4pm-compat/src/causality.rs#L239) — Unforgeable verification token constructed only by causal verification engines.
* [ConsistencyVerified<T>](file:///Users/sac/wasm4pm-compat/src/causality.rs#L268) — Sealed envelope linking a log and a consistency proof.

### Object Lifecycle (`object_lifecycle.rs`)
Tracks object lifecycle phases and lawful transitions:
* [ObjectLifecyclePhase](file:///Users/sac/wasm4pm-compat/src/object_lifecycle.rs#L35) — ConstParamTy enum representing lifecycle stages (Created, Active, Modified, Archived, Deleted).
* [ObjectState<const PHASE: ObjectLifecyclePhase>](file:///Users/sac/wasm4pm-compat/src/object_lifecycle.rs#L69) — State marker.
* [LifecycleTransition<const FROM: ObjectLifecyclePhase, const TO: ObjectLifecyclePhase>](file:///Users/sac/wasm4pm-compat/src/object_lifecycle.rs#L81) — Transition receipt.
* [LifecycledObject<T, const PHASE: ObjectLifecyclePhase>](file:///Users/sac/wasm4pm-compat/src/object_lifecycle.rs#L115) — Carrier tracking phase.
  * **PhantomData Definition**: `_state: PhantomData<ObjectState<PHASE>>` (line 118).
* **Transitions**:
  * [activate(self)](file:///Users/sac/wasm4pm-compat/src/object_lifecycle.rs#L138) — `Created -> Active`
  * [modify(self)](file:///Users/sac/wasm4pm-compat/src/object_lifecycle.rs#L148) — `Active -> Modified`
  * [archive(self) (from Active)](file:///Users/sac/wasm4pm-compat/src/object_lifecycle.rs#L156) — `Active -> Archived`
  * [archive(self) (from Modified)](file:///Users/sac/wasm4pm-compat/src/object_lifecycle.rs#L166) — `Modified -> Archived`
  * [modify(self) (from Modified)](file:///Users/sac/wasm4pm-compat/src/object_lifecycle.rs#L174) — `Modified -> Modified`
  * [delete(self)](file:///Users/sac/wasm4pm-compat/src/object_lifecycle.rs#L184) — `Archived -> Deleted`

### Workflow Branch Tracking (`workflow.rs`)
Tracks parallel workflow branches at compile time:
* **Branch States**:
  * [Pending](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L8) — Initialized but inactive.
  * [Running](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L9) — Active.
  * [Completed](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L11) — Succeeded.
  * [Canceled](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L13) — Terminated.
* [BranchToken<T, S: BranchState>](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L29) — Branch token.
  * **PhantomData Definitions**: `_task: PhantomData<T>` (line 30), `_state: PhantomData<S>` (line 31).
* [ParallelWorkflow<A, B, SA: BranchState, SB: BranchState>](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L61) — Parallel context (AND-Split).
* [CompletedWorkflow](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L133) — Sealed completed workflow.
* [JoinPoint](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L137) — AND-Join synchronization point.
* **Transitions**:
  * [BranchToken::start](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L37) — `Pending -> Running`
  * [BranchToken::complete](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L48) — `Running -> Completed`
  * [ParallelWorkflow::split](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L69) — Mints the concurrent branch tokens.
  * [ParallelWorkflow::complete_a](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L90) — Completes Branch A.
  * [ParallelWorkflow::complete_b](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L101) — Completes Branch B.
  * [ParallelWorkflow::cancel_b_from_a](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L115) — Cancellation transition: completes Branch A and cancels Branch B.
  * [JoinPoint::join_success](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L142) — Synchronizes branches when both succeed.
  * [JoinPoint::join_canceled_b](file:///Users/sac/wasm4pm-compat/src/workflow.rs#L150) — Synchronizes branches when B is canceled.

### Directly-Follows Graphs (`dfg.rs`)
Enforces DFG source/target directions at the type level:
* [DfgSourceMarker](file:///Users/sac/wasm4pm-compat/src/dfg.rs#L146) — Typestate source marker.
* [DfgTargetMarker](file:///Users/sac/wasm4pm-compat/src/dfg.rs#L159) — Typestate target marker.
* [DfgTypedEdge<S: IsDfgSource, T: IsDfgTarget>](file:///Users/sac/wasm4pm-compat/src/dfg.rs#L667) — Typed DFG edge.
  * **PhantomData Definitions**:
    * `_src: core::marker::PhantomData<S>` at [dfg.rs#L671](file:///Users/sac/wasm4pm-compat/src/dfg.rs#L671)
    * `_tgt: core::marker::PhantomData<T>` at [dfg.rs#L672](file:///Users/sac/wasm4pm-compat/src/dfg.rs#L672)

### Cross-Log Correlation (`correlation.rs`)
Enforces schema-correct log correlation:
* [CorrelationKey<const SCHEMA: &'static str>](file:///Users/sac/wasm4pm-compat/src/correlation.rs#L40) — Correlation key.
* [CorrelationWitness<const SCHEMA: &'static str>](file:///Users/sac/wasm4pm-compat/src/correlation.rs#L88) — Schema witness.
* [CorrelatedLog<A, B, const SCHEMA: &'static str>](file:///Users/sac/wasm4pm-compat/src/correlation.rs#L102) — Envelope for a merged log.
  * **PhantomData Definitions**:
    * `_a: PhantomData<A>` at [correlation.rs#L103](file:///Users/sac/wasm4pm-compat/src/correlation.rs#L103)
    * `_b: PhantomData<B>` at [correlation.rs#L104](file:///Users/sac/wasm4pm-compat/src/correlation.rs#L104)
* [CorrelationSchema](file:///Users/sac/wasm4pm-compat/src/correlation.rs#L166) — Enum for correlation strategies (ByCase, ByObject, ByTimestamp, ByAttribute).

### Conformance Checking (`conformance.rs`)
Carries alignment deviation types:
* [Deviation<M = ()>](file:///Users/sac/wasm4pm-compat/src/conformance.rs#L688) — Tagged with alignment-move family marker `M`.
  * **PhantomData Definition**: `witness: PhantomData<M>` at [conformance.rs#L694](file:///Users/sac/wasm4pm-compat/src/conformance.rs#L694).
* Move Markers:
  * [SyncMove](file:///Users/sac/wasm4pm-compat/src/conformance.rs#L420)
  * [LogOnlyMove](file:///Users/sac/wasm4pm-compat/src/conformance.rs#L425)
  * [ModelOnlyMove](file:///Users/sac/wasm4pm-compat/src/conformance.rs#L430)
