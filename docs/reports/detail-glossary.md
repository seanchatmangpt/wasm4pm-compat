# Process-Evidence Domain Glossary & Implementation Map

This report contains definitions, occurrences, and crate-level implementation mappings for the twelve key terms of the `wasm4pm-compat` workspace.

---

## 1. Evidence

*   **Enterprise Definition**: The universal carrier and state-wrapping abstraction for process-mining data structures. It represents a process artifact that is statically tagged with both its lifecycle stage and the authority standard it answers to, preventing state pollution and invalid structural conversions.
*   **Key Occurrences**:
    *   Crate-level type-safeguards detailed in [lib.rs:L56-62](file:///Users/sac/wasm4pm-compat/src/lib.rs#L56-L62)
    *   Definition of the `Evidence` container: [evidence.rs:L96-103](file:///Users/sac/wasm4pm-compat/src/evidence.rs#L96-L103)
    *   Typestate lifecycle markers: [state.rs:L67-100](file:///Users/sac/wasm4pm-compat/src/state.rs#L67-L100)
    *   Ecosystem contracts: [CONSTRUCT8_PROJECT_CONTRACTS.md:L149-155](file:///Users/sac/wasm4pm-compat/CONSTRUCT8_PROJECT_CONTRACTS.md#L149-L155)
*   **Crate Implementation**:
    *   Implemented as the generic structure:
        ```rust
        pub struct Evidence<T, State: EvidenceState, W> {
            pub value: T,
            pub state: PhantomData<State>,
            pub witness: PhantomData<W>,
        }
        ```
    *   `State` is bounded by the sealed `EvidenceState` trait, allowing only seven canonical typestates (`Raw`, `Parsed`, `Admitted`, `Refused`, `Projected`, `Exportable`, `Receipted`) from `src/state.rs`.
    *   `W` represents a compile-time witness (authority marker).
    *   Includes type aliases like `RawOcelEvidence<T>` and `AdmittedXesEvidence<T>`.

---

## 2. Admission

*   **Enterprise Definition**: The process and formal verification verdict that occurs at the boundary when untrusted or raw inputs are evaluated against a standard or paper (Witness) and found structurally compliant.
*   **Key Occurrences**:
    *   Verdict surface design: [admission.rs:L1-21](file:///Users/sac/wasm4pm-compat/src/admission.rs#L1-L21)
    *   The `Admission` struct representation: [admission.rs:L37-41](file:///Users/sac/wasm4pm-compat/src/admission.rs#L37-L41)
    *   The `Admit` trait definition: [admission.rs:L183-241](file:///Users/sac/wasm4pm-compat/src/admission.rs#L183-L241)
*   **Crate Implementation**:
    *   Implemented via `Admission<T, W>`:
        ```rust
        pub struct Admission<T, W> {
            pub value: T,
            witness: PhantomData<W>,
        }
        ```
    *   The `Admit` trait mandates:
        ```rust
        fn admit(raw: Evidence<Self::Raw, Raw, Self::Witness>)
            -> Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>>;
        ```
    *   `Admission::into_evidence` is the sole public pathway to mint `Evidence<T, Admitted, W>`, creating an unbreakable type-safe boundary.

---

## 3. Refusal

*   **Enterprise Definition**: A first-class, strongly-typed boundary verdict indicating that a piece of evidence failed validation against a specific authority. It carries a named law rather than a generic runtime error, ensuring auditability of boundary rejections.
*   **Key Occurrences**:
    *   Module overview: [admission.rs:L9-13](file:///Users/sac/wasm4pm-compat/src/admission.rs#L9-L13)
    *   `Refusal` struct definition: [admission.rs:L105-109](file:///Users/sac/wasm4pm-compat/src/admission.rs#L105-L109)
    *   Module-specific refusal types: e.g., `BpmnRefusal` in [bpmn.rs:L575-595](file:///Users/sac/wasm4pm-compat/src/bpmn.rs#L575-L595) and `PetriRefusal` in [petri.rs]
*   **Crate Implementation**:
    *   Represented by the generic:
        ```rust
        pub struct Refusal<R, W> {
            pub reason: R,
            witness: PhantomData<W>,
        }
        ```
    *   `R` is a domain-specific enum representing broken laws (e.g. `MissingFinalMarking`, `DanglingEventObjectLink`), ensuring that the compiler prevents developers from returning generic error strings.

---

## 4. Witness

*   **Enterprise Definition**: A type-level proof carrier (usually a zero-sized, uninhabited type) naming the specific academic paper, interchange standard, or API specification that governs validation and formatting laws for a piece of evidence.
*   **Key Occurrences**:
    *   Witness markers overview: [witness.rs:L1-31](file:///Users/sac/wasm4pm-compat/src/witness.rs#L1-L31)
    *   `Witness` trait and family: [witness.rs:L39-85](file:///Users/sac/wasm4pm-compat/src/witness.rs#L39-L85)
    *   Auto-generated witness ledger: [witnesses.rs](file:///Users/sac/wasm4pm-compat/src/witnesses.rs)
*   **Crate Implementation**:
    *   Defined as empty `enum` structures tagged with `Witness` trait:
        ```rust
        pub trait Witness {
            const KEY: &'static str;
            const FAMILY: WitnessFamily;
            const TITLE: &'static str;
            const YEAR: Option<u16>;
        }
        ```
    *   Witnesses include standards (e.g., `Ocel20`, `Xes1849`), papers (e.g., `WfNetSoundnessPaper`), or API Grammars. Maintained via `ggen` rule-based synchronization.

---

## 5. LossPolicy

*   **Enterprise Definition**: The predefined strategy and rules of engagement chosen prior to execution that governs how a lossy projection or flattening operation handles discarded structural elements.
*   **Key Occurrences**:
    *   Accounting of loss: [loss.rs:L1-21](file:///Users/sac/wasm4pm-compat/src/loss.rs#L1-L21)
    *   `LossPolicy` enum definition: [loss.rs:L49-58](file:///Users/sac/wasm4pm-compat/src/loss.rs#L49-L58)
    *   Policy guidelines: [docs/LOSS_POLICY.md](file:///Users/sac/wasm4pm-compat/docs/LOSS_POLICY.md)
*   **Crate Implementation**:
    *   Implemented as:
        ```rust
        pub enum LossPolicy {
            RefuseLoss,
            AllowNamedProjection,
            AllowLossWithReport,
        }
        ```
    *   Defaults to `LossPolicy::RefuseLoss` to prevent silent evidence discarding.

---

## 6. LossReport

*   **Enterprise Definition**: An auditable, structured record of a lossy projection that itemizes the exact evidence components discarded during conversion, validating the accountability invariant.
*   **Key Occurrences**:
    *   Concept details: [loss.rs:L17-21](file:///Users/sac/wasm4pm-compat/src/loss.rs#L17-L21)
    *   Struct definition: [loss.rs:L385-394](file:///Users/sac/wasm4pm-compat/src/loss.rs#L385-L394)
*   **Crate Implementation**:
    *   Generic carrier:
        ```rust
        pub struct LossReport<From, To, Items> {
            pub projection: ProjectionName,
            pub policy: LossPolicy,
            pub lost: Items,
            from: PhantomData<From>,
            to: PhantomData<To>,
        }
        ```
    *   Pairs with a projection and a collection of discarded `Items` (e.g., event-to-object links or object types).

---

## 7. ProjectionName

*   **Enterprise Definition**: A static string identifier uniquely naming a specific type of model transformation or projection (e.g., mapping an OCEL to a case-centric XES).
*   **Key Occurrences**:
    *   Overview: [loss.rs:L22-26](file:///Users/sac/wasm4pm-compat/src/loss.rs#L22-L26)
    *   Struct definition: [loss.rs:L161-162](file:///Users/sac/wasm4pm-compat/src/loss.rs#L161-L162)
*   **Crate Implementation**:
    *   Implemented as a newtype wrapper of a static string:
        ```rust
        pub struct ProjectionName(pub &'static str);
        ```
    *   Ensures that projection identifiers are cheap to copy/pass and are hardcoded into the compile targets.

---

## 8. Receipt

*   **Enterprise Definition**: A structural envelope verifying the metadata presence and cryptographic proof elements of a piece of process evidence, indicating format validity and record origin.
*   **Key Occurrences**:
    *   Receipt module guidelines: [receipt.rs:L1-26](file:///Users/sac/wasm4pm-compat/src/receipt.rs#L1-L26)
    *   `ReceiptShape` struct: [receipt.rs:L181-188](file:///Users/sac/wasm4pm-compat/src/receipt.rs#L181-L188)
    *   `ReceiptEnvelope` struct: [receipt.rs:L239-248](file:///Users/sac/wasm4pm-compat/src/receipt.rs#L239-L248)
*   **Crate Implementation**:
    *   Includes `ReceiptShape` and `ReceiptEnvelope`:
        ```rust
        pub struct ReceiptShape {
            pub witness: String,
            pub digest: Digest,
            pub replay_hint: ReplayHint,
        }
        ```
    *   Provides type-level bounds like `WellShaped` trait. It represents the *form* of certified evidence but does not perform the hashing/signing.

---

## 9. Graduation

*   **Enterprise Definition**: The structured seam and execution bridge where a compat structure graduates to a full process-mining execution engine to perform active computation (e.g., discovery, replay, or conformance check execution).
*   **Key Occurrences**:
    *   Seam description: [engine_bridge.rs:L1-20](file:///Users/sac/wasm4pm-compat/src/engine_bridge.rs#L1-L20)
    *   Graduation boundaries documentation: [docs/GRADUATION.md](file:///Users/sac/wasm4pm-compat/docs/GRADUATION.md)
    *   `GraduateToWasm4pm` trait: [engine_bridge.rs:L171-173](file:///Users/sac/wasm4pm-compat/src/engine_bridge.rs#L171-L173)
*   **Crate Implementation**:
    *   Implemented in `src/engine_bridge.rs` (gated under `#[cfg(feature = "wasm4pm")]`).
    *   Uses `GraduationCandidate` and `GraduationReason` to formalize why a value is graduating:
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

*   **Enterprise Definition**: The boundary capacity allowing process-evidence shapes to be modeled, safely converted, and verified at the type-level without forcing heavy runtime dependencies or execution overhead on consumer applications.
*   **Key Occurrences**:
    *   Crate target definition: [lib.rs:L6-7](file:///Users/sac/wasm4pm-compat/src/lib.rs#L6-L7) ("Start with compatibility. Graduate to execution.")
    *   Interop traits: [interop.rs:L298-305](file:///Users/sac/wasm4pm-compat/src/interop.rs#L298-L305)
*   **Crate Implementation**:
    *   Expressed as the foundational goal of `wasm4pm-compat`. It serves as the format boundary layer utilizing zero-cost abstraction patterns (e.g., phantom markers and transparent IDs) to keep the memory footprint lightweight.

---

## 11. Engine

*   **Enterprise Definition**: The external, full-featured process mining run-time engine (specifically `wasm4pm`) which consumes compatibility shapes to execute algorithmic processes like alignment calculations, model discovery, or benchmark gating.
*   **Key Occurrences**:
    *   Covenant description: [engine_bridge.rs:L10-18](file:///Users/sac/wasm4pm-compat/src/engine_bridge.rs#L10-L18)
    *   Graduation guidelines: [docs/GRADUATION.md:L18-25](file:///Users/sac/wasm4pm-compat/docs/GRADUATION.md#L18-L25)
*   **Crate Implementation**:
    *   Absent by design in `wasm4pm-compat`. The crate provides zero execution algorithms (no discovery, no alignments). The engine is decoupled and sits entirely behind the `engine_bridge.rs` graduation candidate seam.

---

## 12. Structure-only

*   **Enterprise Definition**: The structural-integrity constraint of the compatibility layer, ensuring that all types, traits, and modules only define process evidence shapes and transition invariants, strictly delegating all runtime calculations to an execution engine.
*   **Key Occurrences**:
    *   Crate architecture: [lib.rs:L25-35](file:///Users/sac/wasm4pm-compat/src/lib.rs#L25-L35)
    *   Glossary entries: [docs/GLOSSARY.md:L101-106](file:///Users/sac/wasm4pm-compat/docs/GLOSSARY.md#L101-L106)
*   **Crate Implementation**:
    *   Enforced throughout the codebase. Every public data type (like `EventLog`, `PetriNet`, `OcelLog`, `DirectlyFollowsGraph`) is represented as clean, structural schemas with validation helpers but no solver/execution logic.
