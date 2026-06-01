# Nightly-Only Covenant

> This crate is nightly-only. No stable downgrade mission exists.

---

## The Covenant

`wasm4pm-compat` is a **nightly-only** crate. The `rust-toolchain.toml` pins the
toolchain to `nightly` unconditionally. There is no stable build target, no MSRV
policy, no stable fallback, and no path toward stabilization of this crate's
feature set.

Applications that depend on this crate must conform **upward** to its type law.
The crate does not conform downward to stable Rust. The nightly toolchain is not
an implementation detail — it is the type-law substrate.

---

## Unconditional Nightly Features

These six features are declared at the crate root without any cfg gate. They are
always active. Every public API surface is built on them:

| Feature | Purpose |
|---|---|
| `generic_const_exprs` | Const-generic arithmetic expressions in type bounds (`Between01<NUM, DEN>`, `Assert<{ BITS < 64 }>`) |
| `adt_const_params` | Enum values as const generic parameters (`ConstParamTy` law enums, `WorkflowPatternKind`) |
| `const_trait_impl` | Trait implementations in const contexts (`const_eval_select` paths) |
| `min_specialization` | Sealed specialization for zero-cost witness dispatch |
| `portable_simd` | SIMD-width-typed token replay surfaces (structurally present; engines graduate to `wasm4pm`) |
| `incomplete_features` (allow) | Suppresses stability warnings for the above; not a feature gate but a lint allowance |

Source: `src/lib.rs` lines 67–72.

---

## ALIVE Sealed at PAPERLAW_ALIVE_001

The ALIVE certification gate (`cargo test --test ui_tests`) was sealed at
`PAPERLAW_ALIVE_001`. Every compile-fail and compile-pass fixture present at that
seal date is a permanent receipt. Regressions against sealed fixtures are defects,
not engineering trade-offs.

Subsequent ALIVE milestones (`PAPERLAW_ALIVE_002`, `PAPERLAW_ALIVE_003`, …) extend
the gate — they never relax it. The covenant is cumulative and one-directional.

---

## Applications Conform Upward

The crate's type law is the ground truth. If an application cannot compile against
this crate on the pinned nightly toolchain, the application must be updated. The
correct failure mode is a compilation error, not a feature flag that silences the
law.

> Structure-only. Nightly-always. No retreat.


## Nightly regression protocol

When the pinned nightly introduces a breaking change:

1. Update `rust-toolchain.toml` to the last known-good nightly
2. Document the regression in `CHANGELOG.md` with the bad nightly hash
3. Open a tracking issue referencing the upstream rustc bug
4. Mark affected source with `// NIGHTLY-REGRESSION: <hash> <upstream-issue>`

The covenant: we never weaken the type law to accommodate a compiler bug.
