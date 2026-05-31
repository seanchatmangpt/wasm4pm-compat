# Nightly Type Law — Law-Packet Notes

This document records law-packet notes for papers classified as
`COVERED_BY_GRADUATION_BOUNDARY` or `PARTIAL_WITH_REASON` in
`PAPER_COVERAGE_LEDGER.md`. Each entry names what structural surface
this crate provides and what must graduate to wasm4pm.

---

## #21 — No AI Without PI! (van der Aalst, 2025)

**Paper:** No AI Without PI! Object-Centric Process Mining as the Enabler
for Generative, Predictive, and Prescriptive Artificial Intelligence
(arXiv:2508.00116)

**Canon family:** `OBJECT_CENTRIC_PETRI`

**Graduation boundary:**

This crate provides the zero-cost structural surface that grounds all three
AI forms described in the paper:

| Paper concept | This crate's surface | Graduates to wasm4pm |
|---|---|---|
| Object-centric event data (OCED) | `src/ocel.rs` — `OcelLog`, `OcelObject`, `OcelEvent` | — (structure stays here) |
| Process discovery output | `src/petri.rs`, `src/powl.rs`, `src/dfg.rs` — typed net/graph surfaces | Discovery algorithm |
| Compliance analysis | `src/conformance.rs` — `Metric<KIND, NUM, DEN>` with `Between01` bounds | Alignment/replay engine |
| Performance analysis | `src/dfg.rs` — OC-DFG structure | Frequency/time annotation engine |
| Predictive AI input | `src/prediction.rs` — `PredictionTarget` prefix structure | ML model training/inference |
| Prescriptive AI | `src/prediction.rs` + `src/conformance.rs` | Recommendation/intervention engine |

**Structural law this crate enforces:**

- OCED tuple `(E, O, eaval, oaval)` is the only valid evidence carrier for
  all three AI forms. Evidence that cannot be traced to a lawful OCED
  structure is not process intelligence — it is a defect.
- Compliance score is a `Between01<NUM, DEN>` metric, not a free float.
  A compliance "score" that escapes the unit interval is a type error.
- Predictive AI prefix is typed over `Evidence<T, Admitted, W>` — a prefix
  that has not passed admission is not a lawful prediction input.

**What must NOT live in this crate:**

- Discovery algorithms (inductive miner, split miner, etc.)
- Alignment computation or token replay
- Performance frequency/time annotation
- ML model training or inference
- Recommendation generation

These graduate to wasm4pm via the `wasm4pm` feature bridge.
