# wasm4pm-compat Agent Contract

This file is the sole normative operating contract for repository agents.

## Mission and fence

`wasm4pm-compat` is the structure-only type foundry at the compatibility boundary. It
admits, refuses, projects, receipts, and prepares graduation candidates. It does not
execute process-mining algorithms, write into sibling repositories, publish releases,
deploy services, or mint production authority.

## Law-state workflow

```text
parse -> route -> admit/refuse -> diagnose/repair -> actuate -> receipt -> replay/hook
```

```text
O -> O* -> root ggen consumer -> local pack -> first-class source -> real consumer proof -> receipt -> replay -> external standing
```

The canonical graph is authority. `ggen.toml` is the sole active observation carrier.
The pack is the only manufacturing route. Execution receipts report what happened; they
do not rewrite source law.

## Hard invariants

1. Zero unreceipted actuation.
2. No production DO path or sibling-repository write from this crate.
3. Exactly one active ggen manifest: repository-root `ggen.toml`.
4. Active projections live in `packs/wasm4pm-compat-pack/templates` and use frontmatter.
5. Every committed-output SPARQL query has deterministic `ORDER BY` closure.
6. Every first-class output uses checksum freeze and one writer.
7. `UNKNOWN` is not admitted. `UNSUPPORTED` is not refused. `PARTIAL_ALIVE` is not crown completion.
8. Only an external exact-tree verifier may assign `ALIVE`.
9. Gall checkpoints are predecessor-closed; a later checkpoint cannot pass around an earlier failure.
10. Compile-pass and compile-fail fixtures are hand-authored type-law receipts, not pack outputs.
11. Historical reports are evidence, never current configuration or execution authority.
12. No workflow weakening, mutable-state promotion, fabricated receipt, or hidden fallback may obtain green status.

## Standing states

- `UNKNOWN`: evidence is absent, stale, incomplete, or bound to another tree.
- `UNSUPPORTED`: the bounded verifier does not implement the requested surface.
- `BLOCKED`: a required admission, tool, permission, dependency, or observation is unavailable.
- `BUILD_BROKEN`: admitted source reached execution but compilation, tests, lint, packaging, or replay failed.
- `PARTIAL_ALIVE`: a bounded checkpoint passed, but the full crown is incomplete.
- `ALIVE`: the exact-tree external verifier admitted every positive, negative, consumer, checkpoint, receipt, and replay obligation.

## Gall checkpoint ladder

1. `GALL-CP-001` — observation carrier admitted.
2. `GALL-CP-002` — canonical authority and standard version bound.
3. `GALL-CP-003` — committed-output routes deterministically ordered.
4. `GALL-CP-004` — standing and checkpoint projections closed.
5. `GALL-CP-005` — refusal algebra typed and unique.
6. `GALL-CP-006` — actuation confined to the consumer repository.
7. `GALL-CP-007` — negative drift witness refused.
8. `GALL-CP-008` — canonical inputs bound by cryptographic receipt identities.
9. `GALL-CP-009` — replay produces equivalent identities and outcomes.
10. `GALL-CP-010` — crown promotion remains external and exact-tree bound.

Each checkpoint can establish only `PARTIAL_ALIVE`. The standing lane combines checkpoint,
admission, inspection, capability, consumer, and replay receipts.

## Active ggen surfaces

- `ggen.toml`
- `packs/wasm4pm-compat-pack/pack.toml`
- `packs/wasm4pm-compat-pack/ontology.ttl`
- `packs/wasm4pm-compat-pack/templates/*.tmpl`
- `ggen/ontology/*.ttl`
- `ggen/ontology-breeds/*.ttl`
- `ggen/gates/*.rq`
- `scripts/audit-ggen-usage.py`
- `scripts/verify-ggen-contract.sh`
- `tests/ggen_manufacturing_contract.rs`
- `tests/fixtures/ggen_standing_projection.rs`
- `tests/fixtures/ggen_gall_checkpoints.rs`

Do not edit a pack-owned output alone. Change the graph or pack template, run the sync,
then commit the source-law change with its projection and receipt evidence.

## Verification ladder

```bash
python3 scripts/audit-ggen-usage.py --output target/ggen-standing/usage-audit.json
ggen graph validate
ggen doctor run
ggen sync run --dry-run
cargo test --locked --test ggen_manufacturing_contract ten_gall_checkpoints_are_sequential_and_receipted -- --nocapture
cargo test --locked --test ggen_manufacturing_contract -- --nocapture
ggen sync run
ggen receipt verify
bash scripts/verify-ggen-contract.sh
cargo fmt --all -- --check
cargo clippy --locked --all-features --tests -- -D warnings
cargo test --locked --all-features --tests
cargo test --locked --all-features pc_powl2
cargo test --locked --test ui_tests -- --ignored
cargo test --doc --all-features
cargo package --list
cargo publish --dry-run
```

A failed command is evidence. Classify it; never translate it into success.

## Historical evidence fence

`ggen/emitted`, `ggen/gap-closure-receipts`, `ggen/intel`, `checkpoints`, and `archive`
contain dated evidence. Legacy query/template assets outside the local pack are retained as
migration provenance and are not discovered by the active manifest.

## Git and publication

- Resolve the exact base SHA before editing.
- Use a bounded `agent/*` branch.
- Keep the diff inside the compat fence.
- Open a draft PR by default.
- Never merge without explicit owner authorization.
- Do not claim `ALIVE` from review text, diff inspection, or queued status alone.
