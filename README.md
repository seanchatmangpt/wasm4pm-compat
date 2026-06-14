# wasm4pm-compat v26.6.13 — Process Intelligence Compatibility Core

![nightly-only](https://img.shields.io/badge/toolchain-nightly--only-orange)
![no-unsafe](https://img.shields.io/badge/unsafe-forbid%28unsafe__code%29-red)
![structure-only](https://img.shields.io/badge/scope-structure--only-blue)

> **Nightly Rust required. Applications conform upward to future type law.**

---

## Version Alignment

The logical system version, target specification, and documented release standard for this codebase is `26.6.13`. All API behaviors, validation logic, and diagnostic receipts in this repository are designed to conform to the **`wasm4pm-compat v26.6.13`** standard. The root crate, workspace subcrates, and derived manifests are all natively configured to `26.6.13`.

---

## What's New in v26.6.13

This release ships the full witness corpus and five nightly zero-cost innovations, then hardens the crate against nightly churn and closes the gap between *naming* a law and *enforcing* it. The ALIVE gate is green (217 compile-fail + 408 compile-pass receipts) and MIRI reports no undefined behavior across the dependency graph.

*   **271-paper witness corpus.** Every paper in the research library now has a zero-sized witness marker, rendered via `ggen` into seven per-category modules.
*   **Self-validating bibliography.** The new `witness_corpus` module renders `ALL_WITNESS_KEYS` (436 keys) with a compile-time `const` proof that no two witnesses share a `KEY` — closing a collision hole that SPARQL deduplication alone could not enforce in Rust source. A runtime companion test names any offending key.
*   **Authority enforcement, not just labeling.** `LinkedOcel` is the first concrete `Admit` impl in `src/`. The crate no longer merely *names* `DanglingEventObjectLink` / `EmptyEventObjectLinks` — it ships a function that *detects* them and refuses through the typed `Raw → Admitted` one-way door.
*   **Teaching diagnostics.** `#[diagnostic::on_unimplemented]` on the five family-authority traits: a wrong-family witness now reads ``` `PowlPaper` is not a Standard-family authority ``` with the list of witnesses that do qualify, instead of a bare bound error.
*   **Nightly zero-cost innovations.** `WitnessFamily` derives `ConstParamTy`; `Witness` is a `const trait`; the `witness_law` module adds sealed family-authority gating, a compile-time co-citation string law, `gcd`/`NormedBetween01`, and a `portable_simd` family-batch check.
*   **TypeScript bindings extracted to a sidecar.** The `ts`/`specta` surface moved to the `wasm4pm-compat-ts` companion crate, restoring the *exactly three public features* and *no runtime dependencies* invariants in the core crate.
*   **Durability.** The toolchain is pinned to a dated nightly so a const-generics or const-trait syntax flip lands on one known toolchain. A documented in-code finding records why the `generic_const_exprs → min_generic_const_args` migration is **not viable** today: mGCA forbids generic parameters in computed const operations, so the computed-const law kernel (`Between01`, `Metric`, `ConditionCell`) has no stable-floor path, and the two features are mutually exclusive in one crate.

> **In progress:** literal-100% item-level rustdoc. Module-level documentation is complete across all canon modules; remaining public-item docs (compiler-measured via `missing_docs`) are a tracked follow-up.

---

## Toolchain & Runtime Constraints

This crate provides no Minimum Supported Rust Version (MSRV) guarantees and contains no stable Rust fallback mechanisms. It is designed and implemented exclusively for the nightly compiler toolchain.

Applications using this library **must run under nightly Rust (refer to docs/explanation/why-nightly.md)**. 

The toolchain is pinned via `rust-toolchain.toml` to a specific nightly release. The crate root declares `#![feature(generic_const_exprs, adt_const_params, const_trait_impl, min_specialization, portable_simd)]` without conditional gates. This design ensures that the compiler's monomorphization and const-evaluation engines enforce domain-specific type laws before runtime code generation occurs.

---

## Architectural Mandate: What this Crate IS and IS NOT

### What this Crate IS

`wasm4pm-compat` is a zero-cost structural boundary and verification interface for process mining artifacts. It is defined by the following characteristics:

*   **Process-Evidence Focused**: It specializes in modeling and verifying process artifacts (event logs, Petri nets, process trees) as formal, cryptographic evidence.
*   **Structure-Only**: It defines the data schemas, type parameters, and conversion laws for process-evidence structures, but performs no execution or calculation.
*   **Paper-Complete**: It implements structures representing the entire theoretical canon of process mining and process query formalisms from literature (including Petri nets, WF-nets, BPMN, OCEL 2.0, IEEE XES 1849, POWL, process trees, Declare, OCPQ, DFGs, and conformance alignments).
*   **Feature-Capped**: The crate API is strictly limited to structural validation, import/export contracts, and graduation prep. It does not grow to incorporate runtime solvers.
*   **Refusal-First**: It models boundary rejections as first-class, strongly-typed values (`Refusal`) carrying named structural laws rather than generic runtime strings or raw parse errors.
*   **Loss-Aware**: Any lossy projection is explicitly tracked, governed by a caller-supplied `LossPolicy`, and documented using a detailed, typed `LossReport` containing a static `ProjectionName`.
*   **Receipt-Shaped**: It models the structural envelope, witness metadata, and cryptographic digest shapes for provenance receipts.
*   **Graduation-Ready**: It implements the bridge traits and candidates needed to safely graduate static evidence to the execution engine.

### What this Crate IS NOT

To maintain a clean architectural boundary, `wasm4pm-compat` is:

*   **Not a lite wasm4pm**: It is not a subset or stripped-down version of the execution engine.
*   **Not an engine**: It contains no execution environment, solver, or simulation runtime.
*   **Not a conformance checker**: It does not compute fitness, precision, generalization, or trace alignment scores. It only models their static verdict structures.
*   **Not a replay/discovery engine**: It does not execute discovery algorithms (such as Alpha, Inductive, or Heuristics miners) or replay logs against models.
*   **Not a TypeScript/Zod generator**: It does not generate serialization wrappers or frontend interface schemas.
*   **Not a WASM ABI crate**: It does not define low-level WASM linear memory layouts or foreign function interfaces (FFIs).
*   **Not a format laundromat**: It forbids direct, unmonitored format-to-format conversion. Translating data requires admitting the input into a typed compat value under a witness, resolving any data loss under an explicit policy, and then exporting or graduating the result.

---

## Evidence Lifecycle

The central invariant of `wasm4pm-compat` is a typed, one-way lifecycle tracked at compile time using zero-cost typestate markers.

```text
  Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ {Projected | Exportable | Receipted}
    │                                  ▲
    └────────────── refuse ────────────┴──▶ Refused (terminal; carries a named law)
```

The universal carrier struct `Evidence<T, State: EvidenceState, W>` wraps the process payload `T` with two phantom type parameters: `State` representing the lifecycle stage and `W` representing the governing witness. Because `Evidence<T, Raw, W>` and `Evidence<T, Admitted, W>` are distinct types at compile time, functions demanding admitted evidence cannot compile if passed raw inputs.

1.  **Raw**: Untrusted data directly from the boundary.
2.  **Parsed**: Structurally decoded into memory, but not yet evaluated against type law.
3.  **Admitted**: Formally validated against a specific witness standard. This state cannot be constructed directly from outside the crate; it is only reachable by resolving the `Admit` trait.
4.  **Projected**: The result of a named, accounted lossy transformation.
5.  **Exportable**: Approved for conversion back into external serialization formats.
6.  **Receipted**: Sealed inside a provenance-bearing cryptographic receipt envelope.
7.  **Graduation Candidate**: Prepared to exit the compat boundary and pass to the execution engine.

Transitions between states consume the carrier struct by-value (`self`), preventing use-after-move defects at compile time.

---

## Witness Markers

Witnesses are zero-sized empty enums implementing the `Witness` trait (e.g., `Ocel20` for OCEL 2.0, `Xes1849` for IEEE XES 1849, `WfNetSoundnessPaper` for Workflow Net soundness). They serve as type-level markers indicating which authority standard or academic publication governs the validation and formatting laws of a piece of evidence.

Because witnesses are part of the type signature, `Evidence<T, Admitted, Ocel20>` and `Evidence<T, Admitted, Xes1849>` are incompatible types. This prevents the silent mixing of standards. The library tracks witness validation status monotonically using a Join-Semilattice representation (`WitnessState<W: Witness>` with states `Unknown`, `Satisfied`, `Violated`, and `Contradiction`).

Every witness belongs to a `WitnessFamily` (`Standard`, `Paper`, `ApiGrammar`, `RustLaw`, `InternalBridge`). The `witness_law` module gates types by family at compile time: `StandardWitness<W>` only accepts a `Standard`-family witness, and a wrong-family witness fails with a teaching diagnostic naming the law. The full corpus (271 papers, 436 unique keys) is rendered from ontology and self-validated — the `witness_corpus` module carries a compile-time proof that no two witnesses share a key.

---

## Boundary Laws

### The Admission/Refusal Law

Boundary validation is governed by the `Admit` trait, which evaluates raw evidence against a witness and returns `Result<Admission<T, W>, Refusal<R, W>>`. A `Refusal` cannot contain generic error messages or raw strings. Its `R` parameter must be a domain-specific enum variant representing the exact structural law that was violated:

*   **OCEL**: `DanglingEventObjectLink`, `DuplicateObjectId`, `UnqualifiedObjectRelation`.
*   **WF-net**: `MissingFinalMarking`, `UnsoundWfNet`, `DeadTransition`.
*   **XES**: `MissingConceptName`, `NonMonotonicTimestamps`.
*   **POWL**: `CyclicPartialOrder`, `DanglingOperatorChild`.

This ensures that all boundary rejections are typed, auditable, and testable.

### The Loss Law

Transformations that discard evidence (such as flattening multi-perspective OCEL logs into single-perspective XES logs) must implement the `Project` trait. Projection enforces a three-type lock:

$$\text{LossPolicy} \longrightarrow \text{ProjectionName} \longrightarrow \text{LossReport}$$

1.  **LossPolicy**: The caller must explicitly select the loss policy before projection:
    *   `RefuseLoss`: The projection fails and returns a named refusal (e.g., `FlatteningLoss`) if any evidence would be dropped.
    *   `AllowNamedProjection`: The projection is permitted under a static `ProjectionName`.
    *   `AllowLossWithReport`: The projection is permitted and produces a `LossReport` itemizing the discarded items.
2.  **ProjectionName**: A newtype wrapper of a `&'static str` (e.g., `ProjectionName("ocel-flatten-to-xes:by-case")`) representing a static, hardcoded transformation name.
3.  **LossReport**: A structured record containing the projection name, policy, and the itemized collection of lost items, parameterized by the source and target shape tags.

---

## Receipt-Shaped Evidence & The Graduation Path

When a host needs to perform active computation (such as model discovery or conformance checking), the evidence must graduate:

*   **Receipt-Shaped Evidence**: Modeled via `ReceiptShape` and `ReceiptEnvelope`. These structures represent the cryptographic metadata, digests, and replay hints, but perform no actual hashing or signing.
*   **Graduation Bridge**: Decoupled from the execution engine, the `GraduateToWasm4pm` trait (enabled under the `wasm4pm` feature) allows structural shapes to compile a `GraduationCandidate`.
*   **GraduationCandidate**: A structural wrapper containing a `GraduationReason` (such as `NeedsDiscovery`, `NeedsConformanceExecution`, `NeedsReplay`, or `RebuildingProcessMiningLocally`), the subject name, and a hash reference to the grounding evidence. The external engine consumes these candidates to perform the actual process mining calculations.

---

## Feature Contract

The public feature surface of `wasm4pm-compat` is **exactly three** features. Features control boundary capability stages rather than core domain knowledge — the base profile (compiled with `--no-default-features`) always knows the complete canon of process-evidence structures.

| Feature | Default | Capability Added |
|---|:---:|---|
| `formats` | **Yes** | Enables import/export traits, format covenants, and loss-policy interfaces. |
| `strict` | No | Enables strict boundary checks, `ProcessBoundary`, and `StrictViolation` diagnostic markers. |
| `wasm4pm` | No | Enables the graduation bridge, `GraduateToWasm4pm`, and `GraduationCandidate` types. |

There are no per-format features (e.g., no `ocel` or `xes` flags). The entire canon is always compiled. Nightly is not a cargo feature; it is the toolchain requirement.

---

## ggen Ecosystem Projection

`ggen` (the Ostar generative pipeline/stewardship compiler) operates as a provision instrument that translates ontologies (the witness TTLs defining the 271-paper corpus, 436 unique witness keys) and manifests into Rust source definitions, the per-category witness registries (`src/witnesses.rs`, `src/witnesses_*.rs`), the `witness_corpus` uniqueness proof, and negative verification fixtures. `wasm4pm-compat` serves as the target type-law court; it does not depend on `ggen` code or runtimes.

wasm4pm-compat defines the Rust process-evidence court.
ggen will later project into that court.
wasm4pm will later execute judgment after graduation.

---

## Verification Commands

To verify that the compatibility core compiles, conforms to all type-law covenants, and passes all tests, execute the following commands in order:

```bash
# Code formatting check
cargo fmt --check

# Clippy lints with all features enabled
cargo clippy --all-features -- -D warnings

# Build the codebase with all features
cargo build --all-features

# Run standard unit and integration tests
cargo test --all-features --tests

# Run the ALIVE gate (trybuild compile-pass and compile-fail fixtures)
cargo test --test ui_tests -- --ignored

# Run all public documentation tests
cargo test --doc --all-features

# Verify packaging list
cargo package --list

# Verify crate publishability via a dry run
cargo publish --dry-run
```

---

## Documentation Structure

The documentation for `wasm4pm-compat` is organized according to the [Diátaxis](https://diataxis.fr) framework:

*   **Explanations (Process Theory and Design)**:
    *   [Rust Typestate and Process Theory](docs/explanation/rust-typestate-and-process-theory.md) - Deep dive into typestates, affine types, and token conservation.
    *   [Genesis Thursday: Day Five Conceptual Framing](docs/explanation/genesis-thursday.md) - Conceptual framing of compile-time structures vs runtime execution.
    *   [Process-Evidence Domain Glossary](docs/explanation/glossary.md) - Mathematical and crate definitions of key terms.
*   **Reference**:
    *   [Public API for ggen](docs/reference/public-api-for-ggen.md) - Target surface for `ggen` integration.
    *   [Module Map & Layout](docs/reference/module-map.md) - Mapping of Rust modules to physical files.
    *   [Evidence Lifecycle States](docs/reference/lifecycle-states.md) - Detail on lifecycle state transitions.
    *   [Feature Model](docs/reference/feature-model.md) - Details of the strict three-feature limit.
    *   [Publish Readiness Checklist](docs/reference/publish-checklist.md) - Release checklist before publishing.
*   **How-To Guides**:
    *   [Preparing for a Crates.io Release](docs/how-to/prepare-crates-io-publish.md) - Release preparation steps.
*   **Research & Reports**:
    *   [Process Theory Alignment](research/process-theory-alignment.md) - Mathematical alignment with literature.
    *   [Verification Report](docs/reports/v26.6.13-verification-report.md) - Status of mandatory verification gates.

---

## License

This project is licensed under either of:

*   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
*   MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
