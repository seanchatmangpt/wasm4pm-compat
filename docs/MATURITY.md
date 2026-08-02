# Maturity

`wasm4pm-compat` is deliberately **narrow**. It is a structure-only standard,
not a process-mining engine. Maturity here means that the compatibility court is
complete, diagnosable, and easy to adopt while execution remains in `wasm4pm`.

## Maturity stages

| Stage | Meaning | Status |
|---|---|:---:|
| **Skeleton** | crate root, feature model, module graph, docs scaffolding | complete |
| **Structural** | canon shapes exist as typed values with witnesses | active |
| **Boundary** | admission/refusal/loss laws are exercised | active |
| **Interop** | import/export contracts + round-trip claims under `formats` | active |
| **Graduation** | bridge traits hand evidence to `wasm4pm` under `wasm4pm` | active |
| **Operator/DX** | deterministic doctor, capability negotiation, repair plans, JSON identities | partial |
| **Stable** | semver-stable surface and full release matrix | todo |

## Operator/DX standing

The compatibility doctor provides a deterministic view of the current feature
closure, lawful owner for every capability, minimal reversible repairs, and
BLAKE3 identities for replay comparison.

A clean doctor report is `PARTIAL_ALIVE`, not `ALIVE`. Exact-tree crown standing
remains the responsibility of the external verifier.

```bash
cargo run --all-features --bin wasm4pm-compat -- doctor vision2030
```

See [DOCTOR.md](DOCTOR.md).

## What "mature" means here

Maturity is measured by structural completeness, law coverage, lawful routing,
and operator clarity—not by runtime capability. A mature `wasm4pm-compat` still
runs no discovery, conformance, replay, alignment, or optimization.

## When to graduate

Adopt `wasm4pm-compat` to speak the canon and guard boundaries. The moment a
request needs active execution, use the doctor route plan and graduate through
the `wasm4pm` feature. The compat crate stays structure-only by design.
