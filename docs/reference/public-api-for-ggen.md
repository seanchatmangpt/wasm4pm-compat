# Reference: Public API for ggen

This document specifies the expected target surface for `ggen` integration and compiler pipeline alignment within `wasm4pm-compat` version `26.6.5`. These paths are formulated as the **expected target surface for ggen** rather than "guaranteed forever" invariants, aligning code-generation expectations with the evolutionary boundary design.

---

## 1. Sync Targets & Macro Integration

The `ggen` compiler pipeline interacts directly with the codebase by generating and maintaining witness tokens.

* **`src/witnesses.rs`**
  - **Status**: Code-generated and synced via `ggen sync --rule witness-markers`.
  - **Purpose**: Defines empty-enum markers representing the active standards ontology.
  - **Mechanism**: Utilizes the crate's internal `witness_marker!` macro to assert `Witness` and `WitnessFamily` traits at compile time.

---

## 2. Expected Stable Paths

The following 20 paths constitute the expected target surface for ggen and downstream tools:

### 1. `wasm4pm_compat::admission`
Defines the intake gate of the process-evidence boundary.
- **Key Traits**: `Admit` (boundary checker trait).
- **Key Structs**: `Admission<T, W>` (wrapping successfully admitted data `T` with witness `W`), `Refusal<R, W>` (representing a named refusal reason `R` with witness `W`).
- **Signature**:
  ```rust
  pub trait Admit {
      type Raw;
      type Admitted;
      type Reason;
      type Witness: Witness;

      fn admit(raw: Evidence<Self::Raw, Raw, Self::Witness>)
          -> Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>>;
  }
  ```

### 2. `wasm4pm_compat::evidence`
Defines the zero-cost typestate carrier for process-evidence payloads.
- **Key Struct**: `Evidence<T, State, W>` where `T` is the process mining payload, `State` is an `EvidenceState` token, and `W` is a `Witness`.
- **Zero-Cost Invariant**: `std::mem::size_of::<Evidence<T, State, W>>()` is equal to `std::mem::size_of::<T>()`. Transitions are compiled as identity moves.

### 3. `wasm4pm_compat::state`
Exposes the lifecycle typestate markers and their corresponding compile-time transitions.
- **Key Tokens**: `Raw`, `Parsed`, `Admitted`, `Refused`, `Projected`, `Exportable`, `Receipted`.
- **Transition Markers**: `RawToParsed`, `ParsedToAdmitted`, `ParsedToRefused`, `AdmittedToProjected`, `AdmittedToExportable`, `AdmittedToReceipted`, `ProjectedToExportable`, `ProjectedToReceipted`, `ExportableToReceipted`.

### 4. `wasm4pm_compat::witness`
Houses the metadata and ontology markers for standards and academic references.
- **Key Traits**: `Witness` (basic standard identity), `WitnessFamily` (ontological grouping), `WitnessState` (witness validity tracking).
- **Key Macros**: `witness_marker!` (for generating zero-cost empty enum types representing witness standards).

### 5. `wasm4pm_compat::loss`
Encapsulates named, loss-aware model projections.
- **Key Traits**: `Project<To>` (governs conversion from one representation to another).
- **Key Structs/Enums**: `LossPolicy` (explict rules for discarding structure), `LossReport<From, To, Items>` (itemizes flattened nodes), `ProjectionName` (identifies the projection type).

### 6. `wasm4pm_compat::eventlog`
Defines the canonical case-centric event log hierarchy.
- **Key Structs**: `Event` (individual event containing activity name and attributes), `Trace` (an ordered sequence of events), `EventLog` (a collection of traces), `EventLogClassifier` (classification criteria).

### 7. `wasm4pm_compat::ocel`
Defines the object-centric event log (OCEL) shapes mapping directly to the OCEL 2.0 schema.
- **Key Structs**: `OcelLog` (top-level object-centric log), `OcelEvent` (events associated with multiple objects), `Object` (typed domain entities), `EventObjectLink`, `ObjectObjectLink`, `ObjectChange`.

### 8. `wasm4pm_compat::xes`
Defines the IEEE 1849 Extensible Event Stream (XES) interchange shape.
- **Key Structs/Markers**: `XesLog`, `XesTrace`, `XesEvent`, `XesExtension` (extension declarations), `CaseCentricMarker` (ZST separating case-centric and object-centric logs).

### 9. `wasm4pm_compat::bpmn`
Encapsulates the graph shapes of Business Process Model and Notation (BPMN) models.
- **Key Structs**: `BpmnProcess` (the process graph), `BpmnNode` (a vertex), `BpmnEdge` (directed sequence flow), `BpmnTask` (work activity vertex), `BpmnGateway` (logical split/join), `BpmnEvent` (start/intermediate/end vertex).

### 10. `wasm4pm_compat::petri`
Models Petri net, WF-net (Workflow Net), and OC-Petri-net (Object-Centric Petri Net) structural shapes.
- **Key Structs/Markers**: `PetriNet` (places, transitions, arcs), `WfNet` (Workflow net enclosing source/sink and a soundness type-claim), `Marking` (token distribution), `PlaceNodeMarker`, `TransitionNodeMarker`.
- **Soundness Claim States**: `SoundnessUnknown`, `SoundnessClaimed`, `SoundnessWitnessed`.

