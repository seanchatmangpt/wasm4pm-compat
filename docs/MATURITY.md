# Maturity

`wasm4pm-compat` is deliberately **early and narrow**. It is a *structure-only*
standard, not a working process-mining engine. Knowing where it sits on the
maturity curve keeps expectations honest.

## Maturity stages

| Stage          | Meaning                                                        | Status |
|----------------|----------------------------------------------------------------|:------:|
| **Skeleton**   | crate root, feature model, module graph, docs scaffolding      | now    |
| **Structural** | every canon shape exists as a typed value with witnesses       | wip    |
| **Boundary**   | admission/refusal/loss laws are complete and exercised         | wip    |
| **Interop**    | import/export contracts + round-trip claims under `formats`    | wip    |
| **Graduation** | bridge traits hand evidence to `wasm4pm` under `wasm4pm`       | wip    |
| **Stable**     | semver-stable surface, documented MSRV, full test matrix       | todo   |

## What "mature" means here

Maturity is measured by **structural completeness and law coverage**, not by
runtime capability. A "mature" `wasm4pm-compat` still runs **no** discovery,
conformance, replay, alignment, or optimization. Those belong to `wasm4pm`.

## When to graduate

Adopt `wasm4pm-compat` to *speak the canon* and *guard your boundaries*. The
moment you need to **execute** — discover a model, check conformance, replay a
log — enable the `wasm4pm` feature and graduate. The compat crate stays
structure-only by design.
