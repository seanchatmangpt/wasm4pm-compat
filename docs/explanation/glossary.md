# Process-Evidence Domain Glossary (Precise & Comprehensive)

This document provides mathematically precise and comprehensive definitions for the twelve key terms of the `wasm4pm-compat` domain, along with their mapping to concrete crate implementation types and structures.

---

## 1. Evidence

### Mathematical Definition
Let $\mathcal{T}$ be the set of all structural payload types representing process-mining data structures (e.g., event logs, Petri nets, POWL trees, conformance records).
Let $\mathcal{S}$ be the finite set of canonical typestate markers:
$$\mathcal{S} = \{ \text{Raw}, \text{Parsed}, \text{Admitted}, \text{Refused}, \text{Projected}, \text{Exportable}, \text{Receipted} \}$$
Let $\mathcal{W}$ be the set of witness types representing proof authorities.
An **Evidence** instance $E$ is defined as a 3-tuple:
$$E = (v, s, w) \in \mathcal{E}$$
where:
*   $v \in T$ is the payload value of type $T \in \mathcal{T}$.
*   $s \in \mathcal{S}$ is the lifecycle typestate tag.
*   $w \in \mathcal{W}$ is the witness authority marker.

The set of valid states is constrained by a state-transition relation $\mathcal{R}_{\mathcal{S}} \subset \mathcal{S} \times \mathcal{S}$. Any transition $(s_{from}, s_{to}) \notin \mathcal{R}_{\mathcal{S}}$ is structurally impossible at compile time, preventing state pollution and invalid conversions. Specifically, a transition function $f: E(T, S_{from}, W) \to E(T, S_{to}, W)$ exists if and only if $(S_{from}, S_{to}) \in \mathcal{R}_{\mathcal{S}}$.

### Comprehensive Explanation
Evidence acts as the universal carrier and state-wrapping abstraction for process-mining data. Rather than allowing raw structures to float through the system unchecked, the compiler gates them using typestates. By tagging the structure statically with both its lifecycle stage and the authority standard it answers to, we prevent state pollution and invalid structural conversions. For example, a raw, unparsed string cannot be treated as a validated Petri net because their types are structurally distinct and no transition exists to convert them directly without going through the admission boundary.

### Crate Implementation Map
*   **Path**: `src/evidence.rs` (evidence container: lines 96-103, state transitions: `src/state.rs` lines 67-100)
*   **Struct Representation**:
    ```rust
    pub struct Evidence<T, State: EvidenceState, W> {
        pub value: T,
        pub state: PhantomData<State>,
        pub witness: PhantomData<W>,
    }
    ```
*   **Type Alias Example**: `RawOcelEvidence<T>` and `AdmittedXesEvidence<T>`.

---

## 2. Admission

### Mathematical Definition
Let $T_{raw} \in \mathcal{T}$ be a raw/untrusted shape, $T_{admitted} \in \mathcal{T}$ be a validated shape, and $W \in \mathcal{W}$ be a witness authority.
Let $Laws(W)$ be the set of predicates (rules) defined by the authority $W$.
The **Admission** of a value is a structural carrier representing proof of compliance:
$$Admission(T_{admitted}, W) = \{ v \in T_{admitted} \mid \forall P \in Laws(W), P(v) \text{ holds} \}$$
Mathematically, the admission boundary is defined by a transition function:
$$\text{admit} : E(T_{raw}, \text{Raw}, W) \to \text{Result}(Admission(T_{admitted}, W), Refusal(R, W))$$
where $R$ is a domain-specific refusal reason type.
Holding an instance of $Admission(T_{admitted}, W)$ is a type-level guarantee that the valuation predicates $Laws(W)$ have been successfully satisfied.
The only pathway to transition to an admitted evidence state is:
$$\text{into\_evidence} : Admission(T, W) \to E(T, \text{Admitted}, W)$$

### Comprehensive Explanation
Admission is the formal verdict and verification process occurring at the boundary of the trust zone. When untrusted or raw inputs are evaluated against a standard or paper (Witness), and found structurally compliant, they are wrapped in an `Admission` type. This represents a type-safe boundary: the constructor of admitted evidence is private/restricted, and the ONLY public path to mint `Evidence<T, Admitted, W>` is through the `into_evidence()` method of a successful `Admission`. This prevents bypasses where developers might manually construct "admitted" logs without validation.

