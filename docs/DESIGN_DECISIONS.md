# Design Decisions — wasm4pm-compat

This document records the "why" behind the major design choices in this crate.
It is intended for contributors who want to understand intent before making changes,
and for adopters who are evaluating the crate and want to know what they are
committing to.

---

## Why nightly-only

The type law in this crate requires three nightly features that are not available on
stable Rust:

- **`generic_const_exprs`** — enables const-generic boolean expressions in where
  bounds, e.g. `where Require<{ N <= 8 }>: IsTrue`. This is how `ConditionCell<9>`
  fails to compile at the type level rather than at runtime. It is also how
  `Between01<NUM, DEN>` enforces that metric values are provably in `[0, 1]`.

- **`adt_const_params`** — enables enum values as const generic parameters. This is
  how `WfNetConst<SOUNDNESS>` carries a soundness claim as part of its type, and how
  `EvidenceMode` travels as a const generic value rather than a runtime discriminant.

- **`min_specialization`** — enables limited specialization for blanket impls. Used
  in `nightly_foundry` for the token-law surfaces.

- **`portable_simd`** — used in `nightly_foundry` for SIMD-shaped evidence carriers.
  Not required for the base profile.

There is no way to implement the compile-time law gates without these features.
Using stable Rust would require replacing the type-level bounds with runtime checks,
which defeats the entire purpose: a type-law surface is one where *illegal states are
unrepresentable*, not merely *detected at runtime*.

**This is a deliberate, permanent decision.** Applications must conform upward to this
crate's type law, not the other way around. There is no stable build target, no MSRV,
and no stable fallback.

---

## Why structure-only

This crate is a *compatibility surface*, not an execution engine. The distinction is
important:

- A **compatibility surface** defines the agreed shape of evidence and the laws of
  admission, refusal, and projection. Every process-mining tool that speaks to
  `wasm4pm` needs to agree on what an event log looks like, what a Petri net looks
  like, and what it means for evidence to be admitted. This crate is that agreement.

- An **execution engine** discovers process models, computes conformance, replays
  token trajectories, runs alignments, and optimizes plans. This crate does none of
  that.

Keeping these concerns separate means:

1. The compatibility surface compiles in milliseconds with zero runtime dependencies.
2. The engine can evolve independently; it imports shapes from this crate without
   being coupled to its type-law internals.
3. The boundary between "carrying evidence" and "acting on evidence" is explicit and
   auditable. If a PR adds discovery logic to this crate, it is a defect by
   definition.

The canonical "no engine logic here" invariant is documented in `src/lib.rs` and
enforced by code review. The `#![forbid(unsafe_code)]` attribute ensures no unsafe
engine shortcuts are possible.

---

## Why exactly three features

The public feature surface is exactly `formats`, `strict`, and `wasm4pm`. There are
no per-format flags (no `ocel`, `xes`, `bpmn`, etc.).

**The reason:** features control *capability stages*, not *canon knowledge*. The base
profile already knows every process-evidence shape. Splitting by format would mean a
caller who wants to admit an OCEL log and export it as XES would need to enable both
`ocel` and `xes`. This is a false choice: the crate knows both shapes unconditionally,
because they are part of the process-mining canon.

The three features represent three distinct *capabilities* that a consumer might
opt into:

| Feature | What it unlocks |
|---------|----------------|
| `formats` (default) | Import/export contracts, round-trip claims, loss surfaces |
| `strict` | Opt-in boundary judgment: `ProcessBoundary::check()` |
| `wasm4pm` | Graduation bridge traits: `GraduateToWasm4pm`, `GraduationCandidate` |

Adding a fourth feature would require a strong justification for why the new capability
is genuinely orthogonal to these three and is not better represented as a new type
surface in the base profile.

---

## Why `doctest = false` under `[lib]`

Doctests are disabled in the default test run (`cargo test`) for a specific performance
reason: every doctest that touches `generic_const_exprs` or `adt_const_params` types
becomes a separate nightly `rustc` invocation. With 200+ public functions and types
that touch these features, `cargo test` would take 4+ minutes — unacceptable for a
daily development loop.

Doctests are still:

- Rendered by `cargo doc` as usage examples (their primary purpose).
- Runnable explicitly with `cargo test --doc --all-features` as the documentation
  audit gate.

The split is intentional: doctests *teach usage*, trybuild fixtures *prove law*.
Run them on separate cadences.

---

## Why sealed traits for witnesses

The `Witness` trait (in `src/witness.rs`) is public but not sealed. However, the
`EvidenceState` trait (in `src/state.rs`) *is* sealed via a private module.

