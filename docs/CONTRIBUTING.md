# Contributing — wasm4pm-compat

Thank you for contributing to `wasm4pm-compat`. This document explains the
conventions, tooling, and invariants you must follow.

---

## Prerequisites

- Rust nightly (pinned by `rust-toolchain.toml`). Run `rustup show` to confirm
  the active toolchain is nightly.
- No additional system dependencies — the crate has zero runtime dependencies.

---

## The three test surfaces

Each surface has a distinct purpose and run cadence. Understand which surface you
are touching before writing tests.

### Surface 1: Unit and integration tests (fast loop)

```bash
cargo test --all-features --tests
```

These are the tests in `#[cfg(test)]` blocks inside `src/` files and in
`tests/*.rs` integration files. They run in sub-second on most machines and
should pass before every commit. Run them constantly during development.

When to write these: any behavioral assertion about the shape types (construction,
validation, lifecycle transitions, loss accounting, receipt shape-checks).

### Surface 2: Trybuild fixtures — the ALIVE gate

```bash
cargo test --test ui_tests -- --ignored
```

These are the compile-fail and compile-pass fixtures in `tests/ui/`. Each fixture
is a tiny Rust source file that must either fail to compile (compile-fail) or
compile cleanly (compile-pass). They are the **type-law receipts** for this crate.

When to run these: before tagging a release, after adding a new type surface, after
any change to `law.rs`, `state.rs`, `witness.rs`, `evidence.rs`, `admission.rs`, or
any module that contributes to the sealed-trait machinery.

Requirements for a valid type-law receipt:

- A compile-fail fixture must fail for the **named law**, not accidentally (wrong
  import, typo, unstable-feature drift). Every compile-fail fixture has a
  corresponding `.stderr` file with the expected compiler diagnostic.
- A compile-pass fixture must compile cleanly, proving the lawful path is open.
- Do not suppress an expected compile failure by changing the fixture to avoid the
  law — fix the law or document why the law changed.

To update a `.stderr` file after an intentional law change:

```bash
TRYBUILD=overwrite cargo test --test ui_tests -- --ignored
```

Review the diff before committing.

### Surface 3: Documentation audit (explicit opt-in)

```bash
cargo test --doc --all-features
```

Runs all doctests in all public modules. This is the documentation audit gate.
It is slow (200+ separate nightly `rustc` invocations), so it is not part of the
daily dev loop. Run it before a release or after adding new doctests.

Every public `fn` requires a doctest or an explicit `# ignore` with a documented
reason.

---

## Commit taxonomy

This codebase uses a specific commit taxonomy for tracing type-law evolution.
Use these prefixes:

| Prefix | Use for |
|--------|---------|
| `feat(...)` | New type surface, new module, new capability |
| `fix(...)` | Bug fix in shape, validation, or lifecycle |
| `docs(...)` | Documentation, doctests, guides |
| `test(...)` | New unit/integration test without new type surface |
| `chore(...)` | Dependency bump, toolchain bump, housekeeping |
| `paper-ledger` | Adding a paper to the paper coverage ledger |
| `paper-law` | Implementing a paper's structural law as a type surface |
| `type-law` | Improving or extending existing type-law machinery |
| `fixture-pass` | New compile-pass trybuild fixture |
| `fixture-fail` | New compile-fail trybuild fixture + `.stderr` file |

Use conventional commit format: `type(scope): description`, e.g.:

```
feat(conformance): add Metric<KIND, NUM, DEN> with Between01 bounds
fixture-fail: add test for Evidence<T, Raw> used as admitted
paper-ledger: record OCPQ paper coverage in PAPER_COVERAGE_LEDGER.md
```

---

## How to add a new type surface

Follow this checklist when adding a new module or extending an existing one:

1. **Create the module in `src/`** with a `//!` doc comment that states:
   - What the module is.
   - What the module is not (specifically: what it does not execute).
   - When to graduate to `wasm4pm`.

