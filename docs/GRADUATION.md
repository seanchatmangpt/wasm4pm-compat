# Graduation

> **Compat carries the evidence. `wasm4pm` adjudicates it.**

Graduation lives behind the `wasm4pm` feature (`src/graduation.rs`). It is the
**only** public bridge from this structure-only crate toward the full `wasm4pm`
execution engine — and it implements **none** of that engine.

---

## The division of labour

`wasm4pm-compat` knows the *shapes* of process evidence and the *laws* of admission,
refusal, and lossy projection. It executes nothing. When a host needs something
*computed* rather than *shaped*, it has reached the edge of compat's mandate. That
is not a failure — it is the design. The edge has a door, and this is it.

| Compat (structure) | `wasm4pm` (execution) |
|---|---|
| admits / refuses formats | runs discovery |
| shapes conformance verdicts | computes fitness / precision / alignments |
| shapes receipt evidence | mints & chains receipts |
| declares OCPQ queries | executes object-centric queries |
| claims round-trips | gates results behind benchmarks |
| **produces graduation candidates** | **consumes & adjudicates them** |

---

## Trigger signs

A `GraduationReason` names *why* a value must leave the compat layer. Each is a
capability structure cannot provide:

| Reason | Hard signal? | Meaning |
|---|:---:|---|
| `NeedsDiscovery` | ✔ | a model must be *discovered* from a log |
| `NeedsConformanceExecution` | ✔ | a conformance result must be *computed*, not claimed |
| `NeedsReplay` | ✔ | a log must be *replayed* against a model |
| `NeedsReceipts` | | provenance receipts must be *minted & chained* |
| `NeedsBenchmarkGate` | | a benchmark gate must be *run* to admit a result |
| `NeedsObjectCentricQueryExecution` | ✔ | an OCPQ query must be *executed* |
| `RebuildingProcessMiningLocally` | ✔ | the host is *re-implementing* process mining — adopt the engine instead |

A **hard signal** means the host is already past compat's mandate (it is executing,
or re-implementing, process mining). `RebuildingProcessMiningLocally` is the
loudest: if you find yourself rewriting an aligner or a discovery algorithm inside
a structure-only crate, stop and graduate.

---

## How `GraduationCandidate` works

A candidate is a typed, reviewable *case* for graduation:

```text
GraduationCandidate {
    reason:       GraduationReason,  // why
    subject:      String,            // what is graduating
    evidence_ref: String,            // opaque handle to the grounding compat evidence
}
```

It is **structure only**. Producing a candidate changes nothing and performs no
graduation — it makes the *case*. An ungrounded candidate (empty `subject` or
`evidence_ref`, i.e. `!is_grounded()`) is not reviewable and the engine intake
should reject it.

```rust
use wasm4pm_compat::graduation::{GraduationCandidate, GraduationReason};

let candidate = GraduationCandidate::new(
    GraduationReason::NeedsDiscovery,
    "p2p OCEL log",
    "blake3:deadbeef",
);
assert!(candidate.is_grounded());
assert!(candidate.reason.is_hard_signal());
```

---

## The bridge trait

`GraduateToWasm4pm` is the single seam. A host (or the engine's own intake adapter)
implements it on the compat values that may need to graduate:

```rust
use wasm4pm_compat::graduation::{GraduateToWasm4pm, GraduationCandidate, GraduationReason};

struct PendingDiscovery { log_hash: String }

impl GraduateToWasm4pm for PendingDiscovery {
    fn candidate(&self) -> GraduationCandidate {
        GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            "pending discovery",
            self.log_hash.clone(),
        )
    }
}
```

The trait deliberately implements nothing of `wasm4pm`. Compat **produces**
candidates; the engine **consumes** them. The boundary is made explicit, not
crossed.

---

## What graduation is NOT

- **Not** a dependency on `wasm4pm`. There is no engine import in this crate.
- **Not** automatic. A candidate is the *argument for* graduation, reviewed by a
  human or the engine's intake — never a silent escalation.
- **Not** a way to keep doing process mining inside compat. The correct response to
  a hard signal is to *leave*, carrying your evidence with you.

> When structure cannot answer, graduate — and bring your evidence.
