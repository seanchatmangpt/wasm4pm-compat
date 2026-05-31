# Blue River Dam — Doctrine

> **This document is a working-backwards artifact.** It describes the completed system so every
> manufacturing cell can preserve global coherence. **It does not itself certify completion.**
> Completion is earned only by reachable code, passing gates, negative fixtures, receipts, and replay.
>
> **Backward-designed doctrine guides the agents. Forward-earned receipts judge the agents.**

---

## How to read this file

This is a **future-state control artifact**, not a task list and not a completion certificate.
It is written *as if the whole system already exists* so that anyone building one cell can see
which wall their brick belongs to. Its only job is **orientation** — preventing locally-correct work
from drifting into globally-incoherent systems.

| Artifact | Mode | Purpose |
|---|---|---|
| This doctrine (Blue River Dam) | Future-state | Shows the whole completed world |
| The 5-agent prompt | Manufacturing plan | Assigns cells of work |
| The `wasm4pm-compat` repo | Present-state artifact | Must compile, test, document, verify |
| Receipts / cargo matrix / commits | Forward-earned proof | Decide ALIVE vs PARTIAL |
| `MATURITY.md` | Navigation map | Where each artifact sits in the law stack |

**Scope of *this crate* (`wasm4pm-compat`):** Level 2 and Level 3 only — structure and strict
judgment. This crate MUST NOT smuggle in Living LSP, the branchless 8-bit kernel court, or full
`wasm4pm` execution authority. A doorway that ships a hidden engine violates its own doctrine.

---

## Central claim

> **Consequential software cannot be governed by downstream interpretation of activity records.
> It must manufacture admissible process truth upstream.**

Traditional flow: write code → deploy → observe → collect logs → explain later.

Blue River Dam reverses it: define lawful work → admit evidence → refuse weak claims →
execute through bounded cells → emit receipts → replay consequence → let downstream consume proof.

Downstream data is **sediment** — residue left by work. By the time logs, dashboards, and analytics
appear, the governing act already happened. **The dam must be upstream.** Whoever controls admissible
process truth controls the downstream data, audit, governance, automation, and intelligence below it.

---

## What is admitted vs. what is recorded

- An **activity record** says something happened.
- **Process evidence** says what happened *under a witness*, with typed identity, lawful boundary,
  admissible structure, possible refusal, and replayable receipt.
- A **log** is not truth. A **receipt** is a proof record.

---

## The law stack

```
law of the chip
  → branchless execution
    → 8-bit bounded state (Need9 = split)
      → typed admission / refusal
        → external-witness mapping
          → GALL growth
            → Living LSP author-time observation
              → receipts / replay
                → adversarial benchmark judgment
```

Features are secondary. The **law stack** defines what may be admitted, executed, claimed, and closed.

---

## The family (one control point: upstream admission)

| Artifact | Role | Status |
|---|---|---|
| `wasm4pm` | Branchless Rust/WASM execution authority (mining, conformance, replay, receipts, benchmark gates). **The throne room.** | exists |
| `wasm4pm-compat` | Minimal paper-complete **doorway**. 3 features: `formats`, `strict`, `wasm4pm`. Structure-only. | this crate |
| `pm-core` | Zero-cost shareable types crate (no_std + alloc, BTreeMap everywhere, paper-grounded). | sibling |
| `ggen` | CodeManufactory — papers → source shapes, doctests, negative fixtures, receipt ledgers. | future |
| Living LSP / `*-llsp` | Author-time relation-state witness; migration conscience. "Clear is an event, not absence." | future |

`wasm4pm-compat` is the doorway. `wasm4pm` is the throne room. **The doorway must not become the throne room.**

---

## Five-level maturity (summary; full 7×5 matrix in `MATURITY.md`)

| Level | Identity | One-line law |
|---:|---|---|
| 1 | No process mining | Records activity |
| 2 | `wasm4pm-compat` | Structures evidence |
| 3 | Strict compat covenant | Judges evidence claims |
| 4 | Graduation bridge | Prepares execution authority |
| 5 | Full `wasm4pm` | Adjudicates process truth |

Process maturity = the progressive removal of unresolved process uncertainty from execution.

---

## The ten contributions

