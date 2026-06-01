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

## Feature stability

Exactly three public features: `formats`, `strict`, `wasm4pm`. Adding features is a breaking
change. Removing features is a breaking change. This is a hard invariant, not a guideline.
