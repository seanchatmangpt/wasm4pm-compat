# Stability Policy

## What is stable

Public types, traits, and modules in `src/` with documented law surfaces are considered
stable within the nightly toolchain. Breaking changes are recorded in `CHANGELOG.md`.

## What is not stable

- `src/nightly_foundry.rs` — staging module, may be reorganized
- `#[doc(hidden)]` items — internal implementation details
- `.stderr` files — compiler error text may shift across nightly builds
- The exact error message produced by a `compile_fail` fixture — only the error *kind* (E0308, E0599, etc.) is considered stable

## Nightly breakage policy

When a new nightly build introduces a regression (e.g. E0391 cycles in `generic_const_exprs`):

1. Document the regression in `CHANGELOG.md` with the rustc commit hash
2. Pin the known-good nightly in `rust-toolchain.toml` until fixed
3. Add a `// NIGHTLY-REGRESSION: <issue>` comment to affected code
4. File a rustc issue if one does not exist

The project does **not** paper over compiler bugs with workarounds that weaken the type law.
The type law is the product — we wait for the compiler, not the reverse.

## Const-generics durability (the `generic_const_exprs` bet)

The computed-const law kernel — `Between01<NUM, DEN>`, `Metric<KIND, NUM, DEN>`,
`ConditionCell<BITS>`, `NormedBetween01`, and every `Require<{ EXPR }>: IsTrue` bound —
depends on `generic_const_exprs` (GCE), which the compiler team now describes as superseded
by `min_generic_const_args` (mGCA).

**This crate cannot migrate off GCE today.** A live spike (nightly 2026-05-04) established
two hard facts:

1. mGCA and GCE are **mutually exclusive in one crate root** — enabling
   `min_generic_const_args` rejects every existing `Require<{ EXPR }>: IsTrue` bound with
   *"complex const arguments must be placed inside of a `const` block"*.
2. mGCA **forbids generic parameters in computed const operations** — `const { N <= D }`
   over generic `N`, `D` fails with *"generic parameters may not be used in const
   operations"*. The crate's arithmetic laws are exactly such computations.

Therefore the durability posture is: **stay on GCE, pin a dated nightly, and wait for
mGCA's non-min expansion** (which is intended to support computed const arguments). The pin
in `rust-toolchain.toml` ensures a const-generics or const-trait syntax flip lands on one
known toolchain rather than silently. This finding is recorded in-code on `Witness::FAMILY`.

The `Witness` trait's `FAMILY` associated const would ideally become a `type const` (mGCA)
so family gating could be an associated-const-equality bound; the above blocks that, so
family gating stays sealed-trait-based in `witness_law`.

## Feature stability

Exactly three public features: `formats`, `strict`, `wasm4pm`. Adding features is a breaking
change. Removing features is a breaking change. This is a hard invariant, not a guideline.

> **v26.6.13:** a brief fourth feature (`ts`, gating an optional `specta` runtime dependency)
> was removed and the TypeScript/Zod surface extracted to the `wasm4pm-compat-ts` sidecar
> crate, restoring both this invariant and the *no runtime dependencies* guarantee.