2. **Declare it in `src/lib.rs`** with a short doc comment on the `pub mod` line.

3. **Keep it always-on** unless it is genuinely a capability stage. New canon shapes
   belong in the base profile, not behind a feature flag.

4. **Write doctests for every public item.** Doctests must be self-contained and
   must not require external crates or files.

5. **Write unit tests** covering the normal and refusal paths.

6. **Add compile-fail fixtures** for any new illegal-state laws you introduce.
   Compile-fail fixtures must have a matching `.stderr` file.

7. **Add compile-pass fixtures** confirming the lawful path is open.

8. **Add a doctest that uses `ignore`** only when the type requires nightly
   feature flags that cannot propagate from the crate root into a doctest context.
   Document the reason with a `# Compile note` comment.

9. **Do not add engine logic.** If you are tempted to add discovery, conformance
   checking, replay, or optimization — stop. Open an issue or PR against `wasm4pm`
   instead.

10. **Update `src/prelude.rs`** if the new type is commonly needed by adopters.

---

## How to add a paper to the ledger

1. Open `PAPER_COVERAGE_LEDGER.md` (at the repo root).
2. Add a row with: paper number, citation, year, core contribution, and coverage
   status (`type-surface`, `witness-only`, `planned`).
3. If the paper defines a structural law (e.g. a new model class, a new constraint
   family), create a corresponding type surface (see the checklist above).
4. Add the appropriate `witness_marker!` entry to `src/witness.rs` if the paper
   names a new authority that evidence might be admitted against.
5. Commit with `paper-ledger: ...` prefix.

---

## The non-negotiables

These invariants must never be violated. A PR that breaks any of them will be
rejected without review:

### No engine logic in this crate

No process discovery, no conformance checking, no token replay, no alignment
computation, no optimization, no visualization. If you need to add an engine
capability, it belongs in `wasm4pm`. This crate is a compatibility surface.

### No stable fallback

The crate requires nightly unconditionally. Do not add `#[cfg(feature = "stable")]`
guards, do not try to replace nightly features with proc-macros or other workarounds,
and do not add MSRV badges. The type law requires nightly — this is permanent.

### Exactly three public features

The feature set is `formats`, `strict`, `wasm4pm`. Do not add a fourth feature.
Per-format flags (`ocel`, `xes`, `bpmn`, etc.) are explicitly prohibited. If you
believe a new capability stage is needed, open an issue for discussion before
implementing it.

### No `unsafe` code

`#![forbid(unsafe_code)]` is declared at the crate root. No exceptions.

### Named refusal reasons

Every `Refusal<R, W>` must carry a specific named reason type `R`. Using a bare
`String` or `&str` with a value like `"InvalidInput"` is a defect. Name the law
that was broken.

### Lossy projections must use `Project` + `LossPolicy` + `LossReport`

There is no path from one external format directly to another. The only route is:
external → admitted compat value → external or `wasm4pm`. A lossy step on that
path must go through the `Project` trait with an explicit `LossPolicy` and produce
a `LossReport`. Silent structure loss is a defect.

### Every public item needs rustdoc

Every public type, module, trait, and function requires rustdoc stating:
- What it is.
- What it is not.
- That it is structure-only.
- When to graduate to `wasm4pm`.

A PR that adds public items without rustdoc will be asked to add it before merge.

---

## Running the full verification suite

```bash
# Fast loop (run before every commit)
cargo build --all-features
cargo test --all-features --tests
cargo clippy --all-features -- -D warnings
cargo fmt --check

# Minimal canon (no features)
cargo build --no-default-features
cargo test  --no-default-features --tests

# Each capability stage individually
cargo build --no-default-features --features formats
cargo build --no-default-features --features strict
cargo build --no-default-features --features wasm4pm

# Type-law receipt gate (before release / after law changes)
cargo test --test ui_tests -- --ignored

# Documentation audit (before release)
cargo test --doc --all-features
```
