# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## ggen provision (CRITICAL — read before touching src/witnesses.rs)

**`src/witnesses.rs` is ggen-rendered source.** Do not hand-edit it. To change a witness, edit the TTL and run `ggen sync`.

**ggen-lsp is wired.** When you open `.ttl`, `.rq`, `.tera`, or `ggen.toml`, diagnostics push automatically as `<new-diagnostics>`. `GGEN-TPL-001` means a template/query variable mismatch — the provision chain is broken. **Stop and resolve before running `ggen sync`.**

**`src/witnesses.rs` is first-class source.** It is provided by ggen, but it is source.
- If a defect is a local source shape issue: repair it as source.
- If a defect belongs to the pack law (TTL/query/template/manifest): repair the seed and re-run ggen.
- If a direct source patch breaks replay: back-propagate into the seed or ledger the divergence.
- Never refuse to inspect or reason about the file because ggen provided it.

**Run only the witness-markers rule** (the only first-class source rule — others write to `audits/`, `tests/`, `scripts/`):

```bash
cargo make ggen-witnesses       # render src/witnesses.rs
cargo make ggen-witnesses-dry   # dry run — preview without writing
```

**Known issue (fixed 2026-06-03):** `inference enabled = false` in `ggen/ggen.toml` — the alive-gate inline SPARQL was broken and has been replaced with a file reference; inference remains disabled as it is not required for the witness-markers rule.

**Four templates currently have `GGEN-TPL-001` diagnostics** (template/query variable mismatches — do not run those rules until resolved):
- `compile-fail-fixture.tera` — expects `processFormType`, `rustType`, `witnessA`, `witnessB`
- `compile-pass-fixture.tera` — expects `description`, `lawName`, `processFormType`, `witnessType`
- `audit-script.tera` — expects `index`, `moduleName`
- `open-ontologies-integration.tera` — expects `index`

## Nightly Rust is required

This crate is **nightly-only**. The `rust-toolchain.toml` pins the toolchain to `nightly`. There is no stable build target, no MSRV, and no stable fallback. Applications must conform upward to this crate's type law, not the other way around.

The crate root declares these nightly features unconditionally:

```rust
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(min_specialization)]
#![feature(portable_simd)]
#![allow(incomplete_features)]
```

## Testing surfaces

Three distinct surfaces; each has a different purpose and cadence.

| Surface | Purpose | Run how |
|---|---|---|
| Unit + integration tests | Fast behavior checks | `cargo test --all-features --tests` — sub-second |
| Trybuild fixtures (`tests/ui/`) | Type-law receipts (ALIVE gate) | Explicit: `cargo test --test ui_tests -- --ignored` |
| Doctests | Documentation audit | Explicit: `cargo test --doc --all-features` |

**Rule:** Doctests teach usage. Trybuild proves law.

Doctests are **disabled in the default test run** (`doctest = false` in `Cargo.toml`). The reason: this is a nightly-first crate where every doctest that touches `generic_const_exprs` or `adt_const_params` types becomes a separate nightly `rustc` invocation. 200+ such invocations make `cargo test` take 4+ minutes — unacceptable for a dev loop. Doctests are still rendered by `cargo doc` and can be run explicitly.

## Build and verification commands

**Always use `cargo make`.** Never direct `cargo` commands.

```bash
# Dev loop
cargo make check          # fast type check — default profile
cargo make check-all      # type check — all features
cargo make test           # unit + integration — default profile
cargo make test-all       # unit + integration — all features
cargo make test-minimal   # base profile only (no features)

# Lint and format
cargo make clippy         # deny warnings, all features
cargo make fmt            # check formatting
cargo make fmt-fix        # apply formatting

# Documentation
cargo make doc            # build docs — all features
cargo make doc-test       # doctest audit (slow — explicit opt-in)

# Type-law receipt gate (ALIVE gate — explicit opt-in, not daily dev loop)
cargo make alive          # runs trybuild compile-fail/pass fixtures

# Feature-specific CI verification
cargo make build-formats
cargo make build-strict
cargo make build-wasm4pm

# Full CI
cargo make ci             # check-all + test-all + clippy + fmt + alive

# ggen provision
cargo make ggen-witnesses      # render src/witnesses.rs
cargo make ggen-witnesses-dry  # dry run, preview only
```

Run a single test by name (one case where bare cargo is needed):

```bash
cargo test <test_name>
cargo test --all-features <test_name>
```

The crate has **no runtime dependencies**.

## Architecture

### The one-way door

The central invariant is a typed, one-way lifecycle enforced by the type system:

```
Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ {Projected | Exportable | Receipted}
  │                                  ▲
  └────────────── refuse ────────────┴──▶ Refused  (terminal; carries a named law)
```

`Evidence<T, State, W>` (in `src/evidence.rs`) is the universal carrier. `State` and `W` are zero-sized `PhantomData` tags, so `Evidence<T, Raw, W>` and `Evidence<T, Admitted, W>` are different types. The `Admitted` constructor is `pub(crate)` — the **only** public path to admitted evidence is `Admit::admit()`.

