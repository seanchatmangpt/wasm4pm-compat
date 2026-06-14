# The Refusal Law

> Refusal is first-class. Every serious surface refuses with a **specific named
> law** — never a bare `InvalidInput`.

In `wasm4pm-compat`, declining a value is not an error condition to be papered
over; it is a *typed, auditable outcome*. The `Admit` trait returns
`Result<Admission<…>, Refusal<R, W>>`, and the `Project` trait returns
`Result<LossReport<…>, Reason>`. The `R` / `Reason` is the heart of this
document: it must name the **exact law that was broken**, with enough specificity
that a caller knows what to fix and an auditor knows what was rejected.

---

## Why naming matters

A bare refusal hides the boundary's real obligations and makes the contract
untestable. A named refusal does three things at once:

- **Actionable** — the caller learns precisely which structural law failed.
- **Honest** — it documents the obligations a surface actually enforces.
- **Testable** — tests can assert the exact reason, so the contract cannot drift
  (see `tests/admission_refusal.rs`).

A refusal is part of the published surface, just like the admitted value is.

---

## Good vs. bad refusal naming

### ❌ Bad: vague, catch-all, untestable

```rust
// Refuses *everything* the same way. Caller learns nothing.
enum Reason { InvalidInput }
enum Reason { Error }
enum Reason { BadLog }
enum Reason { ParseFailed }          // "parse" is not a structural law
enum Reason { Unknown }
```

These tell you a value was rejected but not **which law** rejected it. They are
the refusal equivalent of `catch (e) { /* swallow */ }`.

### ✅ Good: specific, named after the broken law

Each reason names a concrete structural obligation of a particular witness:

```rust
// OCEL 2.0 (Ocel20) boundary
enum OcelRefusal {
    DanglingEventObjectLink,   // an event references a non-existent object
    UnqualifiedObjectRelation, // an E2O relation lacks its qualifier
    DuplicateObjectId,         // two objects share an identifier
}

// WF-net soundness (WfNetSoundnessPaper) boundary
enum WfNetRefusal {
    MissingFinalMarking,       // no reachable final marking
    UnsoundWfNet,              // soundness criterion violated
    DeadTransition,            // a transition can never fire
}

// XES (Xes1849) boundary
enum XesRefusal {
    MissingConceptName,        // an event lacks `concept:name`
    NonMonotonicTimestamps,    // `time:timestamp` is not ordered within a trace
}

// OCEL → XES flattening (loss boundary)
enum FlattenRefusal {
    FlatteningLoss,            // dropping links the policy does not permit
}

// POWL (PowlPaper) boundary
enum PowlRefusal {
    CyclicPartialOrder,        // the partial order is not acyclic
    DanglingOperatorChild,     // an operator references a missing child
}
```

The test in `tests/admission_refusal.rs` asserts exactly such a reason:

```rust
let refusal = LinkedOcel::admit(Evidence::raw(false)).unwrap_err();
assert_eq!(refusal.reason, OcelRefusal::DanglingEventObjectLink);
```

---

## A litmus test for a refusal name

Ask: *could this name appear in two unrelated boundaries?* If yes, it is too
vague.

- `InvalidInput` — appears everywhere → **bad**.
- `MissingFinalMarking` — only meaningful for WF-net-shaped evidence → **good**.

Ask: *does the name point at structure the witness actually requires?* A refusal
should be readable as "this value violates *clause X* of *witness W*."

- `ParseFailed` describes the *mechanism* (a parser), not the *law* → **bad**.
- `NonMonotonicTimestamps` describes a violated XES obligation → **good**.

---

## Refusal vs. loss

Admission refusal (`Refusal<R, W>`) says *this value may not cross the boundary
at all*. The loss law (see [`LOSS_POLICY.md`](LOSS_POLICY.md)) handles a softer
case: the value *can* cross, but only by **discarding** some evidence. That, too,
is refusable — under `LossPolicy::RefuseLoss`, a projection that would drop
structure returns a **named** reason (e.g. `FlatteningLoss`) instead of losing it
silently. Same discipline, different boundary: never an anonymous "no", never a
secret "yes".

---

## Named reasons in `src/admission.rs`

The `Admit` trait in `src/admission.rs` defines the `Reason` associated type and
`Refusal<R, W>` struct. The module enforces:

1. `Refusal<R, W>` is a first-class struct, not an alias for `String` or `Box<dyn Error>`.
2. The `R` type parameter must name a **specific named law** as its variants.
3. `Refusal::new(reason)` is the only constructor — it accepts the `R` value directly.
4. `Display` for `Refusal<R, W>` formats as `"Refusal: <reason>"` — the witness `W`
   is zero-sized `PhantomData` and carries no display value.

### Named reason examples used across the codebase

| Domain | Named reason variant | Law broken |
|--------|----------------------|------------|
| OCEL 2.0 | `DanglingEventObjectLink` | An event references a non-existent object |
| OCEL 2.0 | `UnqualifiedObjectRelation` | E2O relation lacks its qualifier |
| OCEL 2.0 | `DuplicateObjectId` | Two objects share an identifier |
| WF-net | `MissingFinalMarking` | No reachable final marking |
| WF-net | `UnsoundWfNet` | Soundness criterion violated |
| WF-net | `DeadTransition` | A transition can never fire |
| XES | `MissingConceptName` | Event lacks `concept:name` |
| XES | `NonMonotonicTimestamps` | `time:timestamp` not ordered within a trace |
| Loss | `FlatteningLoss` | Object-centric links dropped without policy |
| POWL | `CyclicPartialOrder` | Partial order is not acyclic |
| POWL | `DanglingOperatorChild` | Operator references a missing child |
| POWL | `InvalidChoiceArity { declared, required_min }` | A choice node has fewer than two branches |
| Declare | `DeclareRefusal::MissingTarget` | A binary constraint names no target activity |
| Causal Net | `CausalNetRefusal::CycleDetected` | Dependency relations form an illegal cycle |
| Process Tree | `ProcessTreeRefusal::CycleDetected` | The tree contains a back-reference |
| Process Tree | `MissingDoBody` | A loop node declares no "do" body child |

The last five were previously **named but unreachable** ("ghost") variants — they named a
law no code path could ever produce. They are now constructible and tested, because a
refusal type that names a law it cannot raise is itself a defect.

### The compile-fail receipt

`tests/ui/compile_fail/refusal_without_named_law.rs` proves this at the type level:
a `Refusal` that uses a catch-all reason type (not a specific named enum variant)
fails to compile. This is the trybuild proof that the law is sealed.

---

## The diagnostic that enforces this

`CompatDiagnostic::MissingRefusalPath` names the meta-law: *every serious surface
must offer a refusal path with a specific named reason.* A boundary that can only
ever say "yes" — or that says "no" with `InvalidInput` — fails this diagnostic and
is not considered a well-formed compat surface.

---

**See also:** [`WITNESS_IMMUTABILITY.md`](WITNESS_IMMUTABILITY.md) — the companion type-level law: an admitted witness `W` is fixed for the value's lifetime, so a new authority requires a new named admission, never a relabeled value.