### Crate Implementation Map
*   **Path**: `src/admission.rs` (verdict surface: lines 1-21, representation: lines 37-41, trait: lines 183-241)
*   **Struct Representation**:
    ```rust
    pub struct Admission<T, W> {
        pub value: T,
        witness: PhantomData<W>,
    }
    ```
*   **Trait Signature**:
    ```rust
    pub trait Admit {
        type Raw;
        type Admitted;
        type Reason;
        type Witness;
        fn admit(raw: Evidence<Self::Raw, Raw, Self::Witness>)
            -> Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>>;
    }
    ```
*   **Verdict Execution**: `Admission::into_evidence` is the sole public pathway to mint `Evidence<T, Admitted, W>`, creating an unbreakable type-safe boundary.

---

## 3. Refusal

### Mathematical Definition
Let $W \in \mathcal{W}$ be a witness authority with a set of validation rules $Laws(W) = \{ P_1, P_2, \dots, P_n \}$.
Let $\mathcal{R}_W$ be a discrete set of named reasons representing the violation of these rules, such that each rule $P_i$ has a corresponding violation indicator $r_i \in \mathcal{R}_W$.
A **Refusal** is a strongly-typed failure verdict represented as:
$$Refusal(R, W) = (reason, w) \in \mathcal{R}_W \times \{ W \}$$
where $reason \in \mathcal{R}_W$ corresponds to the specific rule $P_i$ that failed validation.
Unlike generic error types (e.g. error strings or general dynamic dispatches), $R$ is a domain-specific enum whose members are statically typed, ensuring that every boundary rejection is mapped to a concrete named law.

### Comprehensive Explanation
Refusal represents a first-class, strongly-typed boundary verdict indicating that a piece of evidence failed validation against a specific authority. Rather than returning a generic runtime string or dynamic error object, the system mandates that a refusal must carry a named reason (typically a variant of a domain-specific enum, e.g., `MissingFinalMarking`). This guarantees that failure is audible and traceable to a specific law, preventing generic catch-all errors from obscuring the exact cause of boundary rejection.

### Crate Implementation Map
*   **Path**: `src/admission.rs` (lines 105-109), and domain-specific modules (e.g., `src/bpmn.rs`, `src/petri.rs`).
*   **Struct Representation**:
    ```rust
    pub struct Refusal<R, W> {
        pub reason: R,
        witness: PhantomData<W>,
    }
    ```
*   **Concrete Enums**: E.g., `BpmnRefusal` in `src/bpmn.rs` (lines 575-595), which contains structured domain reasons instead of catch-all error strings.

---

## 4. Witness

### Mathematical Definition
Let $\mathcal{W}$ be the set of witness types.
A **Witness** $W \in \mathcal{W}$ is a type-level proof carrier modeled as an uninhabited type (i.e., card($W$) = 0, represented by an empty enum).
Each witness $W$ is associated with a static metadata tuple:
$$W \cong \langle \text{KEY}, \text{FAMILY}, \text{TITLE}, \text{YEAR} \rangle$$
where:
*   $\text{KEY} \in \text{String}$ (a unique, canonical string identifier, e.g. `"ocel2.0"`).
*   $\text{FAMILY} \in \{ \text{Standard}, \text{Paper}, \text{ApiGrammar}, \text{RustLaw}, \text{InternalBridge} \}$ classifies the authority source.
*   $\text{TITLE} \in \text{String}$ is the human-readable citation or publication title.
*   $\text{YEAR} \in \mathbb{N} \cup \{ \emptyset \}$ is the publication year of the governing specification.

### Comprehensive Explanation
A Witness is a type-level marker (usually a zero-sized, uninhabited enum) that names the specific authority, interchange standard, academic paper, or API specification governing the validation and formatting rules for a piece of process evidence. It contains no data and incurs zero runtime cost. Instead, it guides the compiler's type checking, ensuring that evidence processed under one standard (e.g., OCEL 2.0) is not accidentally mixed with evidence processed under another (e.g., XES) in downstream computations.

