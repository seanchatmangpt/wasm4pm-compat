# Performance Guarantees

`wasm4pm-compat` is zero-cost at runtime. This document explains what that means
and how it is verified.

## Zero-cost abstractions

Every type in this crate that carries phantom type parameters is zero-sized at runtime:

| Type | Runtime size |
|---|---|
| `Evidence<T, State, W>` | `size_of::<T>()` |
| `EventId(u64)` | `size_of::<u64>()` = 8 bytes |
| `WfNetConst<S>` | 0 bytes (unit struct) |
| `LossPolicy` | 1 byte (enum discriminant) |
| Witness markers (e.g. `Ocel20`) | 0 bytes (uninhabited) |
| State tokens (e.g. `Raw`, `Admitted`) | 0 bytes (uninhabited) |

## Verification

The `benches/` directory contains criterion benchmarks that measure:

- `bench/zero_cost_types` — ID construction overhead vs raw integer
- `bench/law_bounds_bench` — const-generic law generates no runtime branching
- `bench/evidence_lifecycle_bench` — state transitions compile away completely
- `bench/id_operations_bench` — newtype `#[repr(transparent)]` zero overhead

Run with: `cargo bench --all-features`

## Compile-time cost

Type-law enforcement happens at compile time, not runtime. The price is paid once
during `cargo build`, not on every execution. Nightly features (`generic_const_exprs`,
`adt_const_params`) may increase compile times; this is expected and acceptable.
