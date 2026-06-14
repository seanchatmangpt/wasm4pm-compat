# The Witness Lattice

> **Start with compatibility. Graduate to execution.**

`wasm4pm-compat` is a **structure-only** process-evidence standard. It knows the
full canon of process-mining shapes — events, traces, logs, OCEL, XES, BPMN,
Petri nets, WF-nets, OC-Petri-nets, POWL, process trees, Declare, OC-Declare,
OCPQ, DFG, conformance verdicts, prediction problems, and receipt-shaped
evidence — but it runs **no engines**. The witness lattice is the spine that
makes this honest: it tracks, at the type level, *which authority* a value
answers to, *where it is* in its lifecycle, and *whether it was admitted or
refused*.

This document explains the four load-bearing ideas of the spine:

1. why **witnesses** exist,
2. why the base crate **knows the canon** even though it executes nothing,
3. why **evidence states** exist,
4. why **raw evidence cannot enter admitted surfaces**, and why **refusal is
   first-class**.

---

## 1. Why witnesses exist

A *witness* (`src/witness.rs`) is an empty enum marker — `Ocel20`, `Xes1849`,
`PowlPaper`, `WfNetSoundnessPaper`, `Wasm4pmBridge`, … — that names the
**standard, paper, API grammar, Rust law, or internal bridge** a value is being
judged against. Each implements the `Witness` trait, carrying only metadata:

```text
Witness::KEY     // "ocel-2.0"
Witness::FAMILY  // WitnessFamily::Standard
Witness::TITLE   // "OCEL 2.0"
Witness::YEAR    // Some(2023)
```

Witnesses are threaded as `PhantomData<W>` through `Evidence`, `Admission`, and
`Refusal`. The point is **non-confusion**: an `Admission<T, Ocel20>` is a
*different type* from an `Admission<T, Xes1849>`. You cannot accidentally feed
an OCEL-admitted value to a surface that expects an XES-admitted one. The
authority travels with the value, for free, and the compiler enforces it.

A witness names an authority. It **does not check** that authority. Checking is
an engine concern — it belongs in `wasm4pm`. Here, the witness is a label that
says "this is the law we are about to hold this value to," and the *holding* is
either a structural admission (this crate) or a real verification (after
graduation).

The five families form the lattice's top stratum:

| Family            | Names…                                                  | Example markers                                   |
|-------------------|---------------------------------------------------------|---------------------------------------------------|
| `Standard`        | published interchange/data standards                    | `Ocel20`, `Xes1849`                               |
| `Paper`           | academic models / model families                        | `PowlPaper`, `WfNetSoundnessPaper`, `OcpqPaper`   |
| `ApiGrammar`      | call grammars a consumer must speak                     | `Pm4pyApiGrammar`, `PmaxConsumerGrammar`          |
| `RustLaw`         | Rust-language laws this crate enforces structurally     | `RustTypestateLaw`                                |
| `InternalBridge`  | bridges toward graduation                               | `Wasm4pmBridge`                                   |

---

## 2. Why the base crate knows the canon

It would be tempting to gate each shape behind its own Cargo feature
(`ocel`, `xes`, `bpmn`, …). We deliberately **do not**. The public feature
surface is exactly three — `formats` (default), `strict`, `wasm4pm` — and they
control **capability stages, not canon knowledge**.

The reasoning:

- **A compatibility standard must be paper-complete to be a standard.** If
  turning on a flag could make the crate *forget* that OCEL exists, then the
  crate is not a standard — it is a configurable subset, and two builds would
  disagree about what process evidence *is*. Canon is invariant.
- **Witnesses already give us granularity.** We do not need per-format flags to
  scope behavior; we scope it by *which witness* a surface is parameterized over.
  The flags then choose *how far* a value may travel (import/export under
  `formats`, strict judgment under `strict`, graduation under `wasm4pm`), not
  *which shapes are knowable*.
- **Structure is cheap.** The canon is shape types, markers, and traits — no
  engines, no heavy dependencies. Knowing every shape costs almost nothing,
  whereas *executing* on them is exactly what `wasm4pm` is for.

