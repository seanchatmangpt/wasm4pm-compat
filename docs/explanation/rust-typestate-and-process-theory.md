# Rust Typestate and Process Theory

This document explains how `wasm4pm-compat` utilizes Rust's type system to model process theory concepts. Specifically, it focuses on the **evidence lifecycle typestate**, how **affine ownership** maps to token conservation, and the boundaries between compile-time and runtime verification.

---

## 1. The Evidence Lifecycle Typestate

In process mining and execution governance, data is not a static property. It progresses through a series of logical stages: from external unverified inputs, through parsing and structural validation, to formal admission, named projection, and ultimate cryptographic sealing and execution.

In `wasm4pm-compat`, this progression is represented as a state machine encoded at compile time using Rust's **Typestate Pattern**. The core type is the `Evidence` carrier:

```rust
pub struct Evidence<T, State: EvidenceState, W> {
    pub value: T,
    pub state: PhantomData<State>,
    pub witness: PhantomData<W>,
}
```

The lifecycle is defined by the following state transitions:

```text
  Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ {Projected | Exportable | Receipted} ──graduate──▶ [Graduated]
    │                                  ▲
    └────────────── refuse ────────────┴──▶ Refused  (terminal: a named law was broken)
```

Each stage is represented by a distinct, uninhabited type (an empty `enum`):

1. **`Raw`**: Untrusted data arriving from outside the system (e.g. raw JSON/XML bytes). Anyone can construct `Evidence<T, Raw, W>` freely using `Evidence::raw()`. However, `Raw` evidence has no public methods allowing it to be inner-extracted or exported, forcing developers to pass it through the validation gates.
2. **`Parsed`**: The raw data has been parsed by a format decoder (e.g., OCEL JSON or XES XML parser). It is structurally well-formed, but its semantic properties are not yet verified.
3. **`Admitted`**: The parsed value has been validated against a named `Witness` authority (e.g., `Ocel20` or `Xes1849`). This transition requires passing through an `Admit` implementation, which returns either an `Admitted` token or a typed `Refusal` (e.g., `OcelRefusal`).
4. **`Projected`**: The admitted evidence has undergone a named, accounted projection (such as projecting object-centric logs to case-centric traces). This stage is accompanied by a `LossReport`, ensuring that any flattening or information loss is declared.
5. **`Exportable`**: The evidence has been cleared to leave the boundaries of the crate. It acts as an "exit visa" for external interchange.
6. **`Receipted`**: The strongest structural stage in `wasm4pm-compat`. The evidence is wrapped in a cryptographic/provenance-bearing receipt envelope, preparing it for hand-off to the execution engine.
7. **`Graduated`**: A transitional conceptual stage where the evidence is wrapped in a `GraduationCandidate` and passed to the `wasm4pm` execution engine. Once graduated, structure-only limits are exceeded, and the `wasm4pm` runtime takes authority.

Because these stages are distinct Rust types, a function signature like:
```rust
fn execute_conformance(evidence: Evidence<OcelLog, Admitted, Ocel20>)
```
cannot be called with `Evidence<OcelLog, Raw, Ocel20>`. The compiler rejects this mismatch at compile time, completely eliminating the possibility of bypassing admission checks at runtime.

---

## 2. Affine Ownership as Token Conservation

In Petri net theory, a marking changes when a transition fires. Firing a transition consumes tokens from input places and deposits tokens in output places. Tokens cannot be silently duplicated or destroyed without violating the conservation laws of the net:
$$\sum_{p \in P} M'(p) = \sum_{p \in P} M(p) - |\bullet t| + |t\bullet|$$

In ordinary programming, variables can be copied, referenced, or duplicated arbitrarily, which makes it easy to model illegal state transitions (such as double-spending a token or reusing a stale state).

Rust solves this using its **affine type system**. By default, variables in Rust have **move semantics**: passing a value to a function or method transfers its ownership, making the original variable name unusable.

