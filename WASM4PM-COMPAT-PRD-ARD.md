# WASM4PM-COMPAT — Product Requirements Document & Architecture Requirements Document

> **Covenant, not specification.**
> Applications conform upward. Nightly is the court.
> Backward-designed doctrine guides the agents. Forward-earned receipts judge the agents.

---

## 1. Product Overview

`wasm4pm-compat` is the **paper-complete process-evidence doorway** for Rust applications.
It is Level 2 and Level 3 of the Blue River Dam five-level maturity model:

| Level | Identity | Law |
|------|---|---|
| **2** | `wasm4pm-compat` (base) | Structures evidence — typed, admitted, refusable |
| **3** | `wasm4pm-compat` + `strict` feature | Judges evidence claims against formal invariants |

This crate is the doorway. `wasm4pm` is the throne room. **The doorway must not become the throne room.**

### 1.1 Governing Doctrine

> "Do not make the future compatible with applications. Make applications compatible with the future."

The stable Rust surface is not the adoption floor. Nightly Rust is the adoption floor, because the type law
that represents paper-derived process invariants requires nightly features. An application that cannot compile
against nightly has not met the process-evidence standard — it has merely compiled.

### 1.2 Three Cargo Features (No More, No Less)

| Feature | Law |
|---|---|
| `formats` | Evidence crosses format boundaries (XES ↔ OCEL ↔ BPMN ↔ PNML). Requires named projection + `LossPolicy` + `LossReport`. |
| `strict` | Caller claims compliance with a formal process invariant. The crate judges it. Refusal is named law. |
| `wasm4pm` | Caller is graduating to full execution authority. Unlocks `graduation.rs` bridge types. |

There is no `nightly` feature. Nightly is unconditional — controlled by `.cargo/config.toml` and the
`RUSTFLAGS="--cfg wasm4pm_compat_nightly"` env var that gates the type-law surfaces in `nightly_foundry.rs`.

---

## 2. Problem Statement

### 2.1 The Stable-First Violation

A stable-first design makes the *type system* conform to *adoption comfort*, not to *paper law*.
Van der Aalst's quality dimensions (§9.2) are real mathematical objects: `FitnessScore ∈ [0,1]`,
`PrecisionScore ∈ [0,1]`, WF-net soundness (Murata 1989 bipartite constraint), OCEL tuple
`(E, O, EA, OA, E2O, O2O)` (Ghahfarokhi 2021). These objects have invariants that cannot be
fully expressed in stable Rust.

Stable Rust:
- Cannot encode const-generic bounds like `PLACES: usize + TRANSITIONS: usize` across trait impls
  (`generic_const_exprs`, `adt_const_params` required)
- Cannot specialize `AdmittedEvidence<T>` on `T: Verifiable` vs fallback (`min_specialization` required)
- Cannot return `impl Trait` from trait methods without GAT + `type_alias_impl_trait`
- Cannot express SIMD-accelerated token replay in safe no_std (`portable_simd` required)

A crate that promises "paper-complete" but compiles only on stable is self-contradicting.
The paper completion is fiction; the stable compilation is real.

### 2.2 The Correct Fence (Chesterton's Gate)

The original fence: **nightly is required because the type law requires it**.

The weakened fence: "nightly is an optional internal foundry for future experimentation, stable is the
adoption surface."

Restoring the fence: **nightly is the court**. Applications are admitted when they compile against
nightly with the full type-law surface active. A crate that compiles on stable has not been admitted;
it has been tolerated.

---

## 3. Product Requirements (PRD)

### 3.1 ALIVE Standard (10-Point)

A module is ALIVE when all of the following hold from real `cargo` output, never from narration:

| # | Requirement | Evidence |
|---|---|---|
| 1 | `cargo +nightly check --all-features` exits 0 | Compiler output |
| 2 | `cargo +nightly test --all-features` exits 0 | Test runner output |
| 3 | `cargo +nightly test --doc --all-features` exits 0 | Doctest output |
| 4 | `cargo +nightly clippy --all-targets --all-features -- -D warnings` exits 0 | Clippy output |
| 5 | `nightly_foundry.rs` exposes at least one real type-law surface using a nightly feature from the canonical set | Source code |
| 6 | At least one compile-fail fixture in `tests/ui/` rejects an invalid shape | `compiletest` or `trybuild` output |
| 7 | At least one compile-pass fixture in `tests/ui/` admits a valid shape | `compiletest` or `trybuild` output |
| 8 | `src/admission.rs` contains at least one named refusal variant (not `InvalidInput`) | Source code |
| 9 | The `formats` feature, when enabled, requires a `LossPolicy` for any lossy projection | `strict` type gate in source |
| 10 | The `strict` feature, when enabled, invokes at least one formal-invariant check with a named refusal | Source code |

