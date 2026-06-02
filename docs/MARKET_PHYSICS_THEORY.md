# Market Physics Theory — Construct8 Foundation

**Author:** AGENT_5  
**Date:** 2026-06-01  
**Status:** ALIVE

---

## 1. The MarketPlanck Cell: Atomic Unit of Market State

The MarketPlanck cell is the zero-dimensional state snapshot that captures all relevant market observables at a single logical moment:

```
MarketPlanckCell = {
  last_price: u64,       // The most recent executed price
  total_volume: u64,     // Cumulative volume since simulation start
  high: u64,             // Highest price in current window
  low: u64,              // Lowest price in current window
  timestamp_ns: u64,     // Logical clock (nanoseconds)
}
```

Each tick transitions the cell from one state to another. The transition is **deterministic**: given a pre-state and a tick event, the post-state is always identical.

### Invariants

- **Price monotonicity (weak)**: last_price can rise or fall, but the sequence is order-preserving.
- **Volume monotonicity (strict)**: total_volume is non-decreasing.
- **High/low bounds**: high >= last_price, low <= last_price (always).
- **Timestamp ordering**: timestamp_ns strictly increases across ticks.

---

## 2. Tick Events and Delta Encoding

A **tick event** is the minimal description of an external market occurrence:

```
TickEvent = {
  price: u64,        // Execution price in cents
  volume: u64,       // Trade size
  timestamp_ns: u64, // When the tick occurred
}
```

A **Construct8Delta** is the serialized form of the tick:

```
delta_bytes = serialize({
  event: "tick",
  price: u64,
  volume: u64,
  timestamp: u64,
})
```

Deltas are immutable, deterministic, and transferable across process boundaries.

---

## 3. State Transition: The MarketPlanck Equation

The core computation is a pure function:

```
post_state = apply_tick(pre_state, tick_event)
```

This produces:

```
post_state = {
  last_price: tick.price,
  total_volume: pre_state.total_volume + tick.volume,
  high: max(tick.price, pre_state.high),
  low: min(tick.price, pre_state.low),
  timestamp_ns: tick.timestamp_ns,
}
```

**Properties:**
- **Purity**: No side effects, no I/O, no randomness.
- **Idempotence**: Replaying the same tick twice produces the same post-state twice.
- **Composability**: Chains of ticks compose: apply(apply(s0, t1), t2) == apply(s0, t2) ∘ apply(_, t1).

---

## 4. Receipt Generation and Verification

A **receipt** captures the full transition proof:

```
receipt = {
  pre_state: MarketPlanckCell,
  delta: bytes,
  post_state: MarketPlanckCell,
  causal_time: u64,
  hash: SHA256(pre || delta || post),
}
```

The receipt hash is the **identity proof** of the transition. Verification works as:

```
receipt_valid := replay_hash(pre, delta, post) == receipt.hash
```

No replay execution occurs—only hash comparison.

---

## 5. Receipt Chains: Lawful Histories

A **receipt chain** is an ordered sequence of receipts where:

1. **Contiguity**: receipt[i].post_state == receipt[i+1].pre_state
2. **Monotonicity**: receipt[i].causal_time < receipt[i+1].causal_time

A valid chain proves that a lawful computation history exists from the root state to the tip state.

```
Chain = [R0, R1, R2, ...]
where R0.pre = initial_state
and R_n.post = final_state
```

---

## 6. Event Horizon: The Boundary Beyond Recovery

An **event horizon** is a computation state from which no lawful recovery is possible. In market physics:

```
event_horizon_crossed := liquidity_index < critical_threshold
```

When crossed, the market enters a state of **irreversible collapse**. No delta can recover the system to a healthy state.

### Properties of the Event Horizon

- **Deterministic detection**: Threshold is fixed at compile time.
- **Irreversible**: Crossing the horizon is a one-way door.
- **Proven by receipt**: A boundary proof documents that the crossing occurred.

---

## 7. The Planck Volume: Constructive Arity

The **Planck volume** is the smallest unit of observable market impact:

```
planck_volume = 1 unit  // Smallest indivisible trade size
```

All volumesobserved are multiples of the Planck volume. This avoids fractional-share paradoxes and ensures deterministic aggregation.

---

## 8. Construct8Delta: Maximum Arity of 8

A Construct8Delta contains at most **8 distinct observables**:

1. event_type (fixed: "tick")
2. price
3. volume
4. timestamp
5-8. (reserved for future use)

Deltas exceeding 8 fields are **refused** by admission. This is a **hard law**, not a guideline—enforced by the type system in wasm4pm-compat.

---

## 9. Collision Detection: Hidden Bodies Manifest

As pressure (congestion) increases, **hidden bodies** (latency sources) become observable:

```
if pressure_level > threshold:
  manifest(hidden_bodies)  // Make invisible delays visible
```

Examples of hidden bodies:
- Network round-trip delay
- GC pause
- Lock contention on the execution queue
- Disk I/O stall

When manifested, they become **collision proofs**—evidence that the topology changed under stress.

---

## 10. From MarketPlanck to Wasm4pm Graduation

The MarketPlanck cell is **structure-only**. Execution logic (order matching, risk management, discovery) graduates to **wasm4pm**.

The boundary contract is:

```
wasm4pm_receipt = graduate(
  cell: MarketPlanckCell,
  reason: GraduationReason,
  evidence_chain: ReceiptChain,
)
```

A valid graduation requires:
- The cell is well-shaped (all invariants hold).
- The evidence chain is contiguous and monotonic.
- The reason is a valid law-level justification (e.g., `NeedsOrderMatching`, `RequiresRiskCheck`).

---

## References

- **Van der Aalst, W. M. P.** (2016). *Process Mining: Data Science in Action*. Springer.
- **Construct8 Specification** (2026). Type laws and phase semantics for deterministic computation.
- **WASM4PM Receipt Algebra** (2026). Receipt chain composition and verification.