### The type law center of gravity

Type law lives in **public modules**, not in a single foundry appendix:

- **`src/law.rs`** — `ConstParamTy` enum set, `Assert`/`IsTrue`/`Require` bounds machinery, `ConditionCell<BITS>`, `Between01<NUM, DEN>`. This is the compile-time law kernel.
- **`src/petri.rs`** — typed bipartite arc types, `WfNetConst<SOUNDNESS>` with non-forgeable witness path.
- **`src/conformance.rs`** — `Metric<KIND, NUM, DEN>` with `Between01` bounds.
- **`src/process_tree.rs`** — `TypedLoopNode<ARITY>` with `Require<{ ARITY == 2 }>: IsTrue`.
- **`src/powl.rs`** — `TreeProjectable` sealed trait, `assert_tree_projectable`.
- **`src/formats.rs`** — `LossyFormatExport` requiring a non-optional loss report.
- **`src/strict.rs`** — `ExportBoundaryConst<HAS_WITNESS, HAS_ROUND_TRIP>` const-generic type.

`src/nightly_foundry.rs` is a staging/experimental module (no cfg gate). It hosts the four paper-derived law surfaces (petri_law, powl_law, evidence_law, token_law) as an always-on companion. The product type law lives in the modules above.

### The three-layer type system

1. **State tokens** (`src/state.rs`) — empty enums (`Raw`, `Parsed`, `Admitted`, `Refused`, `Projected`, `Exportable`, `Receipted`) used as lifecycle markers.
2. **Witness markers** (`src/witness.rs`) — empty enums implementing the `Witness` trait, each naming a specific paper, standard, or law (e.g. `Ocel20`, `Xes1849`, `WfNetSoundnessPaper`). They prevent `Admission<T, Ocel20>` from being confused with `Admission<T, Xes1849>` at the type level.
3. **Evidence** (`src/evidence.rs`) — the carrier that bundles a value `T` with a `State` tag and a `Witness` `W`.

### Admission and refusal (`src/admission.rs`)

`Admit` is the **only** sanctioned `Raw → Admitted` path. Implementations return `Result<Admission<…>, Refusal<R, W>>`. The reason type `R` must name a specific structural law (e.g. `DanglingEventObjectLink`, `MissingFinalMarking`) — bare `InvalidInput` is forbidden.

### Loss accounting (`src/loss.rs`)

`Project` is the **only** sanctioned lossy transformation. It requires:
- A `ProjectionName` (a `&'static str` newtype)
- A `LossPolicy` decided before loss occurs (`RefuseLoss` | `AllowNamedProjection` | `AllowLossWithReport`)
- A `LossReport<From, To, Items>` on every non-refusing path

There is no path from one external format directly to another. The only route is: `external → admitted compat → external | wasm4pm`.

### Feature model

Exactly three public Cargo features — no per-format flags:

| Feature | Default | What it adds |
|---|:---:|---|
| `formats` | yes | import/export contracts, round-trip claims, loss surfaces |
| `strict` | no | opt-in boundary judgment: strict admission/refusal surfaces |
| `wasm4pm` | no | graduation bridge traits toward the wasm4pm execution engine |

Disabling all features does **not** remove canon knowledge — the base profile still defines every process-evidence shape.

### Canon modules (always-on)

All modules in the base profile know the full process-evidence canon:
`law`, `eventlog`, `ocel`, `xes`, `bpmn`, `petri`, `powl`, `process_tree`, `declare`, `ocpq`, `dfg`, `conformance`, `prediction`, `receipt`, `ids`, `evidence`, `admission`, `loss`, `diagnostic`, `witness`, `state`, `interop`, `nightly_foundry`.

## Type-law receipts (ALIVE gate)

The ALIVE certification gate is `cargo test --test ui_tests`. This runs trybuild fixtures:

- **compile-fail fixtures** in `tests/ui/compile_fail/` — each must fail for the **intended named law**, not accidentally. Every fixture has a corresponding `.stderr` file with the expected compiler diagnostic.
- **compile-pass fixtures** in `tests/ui/compile_pass/` — each must compile successfully, proving the lawful path is open.

A compile-fail fixture that fails for the wrong reason (missing import, typo, unstable feature drift) is **not** a valid type-law receipt.

## DX innovation surfaces

Modules that expose builder APIs, `Display` impls, or `From` conversions for ergonomic use:

