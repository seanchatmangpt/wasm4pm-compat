# LSP Rules: Witness Checking (W4PM-WIT) & Execution Leakage (W4PM-ENG)

This document defines the diagnostic engine rules implemented by the `wasm4pm-compat-lsp` checker. These rules enforce structural boundaries within the `wasm4pm-compat` compatibility layer, ensuring it remains structure-only and maintains type-level witness provenance without erasing markers or leaking execution engines.

---

## W4PM-WIT: Witness Marker Misuse & Erasure

### Description
Witness markers (e.g., `Ocel20`, `Xes1849`, `WfNetSoundnessPaper`) are zero-sized types (ZSTs) designed to carry type-level authority proofs. Witness erasure occurs when these type-level proof carriers are converted or simplified into runtime primitives (such as `bool`, `String`, or generic parameters instantiated with raw types). 

By erasing witness markers:
- The compiler can no longer guarantee type-safety between different standards.
- Boundaries become "stringly-typed," exposing the surface to runtime typos and validation bypasses.
- Diagnostics lose the ability to print correct, compile-time metadata associated with specific witnesses.

### AST Patterns Detected

1. **Boolean Erasure (`W4PM-WIT-01`)**
   - **Pattern:** Variables, fields, or parameters named/containing `witness` or matching a known witness family name, typed as `bool`.
   - **Violation Code Example:**
     ```rust
     // Violation: Witness representation is erased to a runtime boolean
     pub struct AdmissionClaim {
         pub has_ocel20_witness: bool,
     }

     pub fn verify_boundary(witness_ok: bool) { ... }
     ```
   - **Remedy:** Thread the zero-sized witness type as a generic parameter or use a type implementing `Witness`.
     ```rust
     pub struct AdmissionClaim<W: Witness> {
         pub _marker: core::marker::PhantomData<W>,
     }
     ```

2. **Stringly-Typed Erasure (`W4PM-WIT-02`)**
   - **Pattern:** Variables, fields, or parameters named/containing `witness` typed as `String`, `&str`, `Cow<'_, str>`, or other string-like representations.
   - **Violation Code Example:**
     ```rust
     // Violation: Stringly-typed witness
     pub fn admit_evidence(data: Log, witness_key: &str) { ... }

     pub struct EventRecord {
         pub witness_name: String,
     }
     ```
   - **Remedy:** Utilize type-level witness markers. If serialization/deserialization is required, map the string key back to the ZST or use the witness registry during intake.

3. **Erased Generic Arguments (`W4PM-WIT-03`)**
   - **Pattern:** Instantiations of generic wrappers (such as `Admission<T, W>`, `Evidence<T, W>`, `Refusal<T, W>`, or `WitnessState<W>`) where the witness argument `W` is substituted by a primitive type (`bool`, `String`, `u8`, etc.) rather than a type implementing `Witness`.
   - **Violation Code Example:**
     ```rust
     // Violation: generic argument is erased/replaced with a primitive type
     let evidence: Evidence<OcelLog, String> = ...;
     let state: WitnessState<bool> = ...;
     ```
   - **Remedy:** Use the actual uninhabited witness enum (e.g., `Ocel20`).

---

## W4PM-ENG: Engine Leakage into Compatibility Crate

### Description
The `wasm4pm-compat` crate must remain strictly **structure-only**. It declares data shapes, formats, and structural laws (admission, refusal, lossy projection policies) but **never executes** process mining algorithms, replay solvers, or model discovery engines. 

Engine leakage occurs when:
- High-level solver libraries or runtime process mining engines are imported into a compatibility boundary.
- Heavy computational process mining tasks (e.g., computing alignments, simulating state spaces, discovering models, executing LTL queries) are implemented locally instead of graduating to `wasm4pm`.

### AST / Dependency Patterns Detected

1. **Disallowed Imports (`W4PM-ENG-01`)**
   - **Pattern:** `use` declarations referencing execution/solving engines.
   - **Prohibited Modules/Crates:**
     - **Process Mining Engines:** `pm4py`, `pm4py_rs`, `rust_process_mining`, `rapid_pm`, `wasm4pm_engine`, `wasm4pm::engine`
     - **Mathematical Solvers / LPs:** `z3`, `egg`, `cbc`, `glpk`, `lp_modeler`, `minisat`, `minilp`, `good_lp`, `coin_cbc`
     - **Database / Replay Storage Engines:** `rocksdb`, `sled`
   - **Violation Code Example:**
     ```rust
     // Violation: Importing execution/solving engine directly into compat
     use z3::{Config, Context, Solver};
     use rust_process_mining::conformance::alignments;
     ```
   - **Remedy:** Remove the engine imports. If execution is needed, implement `GraduateToWasm4pm` to emit a `GraduationCandidate` and hand off to the `wasm4pm` engine.

2. **In-place Replay / Alignment / Discovery Solver Implementations (`W4PM-ENG-02`)**
   - **Pattern:** Implementing local functions that perform heavy algorithmic computations like alignments, token replay, state-space exploration, or inductive/alpha miner discovery.
   - **Target Signatures:** Functions with names containing `align`, `alignment`, `replay`, `token_replay`, `state_space`, `discover_model`, `inductive_miner`, `alpha_miner`, or `evaluate_query` that attempt to parse/solve rather than wrapping structures.
   - **Violation Code Example:**
     ```rust
     // Violation: Local implementation of an alignment solver in a compat module
     pub fn compute_conformance_alignment(log: &OcelLog, net: &PetriNet) -> AlignmentResult {
         // Local solver/replay logic ...
     }
     ```
   - **Remedy:** Replace the solver logic with a graduation boundary.
     ```rust
     pub struct ConformanceExecutionRequest<'a> {
         pub log: &'a OcelLog,
         pub net: &'a PetriNet,
     }

     impl<'a> GraduateToWasm4pm for ConformanceExecutionRequest<'a> {
         fn candidate(&self) -> GraduationCandidate {
             GraduationCandidate::new(
                 GraduationReason::NeedsConformanceExecution,
                 "conformance alignment request",
                 format!("log:{:x}-net:{:x}", hash(self.log), hash(self.net)),
             )
         }
     }
     ```
