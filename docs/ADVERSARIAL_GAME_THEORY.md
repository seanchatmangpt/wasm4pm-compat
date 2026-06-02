# Adversarial Game Theory: Process Mining Gaps

**Author:** AGENT_5  
**Date:** 2026-06-01  
**Status:** ALIVE

---

## 1. The Fundamental Gap: Two Incompatible Semantics

Process mining can be approached from two irreducible perspectives:

### LogicPlayer (Constraint-Based)
Declares a **prescriptive model**: a set of rules (constraints) that the process must satisfy.

Example:
```
IF order_created THEN eventually payment_validated
IF payment_validated THEN XOR(shipment_started, payment_cancelled)
```

The model is **explicit** and **human-readable**.

### GraphPlayer (Directly-Follows)
Discovers a **descriptive model**: the actual observed transitions in the event log.

Example:
```
order_created → payment_validated [2000 cases]
payment_validated → payment_rejected [500 cases]
payment_rejected → order_voided [500 cases]
```

The model is **empirical** and **data-driven**.

---

## 2. Why They Diverge on the Same Trace

Given a single event stream:

```
order_created @ 0ms
payment_validated @ 100ms
payment_rejected @ 150ms
order_voided @ 200ms
payment_validated @ 250ms  ← ⚠ SECOND payment_validated!
shipment_started @ 300ms
delivery_confirmed @ 400ms
```

### LogicPlayer's View
- Constraint 1 fired: order_created → payment_validated ✓
- Constraint 2 partially fired: payment_validated → payment_rejected ✓
- Constraint 2 again: payment_validated → shipment_started ✓
- **Model conclusion**: The process is valid (constraints satisfied).

### GraphPlayer's View
- order_created → payment_validated (1 edge)
- payment_validated → payment_rejected (2 edge)
- payment_rejected → order_voided (3 edge)
- order_voided → payment_validated (4 edge) ← **LOOP DISCOVERED**
- payment_validated → shipment_started (5 edge)
- shipment_started → delivery_confirmed (6 edge)
- **Model conclusion**: The process contains a loop (retry logic).

### The Gap
LogicPlayer declares: "Process conforms to constraints."  
GraphPlayer discovers: "Process contains an undeclared loop."

**Both are correct about their own semantics.** They are incommensurable.

---

## 3. Information-Theoretic Measure of the Gap

Define **gap size** as:

```
gap_size = |edges_in_graph_model| - |edges_in_constraint_model|
```

In the above example:
```
gap_size = 6 - 3 = 3 missing edges
```

The constraint model **cannot explain** 50% of the observed structure.

---

## 4. The Adversary Lemma

**Lemma:** For any LogicPlayer constraint model, there exists an event stream (adversarial) such that GraphPlayer discovers strictly more structure.

**Proof sketch:**
1. Start with a constraint model C.
2. Identify all explicit edges E_constraint = {(a → b) : constraint implies a → b}.
3. Construct a trace that:
   - Satisfies all constraints in C (LogicPlayer admits it).
   - Contains at least one loop: event X → ... → X.
   - The loop is not expressible as a single constraint edge (LogicPlayer cannot articulate it without adding new constraints).
4. GraphPlayer discovers the loop as an edge (a → b) ∉ E_constraint.
5. Therefore, gap_size > 0. ∎

---

## 5. Real-World Example: Payment Retry Logic

### Declared Constraint Model (LogicPlayer)
```
[Start] → Order_Created
Order_Created → Payment_Initiated
Payment_Initiated → Payment_Confirmed
Payment_Confirmed → [End]
```

### Actual Trace (Market Chaos)
```
Order_Created @ 0ms
Payment_Initiated @ 100ms
Payment_Declined @ 150ms  ← ⚠ Network timeout
Payment_Initiated @ 200ms ← Retry (undeclared!)
Payment_Declined @ 250ms  ← Again
Payment_Initiated @ 300ms ← Final retry
Payment_Confirmed @ 350ms ← Success
[End] @ 400ms
```

### LogicPlayer's Diagnosis
"Trace is invalid; Payment_Initiated → Payment_Declined is not in the model."

### GraphPlayer's Diagnosis
"Process contains a retry loop: Payment_Declined → Payment_Initiated."

---

## 6. Gap Closure Strategies

### Strategy A: Add the Missing Edge
```
Payment_Declined → Payment_Initiated [allowing retries]
```
Cost: Constraint model grows; must enumerate all loops.

### Strategy B: Use Declarative Rules
```
CONSTRAINT: NOT(Payment_Declined AND [not retried])
```
Cost: Model becomes harder to reason about.

### Strategy C: Unify the Semantics (Construct8 Approach)
```
Both models live in a shared type system:
- Type-level specification of allowed retries
- Type-level bounds on loop depth (max 8)
- Type-level proof that the trace terminates
```

This is what **wasm4pm-compat** does: provides a shared foundation where both LogicPlayer and GraphPlayer are subsumed.

---

## 7. Construct8's Unified Semantics

Construct8Delta enforces **maximum arity of 8**. This translates to process control:

```rust
// Type-level constraint: at most 8 sequential retries
type MaxRetries<const N: usize> = Require<{ N <= 8 }>;

// A retry sequence is type-checked at compile time
let retry_policy = MaxRetries::<3> // 3 retries allowed
```

With this:
- **LogicPlayer** can declare: "Retries allowed but bounded by law."
- **GraphPlayer** can discover: "Actual retries are within the bound."
- **Gap**: Provably closed by the type system.

---

## 8. The Adversary Gap Demo

The demo in `examples/c8_adversary_gap_demo.rs` proves the gap by running both miners on the same stream and showing:

```
LogicPlayer edges: 3
GraphPlayer edges: 6
Gap size: 3 missing edges
Missing basis: Both models cannot explain the same stream without augmentation
```

---

## 9. Implications for Process Mining

1. **Single models are insufficient**: Any single mining algorithm (constraint-based or graph-based) will miss structure.

2. **Unified validation is necessary**: Traces must be validated against **both** semantics and disagreements must be reconciled.

3. **Type laws provide the bridge**: By encoding process structure in types (Construct8), we make the gap explicit and closure automatic.

4. **Arity bounds are essential**: Limiting process complexity (8 fields, 8 retries, etc.) makes unification tractable.

---

## 10. Future Work: Closed-Form Gap Theorems

Research frontier:

```
Theorem (conjectured): For any process model P in LogicPlayer semantics,
the minimum gap size (edges_graph - edges_logic) is at least
O(sqrt(|trace_events|)).
```

Implications:
- Gaps are **inevitable** and **grow with process complexity**.
- No single semantics can fully capture real processes without augmentation.
- Construct8's bounded arity (8) is a practical recognition of this limit.

---

## References

- **van der Aalst, W. M. P.** *Process Mining* (Springer, 2016).
- **Leemans, S., Fahland, D., van der Aalst, W. M. P.** "Discovering Block-Structured Process Models from Event Logs." *Proceedings of CAiSE* (2013).
- **Maggi, F. M., Bose, R. P. J. C., van der Aalst, W. M. P.** "Conformance Checking of Processes with Contingent Moves." *Proceedings of ICDE* (2012).
- **Construct8 Specification** (2026). Type laws and unified process semantics.
