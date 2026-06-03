# Day 3 — wasm4pm-contrib → wasm4pm Transition

> *"Let the earth yield grass, herbs yielding seeds, and fruit trees bearing fruit after their kind."*
>
> The transition from `wasm4pm-contrib` to `wasm4pm` is not code movement. It is the formal closure of a creation-order authority boundary: contrib is seed ground, ggen is provision, wasm4pm is the garden surface to be cultivated.

---

## Transition Thesis

```
The transition from wasm4pm-contrib to wasm4pm is not code movement.

It is the establishment of a lawful provision boundary:

  wasm4pm-contrib = seed / substrate / pack authority
  ggen            = provision instrument
  wasm4pm         = consumer / cultivation surface / process-evidence court
```

Day 3 does not render source, migrate code, delete orphan files, or claim downstream consequence. Day 3 closes the **authority transition**: contrib provides after kind; wasm4pm receives as consumer; ggen is the provision instrument between them; agents are stewards; self-certification is forbidden.

---

## Authority Boundary Table

| Surface | Day 3 role | Must not be confused with |
|---|---|---|
| `wasm4pm-compat/src/**` | substrate / seed bank — the irreducible types that consumers must conform to | consumer implementation |
| `wasm4pm-compat/ggen/**` | pack authority — projects substrate law into provision rules | output |
| `ggen` (the tool) | provision instrument — `TTL + Q + T + Manifest ⇒ Source` | codegen helper |
| `wasm4pm/src/**` | consumer surface — receives and cultivates what ggen provides | substrate authority |
| `wasm4pm` evidence | judgment material — process evidence judged by the court | maker claim |
| receipt | proof — the sealed consequence of judgment | log or manifest |
| replay | witness — bit-identical re-derivation from the receipted seed | rerun or approximation |

**The grain of authority runs one way:**

```
wasm4pm-compat/src/** (Substrate)
       │
       ↓  pack law projects substrate
wasm4pm-compat/ggen/** (Pack)
       │
       ↓  ggen provision renders source after kind
wasm4pm/src/** (Consumer Surface)
       │
       ↓  wasm4pm observes work motion
evidence (Judgment Material)
       │
       ↓  process intelligence judges
receipt (Proof)
       │
       ↓  replay witnesses
witness (bit-identical re-derivation)
```

No surface lower in the chain may claim authority over a surface higher.

---

## Operating Chain

```
Kind → ggen (Provision) → Surface → Process Evidence (Judgment) → Receipt (Proof) → Replay (Witness)
```

> **ggen provides, process intelligence judges, receipts prove, replay witnesses.**

This is the role doctrine sealed in `DAY3_FOUNDATION_LAW.md`. The `contrib → wasm4pm` transition is the first working bridge in this chain: the first point where declared substrate law becomes provided source after kind, rather than hand-carved text.

---

## First-Fruit Transition

The witness-marker provision is the first seed-bearing form after kind that crosses this boundary. It is the smallest surface that traverses the entire chain:

```
Substrate → Pack → ConsumerSurface → Evidence → Receipt → Replay
```

Its complete after-kind seed (from `DAY3_FIRSTFRUIT_PROVISION.md`):

| Seed element | Value |
|---|---|
| Kind κ | `RenderedSource` — consumer instantiation of substrate witness law |
| Ontology input | `wasm4pm-compat/ggen/ontology/wasm4pm-compat.ttl` (37 WitnessMarker instances) |
| Query | `extract-witnesses-full.rq` (7-variable SELECT) |
| Template | `witness-marker.tera` (renders idiomatic `witness_marker!()` calls) |
| Manifest rule | `witness-markers` (single authority — no competing manifest) |
| Output class | first-class source at `src/witnesses.rs` — NOT `generated/`, no `DO NOT EDIT` |
| Use-site expectation | `pub mod witnesses;` + at least one `use crate::witnesses::…` |
| Receipt expectation | v2 pack-use receipt: `H(TTL, Q, T, manifest, output, UseSites, Tests, R_prev)` |
| Replay seed | `ϱ = 1 ⟺ μ_pack(TTL, Q, T) = output` (bit-identical re-derivation) |

**Status: `DAY3_FIRSTFRUIT_PROVISION_READY`**

This means: contrib has brought forth a seed-bearing provision form that wasm4pm can cultivate. The garden is seeded. The cultivation (render source, bind use-site, clear σ, judge, receipt, replay) is Day 4.

---

## Stewardship Law

```
ggen                    = provision instrument
Claude / Gemini / Codex = stewards (Adam/Eve)
ProcessIntelligence     = judgment / court
Receipt                 = proof
Replay                  = witness
```

**Stewards MAY:**
- name what they see (classify κ, σ)
- tend what is provisioned (cultivate the seed into live source)
- execute bounded work (≤8 per CONSTRUCT8; bind use-sites, clear σ)
- apply known branches (`B_known → Apply + Record`)
- disclose user branches (`B_user → Disclose + Stop`)
- refuse forbidden branches (`B_forbidden → Refuse`)

