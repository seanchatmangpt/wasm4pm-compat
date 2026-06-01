# Changelog — wasm4pm-compat

All notable changes are recorded here. This project uses milestone tags rather than
semver ranges while in the `0.1.x` series.

---

## PAPERLAW_ALIVE_003 — Nightly-first type-law foundry (current)

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