So: the base crate knows OCEL, XES, BPMN, Petri/WF/OC-Petri nets, POWL, process
trees, Declare/OC-Declare, OCPQ, DFG, conformance verdicts, prediction problems,
and receipts — as **structure** — at all times. The witness lattice tells you,
per value, which of those canons is currently in force.

---

## 3. Why evidence states exist

Evidence (`src/evidence.rs`) is `Evidence<T, State, W>`: a value `T`, a
lifecycle `State`, and a witness `W`. The `State` parameter is a typestate token
from `src/state.rs`, each an empty enum:

```text
Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ { Projected | Exportable | Receipted }
  │                                   ▲
  └──────────────── refuse ───────────┴──▶ Refused   (terminal; names a broken law)
```

The states exist because **a value's trustworthiness is not a runtime flag — it
is part of its type**. `Evidence<T, Raw, W>` and `Evidence<T, Admitted, W>` are
distinct types. Code that demands admitted evidence simply cannot be called with
raw evidence; the mistake is a compile error, not a production incident. The
states are zero-cost: they are `PhantomData` tags, erased at runtime.

Each state marks a *position in the boundary protocol*, never a computation:

- `Raw` — untrusted input just off an external format.
- `Parsed` — well-formed shape, not yet judged.
- `Admitted` — passed a named admission law.
- `Refused` — terminal; carries the specific law that was broken.
- `Projected` — produced by a *named, accounted* lossy projection.
- `Exportable` — cleared to leave the crate.
- `Receipted` — sealed in a provenance-bearing receipt shape.

---

## 4. Why raw evidence cannot enter admitted surfaces — and why refusal is first-class

This is the keystone law of the lattice.

**There is no public `Raw → Admitted` conversion.** `Evidence::raw(..)` is the
only freely-available constructor of evidence. `Admitted` evidence can be minted
**only** by an `Admit` impl, via `Admission::into_evidence()`, whose backing
constructor (`Evidence::sealed`) is crate-private. This is the *one-way door*:
untrusted input must pass through a named boundary judgment before any surface
that expects admitted evidence will accept it.

Why enforce it structurally rather than by convention?

- **Laundering is the failure mode we exist to prevent.** A migrated
  that lets a raw external value masquerade as admitted is worse than no layer —
  it provides false assurance. The diagnostic
  `CompatDiagnostic::RawEvidenceExportedAsAdmitted` names exactly this sin.
- **The boundary is the product.** Everything else (shapes, ids, projections)
  serves the boundary. Making the boundary unbypassable is the whole point.

And the other half of the keystone: **refusal is first-class**. When an `Admit`
impl declines, it does not return a bare `Err("invalid input")`. It returns a
`Refusal<R, W>` whose `R` is a **specific named law** — `DanglingEventObjectLink`,
`MissingFinalMarking`, `UnsoundWfNet`, `FlatteningLoss`. A refusal is an
auditable outcome, not an error string. This matters because:

- A named refusal is *actionable*: the caller knows precisely which law was
  broken and what to fix.
- A named refusal is *honest*: it documents the boundary's real obligations,
  which a catch-all `InvalidInput` hides.
- A named refusal is *testable*: `tests/admission_refusal.rs` asserts the exact
  reason, so the boundary's contract cannot silently drift.

The companion `LossPolicy` / `LossReport` / `Project` law (see
[`LOSS_POLICY.md`](LOSS_POLICY.md)) extends the same discipline to *lossy*
transformations: nothing is dropped in secret; every flattening is named,
policied, and reported, with a refusal path under `RefuseLoss`.

---

## When to leave the lattice

The lattice is a *compatibility* spine. When a value needs to be **verified**
against its witness — actually replayed, actually checked for WF-net soundness,
actually conformance-scored — that is the signal to **graduate to `wasm4pm`**.
The `Wasm4pmBridge` witness and the `MigrationRecommended` diagnostic mark that
hand-off. The witness travels with the value into the engine; what was a label
here becomes a checked obligation there.

Structure first. Execution after. The lattice keeps the boundary honest until
then.

**See also:** [`WITNESS_IMMUTABILITY.md`](WITNESS_IMMUTABILITY.md) — once admitted under a witness `W`, that `W` is fixed in the type for the value's lifetime; there is no re-admission to a different authority.
