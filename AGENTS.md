# wasm4pm-compat Agent Contract

This file is the normative operating contract for repository agents.

## Mission and fence

`wasm4pm-compat` is the structure-only type foundry at the compatibility boundary.
It admits, refuses, projects, receipts, and prepares graduation candidates. It does
not execute process-mining algorithms, actuate external systems, mint production
authority, or become a smaller `wasm4pm` engine.

Preserve the existing paper-complete type law. Extend it only through bounded,
reviewable surfaces with named refusals and executable receipts.

## Law-state workflow

Every change follows:

```text
parse -> route -> admit/refuse -> diagnose/repair -> actuate -> receipt -> replay/hook
```

The manufacturing law is:

```text
O -> O* -> ggen projection -> committed artifact -> external verifier -> standing receipt
```

The canonical graph is authority. TOML manifests are admitted observation carriers.
Committed Rust, JSON, Markdown, shell, and workflow files are deterministic projections.
Execution receipts report what actually happened; they do not rewrite source law.

## Hard invariants

1. Zero unreceipted actuation.
2. No direct production DO path from this crate.
3. No handwritten drift on ggen-owned projections. Change the ontology, query, or template, then regenerate.
4. Rendered source is first-class source. Do not create a second-class `generated/` caste.
5. Every SPARQL query used for committed output must define deterministic ordering.
6. `UNKNOWN` is not admitted. `UNSUPPORTED` is not refused. `PARTIAL_ALIVE` is not crown completion.
7. Only an external verifier may assign `ALIVE`, and only for the exact commit/tree whose full conjunctive evidence passed.
8. A capability lane emits evidence; it may not promote its own standing.
9. No engine logic, unrestricted planner, network actuation, registry publication, release mutation, or hidden fallback belongs in compat.
10. No hand-edited generated outputs and no workflow weakening to obtain green status.

## Standing states

- `UNKNOWN`: evidence is absent, stale, incomplete, or bound to another tree.
- `UNSUPPORTED`: the bounded verifier does not implement the requested surface.
- `BLOCKED`: a required admission, tool, permission, dependency, or observation is unavailable.
- `BUILD_BROKEN`: admitted source reached execution but compilation, tests, lint, or packaging failed.
- `PARTIAL_ALIVE`: a bounded checkpoint passed, but the full crown is incomplete.
- `ALIVE`: the exact-tree external verifier admitted every required positive, negative, and receipt/replay obligation.

## Ggen-owned surfaces

The standing-law projection is owned by:

- `ggen/ontology/standing-law.ttl`
- `ggen/queries/extract-standing-law.rq`
- `ggen/templates/standing-law.rs.tera`
- `ggen/standing.ggen.toml`
- `tests/fixtures/ggen_standing_projection.rs` (committed projection)

Do not edit `tests/fixtures/ggen_standing_projection.rs` alone. A lawful change modifies
the graph and, when needed, the query/template, then runs the sync and verifier rails.

## Verification ladder

Run the narrowest relevant verifier first, then expand:

```bash
cargo test --test ggen_manufacturing_contract -- --nocapture
cargo fmt --all -- --check
cargo clippy --all-features --tests -- -D warnings
cargo test --all-features --tests
cargo test --all-features pc_powl2
cargo test --test ui_tests -- --ignored
cargo test --doc --all-features
cargo package --list
cargo publish --dry-run
```

A failed command is evidence. Classify it; do not translate it into success.

## Ggen commands

```bash
ggen sync --locked --manifest ggen/standing.ggen.toml
cargo test --test ggen_manufacturing_contract -- --nocapture
bash scripts/verify-ggen-contract.sh
```

The sync command manufactures the committed projection. The Rust integration test is
the independent verifier and emits a BLAKE3 receipt when `GGEN_RECEIPT_PATH` is set.

## Git and publication

- Resolve and record the exact base SHA before editing.
- Use a bounded `agent/*` branch.
- Stage only intended files.
- Commit generated projections with their source-law changes.
- Open a draft PR; never merge autonomously.
- Do not claim `ALIVE` from review, diff inspection, or remote status text alone.
