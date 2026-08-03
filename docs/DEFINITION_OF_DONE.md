# Definition of Done

A change to `wasm4pm-compat` is done only when every applicable gate below
holds. These gates preserve the structure-only boundary and make failures
operator-visible.

## Structural gates

- [ ] Every canon shape is a small, strongly named type.
- [ ] Witness/state typing uses `PhantomData` markers; IDs remain zero-cost
      transparent wrappers.
- [ ] No engine logic exists here: no discovery, conformance execution, replay,
      alignment, optimization, or visualization.

## Canon gates

- [ ] The base profile (`--no-default-features`) still knows every shape.
- [ ] Features control capability stages, not canon knowledge.

## Boundary gates

- [ ] No raw format-to-format laundering: `external -> admitted compat ->
      external | wasm4pm` only.
- [ ] Every serious surface refuses with a specific named law.
- [ ] Lossy projection carries a `ProjectionName`, `LossPolicy`, `LossReport`,
      and refusal path.

## Evidence gates

- [ ] Receipt-shaped evidence carries provenance fields without claiming
      signature or execution authority.
- [ ] Admission and refusal are first-class values, not panics.
- [ ] Deterministic identities are reproducible over canonical input.

## Feature gates

- [ ] Exactly three public Cargo features exist: `formats`, `strict`, `wasm4pm`.
- [ ] `default = ["formats"]`.
- [ ] No per-format flags.
- [ ] Nightly remains unconditional and pinned.
- [ ] `#![forbid(unsafe_code)]` holds.

## Documentation gates

- [ ] Every public module has module documentation.
- [ ] Public types state what they represent, what they do not do, and when to
      graduate.
- [ ] Operator commands document status semantics and exit codes.

## DX and doctor gates

- [ ] The prelude re-exports the bounded doctor entry point.
- [ ] `doctor core` works without optional feature assumptions.
- [ ] `doctor vision2030 --json` reports the current feature closure.
- [ ] Every disabled feature produces one minimal reversible repair.
- [ ] Discovery, conformance, replay, and optimization never route to compat.
- [ ] Exact-tree standing always routes to the external verifier.
- [ ] Doctor reports and route plans have deterministic canonical JSON and
      BLAKE3 fingerprints.
- [ ] The doctor type cannot represent self-issued `ALIVE` standing.
- [ ] `cargo doc --all-features --no-deps` builds clean.

## Release gates

- [ ] `cargo fmt --all -- --check` is clean.
- [ ] `cargo clippy --locked --all-features --tests -- -D warnings` is clean.
- [ ] `cargo test --locked --all-features --tests` passes.
- [ ] Compile-time law fixtures pass for the intended reason.
- [ ] The external exact-tree standing receipt is produced by the verifier, not
      inferred from inspection or a doctor report.
