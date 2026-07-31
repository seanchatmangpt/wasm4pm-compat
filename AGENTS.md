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
O -> O* -> ggen projections -> committed artifacts -> Gall checkpoints -> external verifier -> standing receipt
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
8. A capability lane or Gall checkpoint emits evidence; it may not promote its own standing.
9. Gall checkpoints are dependency-closed and ordered. A later checkpoint cannot pass when its predecessor failed.
10. No engine logic, unrestricted planner, network actuation, registry publication, release mutation, or hidden fallback belongs in compat.
11. No hand-edited generated outputs and no workflow weakening to obtain green status.

## Standing states

- `UNKNOWN`: evidence is absent, stale, incomplete, or bound to another tree.
- `UNSUPPORTED`: the bounded verifier does not implement the requested surface.
- `BLOCKED`: a required admission, tool, permission, dependency, or observation is unavailable.
- `BUILD_BROKEN`: admitted source reached execution but compilation, tests, lint, or packaging failed.
- `PARTIAL_ALIVE`: a bounded checkpoint passed, but the full crown is incomplete.
- `ALIVE`: the exact-tree external verifier admitted every required positive, negative, checkpoint, and receipt/replay obligation.

## Gall checkpoint ladder

The canonical graph defines exactly ten incremental checkpoints:

1. `GALL-CP-001` — observation carrier admitted.
2. `GALL-CP-002` — canonical authority and standard version bound.
3. `GALL-CP-003` — committed-output routes deterministically ordered.
4. `GALL-CP-004` — standing and checkpoint projections closed.
5. `GALL-CP-005` — refusal algebra typed and unique.
6. `GALL-CP-006` — actuation confined to first-class repository source.
7. `GALL-CP-007` — negative drift witness refused.
8. `GALL-CP-008` — canonical inputs bound by BLAKE3 receipt identities.
9. `GALL-CP-009` — replay produces equivalent identities and outcomes.
10. `GALL-CP-010` — crown promotion remains external and exact-tree bound.

Each successful checkpoint has `PARTIAL_ALIVE` standing. Only the external standing lane
may combine all ten checkpoint receipts with admission, inspection, and capability
receipts to produce `ALIVE`.

## Ggen-owned surfaces

The standing-law and Gall-checkpoint projections are owned by:

- `ggen/ontology/standing-law.ttl`
- `ggen/queries/extract-standing-law.rq`
- `ggen/queries/extract-gall-checkpoints.rq`
- `ggen/templates/standing-law.rs.tera`
- `ggen/templates/gall-checkpoints.rs.tera`
- `ggen/standing.ggen.toml`
- `tests/fixtures/ggen_standing_projection.rs` (committed projection)
- `tests/fixtures/ggen_gall_checkpoints.rs` (committed projection)

Do not edit either committed projection alone. A lawful change modifies the graph and,
when needed, its query/template, then runs the sync and verifier rails.

## Verification ladder

Run the narrowest relevant verifier first, then expand:

```bash
cargo test --locked --test ggen_manufacturing_contract ten_gall_checkpoints_are_sequential_and_receipted -- --nocapture
cargo test --locked --test ggen_manufacturing_contract -- --nocapture
cargo fmt --all -- --check
cargo clippy --locked --all-features --tests -- -D warnings
cargo test --locked --all-features --tests
cargo test --locked --all-features pc_powl2
cargo test --locked --test ui_tests -- --ignored
cargo test --doc --all-features
cargo package --list
cargo publish --dry-run
```

A failed command is evidence. Classify it; do not translate it into success.

## Ggen commands

```bash
ggen sync --locked --manifest ggen/standing.ggen.toml
bash scripts/verify-ggen-contract.sh
```

The sync command manufactures both committed projections. The Rust integration test is
the independent verifier. It emits a Gall checkpoint report when
`GGEN_GALL_REPORT_PATH` is set and an exact-tree standing receipt when
`GGEN_RECEIPT_PATH` is set.

## Git and publication

- Resolve and record the exact base SHA before editing.
- Use a bounded `agent/*` branch.
- Stage only intended files.
- Commit generated projections with their source-law changes.
- Open a draft PR; never merge without explicit owner authorization.
- Do not claim `ALIVE` from review, diff inspection, or remote status text alone.
