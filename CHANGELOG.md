# Changelog — wasm4pm-compat

All notable changes are recorded here. This project uses milestone tags rather than
semver ranges while in the `0.1.x` series.

---

## v26.6.26 — nightly-2026-06-22 toolchain upgrade (current)

**Status:** ALIVE (CROWN target)
**Date:** 2026-06-24

### Summary

Toolchain forward-pinned from nightly-2026-05-04 to nightly-2026-06-22 to align
with the ggen workspace. Fixed a `const_trait_impl` regression that broke all
`witness_marker!` invocations on the newer compiler.

### Root Cause

Between nightly-2026-05-04 and nightly-2026-06-22, the `const_trait_impl`
feature changed its name-resolution path for qualified `impl const module::Trait`
syntax, producing "expected a trait, found type" on every `witness_marker!`
expansion (~550 invocations). Since `Witness` only carries associated *constants*
(not `const fn` methods), `const trait` was never necessary — associated constants
are implicitly const. Reverted `pub const trait Witness` → `pub trait Witness` and
`impl const $crate::witness::Witness` → `impl $crate::witness::Witness` in the
macro. Zero behavioural change; the fix is purely syntactic.

### Changes

- `rust-toolchain.toml`: pin updated to `nightly-2026-06-22`.
- `src/witness.rs`: `pub const trait Witness` → `pub trait Witness`; macro arm
  `impl const $crate::witness::Witness` → `impl $crate::witness::Witness`.

### Verification

- All 200+ unit/integration tests pass on nightly-2026-06-22.
- ALIVE gate (compile-fail + compile-pass trybuild fixtures) green.
- clippy (deny warnings, all features) clean.
- `cargo fmt --check` clean.

---

## v26.6.23 — Workspace unified versioning & documentation alignment

**Status:** ALIVE (CROWN target)
**Date:** 2026-06-23

### Summary

A massive, exhaustive audit of 355 process and workflow papers resulted in the implementation of extensive structural type-safety constraints using combinatorial maximalism. The entire codebase was remediated to enforce paper-derived definitions at compile-time via zero-cost Rust abstractions and generic const evaluation.

### Major Additions

- **Petri Nets & Causal Nets**: Structural constraints for `CausalNet`, Stochastic Petri Nets (enforcing probabilities natively via generic consts `NUM <= DEN`), Free-Choice markers, and Petri Net unfoldings.
- **POWL & Process Trees**: Advanced workflow patterns extension, `SeparableWfNet`/`WfNet2Powl` witnesses (enforcing Kourani 2026 Theorem 4.3), and bounded process tree loop arity checks.
- **Multi-Perspective & XES Interop**: Zero-cost perspective markers (control-flow, data, resource, time) where weight sums are statically checked to not exceed 1.0. Introduced flat XES (IEEE 1849) bounded structural formats and their projection logic to OCEL.
- **OCPQ, Temporal & Prediction**: Object-Centric Process Query (OCPQ) predicates, predictive targets (horizons, drifts, targets), and strict temporal profiles (Adriansyah 2015) restricting unit mixing and order shapes.
- **UI Test Fortification**: Hundreds of trybuild `.stderr` expectations mapped, ensuring that invalid workflow manipulations fail to compile (`compile-fail`).
- **Workspace Synchronization**: Unified all workspace crates (`wasm4pm-compat`, `wasm4pm-compat-ts`, `c8-*`) and configurations to version 26.6.23. All `ggen` manifests synced. No Python scripts were introduced in this audit.

---

## v26.6.13 — 271-paper corpus, durability hardening & authority enforcement

**Status:** ALIVE (CROWN target)
**Date:** 2026-06-13

### Summary

Ships the 271-paper witness corpus and five nightly zero-cost innovations, then
hardens the crate against nightly churn and closes a real authority-enforcement
gap. The ALIVE gate is green (217 compile-fail + 408 compile-pass receipts);
MIRI reports no UB across the dependency graph.