A module that compiles on stable but fails requirement 5, 6, 7, 8, 9, or 10 is **PARTIAL**, not ALIVE.
PARTIAL is honorable when scoped. False ALIVE is breach.

### 3.2 Three-Layer Evidence (Inherited from Chicago TDD)

No claim is complete without:
1. **Type-law surface** — nightly feature encodes paper invariant in the type system
2. **Test assertion** — test proves the type gate admits/rejects correctly
3. **Compile fixture** — compile-fail or compile-pass proves the boundary at the type level

---

## 4. Architecture Requirements Document (ARD)

### 4.1 Nightly Feature → Paper Object Mapping

| Nightly Feature | Paper Invariant Encoded | Module |
|---|---|---|
| `generic_const_exprs` | Petri net bipartite: `places + transitions` bounded by const | `petri.rs`, `nightly_foundry.rs` |
| `adt_const_params` | POWL composition depth const-generic (Kourani 2505.07052 §3) | `powl.rs`, `nightly_foundry.rs` |
| `const_trait_impl` | `FitnessScore::new()` and `PrecisionScore::new()` as const-verified constructors | `conformance.rs`, `nightly_foundry.rs` |
| `type_alias_impl_trait` | `AdmittedEvidence<T>` with `impl Witness` return from trait method | `evidence.rs`, `witness.rs` |
| `min_specialization` | Specialize `AdmittedEvidence<T>` on `T: Verifiable` with formal-invariant check | `strict.rs`, `nightly_foundry.rs` |
| `portable_simd` | SIMD-accelerated token replay scan (fitness computation kernel) | `conformance.rs` (planned) |

### 4.2 Module Responsibilities

| Module | Level | Purpose | Refusal Names |
|---|---|---|---|
| `admission.rs` | L2 | Core admission/refusal type (`Admitted<T>` / `Refused`) | `MissingObjectRelation`, `FlatteningLoss`, `MissingWitness` |
| `evidence.rs` | L2 | `AdmittedEvidence<T>` + `Witness` trait | `UnreplayableClaim` |
| `eventlog.rs` | L2 | XES event log (case × event × attribute) | — |
| `ocel.rs` | L2 | OCEL 2.0: `(E, O, EA, OA, E2O, O2O)` (Ghahfarokhi 2021) | `MissingObjectRelation` |
| `petri.rs` | L2 | Petri net (Murata 1989 bipartite: P, T, F) | `DeadTransition`, `UnsoundWfNet` |
| `powl.rs` | L2 | POWL (Kourani 2505.07052) — partial-order workflow language | `InvalidPowlProjection` |
| `conformance.rs` | L2/L3 | Fitness / precision / quality dimensions (van der Aalst 2016 §9.2) | — |
| `dfg.rs` | L2 | Directly-Follows Graph | — |
| `process_tree.rs` | L2 | Process tree (Leemans 2013 — inductive miner output) | — |
| `declare.rs` | L2 | Declare constraint language | — |
| `bpmn.rs` | L2 | BPMN structural types | — |
| `ocpq.rs` | L2 | Object-Centric Process Query (Küsters & vdA 2506.11541) | `MissingWitness` |
| `xes.rs` | `formats` | XES format crossing (IEEE XES standard) | `FlatteningLoss` |
| `formats.rs` | `formats` | Format covenant enforcement — requires `LossPolicy` for lossy projections | `FlatteningLoss` |
| `loss.rs` | `formats` | `LossPolicy` + `LossReport` for projection honesty | — |
| `interop.rs` | `formats` | OCEL → XES flattening (requires explicit projection + `LossPolicy`) | `FlatteningLoss` |
| `strict.rs` | `strict` | Formal-invariant claim judgment (Rank-1 oracles: Murata bipartite, WF-net soundness) | `DeadTransition`, `UnsoundWfNet`, `InfeasibleConformance` |
| `diagnostic.rs` | `strict` | Diagnostic evidence for refusals | — |
| `witness.rs` | `strict` | External-witness lattice (court record, not a leash) | `UnreplayableClaim`, `MissingWitness` |
| `prediction.rs` | L2 | Next-activity, remaining-time, outcome shapes | — |
| `graduation.rs` | `wasm4pm` | Bridge types for graduating to full `wasm4pm` execution authority | — |
| `receipt.rs` | L2 | Receipt + replay proof shapes (BLAKE3 chain, OCEL replay pointer) | `UnreplayableClaim` |
| `state.rs` | L2 | Bounded state (8-bit, Need9 = split) | — |
| `ids.rs` | L2 | Typed identifiers (CaseId, EventId, ObjectId, ActivityName) | — |
| `nightly_foundry.rs` | L2 (nightly) | Central nightly type-law surface — NOT a placeholder | — |
| `prelude.rs` | all | Public re-exports | — |
| `evidence.rs` | L2 | Admitted evidence wrapper + Witness trait | — |

