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
| **Operator/DX** | deterministic doctor, capability negotiation, repair plans, JSON identities | active |
| **Evolution** | machine-readable contract, drift detection, breaking-change classification | active |
| **Stable** | semver-stable surface and full release matrix | partial |

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

## Evolution contract

`compatibility/contract-v1.json` is the machine-readable compatibility covenant.
It freezes the high-leverage consumer surface: package/toolchain identity, the
three public features, authority boundaries, critical modules, prelude exports,
and the twelve capability owners.

The dependency-free verifier runs before Rust installation:

```bash
python3 scripts/compatibility-contract.py check \
  --root . \
  --contract compatibility/contract-v1.json \
  --output target/compatibility-contract/receipt.json
```

Proposed revisions can be classified before integration:

```bash
python3 scripts/compatibility-contract.py diff \
  compatibility/contract-v1.json \
  /path/to/proposed-contract.json
```

A clean contract check is also bounded to `PARTIAL_ALIVE`; it does not replace
native compilation, type-law fixtures, or the external exact-tree verifier.

See [INNOVATION_80_20.md](INNOVATION_80_20.md).

## What "mature" means here

Maturity is measured by structural completeness, law coverage, lawful routing,
operator clarity, and controlled evolution—not by runtime capability. A mature
`wasm4pm-compat` still runs no discovery, conformance, replay, alignment, or
optimization.

## When to graduate

Adopt `wasm4pm-compat` to speak the canon and guard boundaries. The moment a
request needs active execution, use the doctor route plan and graduate through
the `wasm4pm` feature. The compat crate stays structure-only by design.