### Crate Implementation Map
*   **Path**: `src/witness.rs` (lines 1-85) and the auto-generated witness ledger in `src/witnesses.rs`.
*   **Trait Definition**:
    ```rust
    pub trait Witness {
        const KEY: &'static str;
        const FAMILY: WitnessFamily;
        const TITLE: &'static str;
        const YEAR: Option<u16>;
    }
    ```
*   **Witness Family classification**:
    ```rust
    pub enum WitnessFamily {
        Standard,
        Paper,
        ApiGrammar,
        RustLaw,
        InternalBridge,
    }
    ```

---

## 5. LossPolicy

### Mathematical Definition
Let $\Pi : T_{from} \to T_{to}$ be a projection function mapping an input shape to an output shape.
Let $lost\_items : T_{from} \times T_{to} \to \mathcal{P}(\mathcal{I})$ be a mapping that computes the set of discarded components.
A **LossPolicy** $P_{loss}$ is a discrete parameter:
$$P_{loss} \in \{ \text{RefuseLoss}, \text{AllowNamedProjection}, \text{AllowLossWithReport} \}$$
which defines the assertion rules for the projection:
1.  If $P_{loss} = \text{RefuseLoss}$:
    $$\Pi(x) \text{ succeeds iff } lost\_items(x, \Pi(x)) = \emptyset$$
    Otherwise, the projection must fail and return a refusal.
2.  If $P_{loss} = \text{AllowNamedProjection}$:
    The projection is allowed to discard items, provided the transformation is statically identified by a `ProjectionName`.
3.  If $P_{loss} = \text{AllowLossWithReport}$:
    The projection is allowed to discard items, but it must generate a $LossReport$ containing the explicit evaluation of $lost\_items(x, \Pi(x))$.

### Comprehensive Explanation
The LossPolicy is the predefined strategy and rules of engagement chosen prior to execution that governs how a lossy projection or flattening operation handles discarded structural elements. By forcing the caller to specify the policy up-front, the API prevents silent data loss (e.g., discarding event-to-object links when translating OCEL to XES). The system defaults to `RefuseLoss`, meaning any projection that would drop data will fail unless the user explicitly opts into allowing or reporting the loss.

### Crate Implementation Map
*   **Path**: `src/loss.rs` (lines 49-58) and guidance in `docs/LOSS_POLICY.md`.
*   **Enum Representation**:
    ```rust
    pub enum LossPolicy {
        RefuseLoss,
        AllowNamedProjection,
        AllowLossWithReport,
    }
    ```
*   **Default Implementation**: `LossPolicy::RefuseLoss` is configured as the default enum variant to prevent silent structural degradation.

---

## 6. LossReport

### Mathematical Definition
Let $\Pi : T_{from} \to T_{to}$ be a projection function with name $\pi \in ProjectionName$ governed by policy $P_{loss} = \text{AllowLossWithReport}$.
A **LossReport** is a tuple:
$$L = \langle \pi, P_{loss}, items \rangle \in ProjectionName \times LossPolicy \times \mathcal{P}(\mathcal{I})$$
where:
*   $\pi \in ProjectionName$ is the static projection identifier.
*   $P_{loss}$ is the active policy ($P_{loss} = AllowLossWithReport$).
*   $items \in \mathcal{P}(\mathcal{I})$ is the set of concrete discarded structural components (e.g. event-to-object links, object attribute values).

The report is *lossless* if and only if $items = \emptyset$.

### Comprehensive Explanation
A LossReport is an auditable, structured record of a lossy projection that itemizes the exact evidence components discarded during conversion, validating the accountability invariant. It pairs the projection's static name and policy with the collection of lost items. This ensures that any data degradation can be monitored, logged, and audited, preserving trust across the processing chain.

### Crate Implementation Map
*   **Path**: `src/loss.rs` (lines 385-394).
*   **Struct Representation**:
    ```rust
    pub struct LossReport<From, To, Items> {
        pub projection: ProjectionName,
        pub policy: LossPolicy,
        pub lost: Items,
        from: PhantomData<From>,
        to: PhantomData<To>,
    }
    ```

---

## 7. ProjectionName

### Mathematical Definition
Let $\mathcal{P}_{names}$ be the set of valid projection names.
A **ProjectionName** is a newtype wrapping a static, immutable string literal:
$$ProjectionName(n) \text{ where } n \in \mathcal{S}_{static}$$
where $\mathcal{S}_{static}$ represents the set of all string literals residing in the read-only data segment of the compiled binary.
This enforces $O(1)$ copy/passing cost and ensures that the name is a compile-time constant associated with a specific transformation algorithm $\Pi$, preventing runtime generation of arbitrary name strings.

