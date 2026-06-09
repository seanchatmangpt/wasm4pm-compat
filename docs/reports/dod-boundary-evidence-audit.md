# DoD Boundary and Evidence Gates Compliance Audit (v26.6.9)

## Executive Summary
This report audits compliance of the `wasm4pm-compat` codebase (version `26.6.9`, specifically `/Users/sac/wasm4pm-compat/src`) against the DoD Boundary and Evidence gates:
1. **No format-to-format laundering**: Checked.
2. **Public rejections refuse with a specific named law (not bare `InvalidInput`)**: Checked.
3. **Lossy projection carries `ProjectionName`, `LossPolicy`, `LossReport`, and a refusal path**: Checked.
4. **Admission and refusal are first-class values (not panics)**: Checked.

---

## 1. No Format-to-Format Laundering

The codebase structurally prevents format-to-format laundering (direct conversion from one external format to another without an intermediate admitted state).

### Findings:
- The crate is designed as **structure-only** and contains no implementation for parsing, serializing, or direct translation of format bytes.
- The only way to move between external format bytes and typed compat values is through the traits defined in [formats.rs](file:///Users/sac/wasm4pm-compat/src/formats.rs).
- [ImportFormat::import](file:///Users/sac/wasm4pm-compat/src/formats.rs#L203-L205) enforces a boundary: it accepts only an unadmitted `FormatEnvelope` and returns an `Admission` or a `Refusal` (never a raw value).
- [ExportFormat::export](file:///Users/sac/wasm4pm-compat/src/formats.rs#L232-L232) accepts a typed source and a `LossPolicy`, returning a `FormatExport` or a refusal.
- Format translation must follow the path:
  `FormatEnvelope` $\rightarrow$ `Admission` $\rightarrow$ `Admitted` evidence $\rightarrow$ `ExportFormat` / `Project`.
- Comments in [formats.rs:L13-15](file:///Users/sac/wasm4pm-compat/src/formats.rs#L13-L15) explicitly state:
  > *No raw format-to-format laundering. There is no `import_then_export` that skips the typed admitted middle. Every translation is external $\rightarrow$ admitted compat $\rightarrow$ external.*

---

## 2. Specific Named Law Refusals (No Bare `InvalidInput`)

Public rejections across the codebase do not use generic, catch-all errors like `InvalidInput`. Instead, they return specific named laws corresponding to structural and domain-specific rules.

### Findings:
- There are **17 distinct refusal enums** defined throughout the codebase, representing domain-specific laws:
  1. [BpmnRefusal](file:///Users/sac/wasm4pm-compat/src/bpmn.rs#L575)
  2. [CausalNetRefusal](file:///Users/sac/wasm4pm-compat/src/causal_net.rs#L149)
  3. [ConformanceRefusal](file:///Users/sac/wasm4pm-compat/src/conformance.rs#L289)
  4. [DeclareRefusal](file:///Users/sac/wasm4pm-compat/src/declare.rs#L178)
  5. [OcDeclareRefusal](file:///Users/sac/wasm4pm-compat/src/declare.rs#L267)
  6. [DfgRefusal](file:///Users/sac/wasm4pm-compat/src/dfg.rs#L105)
  7. [EventLogRefusal](file:///Users/sac/wasm4pm-compat/src/eventlog.rs#L176)
  8. [InteropRefusal](file:///Users/sac/wasm4pm-compat/src/interop.rs#L302)
  9. [PetriNetRefusal](file:///Users/sac/wasm4pm-compat/src/models.rs#L5)
  10. [OcelRefusal](file:///Users/sac/wasm4pm-compat/src/ocel.rs#L642)
  11. [OcpqRefusal](file:///Users/sac/wasm4pm-compat/src/ocpq.rs#L835)
  12. [PetriRefusal](file:///Users/sac/wasm4pm-compat/src/petri.rs#L40)
  13. [PowlRefusal](file:///Users/sac/wasm4pm-compat/src/powl.rs#L682)
  14. [PredictionRefusal](file:///Users/sac/wasm4pm-compat/src/prediction.rs#L333)
  15. [ProcessTreeRefusal](file:///Users/sac/wasm4pm-compat/src/process_tree.rs#L539)
  16. [ReceiptRefusal](file:///Users/sac/wasm4pm-compat/src/receipt.rs#L362)
  17. [XesRefusal](file:///Users/sac/wasm4pm-compat/src/xes.rs#L1096)
- Documented comments explicitly prohibit `InvalidInput` (e.g., in [admission.rs:L9-12](file:///Users/sac/wasm4pm-compat/src/admission.rs#L9-L12)):
  > *`Refusal<R, W>` — the value was declined for a specific named reason `R`... A bare "invalid input" is not an acceptable reason here.*
  A grep scan confirms that "InvalidInput" is only mentioned in comments explicitly forbidding it.

---

## 3. Lossy Projection Governance

Lossy projections are structured to prevent unaccounted data loss. They carry the required metadata and refusal options.

### Findings:
- The [Project](file:///Users/sac/wasm4pm-compat/src/loss.rs#L953-L974) trait governs all lossy transformations:
  ```rust
  pub trait Project {
      type From;
      type To;
      type Lost;
      type Reason;

      fn project(
          self,
          policy: LossPolicy,
      ) -> Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason>;
  }
  ```
- The trait implementation receives a [LossPolicy](file:///Users/sac/wasm4pm-compat/src/loss.rs#L49-L58) (`RefuseLoss`, `AllowNamedProjection`, or `AllowLossWithReport`) to define the rules of engagement *before* projection occurs.
- On success, it returns a [LossReport](file:///Users/sac/wasm4pm-compat/src/loss.rs#L385-L394) carrying:
  - `projection: ProjectionName` — naming the projection family.
  - `policy: LossPolicy` — the policy that authorized the loss.
  - `lost: Items` — the concrete list of discarded evidence.
- The refusal path is represented by the `Err(Self::Reason)` variant. If the policy is `LossPolicy::RefuseLoss` and loss would occur, it must return a named refusal (e.g., `XesRefusal::LiftingLoss`).
- Realized examples can be found in [interop.rs:L696-733](file:///Users/sac/wasm4pm-compat/src/interop.rs#L696-L733) for `OcelToXesProjection` and `XesToOcedProjection`.

---

## 4. First-Class Verdict Values (No Panics)

Verdicts are modeled using strongly-typed values, preventing program termination or panics.

### Findings:
- [Admission](file:///Users/sac/wasm4pm-compat/src/admission.rs#L37-L41) and [Refusal](file:///Users/sac/wasm4pm-compat/src/admission.rs#L105-L109) are modeled as distinct, first-class values returned inside Rust's standard `Result` type.
- A scan for `panic!` inside `/src` returned **zero results**, confirming that the library code does not panic.
- Scans for `unwrap()` and `expect()` confirm that their only occurrences are:
  - In doc tests and examples to simplify assertions.
  - Using safe fallback methods like `unwrap_or` or `unwrap_or_default`.

---

## Conclusion
The `wasm4pm-compat` codebase is **100% compliant** with the DoD Boundary and Evidence gates. It successfully enforces a strict, type-level boundary for process-evidence standard shapes.