`wasm4pm-compat` maps this affine behavior directly to token conservation in process theory:

- **State Transitions by Value**: All state-transition methods on the `Evidence` struct consume the instance by value (`self`):
  ```rust
  impl<T, W> Evidence<T, Raw, W> {
      pub fn into_parsed(self) -> Result<Evidence<T, Parsed, W>, Refusal> { ... }
  }
  ```
  When `into_parsed` is called, the original `Evidence<T, Raw, W>` is consumed and destroyed. The compiler guarantees that the raw state can never be reused or referenced again, mirroring the consumption of a token in a Petri net transition.
- **Double-Spent Prevention**: Once evidence has been moved or advanced to a new stage, any attempt to access the old variable triggers a compile error. This makes it impossible to launder raw evidence twice, reuse a refused state, or duplicate execution tokens in a pipeline.
- **Linear Branch Tracking**: For parallel split-join patterns (e.g. `ParallelWorkflow`), the split operation consumes a single workflow token and produces multiple branch tokens (`BranchToken<A, Running>`, `BranchToken<B, Running>`). The join operation consumes both branch tokens by value to produce a single merged token. If a branch is cancelled (`cancel_b_from_a`), the running token is consumed and a `Canceled` token is returned, ensuring that the canceled branch can never be joined as `Completed`.

---

## 3. Compile-Time vs. Runtime Verification Limits

Rust's type system provides a powerful static validator, but it is bounded. We define a clear line between what is checked at compile time and what must graduate to runtime verification.

### Compile-Time Verification (Static Laws)
The type system is **sound and complete** for verifying structural and static properties:
- **Type/Namespace Isolation**: The compiler guarantees that identifiers (e.g., `EventId<K>`, `ObjectId<K>`) cannot cross namespaces, preventing mismatched object/event bindings.
- **Constant Bounds**: Const generics enforce static bounds (such as place capacity `BITS <= 8` or quality metric boundaries `0 <= NUM/DEN <= 1`) at zero runtime cost.
- **One-Way Door Enforcement**: The type system guarantees that `Raw` evidence cannot bypass the `Admitted` gate.
- **Structural Invariants**: The compiler ensures Petri net arcs are strictly bipartite, and process trees enforce correct node arities (such as loops having exactly 2 children).

### Runtime Verification (Dynamic Execution)
Properties that depend on dynamic data values, search graphs, or non-local relationships cannot be verified at compile time (due to undecidability or exponential complexity) and must **graduate to the `wasm4pm` execution engine**:
- **Non-Local OR-Joins**: Evaluating whether an OR-join is enabled requires deciding if any active token can reach the join's input. Since reachability over arbitrary loops is undecidable at compile time, this is evaluated at runtime using Reset Net state-space checks.
- **Conformance Check Alignments**: Computing the alignment between an event log and a Petri net requires an optimization search (such as A* or Dijkstra's algorithm) over the state space. This is computationally expensive and must be run inside `wasm4pm`.
- **Dynamic Multiple Instances (MI)**: When a process spawns a dynamic number of parallel branches (e.g. one task per item in a dynamic cart), the compiler cannot know the number of branches in advance. This requires runtime collections and counters.
- **Cryptographic Receipt Chaining**: Validating signatures, verification keys, and proof chains on a receipt envelope requires runtime cryptographic execution.

By separating the structural doorway (`wasm4pm-compat`) from the execution authority (`wasm4pm`), the system achieves static safety wherever possible, while cleanly graduating heavy computations to the runtime engine.

---

## Related Documentation

- Back to [README](../../README.md)
- [Genesis Thursday: Day Five Conceptual Framing](genesis-thursday.md)
- [Process-Evidence Domain Glossary](glossary.md)
- [Evidence Lifecycle States Reference](../reference/lifecycle-states.md)
- [Process Theory Alignment Research](../../research/process-theory-alignment.md)
