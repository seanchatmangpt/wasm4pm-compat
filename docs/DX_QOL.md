# Developer Experience & Quality of Life

`wasm4pm-compat` should feel **obvious to adopt and impossible to misuse**.

## Adoption surface

- `use wasm4pm_compat::prelude::*;` brings in everything needed to construct the
  core event-log shapes and reason about boundaries. No deep path archaeology.
- Types are **strongly named** so that an autocomplete list reads like the canon:
  `Event`, `Trace`, `EventLog`, `OcelLog`, `Admission`, `Refusal`, `LossReport`.

## Misuse resistance

- **Typestate** (`Raw` → `Parsed` → `Admitted` → `Exportable`/`Receipted`, with
  `Refused`/`Projected` branches) makes illegal transitions unrepresentable.
- **Named refusal laws** mean failures are self-explaining; an error tells you
  *which* structural obligation broke, not just that "something was invalid."
- **Witness markers** carry proof at the type level, so admitted evidence cannot
  be confused with un-admitted input.

## Zero-cost ergonomics

- `#[repr(transparent)]` ID wrappers add type safety with no runtime cost.
- `PhantomData` witness/state markers compile away entirely.
- No dependencies on stable: fast to build, trivial to audit.

## Discoverability

- Module docs (`//!`) explain *what each module is for* and *what it is not*.
- Every public type's rustdoc says when to **graduate** to `wasm4pm`, so the
  upgrade path is always one hop away in the docs.

## Quality guardrails

- `#![forbid(unsafe_code)]`.
- Clippy clean at `-D warnings`, rustfmt enforced.
- A full feature/cfg verification matrix (see README) protects every profile,
  including the canon-only `--no-default-features` build and the nightly foundry
  cfg.
