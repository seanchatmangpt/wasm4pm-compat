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

## v26.10.12 status (2026-10-12)

Per-gate status from the lane-D2 cheap-gate run, updated by lane E15
(2026-10-12). Only commands actually executed this session are marked
VERIFIED; the rest are UNVERIFIED. All four compile/test gates were
re-executed by lane E15 on the same working tree and all exit 0.

| Gate | Command / evidence | Status |
|---|---|---|
| `cargo fmt --all -- --check` clean | `cargo make fmt` (`cargo fmt --all --check`) | VERIFIED-PASS (exit 0; Build Done in 0.61s — earlier lane-D2 VERIFIED-FAIL at `src/ocel.rs:1595` was a race with D6's fix, since landed) |
| `cargo clippy --locked --all-features --tests -- -D warnings` clean | `cargo make clippy` | VERIFIED-PASS (exit 0; Build Done in 6.03s — confirms D5's landing) |
| `cargo test --locked --all-features --tests` passes | `cargo test --locked --all-features --tests` | VERIFIED-PASS (exit 0; 528 passed, 0 failed across 190 test binaries) |
| Compile-time law fixtures (`ui_tests`) | `cargo test --locked --test ui_tests -- --ignored` | VERIFIED-PASS (exit 0; 2 passed, 0 failed, 727.98s) |
| Type-check, all features | `cargo check --locked --all-features` | VERIFIED-PASS (exit 0, no output) |
| Base profile knows every shape (`--no-default-features`) | `cargo check --locked --no-default-features` | VERIFIED-PASS (exit 0, no output) |
| Exactly three public features; `default = ["formats"]`; no per-format flags | `Cargo.toml` `[features]` (lines 97–105): `default = ["formats"]`, `formats`/`strict`/`wasm4pm` only | VERIFIED-PASS (direct read) |
| Nightly unconditional and pinned | `rust-toolchain.toml`: `channel = "nightly-2026-06-22"` | VERIFIED-PASS (direct read) |
| `#![forbid(unsafe_code)]` | `src/lib.rs:155` | VERIFIED-PASS (direct read) |
| `cargo doc --all-features --no-deps` builds clean | not run | UNVERIFIED |
| Doctor gates (prelude entry point, `doctor core`, `doctor vision2030 --json`, minimal reversible repairs, routing, deterministic JSON/BLAKE3, no self-issued `ALIVE`) | not run | UNVERIFIED |
| Structural / canon-shape / boundary / evidence / documentation gates (inspection-class) | not audited this session | UNVERIFIED |