### 4.3 nightly_foundry.rs — Central, Not Placeholder

`nightly_foundry.rs` is gated on `#[cfg(wasm4pm_compat_nightly)]`. When the cfg is active, it must:

1. Use at least one feature from the canonical nightly set (see §4.1)
2. Express at least one paper-derived invariant in the type system (not in runtime logic)
3. Provide at least one compile-fail test target via `tests/ui/` that demonstrates the invariant holds

The module is not "future work." It is the proof that the crate's ALIVE claim is grounded in type law,
not just in successful compilation.

### 4.4 Refusal Law

A system that cannot refuse cannot govern. Refusal must:
- **Name the violated law** — not `InvalidInput`, but `MissingObjectRelation`, `FlatteningLoss`,
  `DeadTransition`, `UnsoundWfNet`, `InvalidPowlProjection`, `MissingWitness`, `UnreplayableClaim`
- **Be first-class** — `Refused` is not an error variant, it is a peer of `Admitted`
- **Be typed** — the refusal type carries the law name, not a runtime string

### 4.5 Format Covenant

No raw format-to-format laundering. The lawful path:

```
recognized external format
  → typed admitted compat structure (via admission.rs)
    → recognized external format OR full wasm4pm (via formats.rs + loss.rs)
```

OCEL → XES flattening must refuse unless explicitly projected with:
1. Named projection (which OCEL fields are preserved)
2. `LossPolicy` (what happens to unmapped relations)
3. `LossReport` (what was discarded and why)

Hidden flattening is refusal.

### 4.6 Compile-Fail Fixtures (tests/ui/)

The `tests/ui/` directory must contain:

**Compile-fail fixtures (invalid shapes are rejected at type level):**
```
tests/ui/
  ocel_missing_e2o.rs           — OCEL without E2O relation cannot be admitted as flat XES
  petri_need9_split.rs          — 9-bit state index violates Need9=split law
  flattening_without_policy.rs  — OCEL→XES without LossPolicy is compile error
  strict_unchecked_claim.rs     — strict::claim() without Verifiable bound is rejected
```

**Compile-pass fixtures (valid shapes are admitted):**
```
tests/ui/
  admitted_ocel.rs              — Valid OCEL with E2O compiles and is Admitted
  admitted_xes.rs               — Valid XES event log is admitted
  strict_verified_claim.rs      — strict::claim() with Verifiable impl compiles
```

### 4.7 ALIVE Verification Commands

```bash
# ALIVE check — all must exit 0
RUSTFLAGS="--cfg wasm4pm_compat_nightly" cargo +nightly check --all-features
RUSTFLAGS="--cfg wasm4pm_compat_nightly" cargo +nightly test --all-features
RUSTFLAGS="--cfg wasm4pm_compat_nightly" cargo +nightly test --doc --all-features
RUSTFLAGS="--cfg wasm4pm_compat_nightly" cargo +nightly clippy \
  --all-targets --all-features -- -D warnings

# Compile-fail / compile-pass fixture check (requires trybuild or compiletest)
RUSTFLAGS="--cfg wasm4pm_compat_nightly" cargo +nightly test --test ui
```

