# Definition of Done

A change to `wasm4pm-compat` is **done** only when every gate below holds. These
gates are derived from the master specification.

## Structural gates

- [ ] Every canon shape is a small, strongly-named, transparent type
      (events, traces, logs, OCEL, XES, BPMN, Petri, WF-net, OC-Petri-net,
      POWL, process tree, Declare, OC-Declare, OCPQ, DFG, conformance verdict,
      prediction problem, receipt-shaped evidence).
- [ ] Witness/state typing uses `PhantomData` markers; IDs are zero-cost
      `#[repr(transparent)]` wrappers.
- [ ] No engine logic anywhere: no discovery, conformance checking, replay,
      alignment, optimization, or visualization.

## Canon gates

- [ ] The base profile (`--no-default-features`) still knows every shape.
- [ ] Features control capability stages, not canon knowledge.

## Boundary gates

- [ ] No raw format-to-format laundering: `external -> admitted compat ->
      external | wasm4pm` only.
- [ ] Every serious surface refuses with a **specific named law**, never a bare
      `InvalidInput`.
- [ ] Lossy projection carries a named `ProjectionName`, a `LossPolicy`, a
      `LossReport`, and a refusal path.

## Evidence gates

- [ ] Receipt-shaped evidence is structure only and carries provenance.
- [ ] Admission and refusal are first-class values, not panics.

## Feature gates

- [ ] Exactly three public Cargo features exist: `formats`, `strict`, `wasm4pm`.
- [ ] `default = ["formats"]`.
- [ ] No per-format flags.
- [ ] Nightly is **not** a feature; the crate requires nightly unconditionally
      (rust-toolchain.toml pins nightly; `nightly_foundry.rs` has no cfg gate).
- [ ] `#![forbid(unsafe_code)]` holds.

## Docs gates

- [ ] Every public module has `//!` module docs.
- [ ] Every public `fn` has a doctest (or a documented `ignore` reason).
- [ ] Every public type has rustdoc stating what it represents, what it does
      **not** do, that it is structure-only, and when it should graduate.

## DX gates

- [ ] The `prelude` re-exports the core adoption surface.
- [ ] `cargo doc --all-features --no-deps` builds clean.

## Release gates

- [ ] Full verification matrix passes (see README "Verification commands").
- [ ] `cargo clippy --all-features -- -D warnings` is clean.
- [ ] `cargo fmt --check` is clean.
- [ ] Nightly toolchain documented (rust-toolchain.toml); no MSRV (nightly-only).
