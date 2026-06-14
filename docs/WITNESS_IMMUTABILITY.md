# Witness Immutability

> **A witness, once admitted, is fixed for the value's lifetime.** There is no
> re-admission to a different authority — only a *new* admission of *new*
> evidence.

In `wasm4pm-compat`, the witness `W` is not a mutable property of a value. When a
value is admitted under witness `W` — yielding `Evidence<T, Admitted, W>` — that
`W` is sealed into the `PhantomData<W>` channel and travels with the value for as
long as the value exists. The witness lattice (see
[`WITNESS_LATTICE.md`](WITNESS_LATTICE.md)) names *which authority* a value
answers to; this document states the companion law: **that authority cannot be
swapped out underneath it.**

---

## The principle

1. **Admission binds the witness permanently.** Once a value is admitted under
   `W`, the type is `Evidence<T, Admitted, W>` and `W` is fixed. The witness is
   not data you can reassign; it is part of the value's type identity.

2. **There is no re-admission to a different witness.** A value that would claim a
   *new* authority must be a **new admission of new evidence** under that witness —
   minted through a fresh `Admit` impl, via `Admission::into_evidence()`, whose
   backing constructor is crate-private. There is no `re_witness`, no
   `cast_witness`, no `Evidence<T, Admitted, A> → Evidence<T, Admitted, B>` door.
   The authority a value answers to is decided once, at admission, and never
   relabeled.

3. **The cross-witness path is uncompilable.** Because the witness threads at the
   type level, distinct witnesses produce distinct types:

   ```text
   Evidence<T, Admitted, Ocel20>   ≠   Evidence<T, Admitted, Xes1849>
   Admission<T, Ocel20>            ≠   Admission<T, Xes1849>
   ```

   Code that demands an `Ocel20`-admitted value simply cannot be called with an
   `Xes1849`-admitted one. There is no implicit coercion between witness families;
   the mistake is a compile error, not a runtime check that might be forgotten.
   The witness is zero-sized `PhantomData`, so this guarantee costs nothing at
   runtime — it is paid entirely in the type system.

---

## The enforcing mechanism

The law is not enforced by a runtime guard. It is a property of three modules
working together:

- **`src/witness.rs`** — defines the `Witness` trait and the empty-enum markers
  (`Ocel20`, `Xes1849`, `PowlPaper`, `WfNetSoundnessPaper`, …). Each marker is a
  *distinct type*, so `PhantomData<Ocel20>` and `PhantomData<Xes1849>` are
  non-interchangeable. This is the source of non-confusion.

- **`src/evidence.rs`** — the `Evidence<T, State, W>` carrier threads `W` as
  `PhantomData<W>`. The typestate transitions (`into_parsed`, `into_projected`,
  `into_exportable`, `into_receipted`) **preserve `W`** — none of them is
  witness-parametric in a way that lets `W` change. The witness rides the value
  through every lawful state without ever being reassigned.

- **`src/admission.rs`** — `Admission<T, W>` and the `Admit` trait are the only
  sanctioned `Raw → Admitted` path, and that path *names* its witness `W`. An
  `Admit` impl decides `W`; there is no second impl that takes an already-admitted
  value and re-emits it under a different `W`.

Because admission is the one-way door (see [`WITNESS_LATTICE.md`](WITNESS_LATTICE.md)
§4) and admission *fixes* the witness, the witness inherits the door's
unbypassability: you cannot relabel an authority without going back through a
fresh, named admission.

---

## The receipts that prove it

These are not illustrations — they are **trybuild compile-fail receipts**. Each
one is a value that *attempts* to cross a witness boundary and **must fail to
compile**. Each has a corresponding `.stderr` capturing the expected diagnostic.
Together they seal the law at the type level:

| Receipt fixture (`tests/ui/compile_fail/`) | Forbidden cross-witness move |
|---|---|
| `evidence_wrong_witness_ocel_as_xes.rs` | Use `Evidence<_, Admitted, Ocel20>` where `Xes1849` is required |
| `evidence_wrong_witness_xes_as_ocel.rs` | Use `Evidence<_, Admitted, Xes1849>` where `Ocel20` is required |
| `admission_wrong_witness_ocel_as_xes.rs` | Pass an `Admission<_, Ocel20>` to an `Xes1849` surface |
| `witness_declare_as_ocpq.rs` | Treat a `Declare`-witnessed value as `Ocpq` |
| `witness_ocel_as_powl.rs` | Treat an `Ocel20`-witnessed value as `PowlPaper` |
| `witness_xes_as_wfnet.rs` | Treat an `Xes1849`-witnessed value as `WfNetSoundnessPaper` |
| `witness_pm4py_as_pmax.rs` | Treat a `Pm4pyApiGrammar` value as `PmaxConsumerGrammar` |
| `witness_yawl_as_inductive_miner.rs` | Treat a `Yawl`-witnessed value as an inductive-miner witness |
| `receipt_admission_wrong_witness_type.rs` | Seal a receipt from an admission of the wrong witness type |
| `receipt_wrong_witness_marker.rs` | Stamp a receipt with a witness marker the evidence was not admitted under |

If any of these ever *compiled*, witness immutability would be broken — a value
would have laundered its authority. The ALIVE gate (`cargo test --test ui_tests`)
keeps them red.

---

## What this is NOT

- **Not a runtime witness check.** Nothing here inspects, at runtime, whether a
  value "really is" OCEL or XES. The law is purely a compile-time type identity;
  there is no validation step, no boolean flag, no reflection. Verifying that a
  value *substantively* satisfies its witness is an engine concern.

- **Not a conversion mechanism.** This document describes the *absence* of a
  re-witnessing path, not a way to perform one. There is deliberately no
  `Ocel20 → Xes1849` cast. The lawful way to move structure between authorities is
  the named, loss-accounted `Project` path (see [`REFUSAL_LAW.md`](REFUSAL_LAW.md)
  and `LOSS_POLICY.md`), which produces *new* evidence — it does not relabel old
  evidence in place.

- **Re-witnessing logic, if it is ever needed, graduates to `wasm4pm`.** If a
  future requirement genuinely demands transforming a value from one verified
  authority to another — actually checking the source, actually re-deriving under
  the target law — that is execution, not structure. It crosses the
  `Wasm4pmBridge` and leaves this crate. Structure first; execution after.

---

## In one sentence

> The witness is chosen once, at admission, and is fixed in the type for the
> value's lifetime; a new authority means new evidence, never a relabeled old
> value — and the compiler, not a runtime check, enforces it.