### Comprehensive Explanation
ProjectionName is a static string identifier uniquely naming a specific type of model transformation or projection (e.g., mapping an OCEL to a case-centric XES). It is wrapped in a newtype to guarantee that projection names are hardcoded into compilation targets, preventing developers from using dynamic runtime error strings or arbitrary, untraced projection names.

### Crate Implementation Map
*   **Path**: `src/loss.rs` (lines 161-162).
*   **Struct Representation**:
    ```rust
    pub struct ProjectionName(pub &'static str);
    ```

---

## 8. Receipt

### Mathematical Definition
Let $W \in \mathcal{W}$ be a witness authority, $D \in String$ be a content digest, and $H \in String$ be a replay hint.
A **ReceiptShape** is a tuple:
$$ReceiptShape = \langle w, d, h \rangle \in \mathcal{W} \times Digest \times ReplayHint$$
A **ReceiptEnvelope** wraps the shape with structural validity checks:
$$ReceiptEnvelope = \{ rs \in ReceiptShape \mid rs \text{ is non-empty and well-formed} \\}$$
We define the shape-checking relation:
$$WellShaped(rs) \iff w \neq \emptyset \land d \neq \emptyset \land h \neq \emptyset$$
This represents the structural metadata confirming the existence of a certificate of validity, without containing the dynamic cryptographic hashing or signature calculation engine.

### Comprehensive Explanation
A Receipt is a structural envelope verifying the metadata presence and cryptographic proof elements of a piece of process evidence, indicating format validity and record origin. It contains a witness identifier, a content digest, and a replay hint. It represents the *form* of certified evidence but does not perform the hashing or signature calculations, which are delegated to the execution engine.

### Crate Implementation Map
*   **Path**: `src/receipt.rs` (lines 181-188, 239-248).
*   **Struct Representation**:
    ```rust
    pub struct ReceiptShape {
        pub witness: String,
        pub digest: Digest,
        pub replay_hint: ReplayHint,
    }
    pub struct ReceiptEnvelope {
        pub shape: ReceiptShape,
        pub subject: String,
    }
    ```
*   **Trait Bounds**:
    ```rust
    pub trait WellShaped {
        fn well_shaped(&self) -> bool;
    }
    ```

---

## 9. Graduation

### Mathematical Definition
Let $\mathcal{C}_{compat}$ be the set of compatibility shapes defined within `wasm4pm-compat`.
Let $\mathcal{R}_{grad}$ be the set of graduation reasons:
$$\mathcal{R}_{grad} = \{ \text{NeedsDiscovery}, \text{NeedsConformanceExecution}, \text{NeedsReplay}, \text{NeedsReceipts}, \text{NeedsBenchmarkGate}, \text{NeedsObjectCentricQueryExecution}, \text{RebuildingProcessMiningLocally} \}$$
The **Graduation** boundary is an bridge mapping a compatibility value to a structured graduation candidate:
$$candidate : \mathcal{C}_{compat} \to GraduationCandidate$$
where $GraduationCandidate = \langle reason \in \mathcal{R}_{grad}, subject \in String, evidence\_ref \in String \rangle$.
This description serves as a formal ticket that is subsequently consumed by an execution engine domain $\mathcal{E}_{run}$ to perform algorithmic computations (such as alignment solver execution, process discovery, etc.).

### Comprehensive Explanation
Graduation is the structured seam and execution bridge where a compatibility structure graduates to a full process-mining execution engine to perform active computation (e.g., discovery, replay, or conformance check execution). Rather than performing these heavy computations inside the lightweight compatibility layer, the code declares its intent to graduate, providing a candidate type and a typed reason.

### Crate Implementation Map
*   **Path**: `src/engine_bridge.rs` (lines 171-173) and guidelines in `docs/GRADUATION.md`.
*   **Trait Definition**:
    ```rust
    pub trait GraduateToWasm4pm {
        fn candidate(&self) -> GraduationCandidate;
    }
    ```