### 11. `wasm4pm_compat::powl`
Supports the Partially Ordered Workflow Language (POWL) shape as a first-class member.
- **Key Structs**: `Powl` (root process expression), `PowlNode` (node carrying an operator/leaf), `OrderEdge` (precedence link).
- **Node Witness Markers**: `Atom`, `PartialOrder`, `Choice`, `Loop`, `Silent`, `Irreducible`, `AcyclicPartialOrder`, `ProcessTreeProjectable`, `ExceedsProcessTree`.

### 12. `wasm4pm_compat::process_tree`
Defines block-structured process tree models.
- **Key Structs**: `ProcessTree` (root of the tree), `ProcessTreeNode` (operator node or activity leaf).
- **Key Enums**: `ProcessTreeOperator` (Sequence, Xor, Parallel, Loop, Silent, Or).
- **Helper Functions**: `operator_minimum_arity`, `operator_maximum_arity`.

### 13. `wasm4pm_compat::declare`
Expresses declarative process models (Declare and Object-Centric Declare).
- **Key Structs**: `Activity` (labeled activity template), `DeclareTemplate` (template kinds like Response, Precedence, Succession), `DeclareScope` (object-centric activation parameters), `DeclareConstraint` (fully scoped declarative constraint).

### 14. `wasm4pm_compat::dfg`
Models Directly-Follows Graphs (DFG).
- **Key Structs**: `DirectlyFollowsGraph` (activity vertices and transition edges), `DfgNode` (activity vertex), `DfgEdge` (transition directed edge), `DfgWeight` (follows-frequencies), `DfgActivityId` (typed activity wrapper).

### 15. `wasm4pm_compat::conformance`
Houses the structures representing conformance analysis verdicts.
- **Key Structs**: `ConformanceVerdict` (aggregated fitness and alignment results), `Deviation` (sync vs log/model only moves), `SyncMove`, `LogOnlyMove`, `ModelOnlyMove`.
- **Bounded Quality Metrics**: `Fitness`, `Precision`, `F1` rational types bounds-checked in `[0, 1]` at compile time via `Metric<KIND, NUM, DEN>`.

### 16. `wasm4pm_compat::prediction`
Models predictive process monitoring problems.
- **Key Structs/Enums**: `PredictionProblem` (prefix trace and horizon details), `PredictionHorizon` (unbounded `FullCase`, fixed `Events(n)`, or real-time `TimeUnits(secs)`), `OutcomeLabel`, `RemainingTime`, `NextActivity`, `DriftSignal`.

### 17. `wasm4pm_compat::receipt`
Defines cryptographic, provenance-bearing evidence envelopes.
- **Key Traits**: `WellShaped` (structural validity check).
- **Key Structs**: `ReceiptShape` (witness name, hash digest, and replay hint), `ReceiptEnvelope` (wraps evidence in a receipt), `ReceiptChain` (nested receipts), `Digest`, `ReplayHint`.

### 18. `wasm4pm_compat::ids`
Contains zero-cost transparent identifier newtypes enforcing namespace isolation.
- **Key Traits**: `TypedId` (generic id interface).
- **Key Structs**: `EventId<K>`, `ObjectId<K>`, `CaseId<K>`, `ActivityId<K>` wrapping `u64` or `u32` with a phantom kind marker `K` to prevent identifier mix-ups.

### 19. `wasm4pm_compat::diagnostic`
Provides the compatibility linter/diagnostic vocabulary.
- **Key Enums**: `CompatDiagnostic` (diagnostics like `MissingWitness`, `MissingRoundTripFixture`, `HiddenFlattening`), `DiagnosticSeverity` (Error, Warning, Info).

### 20. `wasm4pm_compat::prelude`
An adoption convenience prelude containing the minimum essential imports.
- **Exports**: `Evidence`, `Admission`, `Admit`, `Refusal`, `Event`, `Trace`, `EventLog`, `OcelLog`, `Raw` to `Receipted` typestates, and object lifecycle phases.

---

## 3. Engine Bridge Interface (`wasm4pm` feature)

When a capability exceeds structure-only logic (e.g. executing model discovery, token replay, alignment calculations, or cryptographic signing), the compat layer transitions to the `wasm4pm` execution engine.

### `GraduateToWasm4pm`
Declared in `src/engine_bridge.rs` (gated under `wasm4pm` feature). This trait acts as the explicit exit boundary from compat to execution.

```rust
pub trait GraduateToWasm4pm {
    /// Produces a reviewable case for graduation.
    fn candidate(&self) -> GraduationCandidate;
}
```

- **`GraduationReason`**: Encapsulates the trigger signal that initiated graduation (e.g., `NeedsDiscovery`, `NeedsConformanceExecution`, `NeedsReplay`, `NeedsReceipts`, `NeedsBenchmarkGate`, `NeedsObjectCentricQueryExecution`, `RebuildingProcessMiningLocally`).
- **`GraduationCandidate`**: Structure holding the `reason`, `subject` string, and a grounded `evidence_ref` string digest.

---

## Related Documentation

- Back to [README](../../README.md)
- [Module Map & Layout](module-map.md)
- [Evidence Lifecycle States](lifecycle-states.md)
- [Feature Model](feature-model.md)
