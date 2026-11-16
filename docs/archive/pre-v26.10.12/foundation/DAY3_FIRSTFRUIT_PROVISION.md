# Day 3 First-Fruit Provision — Seed-Bearing Form After Kind

> *"Let the earth yield grass, herbs yielding seeds, and fruit trees bearing fruit after their kind, with their seeds in it." And the earth yielded… after their kind.*
>
> Genesis Day 3 does not end "God specified that the earth could yield later." **The earth yielded.** This document is the yield — the first seed-bearing provision form, brought forth after kind, carrying its own seed.

---

## What this is (and is not)

**Is:** A first-class, committed artifact recording the *complete after-kind provision form* for the smallest surface in the system — the witness-marker provision. It carries its own seed: everything needed to reproduce the provision is recorded here. "Seed in itself" means a reader could re-derive the exact output from this record alone.

**Is not:** The rendered `.rs`. This document does not modify Rust, run `ggen sync`, render `src/witnesses.rs`, emit a v2 receipt, or replay. Producing the live source, binding the use-site, judging it, receipting it, and replaying it is **Day 4 cultivation** — the stewards tending what Day 3 brought forth.

The distinction:
```
Day 3:  ggen yields the first seed-bearing form after kind   (this document)
Day 4:  stewards cultivate the seed into live source,
        use-sites, judgment, receipt, and replay
```

---

## The provision chain (the form being brought forth)

```
declared law → provision seed → named output class → expected use-site → expected receipt/replay chain
```

---

## The Seed (witness-marker first-fruit)

The witness-marker provision is the first-fruit candidate because it is the smallest surface that traverses the entire chain `Substrate → Pack → ConsumerSurface → Evidence → Receipt → Replay`. Its complete after-kind seed:

| Seed element | Value |
|---|---|
| **Kind** κ | `RenderedSource` — a consumer instantiation of the substrate witness law. Repair clears σ; κ stays RenderedSource throughout. |
| **Ontology input** | `wasm4pm-compat/ggen/ontology/wasm4pm-compat.ttl` — the `compat:WitnessMarker` instances (37 declared) with `rustType`, `witnessKey`, `witnessTitle`, `witnessYear`, `witnessFamily`, `paperReference`, `description`. |
| **Query** | `wasm4pm-compat/ggen/queries/extract-witnesses-full.rq` — the 7-variable SELECT projecting each WitnessMarker. |
| **Template** | `wasm4pm-compat/ggen/templates/witness-marker.tera` — renders idiomatic `witness_marker!()` calls, indistinguishable from hand-written witness declarations. |
| **Manifest rule** | the `witness-markers` rule (single authority — no competing manifest). |
| **Output class** | first-class source. Path: `src/witnesses.rs` (the consuming crate's `src/`, NOT a `generated/` subfolder; NO `DO NOT EDIT` banner). σ at yield-time records what must NOT be true of the output. |
| **Use-site expectation** | `pub mod witnesses;` declared in the consumer's `lib.rs`, plus at least one operational `use crate::witnesses::…`. An output with no use-site is `ORPHAN` (Σ_refuse). |
| **Receipt expectation** | a v2 pack-use receipt: `R_v2 = H(TTL, Q, T, manifest, output, UseSites, Tests, R_prev)`. A manifest-only (v1) receipt does NOT satisfy this. |
| **Replay seed** | `ϱ(output) = 1 ⟺ μ_pack(TTL, Q, T) = output` (bit-identical re-derivation from the receipted inputs). |

---

## Why this counts as "yield," not "specification"

A specification says: *here is what could be made later.* A seed-bearing yield says: *here is the form, after its kind, complete with the seed to reproduce it.* The difference is that this artifact is itself reproducible-from — it is a real first form that contains its reproduction recipe, exactly as a herb "yielding seed after its kind, with its seed in it." The witness-marker provision form now *exists* as a named, committed, after-kind artifact. The earth has yielded its first herb.

What remains for Day 4 is not "make the first thing" — it is *cultivate this thing into live consequence*: render the source, bind the use-site, judge it, receipt it, replay it. That is tending a garden already seeded, not creating it.

---

## Fruit After Kind (the constraint on the yield)

The yield must preserve category — *fruit after kind*, no collapse:

- The output is `RenderedSource`, and RenderedSource **is source** — not "generated code," not second-class.
- No `generated/` path component (that would be `SECOND_CLASS ∈ σ`).
- No `DO NOT EDIT` banner (that would create a false caste inside source).
- The rendered `witness_marker!()` calls must be indistinguishable from the hand-written ones in `witness.rs` — same form, after the same kind.
- No orphan: a rendered output with no use-site is not fruit; it is `ORPHAN ∈ σ`.

---

## Verdict

**`DAY3_FIRSTFRUIT_PROVISION_READY`**

The first seed-bearing provision form after kind is brought forth and recorded, carrying its complete reproduction seed. The earth has yielded its first herb. The cultivation of this seed into live source, use-site, judgment, receipt, and replay is Day 4 — the stewards tending what Day 3 seeded. It is not done by self-certification: this record does not call the future output "good." Goodness is judged downstream, by the court, on evidence.

*This is the Day 3 yield. See `DAY3_PROVISION_AND_STEWARDSHIP.md` for the provision-instrument and stewardship doctrine that frames it.*