1. **Blue River Dam theory** — the upstream closure layer is the control point of consequential software.
2. **`wasm4pm-compat`** — minimal by execution, complete by structure, strict by opt-in, connected to
   full `wasm4pm` only at graduation. Exactly three public features. *Feature flags are horses; do not multiply horses.*
3. **Full `wasm4pm`** — branchless process mining under external-witness discipline. The crown engine.
4. **Five-level process maturity model** across seven dimensions.
5. **Law of the chip** — every process concept must eventually face registers, masks, indexes, lookup
   tables, memory layout, instruction count, and benchmark truth. The chip accepts bounded state, not prose.
6. **Branchless as first covenant** — constitutional, not an optimization. It forces unresolved
   uncertainty to be removed before the hot path. *Performance is discipline expressed at chip speed.*
7. **8-bit bounded-state court** — 8 condition bits → 256 lawful states → branchless selection.
   **Need9 means split** (a ninth primary bit is decomposition failure, not sophistication).
8. **Receipts and replay as closure law** — logs remember motion; receipts prove consequence.
   *No receipt, no closure.* A claim that cannot replay is narration.
9. **Living LSP** — author-time relation-state observation: raise → route → pending repair →
   clear-through-lifecycle → residual preservation → receipt → replay. *Clear is an event, not absence.*
10. **Project covenant** — charters ask "what/when/metrics"; covenants ask "what are we bound to,
    what witnesses govern us, what must we refuse, what proves closure."

---

## The five movements of the argument

1. **Activity is not evidence.** Modern systems overproduce records and underproduce admissible truth.
   Separate: activity → evidence → admitted evidence → process truth → receipted closure.
2. **Evidence requires witness.** Begin from public forms (OCEL, XES, BPMN, Petri/WF/OC-Petri nets,
   POWL, process trees, Declare, OC-Declare, OCPQ, DFG, conformance verdicts, prediction shapes,
   receipts, replay). *No private foundation where public witness exists.* Witness is court record, not a leash.
3. **Witness requires admission and refusal.** A system that cannot refuse cannot govern. Good refusal
   names the violated law (`MissingObjectRelation`, `FlatteningLoss`, `DeadTransition`, `UnsoundWfNet`,
   `InvalidPowlProjection`, `MissingWitness`, `UnreplayableClaim`). Bad refusal says "invalid input."
4. **Admitted evidence must be lowered.** external witness → typed shape → admission/refusal →
   bounded state → masks/tables → branchless kernel → receipt/replay. Unresolved uncertainty disappears
   before the hot path.
5. **Execution must be receipted.** Reachable, tested, negative-tested, replayable, receipted.
   Correct-but-unreachable code is **PARTIAL**, not ALIVE. **False ALIVE is breach.** PARTIAL is honorable when scoped.

---

## The format covenant (Chapter 4)

No raw format-to-format laundering. The lawful path is:

```
recognized external format
  → typed admitted compat structure
    → recognized external format OR full wasm4pm
```

Lossy projection requires **named projection + `LossPolicy` + `LossReport` + witness + refusal path**.
OCEL→XES flattening must refuse unless explicitly projected with loss policy and loss report.
**Hidden flattening is refusal.**

---

## Evaluation — the crown standard (Chapter 15)

The crown is **whole-system**, not one benchmark: structural coverage of the canon · admission/refusal
distinctness · loss/projection honesty · reachability · replay · receipts · branchless hot path ·
adversarial benchmark judgment. A competitor cannot win by beating one benchmark, because the benchmark
is one surface of the crown, not the crown.

---

## Agents (Chapter 14)

Agents are **downstream consumers**, not truth engines. They may explain, route, compare, summarize,
assist. They may **not** adjudicate process truth. The kernel adjudicates; the agent cites deterministic
artifacts. **No artifact, no claim.**

---

## Final thesis

> Blue River Dam establishes the upstream closure layer as the control point of consequential software.
> `wasm4pm-compat` provides the minimal paper-complete doorway for Rust applications to carry admitted
> process evidence. Full `wasm4pm` provides the branchless Rust/WASM execution authority that mines,
> conforms, replays, receipts, and benchmarks that evidence. Together they convert software development
> from feature authoring into process manufacturing: externally witnessed, typed, refusal-capable,
> reachable, replayable, receipted, and adversarially judgeable.

**Discipline was never added after speed.
Discipline is why speed was possible.
Now the discipline becomes public law.**