**For `EvidenceState`:** sealing is mandatory. The lifecycle contract requires that
only the seven canonical stage tokens (`Raw`, `Parsed`, `Admitted`, `Refused`,
`Projected`, `Exportable`, `Receipted`) are valid `State` parameters for
`Evidence<T, State, W>`. If a downstream crate could invent an eighth stage, it could
bypass the admission gate by constructing `Evidence<T, MyAdmitted, Ocel20>` directly.
The sealed trait closes this loophole at compile time.

**For `Witness`:** sealing is not applied because the witness system is designed to be
extended. An adopter might define their own paper witness (e.g. `MyCustomAlgorithm`)
to tag admissions against a proprietary standard. The witness only carries metadata
constants (`KEY`, `TITLE`, `YEAR`, `FAMILY`); it does not perform checking. There is
no security risk from an open witness trait.

The `TreeProjectable` sealed trait in `powl.rs` uses the same logic as `EvidenceState`:
it gates which node types can enter a POWL tree projection. The seal prevents
external node types from bypassing the projection contract.

---

## Why `PhantomData` for state tokens

The state tokens and witness markers are **empty enums** (`pub enum Raw {}`). They are
uninhabited: you cannot construct a value of type `Raw` or `Ocel20`. They exist only
as type arguments to `PhantomData<State>` and `PhantomData<W>` inside `Evidence`.

The consequences of this design:

- **Zero runtime cost.** `PhantomData<T>` is a zero-sized type. An `Evidence<u32, Raw,
  Ocel20>` has the same memory layout as `u32`. The lifecycle and authority information
  exists only in the type system, not in memory.

- **Illegal states are unrepresentable.** Because `Raw` is uninhabited, you cannot
  accidentally construct a value that claims to be `Raw` when it is not. The type is
  purely a compile-time label.

- **Distinct types prevent confusion.** `Evidence<T, Raw, Ocel20>` and
  `Evidence<T, Admitted, Ocel20>` have different `State` type parameters, so they are
  incompatible types. A function requiring admitted evidence cannot accidentally receive
  raw evidence — the type mismatch is a compile error.

The alternative — using runtime enums or boolean flags for lifecycle stage — would
allow accidental mixing of stages at runtime, require match arms for every operation,
and add memory overhead. `PhantomData` markers give all the safety at zero cost.

---

## Why `pub(crate)` on `Evidence::sealed`

The `Admitted` constructor on `Evidence` is `pub(crate)`:

```rust
pub(crate) fn sealed(value: T) -> Evidence<T, Admitted, W> { ... }
```

This is the enforcement mechanism for the one-way door. If `sealed` were public, any
external crate could construct `Evidence<T, Admitted, W>` directly, bypassing the
`Admit` trait entirely. Making it `pub(crate)` means the only code that can call it
is inside this crate — and inside this crate, it is only called from
`Admission::into_evidence()`, which is only reachable from an `Admit::admit` return
value. The chain is short and auditable.

The same applies to `Evidence::refused` for the `Refused` stage.

---

## Why refusals must carry named reasons

The `Refusal<R, W>` type carries a specific named reason `R`. A bare `InvalidInput`
string is explicitly forbidden as a reason type. This is not mere style: named refusal
reasons make the boundary *auditable*.

When a downstream tool receives a `Refusal<DanglingEventObjectLink, Ocel20>`, it knows:

- Exactly which law was violated (`DanglingEventObjectLink`).
- Exactly which authority was being checked against (`Ocel20`).

With a string-typed `Refusal<String, Ocel20>`, the caller has to parse the string to
recover the law, which is fragile and not machine-actionable.

Named reasons also prevent lazy "catch-all" error handling. If you have to name the
reason, you have to think about what the reason actually is.

---

## Why `LossPolicy` is decided before projection

The `Project` trait requires the caller to pass a `LossPolicy` before the projection
runs. This ensures the decision "what happens if this projection is lossy" is made
explicitly, not discovered after the fact.

The alternative — projecting first, then deciding — creates a window where loss can
occur before the caller has committed to how it will be handled. Requiring the policy
upfront means:

1. Under `RefuseLoss`, no bytes are wasted computing a result that will be refused.
2. Under `AllowLossWithReport`, the report is produced alongside the result in a single
   pass.
3. The caller cannot accidentally ignore loss by forgetting to check a return value.

---

## Why `ReceiptShape` does not compute digests

`ReceiptShape`, `ReceiptEnvelope`, and `ReceiptChain` carry `Digest` and `ReplayHint`
values but never compute them. This is consistent with the structure-only contract:
computing a digest requires hashing, which is an execution concern.

In practice, the `wasm4pm` engine mints receipts by:
1. Computing the digest of the admitted evidence.
2. Generating the replay hint (a plan reference or content address).
3. Constructing a `ReceiptEnvelope` with those values.
4. Returning the envelope as the provenance record.

The compat layer only cares about *whether the form is present* (did the envelope
arrive with a non-empty digest, a non-empty witness, a non-empty replay hint?). That
structural check is what `is_well_shaped()` provides.