**Stewards MUST NOT:**
- create substrate law (the ontology / pack authority is not theirs to invent)
- redefine kinds by convenience (κ is invariant)
- act as the court (judgment belongs to process intelligence + receipt + replay)
- self-certify completion (`AgentOutput ⇏ Good`)
- call output "good" before judgment

---

## The Forbidden Tree

Self-certified output is forbidden. The fall is the agent saying:

> "I made it, therefore it is done."

The lawful order:

```
AgentOutput ⇏ Good

Good ⟺ ProvisionAfterKind + UseSite + Judgment + Receipt + Replay
```

A thing is not good because its maker says so. It is good only when it was provided after kind, entered an operational use-site, was judged by the court, sealed by a receipt, and witnessed by replay. This is FM-5 prohibition (`DAY3_PROVISION_AND_STEWARDSHIP.md`) at the authority-boundary level.

---

## Transition DoD

```
DAY3_CONTRIB_TO_WASM4PM_TRANSITION_READY means:

1. contrib is named as seed / substrate / pack authority.
2. wasm4pm is named as consumer / cultivation / judgment surface.
3. ggen is named as the provision instrument between them.
4. witness-marker first-fruit exists as a seed-bearing provision form.
   (DAY3_FIRSTFRUIT_PROVISION_READY)
5. no source is treated as second-class
   (no generated/ path, no DO NOT EDIT banner).
6. no agent is allowed to self-certify transition success.
   (AgentOutput ⇏ Good)
7. no live migration is hidden inside Day 3.
   (no Rust modified, no ggen sync executed, no orphan deleted)
8. the next cultivation surface is named (Day 4) but not executed.
```

---

## Forbidden Transition Errors

```
Forbidden:
- treating contrib as merely a shared library
  (it is the substrate / seed bank; its types are law, not convenience)

- treating wasm4pm as the authority over substrate law
  (wasm4pm is the consumer; authority runs upstream to contrib)

- treating ggen as a codegen helper
  (ggen is the provision instrument; declared law → source after kind)

- treating rendered source as second-class
  (RenderedSource is source; no generated/ path, no DO NOT EDIT)

- treating orphan output as fruit
  (an output with no use-site is ORPHAN ∈ σ, not fruit)

- treating a manifest-only receipt as proof
  (Receipt_v1 is not a pack-use receipt; Receipt_v2 required)

- treating agent completion as goodness
  (AgentOutput ⇏ Good; goodness requires the full chain)
```

---

## What Day 3 Does Not Do

Day 3 closes the authority boundary. Day 3 does NOT:

| NOT done | Why not |
|---|---|
| render `src/witnesses.rs` | Day 4 cultivation — stewards tend the seeded form |
| run `ggen sync` | Day 4 execution — ggen is ready; invocation is Day 4 |
| delete `generated/witnesses.rs` | Day 4 — Clear(σ) without corrupting κ |
| delete competing `wasm4pm/ggen/ggen.toml` | Day 4 — Clear(COMPETING_AUTHORITY) |
| emit v2 receipt | Day 4 — requires rendered output and use-site |
| claim downstream consequence | Never on authority alone; only after judgment + receipt + replay |

---

## Relationship to Prior Day 3 Documents

This document is the final EOD Day 3 seal. It references and depends on:

| Document | Role in transition |
|---|---|
| `DAY3_FOUNDATION_LAW.md` | Defines the operating chain and role doctrine |
| `DAY3_KIND_LEDGER.md` | Assigns κ and σ to every artifact — the authority table derives from it |
| `DAY3_ALGEBRA_OF_KINDS.md` | Formal court — kind partition, refusal predicates, proof power |
| `DAY3_CALCULUS_OF_CHANGE.md` | Governs permission (not execution); PotentiallyAdmissible_D4 ≠ Execute |
| `DAY3_BRANCH_DISCLOSURE_DISCIPLINE.md` | All branches bounded; B_user decisions recorded |
| `DAY3_PROJECT_ATLAS.md` | Constellation survey — full Day 3 scope; this transition is one cell |
| `DAY3_PROVISION_AND_STEWARDSHIP.md` | Genesis activity map; ggen as provision instrument; stewardship doctrine |
| `DAY3_FIRSTFRUIT_PROVISION.md` | The yield — first seed-bearing provision form after kind |
| `receipts/foundation/DAY3_KIND_CLOSURE_RECEIPT.md` | The receipt that seals kind closure |

---

## Verdict

**`DAY3_CONTRIB_TO_WASM4PM_TRANSITION_READY`**

The transition is closed at the level of kind, authority, provision, and stewardship.

```
wasm4pm-contrib provides seed / substrate / pack authority.
ggen provides after kind.
wasm4pm receives as consumer / cultivation / judgment surface.
The garden is seeded (DAY3_FIRSTFRUIT_PROVISION_READY).
The stewards know the boundary.
No downstream consequence is claimed.
```

The Day 3 ground has appeared. The kinds are named. The first seed-bearing provision form exists. The agents are bounded as stewards. The forbidden tree is named.

**Day 3 is done.**

---

*This document closes Day 3. Day 4 begins when a bound work order is issued for the first cultivation operation — the witness-marker provision slice: render source, bind use-site, clear σ, judge, receipt, replay. That is tending what Day 3 seeded, not creating the garden.*