---

## 5. Definition of Done

A module is **not complete** because it compiles on stable.

A module is **complete** when:
> Its paper object is represented as a real type surface, and every invariant expressible
> in nightly Rust is encoded in the type system or as a named refusal.

Judgment:
1. Run the ALIVE verification commands (§4.7). All must exit 0.
2. At least one `nightly_foundry.rs` surface uses a canonical nightly feature (§4.1).
3. At least one compile-fail fixture in `tests/ui/` rejects an invalid shape.
4. At least one refusal variant uses a named law (not `InvalidInput`).
5. No module claims ALIVE from narration — only from cargo output.

---

## 6. Anti-Requirements

This crate **MUST NOT**:

| Anti-requirement | Rationale |
|---|---|
| Include a `nightly` Cargo feature | Nightly is unconditional. A feature that can be disabled is a workaround, not a covenant. |
| Include execution engines (WASM compilation, algorithm implementations) | This is Level 2/3. Doorways do not contain throne rooms. |
| Provide a stable-Rust-only surface as the "real" API | Stable compilation without nightly type law is false ALIVE. |
| Silently flatten OCEL to XES | Hidden flattening is refusal. Every projection requires named `LossPolicy`. |
| Use `InvalidInput` as a refusal reason | Good refusal names the violated law. Bad refusal says "invalid input." |
| Claim ALIVE from narration | "Code passes tests because tests derive expected values from implementation" — that is FM-5. |
| Smuggle in Living LSP, the branchless 8-bit kernel court, or full `wasm4pm` execution | A doorway that ships a hidden engine violates its own doctrine. |
| Have more than 3 public Cargo features | Feature flags are horses. Do not multiply horses. |

---

## 7. Paper Grounding Index

| Paper | Citation | Module |
|---|---|---|
| van der Aalst (2016) Process Mining §9.2 | Quality dimensions: fitness, precision, generalization, simplicity | `conformance.rs` |
| Adriansyah (2014) PhD | Alignments — exact conformance checking | `conformance.rs` |
| Leemans, Fahland & van der Aalst (2013) | Inductive miner — process tree discovery | `process_tree.rs` |
| Murata (1989) IEEE Proc. | Petri net theory — bipartite (P, T, F) structure, boundedness, liveness | `petri.rs`, `strict.rs` |
| Kourani & van der Aalst (arXiv:2602.15739) | WF-net soundness: deadlock-free, live, bounded | `petri.rs`, `strict.rs` |
| Kourani (arXiv:2505.07052) | POWL — Partial-Order Workflow Language | `powl.rs` |
| Küsters & van der Aalst (arXiv:2506.11541) | OCPQ — Object-Centric Process Query | `ocpq.rs` |
| Ghahfarokhi et al. (2021) ICSOC | OCEL 1.0 standard definition | `ocel.rs` |
| OCEL 2.0 Standard (IEEE TF PM, 2023) | OCEL 2.0 formal tuple `(E, O, EA, OA, E2O, O2O)` | `ocel.rs` |
| van der Aalst & Berti (2020) FI | Object-Centric Petri Nets (arXiv:2010.02047) | `petri.rs`, `ocpq.rs` |
| Munoz-Gama & Carmona (2010) BPM | ETConformance precision (escaping-edge analysis) | `conformance.rs` |
| van der Aalst, Reijers & Song (2005) CSCW | Social network mining (handover-of-work, working-together) | (future: `social.rs`) |

---

## 8. Relationship to Blue River Dam

This document is one surface of the Blue River Dam doctrine.
The dam is upstream. This crate is the admission layer.

```
recognized external format
  → [wasm4pm-compat admission layer — THIS CRATE]
    → typed admitted process evidence
      → [wasm4pm execution authority — throne room]
        → receipts / replay / adversarial benchmark
```

The crate does not itself certify completion. Completion is earned only by
reachable code, passing gates, negative fixtures, receipts, and replay.

**Backward-designed doctrine guides the agents.**
**Forward-earned receipts judge the agents.**

---

*Repository: https://github.com/seanchatmangpt/wasm4pm-compat*
*Version: 26.6.8 | Rust edition: 2024 | Toolchain: nightly (covenant)*
