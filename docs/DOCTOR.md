# Compatibility Doctor

The compatibility doctor is the single operator surface for answering:

- what this build can admit now;
- which Cargo feature opens a blocked structural route;
- which request must graduate to `wasm4pm`;
- which claim requires the exact-tree external verifier;
- what smallest reversible repair closes each blocked edge.

It is intentionally **not** an engine. It performs no discovery, conformance,
replay, optimization, repository mutation, network access, release operation, or
standing promotion.

## Fast path

```bash
cargo run --all-features --bin wasm4pm-compat -- doctor vision2030
cargo run --all-features --bin wasm4pm-compat -- doctor vision2030 --json
```

A successful report has `PARTIAL_ALIVE` standing. That means the bounded doctor
checks passed and every out-of-bound capability has a lawful route. It does not
mean the repository is `ALIVE`; only the exact-tree external verifier may award
that state.

## Profiles

| Profile | Required compat-side capabilities |
|---|---|
| `core` | typestate evidence, named refusals, diagnostics |
| `boundary` | core + receipt shapes + deterministic digests + DfCM + doctor |
| `interop` | boundary + `formats` |
| `graduation` | boundary + `wasm4pm` graduation bridge |
| `vision2030` | all compat capabilities + strict boundary + lawful engine and verifier routes |

The `vision2030` profile treats execution and crown standing as **routed**, not
missing. Correct ownership is part of the capability, not a deficiency.

## Capability negotiation

```bash
cargo run --all-features --bin wasm4pm-compat -- capabilities
cargo run --all-features --bin wasm4pm-compat -- capabilities --json
```

Each capability reports one of three states:

- `available`: present in the current compat build;
- `blocked`: owned by compat but its Cargo feature is disabled;
- `routed`: intentionally owned by `wasm4pm` or the external verifier.

## Route planning

```bash
cargo run --all-features --bin wasm4pm-compat -- \
  plan admit project discover replay verify-standing
```

The planner preserves the boundary:

```text
admit / project / export / receipt / diagnose -> wasm4pm-compat
discover / conformance / replay / optimize    -> wasm4pm
verify-standing                               -> external verifier
```

When a required feature is disabled, the plan returns `BLOCKED` and emits one
reversible repair. It never silently substitutes another route.

## Diagnostic catalog

```bash
cargo run --bin wasm4pm-compat -- diagnostics
cargo run --bin wasm4pm-compat -- explain W4PM_COMPAT_005
cargo run --bin wasm4pm-compat -- explain HiddenFlattening --json
```

Every diagnostic has a stable machine code, Rust variant name, severity, exact
accusation, and minimal lawful repair.

## Replay identity

Doctor reports and route plans expose a deterministic BLAKE3 fingerprint over
canonical JSON. The fingerprint is a content identity for replay comparison; it
is not a signature and does not confer provenance authority.

## Exit codes

| Code | Meaning |
|---:|---|
| `0` | `PARTIAL_ALIVE` — bounded checks passed |
| `1` | `UNKNOWN` — no checks or intents selected |
| `2` | `BLOCKED` — reversible prerequisite absent |
| `3` | `BUILD_BROKEN` — reserved for compile/test adapters |
| `4` | `UNSUPPORTED` — doctor does not model the requested surface |
| `64` | CLI usage error |
