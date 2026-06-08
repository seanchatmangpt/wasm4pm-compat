# Reference: Evidence Lifecycle States

This document details the compile-time evidence states, their corresponding typestate markers, constructors, transition rules, and the graduation mechanism to the `wasm4pm` execution engine.

---

## The Canonical States

All evidence states in `wasm4pm-compat` are represented by uninhabited, zero-sized empty enums implementing the sealed `EvidenceState` trait defined in `src/state.rs`.

| State Name | Typestate Marker | Allowed Next States | Transition Method / Constructor | Purpose |
| :--- | :--- | :--- | :--- | :--- |
| **`Raw`** | `Raw` | `Parsed`, `Refused` | `Evidence::raw(value)` | Entry gate for untrusted external inputs. |
| **`Parsed`** | `Parsed` | `Admitted`, `Refused` | `evidence.into_parsed()` | Syntactically decoded and well-formed shape, but not yet semantic-bound. |
| **`Admitted`** | `Admitted` | `Projected`, `Exportable`, `Receipted` | `Admit::admit(raw)` | Certified as structurally valid against a named witness. Constructor is gated behind the `Admit` trait. |
| **`Refused`** | `Refused` | *None (Terminal)* | `evidence.into_refused()` / `raw.refuse()` | Blocked boundary entry carrying a specific named law violation reason. |
| **`Projected`** | `Projected` | `Exportable`, `Receipted` | `admitted.project(name, policy)` | Result of a named, accounted lossy projection. |
| **`Exportable`** | `Exportable` | `Receipted` | `admitted.into_exportable()` / `projected.into_exportable()` | Cleared to exit the process boundary. |
| **`Receipted`** | `Receipted` | *None (Terminal in Compat)* | `admitted.into_receipted()` / `exportable.into_receipted()` | Sealed inside a cryptographic provenance-bearing receipt envelope. |
| **`Graduated`** | *N/A (Feature-gated)* | *N/A (Engine Domain)* | `GraduateToWasm4pm::candidate()` | Escapes compat boundary; hand-off to the `wasm4pm` execution engine. |

---

## State Transition Rules

The state transition flow is unidirectional. The Rust compiler statically enforces that transitions cannot skip steps (e.g. converting `Raw` directly to `Exportable` without admission) or backtrack:

```text
  Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ {Projected | Exportable | Receipted} ──graduate──▶ Graduated (Engine)
    │                                  ▲
    └────────────── refuse ────────────┴──▶ Refused (terminal; carries named law reason)
```

### Transition Descriptions:

1. **`Raw` → `Parsed` (Parsing Gate)**:
   - Takes unvalidated bytes or structures and performs syntactic validation.
   - Example: A raw JSON string containing OCEL data is parsed into `Evidence<OcelLog, Parsed, W>`.

2. **`Parsed` → `Admitted` (Admission Gate)**:
   - Evaluates the parsed data against a specific `Witness` standard.
   - Enforced by the `Admit` trait. Only data that satisfies all semantic invariants of the witness is admitted.
   - Example: `LinkedOcel::admit(...)` checks that no event-object link references a non-existent object.

3. **`Parsed` / `Raw` → `Refused` (Refusal)**:
   - If syntactic or semantic validation fails, the evidence is immediately routed to the `Refused` state.
   - The refused value carries the named law reason (e.g., `DanglingEventObjectLink`, `UnsoundWfNet`) to preserve auditability.

4. **`Admitted` → `Projected` (Projection)**:
   - Gated under the `formats` feature. Applies an explicit `LossPolicy` to convert evidence between formats (e.g. OCEL to case-centric XES).
   - Generates a `LossReport` to account for any discarded structure, preventing secret flattening.

5. **`Admitted` / `Projected` → `Exportable` (Export Authorization)**:
   - Grants the evidence an "exit visa" to leave the boundary of the migrated.

6. **`Admitted` / `Projected` / `Exportable` → `Receipted` (Provenance Seal)**:
   - Seals the evidence in a cryptographic `ReceiptEnvelope` that embeds a content digest (e.g., hex BLAKE3) and a replay hint.

7. **`Admitted` / `Projected` / `Exportable` / `Receipted` → `Graduated` (Graduation Boundary)**:
   - Gated under the `wasm4pm` feature.
   - When operations require execution semantics (such as model discovery, conformance replay, alignment calculations, or query execution), the compat value implements the `GraduateToWasm4pm` trait.
   - This produces a `GraduationCandidate` that is consumed and processed by the `wasm4pm` execution engine.

---

## Zero-Cost Guarantees

Every evidence stage wrapper is defined via `PhantomData` generics:

```rust
pub struct Evidence<T, State: EvidenceState, W> {
    pub value: T,
    pub state: PhantomData<State>,
    pub witness: PhantomData<W>,
}
```

- **Size Invariant**: `std::mem::size_of::<Evidence<T, State, W>>()` is exactly equal to `std::mem::size_of::<T>()`.
- **Runtime Cost**: Every state transition method (e.g., `into_parsed()`, `into_exportable()`) compiles down to a direct identity move on the underlying `T` bytes. There are no allocations, pointer indirections, or runtime assertions generated.
- **Compile-Time Enforcement**: Downstream crates cannot invent custom lifecycle states; implementation of `EvidenceState` is locked via a sealed sub-trait (`private::Sealed`).

---

## Related Documentation

- Back to [README](../../README.md)
- [Public API for ggen](public-api-for-ggen.md)
- [Module Map & Layout](module-map.md)
- [Feature Model](feature-model.md)
- [Rust Typestate and Process Theory](../../docs/explanation/rust-typestate-and-process-theory.md)