### Witnesses & corpus integrity

- 271-paper witness corpus across seven per-category modules (rendered via ggen).
- **Self-validating bibliography**: new `witness_corpus` module renders
  `ALL_WITNESS_KEYS` (436 keys) with a compile-time `const` proof that no two
  witnesses share a `KEY` — closing a hole that SPARQL dedup alone could not
  enforce in Rust source. A runtime companion test names any collision.

### Nightly zero-cost innovations

- `WitnessFamily` derives `ConstParamTy`; `FamilyGated<const F: WitnessFamily>`.
- `pub const trait Witness` with `impl const` across all witnesses.
- `witness_law` module: sealed family-authority traits, `CoCitedKey` string law,
  `families_match_simd` (portable_simd), `gcd`/`NormedBetween01`.

### Teaching diagnostics

- `#[diagnostic::on_unimplemented]` on the five family-authority traits: a
  wrong-family witness now reads `` `PowlPaper` is not a Standard-family
  authority `` instead of a bare bound error. (Finding: the arithmetic const-laws
  fail as `E0308` const-unification, where custom messages are unreachable.)

### Authority enforcement (not just labeling)

- `LinkedOcel`: the first concrete `Admit` impl in `src/`. The crate now not only
  *names* `DanglingEventObjectLink` / `EmptyEventObjectLinks` but ships a function
  that *detects* them and refuses through the typed `Raw → Admitted` one-way door.

### Durability

- Pinned `rust-toolchain.toml` to `nightly-2026-05-04` (+ components) so a
  const-generics / const-trait syntax flip lands on one known toolchain.
- **Finding (documented in-code):** the `generic_const_exprs → min_generic_const_args`
  migration is *not viable* — mGCA forbids generic params in computed const
  operations, so the computed-const law kernel (`Between01`, `Metric`,
  `ConditionCell`) has no stable-floor path; the two features are mutually
  exclusive in one crate. Watch mGCA's non-min expansion.

### Hygiene & structure

- TypeScript bindings extracted to the `wasm4pm-compat-ts` sidecar crate,
  restoring the *exactly three public features* and *no runtime dependencies*
  invariants (`ts`/`specta` removed from the core crate).
- 10 rough examples moved to `examples/illustrative/`; backups, `node_modules`,
  and TS artifacts excluded from the publish tarball and gitignored.
- `wasm4pm-compat-lsp` dropped from workspace members (separate tool/cadence).
- MIRI verification bridge wired (`cargo make miri`); restored the
  `SeparableWfNet` non-forgeability seal lost in a prior refactor.

### Known follow-up

- Literal-100% item-level rustdoc is in progress: module-level docs are complete;
  ~690 public item docs (compiler-measured via `missing_docs`, largely serde
  struct fields in `ocel`/`models`/`petri`) remain a tracked effort.

### ALIVE gate

```
cargo test --test ui_tests -- --ignored
```

## v26.6.8–v26.6.11 — OCEL 2.0 canon, builder ergonomics & ghost-refusal enforcement

**Status:** ALIVE
**Span:** the patch series leading into v26.6.13.

### Summary

The work between the type-law foundry and the v26.6.13 corpus release: it landed the
OCEL 2.0 / Petri / conformance canon, hardened the refusal surface against unreachable
("ghost") variants, added builder ergonomics, and modernized the surrounding tooling.

### Canon (v26.6.8 → v26.6.9)

- OCEL 2.0 object-centric types, Petri-net const-generic soundness typestate, and van der
  Aalst conformance verdict shapes (v26.6.8).
- The W4PM-001 through W4PM-010 patch set; version bump to 26.6.9.

### Builder & DX ergonomics (v26.6.11)

- `PowlBuilder` — an ergonomic arena builder for POWL models with a fluent surface
  (`atom`, `silent`, `partial_order`, `choice`, `loop_node`, `choice_graph`) and a checked
  `build()` (plus `build_unchecked()`) returning a named `PowlRefusal` on malformed input.
