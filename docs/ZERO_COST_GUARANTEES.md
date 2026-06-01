# Zero-Cost Guarantees

wasm4pm-compat makes a hard engineering claim: **the type law costs nothing at
runtime**. This document defines what "zero-cost" means for each type family,
what the Rust language guarantees back that claim, and where the empirical
benchmark evidence lives.

---

## Claim taxonomy

There are three distinct zero-cost claims in this crate:

| Claim | Mechanism | Where proven |
|---|---|---|
| PhantomData tags add no bytes | `PhantomData<T>` has `size_of == 0` | `benches/zero_cost_types.rs` |
| `#[repr(transparent)]` newtypes match their inner layout | Language layout guarantee | `benches/id_operations_bench.rs` |
| Const-generic bounds add no runtime branches | Monomorphization erases type params | `benches/law_bounds_bench.rs` |
| State-transition methods are pure moves | `#[inline]` struct-field rewrapping | `benches/evidence_lifecycle_bench.rs` |

---

## Evidence<T, State, W>

`Evidence<T, State, W>` (defined in `src/evidence.rs`) is the universal carrier.
It has three fields:

```rust
pub struct Evidence<T, State: EvidenceState, W> {
    pub value: T,
    pub state: PhantomData<State>,
    pub witness: PhantomData<W>,
}
```

`PhantomData<State>` and `PhantomData<W>` are zero-sized. The Rust compiler
guarantees that a struct whose only non-`T` fields are `PhantomData` has the
same size and alignment as `T`. This is verified by compile-time `const`
assertions in `benches/zero_cost_types.rs`:

```rust
const _: () = {
    assert!(size_of::<Evidence<u64, Raw, Ocel20>>() == size_of::<u64>());
    assert!(size_of::<Evidence<u32, Raw, Ocel20>>() == size_of::<u32>());
    assert!(size_of::<Evidence<u8,  Raw, Ocel20>>() == size_of::<u8>());
};
```

These are `const` blocks — if the layout assertion fails the crate does not
compile. They are not tests that can be skipped.

### State-transition methods

Every `into_*` method on `Evidence` is:

```rust
#[inline]
pub fn into_parsed(self) -> Evidence<T, Parsed, W> {
    Evidence { value: self.value, state: PhantomData, witness: PhantomData }
}
```

This is a field move plus construction of two zero-sized fields. With
optimizations enabled (`--release`) the compiler eliminates it entirely — the
move becomes an identity on the underlying `T` bits. The benchmark
`bench_admitted_to_receipted_via_projected` chains four such transitions:

```
Admission::new(u64) -> into_evidence() -> into_projected() -> into_receipted()
```

and measures latency indistinguishable from `bench_u64_baseline` (a bare
`black_box(black_box(42u64))`).

---

## Typed id newtypes (ids module)

Every typed id — `EventId<K>`, `ObjectId<K>`, `TraceId<K>`, `CaseId<K>`,
`ActivityId<K>`, `RelationId<K>`, `TraceId<K>`, `EventTypeId<K>`,
`ObjectTypeId<K>` — is declared as:

```rust
#[repr(transparent)]
pub struct EventId<K> {
    raw: u64,
    _kind: PhantomData<K>,
}
```

`#[repr(transparent)]` is a Rust language guarantee: the struct has the same
ABI as its single non-zero-sized field. A `*const EventId<Log>` and a
`*const u64` are interchangeable at the FFI level. The compile-time proof:

```rust
const _: () = {
    enum Log {}
    assert!(size_of::<EventId<Log>>()    == size_of::<u64>());
    assert!(size_of::<ObjectId<Log>>()   == size_of::<u64>());
    assert!(size_of::<TraceId<Log>>()    == size_of::<u64>());
    assert!(size_of::<ActivityId<Log>>() == size_of::<u32>());
};
```

`benches/id_operations_bench.rs` benchmarks construction, `.raw()` extraction,
`Display` formatting, and `From`/`Into` conversions against raw `u64`/`u32`
baselines to confirm that typed id operations produce no measurable overhead.

---

## Const-generic law bounds (law module)

`ConditionCell<BITS>` and `Between01<NUM, DEN>` (defined in `src/law.rs`) carry
no runtime state:

```rust
pub struct ConditionCell<const BITS: usize> where Require<{ BITS <= 8 }>: IsTrue {
    _private: (),
}
```

The `_private: ()` field is zero-sized. The `where` bound is evaluated by the
compiler at monomorphization time — if `BITS > 8` the crate does not compile.
At runtime there is no branch, no flag, no lookup.

`benches/law_bounds_bench.rs` constructs `ConditionCell<8>`, `ConditionCell<4>`,
and `ConditionCell<1>` and measures that all three produce identical (negligible)
latency. The same applies to `Between01<3,4>`, `Between01<1,2>`, and
`Between01<1,1>` — different const parameters, identical runtime behavior.

The same property holds for `Metric<KIND, NUM, DEN>` (and its type aliases
`FitnessConst`, `PrecisionConst`, `F1Const`, `GeneralizationConst`,
`SimplicityConst`): the `QualityMetricKind` const param is erased after
monomorphization and generates no runtime dispatch.

---

## ExportBoundaryConst<HAS_WITNESS, HAS_ROUND_TRIP>

`ExportBoundaryConst` (defined in `src/law.rs` under the `strict` feature) is
a zero-sized struct with two `bool` const params. The compile-time law
`enforce_export_round_trip<B: HasRoundTripFixture>` is a sealed-trait bound:
only `ExportBoundaryConst<true, true>` satisfies it. This is checked at
compile time; there is no runtime flag.

---

## Running the benchmarks

```bash
# All four benchmark suites.
cargo bench --all-features

# Individual suites.
cargo bench --bench zero_cost_types       --all-features
cargo bench --bench law_bounds_bench      --all-features
cargo bench --bench evidence_lifecycle_bench --all-features
cargo bench --bench id_operations_bench   --all-features
```

HTML reports land in `target/criterion/` when criterion is built with the
`html_reports` feature (enabled in `[dev-dependencies]`).

---

## Interpreting the results

A zero-cost type should produce benchmark latency that is:

1. **Statistically indistinguishable** from the corresponding raw-primitive
   baseline (`u64`/`u32` identity), or
2. **Bounded by formatting cost** for `Display`-related benchmarks where string
   allocation is the dominant term regardless of whether the id is typed or raw.

Criterion reports the mean, standard deviation, and confidence interval for each
function. If a typed-id bench shows latency consistently higher than the raw
baseline at `p < 0.05`, that is a regression — file it as a defect, not a
discrepancy (Chicago TDD doctrine).