| Module | DX surface |
|---|---|
| `eventlog` | `Event::new(…).at_ns(…).by(…).with_lifecycle(…)` — full builder chain; `EventLog::from_traces([…])`, `Trace::new(id, [events])` |
| `ocel` | `OcelEvent::new(…).at_ns(…)`, `EventObjectLink::new(…).qualified(…)`, `ObjectObjectLink::new(…).qualified(…)`, `ObjectChange::new(…).at_ns(…)` — builder chains on all link/change types |
| `loss` | `ProjectionName` implements `Display`; `LossPolicy::is_refusing()`, `is_named()`, `is_reporting()` — guard helpers without pattern-matching |
| `admission` | `Admission<T,W>` and `Refusal<R,W>` implement manual `Debug` so witness `W` need not be `Debug` — enables `Result::expect` / `expect_err` |
| `evidence` | `Evidence::raw(v)` free constructor; full typestate transition chain as infallible builder methods (`into_parsed`, `into_admitted` via `Admit`, `into_projected`, `into_exportable`, `into_receipted`) |
| `ids` | Zero-cost `#[repr(transparent)]` newtype wrappers with `From<&str>`, `From<String>`, and `Display` for all identifier types |
| `witness` | All witness markers implement `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`; `Witness::KEY`, `TITLE`, `YEAR`, `FAMILY` const metadata |
| `strict` | `ProcessBoundary::fully_attested(kind, name)` convenience constructor; `StrictViolation::law()` returns a `&'static str` human-readable law name |
| `graduation` | `GraduationReason::tag()`, `is_hard_signal()` — label helpers; `GraduationCandidate::is_grounded()` |

## Example programs

Runnable examples in `examples/` (run with `cargo run --example <name>`):

| Example | Feature flag | What it demonstrates |
|---|---|---|
| `basic_eventlog` | (none) | `Event`/`Trace`/`EventLog` builder chain, `validate()`, `EventStream` append-only buffer |
| `basic_ocel` | (none) | `OcelLog` with E2O/O2O links and object changes, structural `validate()` |
| `evidence_lifecycle` | (none) | `Evidence<T, State, W>` one-way typestate: `Raw → Parsed → Admitted → Receipted`; illegal transitions rejected at compile time |
| `witness_authority` | (none) | Witness markers as zero-cost distinct types; cross-standard admission confusion rejected at compile time |
| `loss_projection` | (none) | `LossPolicy` / `LossReport` / `LossChain` — named, auditable structural loss; compiler enforces accounting |
| `ocel_to_xes_projection` | `formats` | OCEL → XES under format covenant: `ProjectionName`, `LossPolicy`, named `XesExportRefusal` |
| `petri_net_construction` | (none) | WF-net typed arcs, `WfNetConst` soundness typestate (`Unknown → Claimed → Witnessed`), non-forgeable `SoundnessProof` |
| `conformance_metrics` | (none) | Fitness/precision/generalization/simplicity as compile-time `[0,1]` rationals; out-of-range rejected by compiler |
| `declare_constraint_model` | (none) | Declare binary constraints (`Response`, `Precedence`), unary existence, OC-Declare object-type scoping |
| `ocpq_typed_query` | (none) | OCPQ typed query shapes: scope strategies, predicate families, const-generic cardinality bounds |
| `powl_process_tree` | (none) | POWL partial orders, `TypedLoopNode<ARITY>` (arity-2 enforced), `TreeProjectable` sealed gate |
| `causal_net_shape` | (none) | `CausalNet` / `CausalBinding` — Heuristics Miner output shapes (structure only, no mining) |
| `receipt_chain` | (none) | `ReceiptEnvelope`, `ReceiptChain`, `ReceiptChainConst<N>` (stack-arity-enforced), `GraduationReceipt` |
| `sealing_admit_chain` | (none) | `SealingAdmit` seam: BLAKE3 fold → `ChainProof` → `RuntimeSeal` → `SealedAdmission` → `Admitted` evidence; tamper witness |
| `prediction_problem_shape` | (none) | `PredictionProblem<T>` with all six `PredictionTarget` kinds, three `PredictionHorizon` variants, `ComplianceKind`, and six named `PredictionRefusal` laws |
| `strict_boundary_claim` | `strict` | `ProcessBoundary` declaration, `StrictCheck`, named violations: `MissingLossPolicy`, `MissingRefusalPath` |
| `graduation_candidate` | `wasm4pm` | `GraduateToWasm4pm` bridge, `GraduationCandidate` grounded vs ungrounded |
| `c8_adversary_gap_demo` | (none) | Two-strategy divergence proof (LogicPlayer vs GraphPlayer); unified-semantics witness |
| `c8_collider_demo` | (none) | Collider topology mutation: hidden-body manifestation + collision proof emission |
| `c8_event_horizon_demo` | (none) | Event-horizon boundary detection; boundary proof receipts |
| `c8_market_planck_demo` | (none) | MarketPlanck cell state transitions with receipt generation |

## Invariants that must never be violated

- `#![forbid(unsafe_code)]` — no exceptions.
- Exactly three public Cargo features. Adding per-format flags breaks the contract.
- Every refusal must carry a **specific named law** as the reason type. `InvalidInput` or string-typed catch-alls are defects.
- Lossy projections must go through `Project` with a `LossPolicy` and emit a `LossReport`. Silent structure loss is a defect.
- No engine logic (discovery, conformance checking, replay, alignment) belongs in this crate. Those graduate to `wasm4pm`.
- Every public type, module, and function requires rustdoc stating what it **is**, what it is **not**, that it is structure-only, and when to graduate.
- Every public `fn` requires a doctest (or an explicit `ignore` with a documented reason).
- Type law must live in public modules — never hidden behind a cfg gate or a single foundry appendix.