- `Evidence::inner()` — borrow the payload without consuming the typestate carrier.
- `Evidence::from_boundary()` — a `const` alias for `Evidence::raw()` that signals intent at
  call sites where the value originates at a process boundary.

### Refusal law — ghost-variant enforcement

- Enforced previously-unreachable refusal variants so every named law is actually
  constructible and tested: `DeclareRefusal` (`declare`), `CausalNetRefusal` (`causal_net`),
  `InvalidChoiceArity { declared, required_min }` (`powl`), and `CycleDetected`
  (`process_tree`). A refusal type that names a law but can never be produced is itself a
  defect; these close that gap.

### Receipts

- CROWN provenance fields on the validation receipt: `run_id`, `output_hash`,
  `replay_pointer` (the `ReceiptEnvelope` carries `output_hash`).

### Tooling

- `wasm4pm-compat-lsp` migrated from `tower-lsp` to `lsp-max`.
- ggen substrate: breed-scaffold pack (ontology + queries + templates), 100 verified paper
  pointers, and hardened anti-cheat / meta-gate templates (raw-string needles, numeric-only
  hardcode lock, structural fixture-provenance gate). Signing keys rotated; `.ggen/keys/`
  untracked and gitignored.

## PAPERLAW_ALIVE_003 — Nightly-first type-law foundry

**Status:** ALIVE (CROWN target)

### Summary

This milestone consolidates the nightly-first type-law foundry and establishes the
ALIVE gate as the definitive certification surface.

### What shipped

- **`nightly_foundry.rs`** — always-on staging area for paper-derived law surfaces.
  Contains four surfaces: `petri_law`, `powl_law`, `evidence_law`, and `token_law`,
  using `generic_const_exprs`, `adt_const_params`, `min_specialization`, and
  `portable_simd` respectively.

- **`law.rs`** — compile-time law kernel: `Assert`/`IsTrue`/`Require` bounds
  machinery, `ConditionCell<BITS>` (Need9-means-split law), `Between01<NUM, DEN>`
  (provably-in-[0,1] metric bounds), `ConstParamTy` enum set, `EvidenceMode`.

- **`petri.rs`** — typed bipartite arc types, `WfNetConst<SOUNDNESS>` with
  non-forgeable witness path.

- **`conformance.rs`** — `Metric<KIND, NUM, DEN>` with `Between01` bounds.

- **`process_tree.rs`** — `TypedLoopNode<ARITY>` with `Require<{ ARITY == 2 }>: IsTrue`.

- **`powl.rs`** — `TreeProjectable` sealed trait, `assert_tree_projectable`.

- **`formats.rs`** — `LossyFormatExport` requiring a non-optional loss report.

- **`strict.rs`** — `ExportBoundaryConst<HAS_WITNESS, HAS_ROUND_TRIP>` const-generic
  type; `ProcessBoundary::check()` returning named `StrictViolation`s.

- **Trybuild fixture expansion** — compile-fail and compile-pass fixtures covering
  admission, refusal, loss, receipt, Petri nets, POWL, process trees, Declare, XES,
  OCEL, BPMN, conformance, and strict-mode surfaces.

- **Paper coverage** — type surfaces traceable to 30+ process-mining papers.

### ALIVE gate

```bash
cargo test --test ui_tests -- --ignored
```

---

## PAPERLAW_ALIVE_002 — Paper corpus expansion + trybuild receipts

### Summary

This milestone expanded the paper coverage ledger and established the trybuild
fixture system as the primary type-law receipt mechanism.

### What shipped

- **Witness marker expansion** — added `YawlPaper`, `SeparableWfNetPaper`,
  `WorkflowPatternsPaper`, `InductiveMiner`, `DeclareConstraints`, `AlignmentPaper`,
  `OcPetriNets`, `LogSkeleton`, `AlphaMiner`, `XesLifecycleExt`, `XesConceptExt`,
  `OcelObjectType`, `OcelEventType`, `OcelAttributeType`, `WfNet2Powl`,
  `DivergenceWitness`, `ConvergenceWitness`.