*   **Graduation Reasons**:
    ```rust
    pub enum GraduationReason {
        NeedsDiscovery,
        NeedsConformanceExecution,
        NeedsReplay,
        NeedsReceipts,
        NeedsBenchmarkGate,
        NeedsObjectCentricQueryExecution,
        RebuildingProcessMiningLocally,
    }
    ```

---

## 10. Compatibility

### Mathematical Definition
Let $C$ be the compatibility layer. Let $\mathcal{O}_C$ be the set of operations defined in $C$ on a structure $x$.
For all $op \in \mathcal{O}_C$:
$$\text{TimeComplexity}(op) \le O(N)$$
$$\text{SpaceComplexity}(op) \le O(N)$$
where $N$ is the size of the structural representation.
Furthermore, the external run-time dependency set $\mathcal{D}_{ext}$ is empty:
$$\mathcal{D}_{ext}(C) = \emptyset$$
This guarantees that the compatibility layer acts purely as a format representation and type-safe boundary layer with zero algorithmic execution overhead (no linear solvers, graph matching, or heavy runtime dependencies).

### Comprehensive Explanation
Compatibility is the boundary capacity allowing process-evidence shapes to be modeled, safely converted, and verified at the type-level without forcing heavy runtime dependencies or execution overhead on consumer applications. It serves as a lightweight, zero-cost abstraction layer using Rust's phantom types and static checks to enforce constraints.

### Crate Implementation Map
*   **Path**: Crate target definition in `src/lib.rs` (lines 6-7).
*   **Interop implementation**: `src/interop.rs` (lines 298-305).

---

## 11. Engine

### Mathematical Definition
Let $\mathcal{M}$ be the runtime engine (e.g., `wasm4pm`).
Let $\mathcal{A}_{exec}$ be the set of algorithmic processes (e.g., alignment solver, miner, conformance checkers).
An **Engine** is modeled as a state machine execution environment that implements:
$$\forall a \in \mathcal{A}_{exec}, \quad a : C_{compat} \to Result(V_{exec}, E_{exec})$$
where $C_{compat}$ is a compatibility shape, and $V_{exec}$ is the execution verdict or computed metric.
The engine contains all state-transition solvers and resource-heavy algorithms, remaining decoupled from the compatibility definitions.

### Comprehensive Explanation
The Engine is the external, full-featured process mining run-time engine (specifically `wasm4pm`) which consumes compatibility shapes to execute algorithmic processes like alignment calculations, model discovery, or benchmark gating. It is decoupled and sits entirely behind the engine bridge to keep the compatibility layer lightweight.

### Crate Implementation Map
*   **Path**: `src/engine_bridge.rs` (lines 10-18), graduation guidelines in `docs/GRADUATION.md` (lines 18-25).
*   **Implementation Status**: Absent by design in `wasm4pm-compat`. The compatibility layer provides zero execution algorithms (no discovery, no alignments).

---

## 12. Structure-only

### Mathematical Definition
Let $M$ be a module in the compatibility layer. Let $\mathcal{F}_M$ be the set of functions and $\mathcal{T}_M$ be the set of types in $M$.
The module is **Structure-only** if:
1.  For all $f \in \mathcal{F}_M$:
    $$\text{SideEffects}(f) = \emptyset \quad \land \quad \text{Complexity}(f) \text{ is not solver-complete/algorithmic}$$
2.  For all $T \in \mathcal{T}_M$:
    $T$ contains only structural payload fields, enums, or type-level phantom markers representing states or witnesses.

### Comprehensive Explanation
Structure-only is the structural-integrity constraint of the compatibility layer, ensuring that all types, traits, and modules only define process evidence shapes and transition invariants, strictly delegating all runtime calculations to an execution engine. This ensures that the compatibility layer remains clean, easy to audit, and free of heavy algorithmic complexity.

### Crate Implementation Map
*   **Path**: Enforced across the entire crate; documented in `src/lib.rs` (lines 25-35), and in module-level `//!` docs (e.g., `src/admission.rs` lines 18-21, `src/evidence.rs` lines 91-93).

---

## Related Documentation

- Back to [README](../../README.md)
- [Rust Typestate and Process Theory](rust-typestate-and-process-theory.md)
- [Genesis Thursday: Day Five Conceptual Framing](genesis-thursday.md)
- [Evidence Lifecycle States Reference](../reference/lifecycle-states.md)
