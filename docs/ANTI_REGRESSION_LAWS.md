# Anti-Regression Laws

> These are not preferences. They are invariants. Violating any one of them is a
> defect against the type-law covenant.

This document lists patterns that are **explicitly forbidden** in `wasm4pm-compat`.
Each entry names the forbidden pattern, the law it violates, and the canonical
detection signal.

---

## 1. Stable-First Language in Docs or Code

**Forbidden:** Any documentation or comment that implies a stable Rust build
target exists, that stable Rust support is planned, that the crate will compile
on stable "in a future release", or that nightly features are "temporary".

**Law violated:** Nightly-Only Covenant (`docs/NIGHTLY_ONLY_COVENANT.md`).

**Detection:**
- Phrases such as: "stable support", "MSRV", "once stabilized", "stable fallback",
  "stable-compatible", "when this feature stabilizes we will…"
- Any `#[cfg(feature = "nightly")]` guard — nightly is unconditional, not a feature.
- `rust-toolchain.toml` channel changed from `nightly` to `stable` or `beta`.

**Canonical fix:** Remove the language. The crate is nightly-only. No migration
path exists and none is planned.

---

## 2. Extra Cargo Features Beyond {formats, strict, wasm4pm}

**Forbidden:** Adding any Cargo feature not in the canonical set of exactly three:
`formats`, `strict`, `wasm4pm`. Per-format flags (`ocel`, `xes`, `bpmn`, `yawl`,
etc.), per-paper flags, per-law flags, or convenience meta-features are all
forbidden.

**Law violated:** Feature model invariant (`src/lib.rs` module-level docs, `CLAUDE.md`).

**Detection:**
- `Cargo.toml` `[features]` section with any key other than `formats`, `strict`,
  `wasm4pm`, and the implicit `default`.
- A `#[cfg(feature = "…")]` guard on a canon module (all canon modules are always-on).

**Canonical fix:** Move the code into the base profile (always-on) or into one
of the three existing features. If it genuinely belongs behind a fourth capability
stage, that is an architectural decision requiring a full covenant review — not a
unilateral feature addition.

---

## 3. Doctest Storm (doctest=false Must Be Preserved)

**Forbidden:** Removing or commenting out `doctest = false` under `[lib]` in
`Cargo.toml`, or adding any configuration that re-enables doctests in the default
`cargo test` invocation.

**Law violated:** Testing surfaces invariant (`CLAUDE.md` — doctest section).

**Detection:**
- `cargo test` taking more than 60 seconds on a cold build indicates doctest
  re-enablement.
- `Cargo.toml` `[lib]` section missing `doctest = false`.
- CI log showing 200+ doctest invocations in the default test run.

**Why this matters:** Every doctest touching `generic_const_exprs` or
`adt_const_params` is a separate `rustc` invocation. On a nightly-only crate
with 200+ such doctests, the default test run becomes 4+ minutes — unacceptable
for a dev loop. Doctests are still rendered by `cargo doc` and can be run
explicitly with `cargo test --doc --all-features`.

**Canonical fix:** Restore `doctest = false` under `[lib]`. Run doctests
explicitly when needed.

---

## 4. Engine Creep (Discovery / Conformance / Replay / Query / Prediction Execution)

**Forbidden:** Any function, method, or trait implementation in `wasm4pm-compat`
that executes process mining computation. This includes but is not limited to:

- Process model discovery (inductive miner, split miner, heuristic miner, etc.)
- Conformance checking computation (fitness, precision, alignment, token replay)
- OCPQ or OCEL query evaluation
- Prediction model training or inference
- Alignment cost computation

**Law violated:** Graduation boundary (`docs/GRADUATION_BOUNDARIES.md`).

**Detection:**
- The compile-fail fixture `engine_creep_discovery_absent.rs` must continue to
  fail. If it passes, the engine boundary has been breached.
- Any import of a search, optimization, or ML crate (`petgraph` algorithms,
  `nalgebra`, ML frameworks, etc.) is a hard signal.
- A `GraduationReason::RebuildingProcessMiningLocally` candidate being produced
  from within `wasm4pm-compat` itself is the loudest signal.

