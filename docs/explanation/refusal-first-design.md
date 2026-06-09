# Explanation: Refusal-First Boundary Design

This document discusses the architectural context and advantages of the "refusal-first" design pattern implemented in `wasm4pm-compat` version `26.6.9`.

---

## The Anti-Pattern: Generic Runtime Errors

In traditional software development, validation failures are typically handled by returning generic error strings, throwing dynamic exceptions (e.g. `Box<dyn Error>`), or returning basic booleans (`false`). 

While this works for simple applications, it is problematic in process intelligence systems because:
1. **Loss of Provenance**: Downstream systems cannot programmatically audit why a specific log was rejected.
2. **Poor Agent Loop Integration**: In agentic process-mining environments (like PMAx), LLM-based agents need precise, structured feedback to correct validation errors. A message like `Error: invalid log` provides no actionable guidance.
3. **Weak Type Boundaries**: If errors are not typed, the compiler cannot enforce that specific boundary gates handle all possible rejection laws.

---

## The Solution: Refusal-First Design

`wasm4pm-compat` treats validation failures as first-class, strongly-typed domain results called **Refusals**. 

```rust
pub struct Refusal<R, W> {
    pub reason: R,
    witness: PhantomData<W>,
}
```

The key properties of this pattern are:

### 1. Strongly-Typed Law Vocabularies
Instead of returning a string, the `reason` field is a domain-specific enum (such as `OcelRefusal` or `PetriRefusal` or `StrictViolation`). Each enum variant corresponds to a specific, named rule from process-mining literature or system covenants:
- `OcelRefusal::DanglingEventObjectLink`
- `PetriRefusal::MissingInitialMarking`
- `StrictViolation::MissingLossPolicy`

Developers cannot return arbitrary strings; they must map every rejection to an explicit law.

### 2. Typestate Gating at the Boundary
Because `Admission` and `Refusal` are distinct types, the compiler forces you to handle the failure path. When calling `Admit::admit()`, you receive a `Result<Admission<T, W>, Refusal<R, W>>`. The only way to obtain the `Evidence<T, Admitted, W>` wrapper is to consume the `Admission` struct:

```rust
// The compiler guarantees that raw evidence cannot be laundered
// into admitted evidence without matching the Result variants
let admitted = match OcelLog::admit(raw) {
    Ok(admission) => admission.into_evidence(),
    Err(refusal) => {
        // Precise, programmatic handling of the refusal reason
        log_violation_to_audit_ledger(refusal.reason);
        panic!("Validation failed");
    }
};
```

### 3. Loop Feedback for Autonomous Agents
In PMAx, AI agents operate inside execution sandboxes. When an agent attempts to write a log that violates type laws, the runtime catches the failure and returns the structured `Refusal`. The agent parses the enum variant (e.g. `MissingObjectScope`) and executes iterative corrections on the source code or query structure to resolve that specific constraint.