- **Trybuild scaffolding** — `tests/ui_tests.rs` with compile-fail and compile-pass
  fixture directories. Initial fixtures for admission, refusal, and id-type mixing.

- **`receipt.rs` expansion** — `ReceiptChain`, `ReceiptChainConst<N>`,
  `GraduationReceipt`, `ReceiptVerdict`, `WellShaped` trait.

- **`loss.rs` expansion** — `NamedLossConst<NAME>`, `LossChain`,
  `ProjectionBoundary<NAME>`.

- **`ids.rs`** — `#[repr(transparent)]` identifier wrappers: `CaseId`, `ActivityId`,
  `TraceId`, `ObjectId`, `EventId`.

- **Paper coverage ledger** — `PAPER_COVERAGE_LEDGER.md` tracking 50+ papers.

---

## PAPERLAW_ALIVE_001 — Nightly center-of-gravity shift

### Summary

This milestone shifted the crate's center of gravity from stable-compatible shapes
to nightly-first type law. The unconditional nightly features were declared at the
crate root, and the first type-law surfaces were introduced.

### What shipped

- **Unconditional nightly features** declared at crate root:
  `generic_const_exprs`, `adt_const_params`, `const_trait_impl`,
  `min_specialization`, `portable_simd`, `allow(incomplete_features)`.

- **`#![forbid(unsafe_code)]`** declared unconditionally.

- **`law.rs` kernel** — initial `Assert`/`IsTrue`, `ConditionCell<BITS>`.

- **`state.rs`** — seven typestate tokens with sealed `EvidenceState` trait and
  `Projectible` sealed trait.

- **`evidence.rs`** — `Evidence<T, State, W>` universal carrier with all lifecycle
  transitions.

- **`witness.rs`** — `Witness` trait, `WitnessFamily` enum, initial witness markers:
  `Ocel20`, `Xes1849`, `Pm4pyApiGrammar`, `PmaxConsumerGrammar`, `PowlPaper`,
  `ObjectCentricPetriNetPaper`, `WfNetSoundnessPaper`, `OcpqPaper`, `DeclareFamily`,
  `PredictiveMonitoringFamily`, `ReceiptFamily`, `RustTypestateLaw`, `Wasm4pmBridge`.

- **`admission.rs`** — `Admission<T, W>`, `Refusal<R, W>`, `Admit` trait.

- **`loss.rs`** — `LossPolicy`, `ProjectionName`, `NamedLoss`, `LossReport<From, To,
  Items>`, `Project` trait, `IsEmpty` helper trait.

- **`receipt.rs`** — `Digest`, `ReplayHint`, `ReceiptShape`, `ReceiptEnvelope`,
  `ReceiptRefusal`.

- **Feature model** — `formats` (default), `strict`, `wasm4pm`. Three and only three.

---

## Initial — wasm4pm-compat inception

### Summary

The first commit established the crate with its core identity: a structure-only,
paper-complete compatibility surface for process mining.

### What shipped

- Crate skeleton: `Cargo.toml`, `rust-toolchain.toml`, `src/lib.rs`, `src/prelude.rs`.

- Core canon modules: `eventlog`, `ocel`, `xes`, `bpmn`, `petri`, `powl`,
  `process_tree`, `declare`, `ocpq`, `dfg`, `conformance`, `prediction`, `receipt`,
  `ids`, `evidence`, `admission`, `loss`, `diagnostic`, `witness`, `state`, `interop`.

- `examples/basic_eventlog.rs`, `examples/basic_ocel.rs`.

- License: `MIT OR Apache-2.0`.

- Crate description: "Minimal paper-complete, feature-capped Rust process-evidence
  crate. Start with compatibility. Graduate to execution."