**Canonical fix:** Move the execution code to `wasm4pm`. Carry the evidence
shapes here; adjudicate them there.

---

## 5. Unsealed Compile-Fail (Fail Fixture Without .stderr)

**Forbidden:** A compile-fail fixture in `tests/ui/compile_fail/` without a
corresponding `.stderr` file containing the expected compiler diagnostic.

**Law violated:** ALIVE gate integrity (`CLAUDE.md` — type-law receipts section).

**Detection:**
- `tests/ui/compile_fail/*.rs` without a matching `*.stderr` file.
- A trybuild test that accepts "any error" rather than the named law's specific
  diagnostic.
- `.stderr` file present but empty, or containing only whitespace.

**Why this matters:** A compile-fail fixture that fails for the wrong reason
(missing import, typo, unstable feature drift) is **not** a valid type-law
receipt. The `.stderr` file is the sealed diagnostic — it names what the compiler
said and why. Without it, the fixture proves nothing.

**Canonical fix:** Run the trybuild test suite once with `TRYBUILD=overwrite` to
generate `.stderr` files, then commit them. Every `.stderr` file is a permanent
receipt of the compiler's judgment.

---

## 6. Paper Without Ledger Row

**Forbidden:** A witness type, a law module, or a compile fixture that references
a specific paper (by arXiv ID, author, or title) that has no corresponding row in
`docs/PAPER_COVERAGE_LEDGER.md`.

**Law violated:** Paper coverage covenant (`docs/PAPER_COVERAGE_LEDGER.md` format
and `CLAUDE.md` — every public type requires rustdoc stating what it is and is not).

**Detection:**
- A `witness::` type that cites a paper not listed in the ledger.
- A `compile_fail/*.rs` fixture comment citing a paper arXiv ID not in the ledger.
- `grep -r "arXiv\|van der Aalst\|Leemans\|Kourani" tests/ui/` returns a paper
  reference not present in `PAPER_COVERAGE_LEDGER.md`.

**Canonical fix:** Add the ledger row before or in the same commit as the witness
type or fixture. The ledger is the paper-to-type mapping. No paper is covered
without a row.

---

## 7. Law Claim Without Pass/Fail Fixture

**Forbidden:** A documentation claim, a witness type assertion, or a law module
export that asserts a type-law property without both a compile-pass fixture (proving
the lawful path is open) and a compile-fail fixture (proving the unlawful path is
closed).

**Law violated:** ALIVE gate completeness (`CLAUDE.md` — trybuild fixtures section).

**Detection:**
- A new `witness::` type added without corresponding entries in both
  `tests/ui/compile_pass/` and `tests/ui/compile_fail/`.
- A law surface in `src/law.rs`, `src/petri.rs`, `src/conformance.rs`, etc.
  that has a compile-pass fixture but no compile-fail fixture (or vice versa).
- The type-law crosswalk in `docs/NIGHTLY_TYPE_LAW.md` has a row with a missing
  pass or fail fixture column.

**Why this matters:** A pass fixture alone proves the door is open; it does not
prove the door can be closed. A fail fixture alone proves the lock exists; it
does not prove the key works. Both are required for a valid type-law receipt.

**Canonical fix:** Provide both fixtures. If the fail fixture is structurally
impossible (the type system already prevents every conceivable violation), document
this explicitly in the crosswalk row and provide a comment-only fail fixture that
explains why no `.rs` file is possible. This case is rare and requires justification.

---

## Summary Table

| # | Forbidden Pattern | Law Violated | Hard Stop? |
|---|---|---|---|
| 1 | Stable-first language | Nightly-Only Covenant | Yes |
| 2 | Extra Cargo features | Feature model invariant | Yes |
| 3 | Doctest storm (remove doctest=false) | Testing surfaces invariant | Yes |
| 4 | Engine creep (execution in compat) | Graduation boundary | Yes |
| 5 | Unsealed compile-fail (no .stderr) | ALIVE gate integrity | Yes |
| 6 | Paper without ledger row | Paper coverage covenant | Yes |
| 7 | Law claim without pass/fail fixture | ALIVE gate completeness | Yes |

All seven are hard stops. None admit exceptions without a full covenant review.
