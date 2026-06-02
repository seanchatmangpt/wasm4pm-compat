# c8-time: Vector Clock and Monotonic Timestamp Primitives

Causal consistency primitives for market event ordering.

## Core Types

- **`VectorClock8`**: An 8-lane vector clock tracking logical causality across 8 independent axes.
- **`VectorClockCompare`**: Comparison result (Before, After, Concurrent, Equal).
- **`MonotonicStamp`**: A nanosecond-precision timestamp that never decreases.

## Key Operations

### VectorClock8
- `zero()` — Create a clock with all lanes at zero.
- `lanes()` — Inspect current clock state.
- `tick_lane(i)` — Increment lane `i`.
- `merge(other)` — Take component-wise maximum with another clock.
- `compare(other)` → `VectorClockCompare` — Compare causality.

### MonotonicStamp
- `from_nanos(u64)` — Create from nanosecond value.
- `nanos()` — Get nanosecond value.
- `assert_not_before(prev)` — Panic if this stamp regresses.

## Philosophy

Vector clocks and monotonic timestamps are the dual foundation of market causality:
- **Vector clocks** capture *logical* ordering: which events can have caused which.
- **Monotonic timestamps** enforce *physical* ordering: time never flows backward.

Together they ensure that market Planck cells can be ordered correctly even under concurrent processing.
